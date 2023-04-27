#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp37_enumerable {
    use openbrush::{
        contracts::psp37::extensions::{
            batch::*,
            burnable::*,
            enumerable::*,
            mintable::*,
        },
        traits::Storage,
    };
    use ink::prelude::string::String;

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data<enumerable::Balances>,

        base_uri: String
    }


    impl PSP37 for Contract {}

    impl PSP37Mintable for Contract {}

    impl PSP37Burnable for Contract {}

    impl PSP37Enumerable for Contract {}

    impl PSP37Batch for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
        #[ink(message)]
        pub fn set_base_uri(&mut self, base_uri: String) -> Result<(), PSP37Error> {
            self.base_uri = base_uri;
            Ok(())
        }
        #[ink(message)]
        pub fn get_base_uri(&self) -> String {
            self.base_uri.clone()
        }
    }
}
