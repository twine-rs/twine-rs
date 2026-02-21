// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A managed pool of [`TwineCtl`] devices with handle-based reservation tracking.
//!
//! [`DevicePool`] owns a set of devices, each identified by a unique [`DeviceId`]
//! and annotated with a [`DeviceCapability`] variant. Callers create
//! [`DevicePoolHandle`]s to reserve devices for exclusive use — either by ID,
//! by capability, or by requesting any available device.
//!
//! When a handle is dropped, all of its reserved devices are automatically
//! released back to the pool.
//!
//! # Example
//!
//! ```rust,ignore
//! use twine_ctl::devpool::*;
//!
//! let pool = DevicePool::new();
//! pool.add_device(DeviceId::new("dev-1"), DeviceCapability::FullFunctionDevice, my_device)?;
//!
//! let mut handle = pool.handle();
//!
//! // Request by capability — no need to know the device ID.
//! let id = handle.request_by_capability(DeviceCapability::FullFunctionDevice)?;
//!
//! // Use the reserved device.
//! handle.with_device(&id, |dev| {
//!     // interact with dev
//! }).unwrap();
//!
//! // Release everything at once (also happens automatically on drop).
//! handle.release_all();
//! ```

use std::{collections::HashMap, sync::{Arc, Mutex}};

mod error;
mod handle;

pub use error::DevicePoolError;
pub use handle::{DevicePoolHandle, HandleId};

use crate::TwineCtl;
pub use crate::device::{DeviceCapability, DeviceId};

/// Metadata stored alongside a device in the pool.
pub(super) struct PoolEntry {
    pub(super) device: Arc<Mutex<Box<dyn TwineCtl + Send>>>,
    pub(super) capability: DeviceCapability,
}

/// Shared mutable state behind the pool's `Arc<Mutex<_>>`.
pub(super) struct PoolInner {
    /// Every device in the pool, whether available or reserved.
    pub(super) devices: HashMap<DeviceId, PoolEntry>,

    /// Maps each reserved device to the [`HandleId`] that holds it.
    pub(super) reservations: HashMap<DeviceId, HandleId>,

    /// Monotonically increasing counter for assigning [`HandleId`]s.
    pub(super) next_handle_id: u64,
}

/// A pool of [`TwineCtl`] devices that supports handle-based reservation.
///
/// `DevicePool` is cheaply cloneable — all clones reference the same
/// underlying pool. Interaction with the pool's devices happens exclusively
/// through [`DevicePoolHandle`]s obtained via [`DevicePool::handle`].
#[derive(Clone)]
pub struct DevicePool {
    inner: Arc<Mutex<PoolInner>>,
}

impl DevicePool {
    /// Creates a new, empty device pool.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(PoolInner {
                devices: HashMap::new(),
                reservations: HashMap::new(),
                next_handle_id: 0,
            })),
        }
    }

    /// Adds a device to the pool under the given [`DeviceId`] and
    /// [`DeviceCapability`].
    ///
    /// # Errors
    ///
    /// Returns [`DevicePoolError::DuplicateDevice`] if a device with the same
    /// ID already exists in the pool, whether it is currently available or
    /// reserved by a handle.
    pub fn add_device(
        &self,
        id: DeviceId,
        capability: DeviceCapability,
        device: Box<dyn TwineCtl + Send>,
    ) -> Result<(), DevicePoolError> {
        let mut inner = self.inner.lock().expect("pool lock poisoned");

        if inner.devices.contains_key(&id) {
            return Err(DevicePoolError::DuplicateDevice(id));
        }

        inner.devices.insert(
            id,
            PoolEntry {
                device: Arc::new(Mutex::new(device)),
                capability,
            },
        );
        Ok(())
    }

    /// Creates a new [`DevicePoolHandle`] with a unique [`HandleId`].
    ///
    /// The returned handle can be used to reserve devices from this pool.
    /// When the handle is dropped, any devices it holds are automatically
    /// released back to the pool.
    #[must_use]
    pub fn handle(&self) -> DevicePoolHandle {
        let mut inner = self.inner.lock().expect("pool lock poisoned");
        let id = HandleId::new(inner.next_handle_id);
        inner.next_handle_id += 1;

        DevicePoolHandle::new(id, Arc::clone(&self.inner))
    }

    /// Returns the number of devices currently available (not reserved).
    #[must_use]
    pub fn available_count(&self) -> usize {
        let inner = self.inner.lock().expect("pool lock poisoned");
        inner.devices.len() - inner.reservations.len()
    }

    /// Returns the total number of devices in the pool, including reserved.
    #[must_use]
    pub fn device_count(&self) -> usize {
        self.inner
            .lock()
            .expect("pool lock poisoned")
            .devices
            .len()
    }
}

impl Default for DevicePool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TwineCtlError;
    use twine_codec::{
        Channel, ChannelMask, NetworkName, NetworkRole, OperationalDataset, PanId, Rloc16,
    };

    /// Minimal stub implementing [`TwineCtl`] for pool-management tests.
    ///
    /// None of the trait methods are called during these tests; they exist
    /// solely to satisfy the trait bound.
    struct StubDevice;

    #[async_trait::async_trait]
    impl TwineCtl for StubDevice {
        async fn new_random_network(&mut self) -> Result<(), TwineCtlError> {
            unimplemented!()
        }
        async fn active_dataset(&mut self) -> Result<OperationalDataset, TwineCtlError> {
            unimplemented!()
        }
        async fn attach_with_dataset(
            &mut self,
            _dataset: &OperationalDataset,
        ) -> Result<(), TwineCtlError> {
            unimplemented!()
        }
        async fn pending_dataset(&mut self) -> Result<OperationalDataset, TwineCtlError> {
            unimplemented!()
        }
        async fn channel(&mut self) -> Result<Channel, TwineCtlError> {
            unimplemented!()
        }
        async fn preferred_channel_mask(&mut self) -> Result<ChannelMask, TwineCtlError> {
            unimplemented!()
        }
        async fn supported_channel_mask(&mut self) -> Result<ChannelMask, TwineCtlError> {
            unimplemented!()
        }
        async fn factory_reset(&mut self) -> Result<(), TwineCtlError> {
            unimplemented!()
        }
        async fn network_name(&mut self) -> Result<NetworkName, TwineCtlError> {
            unimplemented!()
        }
        async fn pan_id(&mut self) -> Result<PanId, TwineCtlError> {
            unimplemented!()
        }
        async fn reset(&mut self) -> Result<(), TwineCtlError> {
            unimplemented!()
        }
        async fn rloc16(&mut self) -> Result<Rloc16, TwineCtlError> {
            unimplemented!()
        }
        async fn role(&mut self) -> Result<NetworkRole, TwineCtlError> {
            unimplemented!()
        }
        async fn version(&mut self) -> Result<String, TwineCtlError> {
            unimplemented!()
        }
        async fn uptime(&mut self) -> Result<String, TwineCtlError> {
            unimplemented!()
        }
    }

    fn stub() -> Box<dyn TwineCtl + Send> {
        Box::new(StubDevice)
    }

    fn dev_id(name: &str) -> DeviceId {
        DeviceId::new(name)
    }

    /// Shorthand aliases for readability.
    const FFD: DeviceCapability = DeviceCapability::FullFunctionDevice;
    const BBR: DeviceCapability = DeviceCapability::BackboneBorderRouter;
    const RFD: DeviceCapability = DeviceCapability::ReducedFunctionDevice;

    // == DevicePool::add_device =============================================

    #[test]
    fn add_device_succeeds() {
        let pool = DevicePool::new();
        assert!(pool.add_device(dev_id("a"), FFD, stub()).is_ok());
        assert_eq!(pool.device_count(), 1);
        assert_eq!(pool.available_count(), 1);
    }

    #[test]
    fn add_multiple_devices_with_distinct_ids() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();
        pool.add_device(dev_id("b"), BBR, stub()).unwrap();
        pool.add_device(dev_id("c"), RFD, stub()).unwrap();
        assert_eq!(pool.device_count(), 3);
    }

    #[test]
    fn add_duplicate_device_fails() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();

        let err = pool.add_device(dev_id("a"), FFD, stub()).unwrap_err();
        assert!(matches!(err, DevicePoolError::DuplicateDevice(id) if id == dev_id("a")));
    }

    #[test]
    fn add_duplicate_while_reserved_fails() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();

        let mut handle = pool.handle();
        handle.request_device(&dev_id("a")).unwrap();

        let err = pool.add_device(dev_id("a"), FFD, stub()).unwrap_err();
        assert!(matches!(err, DevicePoolError::DuplicateDevice(_)));
    }

    // == DevicePool::handle =================================================

    #[test]
    fn handles_have_unique_ids() {
        let pool = DevicePool::new();
        let h1 = pool.handle();
        let h2 = pool.handle();
        let h3 = pool.handle();
        assert_ne!(h1.id(), h2.id());
        assert_ne!(h2.id(), h3.id());
    }

    // == request_device (by ID) =============================================

    #[test]
    fn request_device_by_id() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();

        let mut handle = pool.handle();
        assert!(handle.request_device(&dev_id("a")).is_ok());
        assert_eq!(handle.reserved_count(), 1);
        assert_eq!(pool.available_count(), 0);
    }

    #[test]
    fn request_multiple_devices_by_id() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();
        pool.add_device(dev_id("b"), BBR, stub()).unwrap();

        let mut handle = pool.handle();
        handle.request_device(&dev_id("a")).unwrap();
        handle.request_device(&dev_id("b")).unwrap();
        assert_eq!(handle.reserved_count(), 2);
        assert_eq!(pool.available_count(), 0);
    }

    #[test]
    fn request_nonexistent_device_fails() {
        let pool = DevicePool::new();

        let mut handle = pool.handle();
        let err = handle.request_device(&dev_id("missing")).unwrap_err();
        assert!(matches!(err, DevicePoolError::DeviceNotFound(_)));
    }

    #[test]
    fn request_already_reserved_device_fails() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();

        let mut h1 = pool.handle();
        h1.request_device(&dev_id("a")).unwrap();

        let mut h2 = pool.handle();
        let err = h2.request_device(&dev_id("a")).unwrap_err();
        assert!(matches!(err, DevicePoolError::DeviceUnavailable(_)));
    }

    // == request_any ========================================================

    #[test]
    fn request_any_succeeds() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();

        let mut handle = pool.handle();
        let id = handle.request_any().unwrap();
        assert_eq!(id, dev_id("a"));
        assert_eq!(pool.available_count(), 0);
    }

    #[test]
    fn request_any_empty_pool_fails() {
        let pool = DevicePool::new();

        let mut handle = pool.handle();
        let err = handle.request_any().unwrap_err();
        assert!(matches!(err, DevicePoolError::NoDevicesAvailable));
    }

    #[test]
    fn request_any_skips_reserved_devices() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();
        pool.add_device(dev_id("b"), FFD, stub()).unwrap();

        let mut h1 = pool.handle();
        h1.request_any().unwrap();

        let mut h2 = pool.handle();
        let id = h2.request_any().unwrap();
        // Whichever one h1 didn't take, h2 should get.
        assert!(id == dev_id("a") || id == dev_id("b"));
        assert_eq!(pool.available_count(), 0);
    }

    // == request_by_capability ==============================================

    #[test]
    fn request_by_capability_ffd() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("ffd-1"), FFD, stub()).unwrap();
        pool.add_device(dev_id("rfd-1"), RFD, stub()).unwrap();

        let mut handle = pool.handle();
        let id = handle.request_by_capability(FFD).unwrap();
        assert_eq!(id, dev_id("ffd-1"));
    }

    #[test]
    fn request_by_capability_bbr() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("bbr-1"), BBR, stub()).unwrap();
        pool.add_device(dev_id("ffd-1"), FFD, stub()).unwrap();

        let mut handle = pool.handle();
        let id = handle.request_by_capability(BBR).unwrap();
        assert_eq!(id, dev_id("bbr-1"));
    }

    #[test]
    fn request_by_capability_rfd() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("rfd-1"), RFD, stub()).unwrap();
        pool.add_device(dev_id("bbr-1"), BBR, stub()).unwrap();

        let mut handle = pool.handle();
        let id = handle.request_by_capability(RFD).unwrap();
        assert_eq!(id, dev_id("rfd-1"));
    }

    #[test]
    fn request_by_capability_no_match_fails() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("ffd-1"), FFD, stub()).unwrap();

        let mut handle = pool.handle();
        let err = handle.request_by_capability(BBR).unwrap_err();
        assert!(matches!(
            err,
            DevicePoolError::NoDeviceWithCapability(DeviceCapability::BackboneBorderRouter)
        ));
    }

    #[test]
    fn request_by_capability_exhausted_fails() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("ffd-1"), FFD, stub()).unwrap();

        let mut h1 = pool.handle();
        h1.request_by_capability(FFD).unwrap();

        let mut h2 = pool.handle();
        let err = h2.request_by_capability(FFD).unwrap_err();
        assert!(matches!(err, DevicePoolError::NoDeviceWithCapability(FFD)));
    }

    #[test]
    fn request_multiple_by_capability() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("ffd-1"), FFD, stub()).unwrap();
        pool.add_device(dev_id("ffd-2"), FFD, stub()).unwrap();
        pool.add_device(dev_id("bbr-1"), BBR, stub()).unwrap();

        let mut handle = pool.handle();
        let id1 = handle.request_by_capability(FFD).unwrap();
        let id2 = handle.request_by_capability(FFD).unwrap();
        let id3 = handle.request_by_capability(BBR).unwrap();

        assert_ne!(id1, id2);
        assert_eq!(id3, dev_id("bbr-1"));
        assert_eq!(handle.reserved_count(), 3);
    }

    // == device access ======================================================

    #[test]
    fn with_device_provides_access_to_reserved_device() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();

        let mut handle = pool.handle();
        handle.request_device(&dev_id("a")).unwrap();
        let result = handle.with_device(&dev_id("a"), |_dev| 42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn with_device_fails_for_unreserved() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();

        let handle = pool.handle();
        let err = handle.with_device(&dev_id("a"), |_dev| ()).unwrap_err();
        assert!(matches!(err, DevicePoolError::NotReservedByHandle(_)));
    }

    #[test]
    fn with_device_fails_for_nonexistent() {
        let pool = DevicePool::new();

        let handle = pool.handle();
        let err = handle.with_device(&dev_id("nope"), |_dev| ()).unwrap_err();
        assert!(matches!(err, DevicePoolError::NotReservedByHandle(_)));
    }

    // == release_device =====================================================

    #[test]
    fn release_device_returns_it_to_pool() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();

        let mut handle = pool.handle();
        handle.request_device(&dev_id("a")).unwrap();
        assert_eq!(pool.available_count(), 0);

        handle.release_device(&dev_id("a")).unwrap();
        assert_eq!(pool.available_count(), 1);
        assert_eq!(handle.reserved_count(), 0);
    }

    #[test]
    fn release_unreserved_device_fails() {
        let pool = DevicePool::new();

        let mut handle = pool.handle();
        let err = handle.release_device(&dev_id("a")).unwrap_err();
        assert!(matches!(err, DevicePoolError::NotReservedByHandle(_)));
    }

    #[test]
    fn released_device_retains_capability() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("bbr-1"), BBR, stub()).unwrap();

        let mut h1 = pool.handle();
        h1.request_by_capability(BBR).unwrap();
        h1.release_device(&dev_id("bbr-1")).unwrap();

        // Another handle should be able to find it by capability again.
        let mut h2 = pool.handle();
        let id = h2.request_by_capability(BBR).unwrap();
        assert_eq!(id, dev_id("bbr-1"));
    }

    // == release_all ========================================================

    #[test]
    fn release_all_returns_all_devices() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();
        pool.add_device(dev_id("b"), BBR, stub()).unwrap();
        pool.add_device(dev_id("c"), RFD, stub()).unwrap();

        let mut handle = pool.handle();
        handle.request_device(&dev_id("a")).unwrap();
        handle.request_device(&dev_id("b")).unwrap();
        handle.request_device(&dev_id("c")).unwrap();
        assert_eq!(pool.available_count(), 0);

        handle.release_all();
        assert_eq!(pool.available_count(), 3);
        assert_eq!(handle.reserved_count(), 0);
    }

    #[test]
    fn release_all_on_empty_handle_is_noop() {
        let pool = DevicePool::new();
        let mut handle = pool.handle();
        handle.release_all(); // should not panic
        assert_eq!(handle.reserved_count(), 0);
    }

    #[test]
    fn release_all_preserves_capabilities() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("ffd-1"), FFD, stub()).unwrap();
        pool.add_device(dev_id("bbr-1"), BBR, stub()).unwrap();

        let mut h1 = pool.handle();
        h1.request_by_capability(FFD).unwrap();
        h1.request_by_capability(BBR).unwrap();
        h1.release_all();

        let mut h2 = pool.handle();
        assert!(h2.request_by_capability(FFD).is_ok());
        assert!(h2.request_by_capability(BBR).is_ok());
    }

    // == drop releases devices ==============================================

    #[test]
    fn drop_releases_all_devices() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();
        pool.add_device(dev_id("b"), BBR, stub()).unwrap();

        {
            let mut handle = pool.handle();
            handle.request_device(&dev_id("a")).unwrap();
            handle.request_device(&dev_id("b")).unwrap();
            assert_eq!(pool.available_count(), 0);
        } // handle dropped here

        assert_eq!(pool.available_count(), 2);
        assert_eq!(pool.device_count(), 2);
    }

    #[test]
    fn dropped_device_can_be_re_reserved() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();

        {
            let mut h = pool.handle();
            h.request_device(&dev_id("a")).unwrap();
        }

        let mut h2 = pool.handle();
        assert!(h2.request_device(&dev_id("a")).is_ok());
    }

    #[test]
    fn dropped_device_retains_capability() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("rfd-1"), RFD, stub()).unwrap();

        {
            let mut h = pool.handle();
            h.request_by_capability(RFD).unwrap();
        }

        let mut h2 = pool.handle();
        let id = h2.request_by_capability(RFD).unwrap();
        assert_eq!(id, dev_id("rfd-1"));
    }

    // == multiple handles ===================================================

    #[test]
    fn multiple_handles_reserve_different_devices() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();
        pool.add_device(dev_id("b"), BBR, stub()).unwrap();

        let mut h1 = pool.handle();
        let mut h2 = pool.handle();

        h1.request_device(&dev_id("a")).unwrap();
        h2.request_device(&dev_id("b")).unwrap();

        assert_eq!(h1.reserved_count(), 1);
        assert_eq!(h2.reserved_count(), 1);
        assert_eq!(pool.available_count(), 0);
    }

    #[test]
    fn handles_compete_for_same_capability() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("ffd-1"), FFD, stub()).unwrap();

        let mut h1 = pool.handle();
        let mut h2 = pool.handle();

        h1.request_by_capability(FFD).unwrap();

        let err = h2.request_by_capability(FFD).unwrap_err();
        assert!(matches!(err, DevicePoolError::NoDeviceWithCapability(_)));
    }

    // == counts =============================================================

    #[test]
    fn counts_reflect_reservations() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();
        pool.add_device(dev_id("b"), BBR, stub()).unwrap();
        pool.add_device(dev_id("c"), RFD, stub()).unwrap();

        assert_eq!(pool.device_count(), 3);
        assert_eq!(pool.available_count(), 3);

        let mut handle = pool.handle();
        handle.request_device(&dev_id("a")).unwrap();

        assert_eq!(pool.device_count(), 3);
        assert_eq!(pool.available_count(), 2);
    }

    // == reserved_devices ===================================================

    #[test]
    fn reserved_devices_lists_held_ids_and_capabilities() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("x"), FFD, stub()).unwrap();
        pool.add_device(dev_id("y"), RFD, stub()).unwrap();

        let mut handle = pool.handle();
        handle.request_device(&dev_id("x")).unwrap();
        handle.request_device(&dev_id("y")).unwrap();

        let mut entries = handle.reserved_devices();
        entries.sort_by(|a, b| a.0.as_str().cmp(b.0.as_str()));
        assert_eq!(
            entries,
            vec![(dev_id("x"), FFD), (dev_id("y"), RFD)]
        );
    }

    // == mixed workflows ====================================================

    #[test]
    fn request_release_request_cycle() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();

        let mut handle = pool.handle();
        handle.request_device(&dev_id("a")).unwrap();
        handle.release_device(&dev_id("a")).unwrap();
        handle.request_device(&dev_id("a")).unwrap();

        assert_eq!(handle.reserved_count(), 1);
    }

    #[test]
    fn release_one_keep_others() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();
        pool.add_device(dev_id("b"), BBR, stub()).unwrap();
        pool.add_device(dev_id("c"), RFD, stub()).unwrap();

        let mut handle = pool.handle();
        handle.request_device(&dev_id("a")).unwrap();
        handle.request_device(&dev_id("b")).unwrap();
        handle.request_device(&dev_id("c")).unwrap();

        handle.release_device(&dev_id("b")).unwrap();

        assert_eq!(handle.reserved_count(), 2);
        assert_eq!(pool.available_count(), 1);
        assert!(handle.with_device(&dev_id("a"), |_| ()).is_ok());
        assert!(handle.with_device(&dev_id("b"), |_| ()).is_err());
        assert!(handle.with_device(&dev_id("c"), |_| ()).is_ok());
    }

    // == request_by_capability_excluding ====================================

    #[test]
    fn request_by_capability_excluding_skips_excluded_id() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("ffd-1"), FFD, stub()).unwrap();
        pool.add_device(dev_id("ffd-2"), FFD, stub()).unwrap();

        let mut handle = pool.handle();
        let id = handle
            .request_by_capability_excluding(FFD, &[dev_id("ffd-1")])
            .unwrap();
        assert_eq!(id, dev_id("ffd-2"));
    }

    #[test]
    fn request_by_capability_excluding_all_matching_fails() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("ffd-1"), FFD, stub()).unwrap();

        let mut handle = pool.handle();
        let err = handle
            .request_by_capability_excluding(FFD, &[dev_id("ffd-1")])
            .unwrap_err();
        assert!(matches!(err, DevicePoolError::NoDeviceWithCapability(FFD)));
    }

    #[test]
    fn request_by_capability_excluding_empty_list_behaves_like_plain() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("ffd-1"), FFD, stub()).unwrap();

        let mut handle = pool.handle();
        let id = handle
            .request_by_capability_excluding(FFD, &[])
            .unwrap();
        assert_eq!(id, dev_id("ffd-1"));
    }

    #[test]
    fn request_by_capability_excluding_ignores_unrelated_ids() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("ffd-1"), FFD, stub()).unwrap();
        pool.add_device(dev_id("bbr-1"), BBR, stub()).unwrap();

        let mut handle = pool.handle();
        // Excluding a BBR id should not affect FFD selection.
        let id = handle
            .request_by_capability_excluding(FFD, &[dev_id("bbr-1")])
            .unwrap();
        assert_eq!(id, dev_id("ffd-1"));
    }

    #[test]
    fn request_by_capability_excluding_multiple_ids() {
        let pool = DevicePool::new();
        pool.add_device(dev_id("ffd-1"), FFD, stub()).unwrap();
        pool.add_device(dev_id("ffd-2"), FFD, stub()).unwrap();
        pool.add_device(dev_id("ffd-3"), FFD, stub()).unwrap();

        let mut handle = pool.handle();
        let id = handle
            .request_by_capability_excluding(FFD, &[dev_id("ffd-1"), dev_id("ffd-2")])
            .unwrap();
        assert_eq!(id, dev_id("ffd-3"));
    }

    // == concurrent access ==================================================

    #[test]
    fn concurrent_device_access_across_handles() {
        use std::sync::Arc as StdArc;
        use std::thread;

        let pool = DevicePool::new();
        pool.add_device(dev_id("a"), FFD, stub()).unwrap();
        pool.add_device(dev_id("b"), BBR, stub()).unwrap();

        let mut h1 = pool.handle();
        let mut h2 = pool.handle();

        h1.request_device(&dev_id("a")).unwrap();
        h2.request_device(&dev_id("b")).unwrap();

        // Move each handle into its own thread and access devices in parallel.
        let barrier = StdArc::new(std::sync::Barrier::new(2));

        let b1 = StdArc::clone(&barrier);
        let t1 = thread::spawn(move || {
            h1.with_device(&dev_id("a"), |_dev| {
                b1.wait(); // both threads inside with_device simultaneously
                42
            })
            .unwrap()
        });

        let b2 = StdArc::clone(&barrier);
        let t2 = thread::spawn(move || {
            h2.with_device(&dev_id("b"), |_dev| {
                b2.wait();
                99
            })
            .unwrap()
        });

        assert_eq!(t1.join().unwrap(), 42);
        assert_eq!(t2.join().unwrap(), 99);
    }
}
