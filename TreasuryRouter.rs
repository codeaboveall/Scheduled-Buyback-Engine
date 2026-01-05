pub struct RoutingResult {
    pub buyback_amount: u64,
    pub lp_amount: u64,
    pub distribution_amount: u64,
}

pub fn route(amount: u64, buyback_bps: u16, lp_bps: u16, dist_bps: u16) -> RoutingResult {
    RoutingResult {
        buyback_amount: amount * buyback_bps as u64 / 10_000,
        lp_amount: amount * lp_bps as u64 / 10_000,
        distribution_amount: amount * dist_bps as u64 / 10_000,
    }
}
