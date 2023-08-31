#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod property_tokenization {
    use ink::{
        storage::Mapping,
        prelude::string::String,
    };

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Metadata {
        address: String,
        area: u64,
        description: String,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct PropertyDetails {
        owner: AccountId,
        total_tokens: u64,
        metadata: Metadata,
        offered_ownership_percentage: u32,
    }

    #[ink(storage)]
    pub struct PropertyTokenization {
        /// Stores the `AccountId` - owner of the properpty, along with the `PropertyDetails` for the given property
        admin: AccountId,
        property: ink::storage::Mapping<AccountId, PropertyDetails>
    }

    impl PropertyTokenization {
        /// Constructor that initializes the Property tokenization with the data input
        #[ink(constructor)]
        pub fn new(account_id: AccountId, property_details: PropertyDetails) -> Self {
            
            let property_to_save = PropertyDetails {
                owner: account_id,
                total_tokens: property_details.total_tokens,
                offered_ownership_percentage: property_details.offered_ownership_percentage,
                metadata: property_details.metadata,
            };

            let mut mapping = Mapping::new();
            mapping.insert(account_id, &property_to_save);

            Self {
                admin: account_id,
                property: mapping,
            }
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.admin
        }
    }
}
