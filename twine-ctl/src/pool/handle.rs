// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Handle-based reservation tracking for a [`DevicePool`](super::DevicePool).

use core::fmt;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::device::{DeviceCapability, DeviceId};
use crate::TwineCtl;

use super::{DevicePoolError, PoolInner};

/// Unique identifier assigned to each [`DevicePoolHandle`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct HandleId(u64);

impl HandleId {
    /// Creates a new [`HandleId`] from a raw counter value.
    pub(super) fn new(id: u64) -> Self {
        Self(id)
    }
}

impl fmt::Display for HandleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "handle-{}", self.0)
    }
}

/// Cached reference to a reserved device, held by a [`DevicePoolHandle`].
///
/// Cloning the [`Arc`] avoids touching the pool lock when accessing the
/// reserved device.
struct ReservationEntry {
    device: Arc<Mutex<Box<dyn TwineCtl + Send>>>,
    capability: DeviceCapability,
}

/// An exclusive handle into a [`DevicePool`](super::DevicePool).
///
/// A handle tracks which devices its owner has reserved. Devices reserved
/// through a handle are inaccessible to other handles until explicitly
/// released or until this handle is dropped.
///
/// Devices themselves always remain owned by the pool. The handle caches
/// [`Arc`] references so that device access does not contend on the
/// pool-level lock — multiple handles can use their own devices in parallel.
pub struct DevicePoolHandle {
    /// Unique identity of this handle.
    id: HandleId,

    /// Back-reference to the shared pool state (used for reservation
    /// bookkeeping only).
    pool: Arc<Mutex<PoolInner>>,

    /// Cached references to devices reserved by this handle.
    reserved: HashMap<DeviceId, ReservationEntry>,
}

impl DevicePoolHandle {
    /// Creates a new handle.
    ///
    /// This is only called by [`DevicePool::handle`](super::DevicePool::handle).
    pub(super) fn new(id: HandleId, pool: Arc<Mutex<PoolInner>>) -> Self {
        Self {
            id,
            pool,
            reserved: HashMap::new(),
        }
    }

    /// Returns the unique identifier assigned to this handle.
    #[must_use]
    pub fn id(&self) -> HandleId {
        self.id
    }

    /// Reserves a specific device by its [`DeviceId`].
    ///
    /// The device remains owned by the pool but is marked as reserved by
    /// this handle until it is released or the handle is dropped.
    ///
    /// # Errors
    ///
    /// * [`DevicePoolError::DeviceNotFound`] — no device with this ID exists.
    /// * [`DevicePoolError::DeviceUnavailable`] — the device is reserved by
    ///   another handle.
    pub fn request_device(&mut self, id: &DeviceId) -> Result<(), DevicePoolError> {
        let mut inner = self.pool.lock().expect("pool lock poisoned");

        let entry = inner
            .devices
            .get(id)
            .ok_or_else(|| DevicePoolError::DeviceNotFound(id.clone()))?;

        if inner.reservations.contains_key(id) {
            return Err(DevicePoolError::DeviceUnavailable(id.clone()));
        }

        let reservation = ReservationEntry {
            device: Arc::clone(&entry.device),
            capability: entry.capability,
        };

        inner.reservations.insert(id.clone(), self.id);
        self.reserved.insert(id.clone(), reservation);
        Ok(())
    }

    /// Reserves any available device from the pool.
    ///
    /// Returns the [`DeviceId`] of the device that was reserved. The specific
    /// device chosen is not guaranteed to follow any particular order.
    ///
    /// # Errors
    ///
    /// Returns [`DevicePoolError::NoDevicesAvailable`] when the pool has no
    /// unreserved devices.
    pub fn request_any(&mut self) -> Result<DeviceId, DevicePoolError> {
        let mut inner = self.pool.lock().expect("pool lock poisoned");

        let (id, entry) = inner
            .devices
            .iter()
            .find(|(k, _)| !inner.reservations.contains_key(*k))
            .ok_or(DevicePoolError::NoDevicesAvailable)?;

        let id = id.clone();
        let reservation = ReservationEntry {
            device: Arc::clone(&entry.device),
            capability: entry.capability,
        };

        inner.reservations.insert(id.clone(), self.id);
        self.reserved.insert(id.clone(), reservation);
        Ok(id)
    }

    /// Reserves an available device that matches the requested
    /// [`DeviceCapability`].
    ///
    /// Returns the [`DeviceId`] of the device that was reserved. When multiple
    /// devices match, the specific device chosen is unspecified.
    ///
    /// # Errors
    ///
    /// Returns [`DevicePoolError::NoDeviceWithCapability`] when no unreserved
    /// device with the requested capability is in the pool.
    pub fn request_by_capability(
        &mut self,
        capability: DeviceCapability,
    ) -> Result<DeviceId, DevicePoolError> {
        self.request_by_capability_excluding(capability, &[])
    }

    /// Reserves an available device that matches the requested
    /// [`DeviceCapability`], skipping any devices whose [`DeviceId`]
    /// appears in `exclude`.
    ///
    /// This is useful when a caller needs a device of a given capability but
    /// wants to avoid specific devices (for example, devices that have
    /// already been assigned a particular role in a test topology).
    ///
    /// Returns the [`DeviceId`] of the device that was reserved.
    ///
    /// # Errors
    ///
    /// Returns [`DevicePoolError::NoDeviceWithCapability`] when no unreserved,
    /// non-excluded device with the requested capability is in the pool.
    pub fn request_by_capability_excluding(
        &mut self,
        capability: DeviceCapability,
        exclude: &[DeviceId],
    ) -> Result<DeviceId, DevicePoolError> {
        let mut inner = self.pool.lock().expect("pool lock poisoned");

        let (id, entry) = inner
            .devices
            .iter()
            .find(|(id, e)| {
                e.capability == capability
                    && !inner.reservations.contains_key(*id)
                    && !exclude.contains(id)
            })
            .ok_or(DevicePoolError::NoDeviceWithCapability(capability))?;

        let id = id.clone();
        let reservation = ReservationEntry {
            device: Arc::clone(&entry.device),
            capability: entry.capability,
        };

        inner.reservations.insert(id.clone(), self.id);
        self.reserved.insert(id.clone(), reservation);
        Ok(id)
    }

    /// Provides mutable access to a device reserved by this handle via a
    /// closure.
    ///
    /// This method locks only the individual device — it does **not**
    /// contend on the pool-level lock, allowing multiple handles to access
    /// their own devices concurrently.
    ///
    /// # Errors
    ///
    /// Returns [`DevicePoolError::NotReservedByHandle`] if the device is not
    /// reserved by this handle.
    pub fn with_device<F, R>(&self, id: &DeviceId, f: F) -> Result<R, DevicePoolError>
    where
        F: FnOnce(&mut (dyn TwineCtl + Send)) -> R,
    {
        let entry = self
            .reserved
            .get(id)
            .ok_or_else(|| DevicePoolError::NotReservedByHandle(id.clone()))?;

        let mut device = entry.device.lock().expect("device lock poisoned");
        Ok(f(device.as_mut()))
    }

    /// Releases a specific device back to the pool.
    ///
    /// # Errors
    ///
    /// Returns [`DevicePoolError::NotReservedByHandle`] if the device is not
    /// currently reserved by this handle.
    pub fn release_device(&mut self, id: &DeviceId) -> Result<(), DevicePoolError> {
        if self.reserved.remove(id).is_none() {
            return Err(DevicePoolError::NotReservedByHandle(id.clone()));
        }

        let mut inner = self.pool.lock().expect("pool lock poisoned");
        inner.reservations.remove(id);
        Ok(())
    }

    /// Releases all devices held by this handle back to the pool.
    pub fn release_all(&mut self) {
        let mut inner = self.pool.lock().expect("pool lock poisoned");

        for (id, _) in self.reserved.drain() {
            inner.reservations.remove(&id);
        }
    }

    /// Returns the [`DeviceId`] and [`DeviceCapability`] of every device
    /// reserved by this handle.
    #[must_use]
    pub fn reserved_devices(&self) -> Vec<(DeviceId, DeviceCapability)> {
        self.reserved
            .iter()
            .map(|(id, entry)| (id.clone(), entry.capability))
            .collect()
    }

    /// Returns the number of devices reserved by this handle.
    #[must_use]
    pub fn reserved_count(&self) -> usize {
        self.reserved.len()
    }
}

impl Drop for DevicePoolHandle {
    fn drop(&mut self) {
        let mut inner = match self.pool.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                log::error!(
                    "pool lock poisoned while dropping {}: recovering",
                    self.id
                );
                poisoned.into_inner()
            }
        };

        for (id, _) in self.reserved.drain() {
            inner.reservations.remove(&id);
        }
    }
}
