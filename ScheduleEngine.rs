use crate::state::EngineState;

pub fn schedule_satisfied(
    state: &EngineState,
    now: i64,
    treasury_balance: u64,
) -> bool {
    let time_ok = now - state.last_execution_ts >= state.min_interval_seconds;
    let balance_ok = treasury_balance >= state.min_accumulated_lamports;
    time_ok || balance_ok
}
