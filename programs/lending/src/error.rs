use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode{
  #[msg("Insufficient Funds")]
  InsufficientFunds,

  #[msg("Requested amount exceeds borrowable amount")]
  OverBorrowableAmount,
  
  #[msg("Requested amount exceeds depositable amount")]
  OverRepay,

  #[msg("User is not under collateralized, and cannot be liquidated")]
  NotUnderCollateralized,

}