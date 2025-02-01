use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode{
  #[msg("Insufficient Funds")]
  InsufficientFunds,

  #[msg("Requested amount exceeds borrowable amount")]
  OverBorrowableAmount,
  
  #[msg("Requested amount exceeds depositable amount")]
  OverRepay,
}