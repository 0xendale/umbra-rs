pub mod derivation;
pub mod error;
pub mod sol;

pub use error::SweepSolError;
pub use sol::{build_and_sign_sweep_sol_transaction, MIN_REMAINING_LAMPORTS};

use crate::derivation::ScalarSigner;
use solana_sdk::pubkey::Pubkey;

/// Parameters required to build a SOL sweep transaction.
pub struct SweepSolParams<'a> {
    /// Ephemeral private-key signer derived from scalar
    pub signer: &'a ScalarSigner,
    /// Recipient of the swept funds
    pub to: Pubkey,
    /// Amount of lamports to sweep
    pub amount: u64,
    /// Recent blockhash to sign transaction with
    pub recent_blockhash: solana_sdk::hash::Hash,
}
