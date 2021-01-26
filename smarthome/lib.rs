#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod smarthome {

    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        DevDoesNotExist,
        WrongDevPasswordHash,
        DeviceAlreadyClaimed,
    }

    type DeviceId = AccountId;
    pub type Result<T> = core::result::Result<T, Error>;

    use ink_storage::{
        collections::{HashMap as StorageHashMap, Stash as StorageStash},
        traits::{PackedLayout, SpreadLayout},
    };

    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Copy)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    struct Dev {
        /// TODO: have Dev be a trait with different possible states depending on device type
        state: bool,
        owner: Option<AccountId>,
    }

    #[ink(storage)]
    pub struct Smarthome {
        devs: StorageHashMap<DeviceId, Dev>,
        /// The owner of a device and device index to the hash of a smart device
        owner_to_dev: StorageHashMap<(AccountId, u32), DeviceId>,
        owner_devs_count: StorageHashMap<AccountId, u32>,
        ceo: AccountId,
    }

    impl Smarthome {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            let owner = Self::env().caller;
            Self {
                devs: StorageHashMap::new(),
                owner_to_dev: StorageHashMap::new(),
                owner_devs_count: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn add_new_device(&mut self, owner: AccountId) {
            let devId = self.env().caller;
            let dev = self.devs.get(devId);
            match dev {
                Some(d) => d,
                None => Self::mint_device(devId),
            };

            if Self::device_has_owner(devId) {
                return Err(Error::DeviceAlreadyClaimed);
            }
            // self.devs.insert()
            Ok(dev);
        }

        /// Get the number of devices for an account
        pub fn device_count(&mut self) -> Result<u32> {
            let owner = self.env().caller();
            *(self.owner_devs_count.get(&owner).unwrap_or(&0))
        }

        fn mint_device(&self, devId: DeviceId, owner: AccountId) -> Dev {
            assert!(self.devs.get(devId) == None);
            let dev = Dev {
                state: false,
                owner: Some(owner),
            };

            self.devs.insert(devId, dev);

            let owner_dev_count = *(self.owner_devs_count.get(&owner).unwrap_or(&0));

            self.owner_to_dev
                .insert((owner, owner_dev_count), device_id);
            self.owner_devs_count.insert(owner, owner_dev_count + 1);
            return dev;
        }

        fn dev_pass_match(dev: &Dev, h1: Hash) {
            assert!(dev.pass_hash == h1);
        }

        fn device_has_owner(dev: &Dev) -> bool {
            dev.owner != None
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[test]
        fn default_works() {
            let smarthome = Smarthome::new();
            // assert_eq!(smarthome.get(), false);
        }

        /// We test a simple use case of our contract.
        #[test]
        fn add_device() {
            let mut smarthome = Smarthome::new();

            // assert_eq!(smarthome.get(), false);

            // assert_eq!(smarthome.get(), true);
        }
    }
}
