use anchor_lang::prelude::*;

#[error_code]
pub enum SBEError {
    #[msg("Schedule conditions not met")]
    ScheduleNotSatisfied,
    #[msg("Invalid routing configuration")]
    InvalidRoutingConfig,
    #[msg("Unauthorized caller")]
    Unauthorized,
    #[msg("Execution already performed in this window")]
    ExecutionAlreadyPerformed,
}
