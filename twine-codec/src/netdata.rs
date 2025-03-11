pub enum RoutePreference {
    Low,
    Medium,
    High,
}

pub struct ExternalRouteConfig {
    rloc16: u16,
    preference: RoutePreference,
    nat64: bool,
    stable: bool,
    next_hop_is_self: bool,
    adv_pio: bool,
}
