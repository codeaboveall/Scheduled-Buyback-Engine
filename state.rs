use anchor_lang::prelude::*;

#[account]
pub struct EngineState {
    pub authority: Pubkey,
    pub treasury: Pubkey,
    pub last_execution_ts: i64,
    pub min_interval_seconds: i64,
    pub min_accumulated_lamports: u64,
    pub buyback_bps: u16,
    pub lp_bps: u16,
    pub distribution_bps: u16,
    pub bump: u8,
}
