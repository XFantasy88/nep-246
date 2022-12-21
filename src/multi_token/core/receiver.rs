use crate::multi_token::token::TokenId;
use near_sdk::{ext_contract, AccountId, PromiseOrValue};

/// Used when an MT is transferred using `mt_transfer_call`. This trait is implemented on the receiving contract, not on the MT contract.
#[ext_contract(ext_mt_receiver)]
pub trait MultiTokenReceiver {
    /// Take some action after receiving a non-fungible token
    ///
    /// Requirements:
    /// * Contract MUST restrict calls to this function to a set of whitelisted NFT
    ///   contracts
    ///
    /// Arguments:
    /// * `sender_id`: the sender of `mt_transfer_call`
    /// * `previous_owner_id`: the account that owned the MT prior to it being
    ///   transferred to this contract, which can differ from `sender_id` if using
    ///   Approval Management extension
    /// * `token_id`: the `token_id` argument given to `mt_transfer_call`
    /// * `msg`: information necessary for this contract to know how to process the
    ///   request. This may include method names and/or arguments.
    ///
    /// Returns true if token should be returned to `sender_id`
    fn mt_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> PromiseOrValue<bool>;
}
