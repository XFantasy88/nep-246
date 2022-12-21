use crate::multi_token::token::TokenId;
use near_sdk::{ext_contract, AccountId};

/// Approval receiver is the trait for the method called (or attempted to be called) when an MT contract adds an approval for an account.
#[ext_contract(ext_mt_approval_receiver)]
pub trait MultiTokenApprovalReceiver {
    /// Respond to notification that contract has been granted approval for a token.
    ///
    /// Notes
    /// * Contract knows the token contract ID from `predecessor_account_id`
    ///
    /// Arguments:
    /// * `token_id`: the token to which this contract has been granted approval
    /// * `owner_id`: the owner of the token
    /// * `approval_id`: the approval ID stored by MT contract for this approval.
    ///   Expected to be a number within the 2^53 limit representable by JSON.
    /// * `msg`: specifies information needed by the approved contract in order to
    ///    handle the approval. Can indicate both a function to call and the
    ///    parameters to pass to that function.
    fn mt_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    ) -> near_sdk::PromiseOrValue<String>; // TODO: how to make "any"?
}
