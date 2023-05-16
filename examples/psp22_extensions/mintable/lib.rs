#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub use self::my_psp22_mintable::{Contract, ContractRef};

#[openbrush::contract]
pub mod my_psp22_mintable {
    use openbrush::{
        contracts::psp22::extensions::mintable::*,
        traits::Storage,
    };

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
    }
}
