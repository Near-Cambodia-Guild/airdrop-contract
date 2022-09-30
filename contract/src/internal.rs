use crate::*;

impl Contract {
    /********************/
    /* Internal methods */
    /********************/

    pub(crate) fn internal_check_owner(&self) {
        assert!(self.owner == env::predecessor_account_id(), "Contract: caller is not the owner");
    }

    pub(crate) fn internal_valid_account(&self, account_id: &[u8]) {
        assert!(env::is_valid_account_id(account_id), " Contract: given account id is invalid");
    }

    pub(crate) fn internal_transfer_ownership(&mut self, new_owner: AccountId) {
        let old_owner = self.owner.to_owned();
        let event = EventData {
            event: "ownership_transferred".into(),
            data: Some(HashMap::from([
                ("old_owner".into(), old_owner.to_string()),
                ("new_owner".into(), new_owner.to_string())
            ])),
            ..Default::default()
        };
        let event_str = near_sdk::serde_json::to_string(&event).unwrap();

        self.owner = new_owner;
        log!(event_str);
    }

    pub(crate) fn internal_airdrop(&mut self, participants: Vec<Participant>) {
        // distribute airdrop for each participant in a loop
        participants.into_iter().for_each(|p| {
            self.internal_valid_account(p.account.as_bytes());
            Promise::new(p.account.to_owned()).transfer(p.amount.0);
        });
    }

    pub(crate) fn internal_withdraw(&self, amount: Balance, beneficiary: AccountId) -> Promise {
        assert!(amount > 0, "Withdrawal: amount should be positive");

        Promise::new(beneficiary.to_owned()).transfer(amount)
            .then(
                Self::ext(env::current_account_id())
                    // .with_static_gas(XCC_GAS)
                    .on_withdraw(amount.into(), beneficiary)
            )
    }

    // let airdrop_amount: Balance = participants.into_iter().fold(0, |mut acc, p| {
    //     self.internal_valid_account(p.account.as_bytes());
    //     let amount = p.amount.0;

    //     Promise::new(p.account.to_owned()).transfer(amount)
    //         .then(
    //             Self::ext(env::current_account_id())
    //                 .with_static_gas(XCC_GAS)
    //                 .on_airdrop(amount.into(), p.account, &mut acc)
    //         );
    //     acc
    // });

    // self.distributed += airdrop_amount;

    // participants.into_iter().fold(Promise::new(env::current_account_id()), |acc, p| {
    //     let promise = Promise::new(p.to_owned()).transfer(ONE_NEAR)
    //     .then(
    //         Self::ext(env::current_account_id())
    //             // .with_static_gas(XCC_GAS)
    //             .airdrop_cb(p, ONE_NEAR)
    //     );
    //     acc.and(promise)
    // })
}