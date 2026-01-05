use crate::state::EngineState;
use crate::ScheduleEngine::schedule_satisfied;
use crate::TreasuryRouter::route;

pub fn execute_buyback(
    state: &mut EngineState,
    now: i64,
    treasury_balance: u64,
) {
    if !schedule_satisfied(state, now, treasury_balance) {
        return;
    }

    let _routed = route(
        treasury_balance,
        state.buyback_bps,
        state.lp_bps,
        state.distribution_bps,
    );

    state.last_execution_ts = now;
}
