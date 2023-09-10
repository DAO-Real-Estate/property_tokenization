#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod property_tokenization {
    use ink::{
        storage::Mapping,
        prelude::string::String,
    };

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Wrapper, will get rid off
        Undefinied,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Metadata {
        address: String,
        area: u32,
        description: String,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct PropertyDetails {
        total_tokens: u64,
        metadata: Metadata,
        total_offered_ownership_percentage: u8,
    }

    #[ink(storage)]
    pub struct PropertyTokenization {
        /// Stores the `AccountId` - owner of the properpty, along with the `PropertyDetails` for the given property
        property: ink::storage::Mapping<AccountId, PropertyDetails>
    }

    impl PropertyTokenization {
        /// Constructor that initializes the Property tokenization 
        #[ink(constructor)]
        pub fn new() -> Self {
            
            Self {
                property: Mapping::default(),
            }
        }

        /// adds new property into the storage of our contract
        #[ink(message)]
        pub fn add_new_property(&mut self, property_details: PropertyDetails) {
            let caller = self.env().caller();
            self.property.insert(caller, property_details);
        }

        //todo implement selling tokens to other users
    }
}
