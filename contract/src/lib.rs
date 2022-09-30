/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */
use std::collections::HashMap;
use std::ops::{Sub, Mul};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{log, env, near_bindgen, AccountId, Balance, Promise, PromiseError, Gas, PanicOnDefault};

mod internal;
mod model;

use model::*;

const TGAS: Gas = Gas(1_000_000_000_000);
// const XCC_GAS: Gas = Gas(20000000000000);

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    /********************/
    /* Mutable methods */
    /********************/

    #[init]
    pub fn new(owner: AccountId) -> Self {
        log!("contract initialization!");
        Self { owner }
    }

    pub fn airdrop(&mut self, participants: Vec<Participant>) {
        self.internal_check_owner();

        self.internal_airdrop(participants);
    }

    // #[payable]
    // pub fn deposit(&mut self) {
    //     self.internal_check_owner();
    //     let amount: Balance = env::attached_deposit();
    // }

    pub fn withdraw(&mut self, amount: U128, beneficiary: AccountId) -> Promise {
        assert!(env::prepaid_gas() >= TGAS.mul(11), "Gas: Not enough");

        self.internal_check_owner();
        self.internal_valid_account(beneficiary.as_bytes());

        let amount: Balance = amount.into();
        assert!(amount <= self.available_withdraw(), "Withdrawal: exceed available balance");
        
        self.internal_withdraw(amount, beneficiary)
    }

    pub fn withdraw_all(&mut self, beneficiary: AccountId) -> Promise {
        self.internal_check_owner();
        self.internal_valid_account(beneficiary.as_bytes());

        self.internal_withdraw(self.available_withdraw(), beneficiary)
    }

    pub fn transfer_ownership(&mut self, new_owner: AccountId) {
        self.internal_check_owner();
        self.internal_valid_account(new_owner.as_bytes());

        self.internal_transfer_ownership(new_owner);
    }

    /********************/
    /* View methods */
    /********************/

    pub fn owner(self) -> AccountId {
        self.owner
    }

    // pub fn distributed_amount(self) -> Balance {
    //     self.distributed
    // }

    pub fn available_withdraw(&self) -> Balance {
        let balance = env::account_balance();
        let storage_usage: Balance = env::storage_usage().into();
        balance.sub(storage_usage)
    }

    /*************/
    /* Callbacks */
    /*************/

    #[private]
    pub fn on_withdraw(
        #[callback_result] res: Result<(), PromiseError>, 
        amount: U128, 
        beneficiary: AccountId
    ) -> bool {
        if res.is_ok() {
            let event = EventData {
                event: "balance_withdrawn".into(),
                data: Some(HashMap::from([
                    ("amount".into(), amount.0.to_string()),
                    ("beneficiary".into(), beneficiary.into())
                ])),
                ..Default::default()
            };
            let event_str = near_sdk::serde_json::to_string(&event).unwrap();
            log!(event_str);
        }
        res.is_ok()
    }

    // #[private]
    // pub fn on_airdrop(
    //     #[callback_result] res: Result<(), PromiseError>, 
    //     amount: U128, 
    //     participant: AccountId,
    //     acc: &mut u128,
    // )  -> bool {
    //     if res.is_ok() {
    //         *acc += amount.0;
    //         log!("{}", acc);

    //         let event = EventData {
    //             event: "airdrop_distributed".into(),
    //             data: Some(HashMap::from([
    //                 ("amount".into(), amount.0.to_string()),
    //                 ("participant".into(), participant.into())
    //             ])),
    //             ..Default::default()
    //         };
    //         let event_str = near_sdk::serde_json::to_string(&event).unwrap();
    //         log!(event_str);
    //     }
    //     res.is_ok()
    // }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, ONE_NEAR};

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_check_owner() {
        let mut context = get_context(accounts(1));
        testing_env!(context.is_view(true).build());
        let contract = Contract::new(accounts(1).into());
        assert_eq!(contract.owner(), accounts(1));
    }

    #[test]
    fn test_transfer_ownership() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1).into());
        contract.transfer_ownership(accounts(2).into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.owner(), accounts(2));
    }
 
    #[test]
    fn test_distribute_airdrop() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1).into());
        let accounts: Vec<AccountId> = ["a.testnet", "b.testnet", "c.testnet"].into_iter().map(|p| p.parse().unwrap()).collect();
        let _accounts_len = accounts.len();
        let participants: Vec<Participant> = accounts.into_iter().map(|a| Participant { account: a, amount: ONE_NEAR.into()}).collect();
        contract.airdrop(participants);
    }

    #[test]
    fn test_withdraw() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1).into());
        let initial_balance = env::account_balance();
        let withdraw_amount = ONE_NEAR.mul(10);

        contract.withdraw(withdraw_amount.into(), accounts(2).into());
        assert_eq!(initial_balance.sub(withdraw_amount), env::account_balance());
    }

    #[test]
    fn test_withdraw_all() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1).into());

        contract.withdraw_all(accounts(2).into());
        assert_eq!(env::account_balance(), env::storage_usage().into());
    }
}
