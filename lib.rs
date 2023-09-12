#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod property_tokenization {
    use ink::{
        prelude::{string::String, vec::Vec},
        storage::Mapping,
    };

    /// Custom type for the property id
    pub type PropertyId = u32;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        Unauthorized,
        NoSuchOwner,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Metadata {
        /// Physical address of the property.
        address: String,
        /// What area is being offered.
        area_offered: u32,
        /// Total area of the property.
        total_area: u32,
        /// Short description of the property.
        description: String,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct PropertyDetails {
        /// The id of the property
        property_id: PropertyId,
        /// Total tokens that are offered for the property.
        total_tokens: u64,
        /// Additional information about the property.
        metadata: Metadata,
        /// What percentage of the property is being offered as tokens.
        total_offered_ownership_percentage: u8,
        /// only `verified` properties can be listed for tokenization.
        /// Until property is not verified, it can only be viewed.
        is_verified: bool,
    }

    #[ink(storage)]
    pub struct PropertyTokenization {
        /// administrator of PropertyTokenization contract. Single entity that can verify properties
        admin: AccountId,
        /// Stores the `AccountId` - owner of the properpty, along with the `PropertyDetails` for the given property
        property: Mapping<AccountId, Vec<PropertyDetails>>,
    }

    impl PropertyTokenization {
        /// Constructor that initializes the Property tokenization
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            Self {
                admin,
                property: Mapping::default(),
            }
        }

        /// adds new property into the storage of our contract
        #[ink(message)]
        pub fn add_new_property(&mut self, property_details: PropertyDetails) -> Result<(), Error> {
            let caller = self.env().caller();
            let found_properties = self.property.get(caller);

            match found_properties {
                Some(mut properties) => {
                    properties.push(property_details);
                    self.property.insert(caller, &properties);
                }
                None => return Err(Error::NoSuchOwner),
            }

            Ok(())
        }

        #[ink(message)]
        pub fn verify_property(
            &mut self,
            property_owner: AccountId,
            property_id: PropertyId,
        ) -> Result<(), Error> {
            if self.admin != self.env().caller() {
                return Err(Error::Unauthorized);
            }

            let all_properties_for_owner = self.property.get(property_owner);

            match all_properties_for_owner {
                Some(mut properties) => {
                    for p in &mut properties {
                        if p.property_id == property_id {
                            p.is_verified = true;
                        }
                    }
                }
                None => return Err(Error::NoSuchOwner),
            };
            Ok(())
        }
        //todo write tests for verify_property() and add_new_property()
        //todo implement selling tokens to other users
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rand;
        
        #[ink::test]
        fn default_works() {
            let rng = rand::random::<[u8; 32]>();

            let contract = PropertyTokenization::new(AccountId::from(rng));
            dbg!(&contract);
        }
    }
}
