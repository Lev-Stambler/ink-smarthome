#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod smarthome {

    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        DevDoesNotExist,
        DevExists,
    }

    type DeviceId = AccountId;
    pub type Result<T> = core::result::Result<T, Error>;

    use ink_storage::{
        collections::{HashMap as StorageHashMap, Stash as StorageStash},
        traits::{PackedLayout, SpreadLayout},
    };

    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
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
            let ceo = Self::env().caller();
            Self {
                ceo,
                devs: StorageHashMap::default(),
                owner_to_dev: StorageHashMap::default(),
                owner_devs_count: StorageHashMap::default(),
            }
        }

        #[ink(message)]
        pub fn add_new_device(&mut self, owner: AccountId) -> Result<()> {
            let dev_id = self.env().caller();
            let dev_opt = self.devs.get(&dev_id);
            if dev_opt.is_some() {
                return Err(Error::DevExists);
            }
            let _ = self.mint_device(dev_id, owner);

            Ok(())
        }

        /// Get the number of devices for an account
        pub fn device_count(&mut self, owner: AccountId) -> Result<u32> {
            Ok(*(self.owner_devs_count.get(&owner).unwrap_or(&0)))
        }

        fn mint_device(&mut self, dev_id: DeviceId, owner: AccountId) {
            assert!(self.devs.get(&dev_id).is_none());
            let dev = Dev {
                state: false,
                owner: Some(owner),
            };

            self.devs.insert(dev_id, dev);

            let owner_dev_count = *(self.owner_devs_count.get(&owner).unwrap_or(&0));

            self.owner_to_dev.insert((owner, owner_dev_count), dev_id);
            self.owner_devs_count.insert(owner, owner_dev_count + 1);
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
        use ink_lang as ink;
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_env::test;
        type Accounts = test::DefaultAccounts<Environment>;
        fn default_accounts() -> Accounts {
            test::default_accounts().expect("Test environment is expected to be initialized.")
        }

        // Return the callee
        fn set_sender(sender: AccountId) -> AccountId {
            let callee =
                ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or([0x0; 32].into());
            test::push_execution_context::<Environment>(
                sender,
                callee,
                1000000,
                1000000,
                test::CallData::new(ink_env::call::Selector::new([0x00; 4])), // dummy
            );
            return callee;
        }

        fn undo_set_sender() {
            test::pop_execution_context();
        }

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let _ = Smarthome::new();
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn add_device() {
            let accounts = default_accounts();
            let smarthome_item = accounts.bob;
            let mut smarthome = Smarthome::new();


            // Change sender to the smart home item
            let owner = set_sender(smarthome_item);
            assert_eq!(smarthome.device_count(owner), Ok(0));
            let _ = smarthome.add_new_device(owner);

            // Change the sender back to the owner
            undo_set_sender();
            assert_eq!(smarthome.device_count(owner), Ok(1));
        }
    }
}
