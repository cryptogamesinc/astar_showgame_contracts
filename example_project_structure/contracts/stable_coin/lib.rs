#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// This is a simple `PSP22` which will be used as a stable coin and a collateral token in our lending contract
#[openbrush::contract]
pub mod token {
    use lending_project::traits::stable_coin::*;
    use openbrush::{
        contracts::psp22::extensions::{
            metadata::*,
            mintable::*,
        },
        traits::{
            Storage,
            String,
        },
    };

    /// Define the storage for PSP22 data and Metadata data
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct StableCoinContract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    /// Implement PSP22 Trait for our coin
    impl PSP22 for StableCoinContract {}

    /// Implement PSP22Metadata Trait for our coin
    impl PSP22Metadata for StableCoinContract {}

    /// implement PSP22Mintable Trait for our coin
    impl PSP22Mintable for StableCoinContract {}

    // It forces the compiler to check that you implemented all super traits
    impl StableCoin for StableCoinContract {}

    impl StableCoinContract {
        /// Constructor with name and symbol
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
            let mut instance = Self::default();

            instance.metadata.name = name;
            instance.metadata.symbol = symbol;
            instance.metadata.decimals = 18;
            let total_supply = 1_000_000 * 10_u128.pow(18);
            assert!(instance._mint_to(Self::env().caller(), total_supply).is_ok());

            instance
        }
    }
}
