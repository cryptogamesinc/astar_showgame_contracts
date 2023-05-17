#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub use self::my_psp22_mintable::{Contract, ContractRef}; // add

#[openbrush::contract]
pub mod my_psp22_mintable {
    use openbrush::{
        contracts::psp22::extensions::mintable::*,
        traits::Storage,
    };
    use ink::prelude::vec::Vec;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
    }

    impl PSP22 for Contract {}
    impl PSP22Mintable for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            assert!(instance._mint_to(Self::env().caller(), total_supply).is_ok());

            instance
        }

        #[ink(message)]
        pub fn mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self.mint(account, amount)
        }

        #[ink(message)]
        pub fn transfer_from_contract(&mut self, from: AccountId, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error> {
            self._transfer_from_to(from, to, value, data)?;
            Ok(())
        }
    }
}