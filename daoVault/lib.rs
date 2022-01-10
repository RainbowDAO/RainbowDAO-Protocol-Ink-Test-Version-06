#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

pub use self::dao_vault::{
    DaoVault,
};

#[ink::contract]
mod daoVault {

    use alloc::string::String;
    use alloc::vec::Vec;
    use erc20::Erc20;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout,SpreadLayout},
    };

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct DaoVault {
        vault_manager:AccountId,
        amount_of_user:u64,
        allow_tokens:StorageHashMap<String,AccountId>,
        in_out_tokens:StorageHashMap<(AccountId,String), u64>,
    }

    impl DaoVault {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(owner:AccountId,) -> Self {
            Self {
                vault_manager :owner,
                amount_of_user:0,
                allow_tokens:StorageHashMap::default(),
                in_out_tokens:StorageHashMap::default(),
             }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn deposit(& mut self, token_name:String,token_address1:AccountId,amount1:u64) -> bool {
            assert_eq!(self.allow_tokens.get(&token_name) == Some(&token_address1),true);
            
            let caller = self.env().caller();

            self.in_out_tokens.insert((caller,token_name), self.amount_of_user);
            
            self.env().emit_event(DepositTokenEvent{
                token_address:token_address1,
                depositer:caller,
                amount:amount1,
            });
            true
        }
        #[ink(message)]
        pub fn withdrawer(&mut self, token_name:String, token_address1:AccountId, amount1:u64) -> bool{
            assert_eq!(self.allow_tokens.get(&token_name.clone()) == Some(&token_address1) , true);
            let caller = self.env().caller();
            let vault_addr = self.env().account_id();
            
           self.amount_of_user -= amount1;
            self.in_out_tokens.insert((caller,token_name.clone()), self.amount_of_user);
            
            self.env().emit_event(WithdrawTokenEvent{
                token_address:token_address1,
                withdrawer:caller,
                amount:amount1,
            });
            true
        }
        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let daoVault = DaoVault::default();
            assert_eq!(daoVault.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut daoVault = DaoVault::new(false);
            assert_eq!(daoVault.get(), false);
            daoVault.flip();
            assert_eq!(daoVault.get(), true);
        }
    }
}
