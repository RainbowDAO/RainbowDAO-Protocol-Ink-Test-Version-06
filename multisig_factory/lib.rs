#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
pub use self::multisig_factory ::{
    MultisigFactory ,
};
use ink_lang as ink;

#[ink::contract]
mod multisig_factory {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use multiSig::MultiSig;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{collections::HashMap as StorageHashMap, };
    const CONTRACT_INIT_BALANCE: u128 = 1000 * 1_000_000_000_000;
    #[ink(storage)]
    pub struct MultisigFactory {
        /// Stores a single `bool` value on the storage.
        multiSig:StorageHashMap<u64,AccountId>,
        index:u64,
        user_multisign:StorageHashMap<AccountId,Vec<AccountId>>,
    }

    impl MultisigFactory {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                multiSig:StorageHashMap::new(),
                index:0,
                user_multisign:StorageHashMap::new()
            }
        }
        #[ink(message)]
        pub fn new_multiSig(
            &mut self,
            multisig_hash:Hash,
            owners: Vec<AccountId>,
            min_sign_count: i32,
        ) -> AccountId {
            let version =  self.index;
            let salt = version.to_le_bytes();
            let instance_params = MultiSig::new(owners.clone(),min_sign_count)
                .endowment(CONTRACT_INIT_BALANCE)
                .code_hash(multisig_hash)
                .salt_bytes(salt)
                .params();
            let init_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = init_result.expect("failed at instantiating the `multiSig` contract");
            assert_eq!(self.index + 1 > self.index, true);
            self.multiSig.insert(self.index, contract_addr);
            self.index += 1;
            for i in &owners {
                let user_mul = self.user_multisign.entry(i.clone()).or_insert(Vec::new());
                user_mul.push(contract_addr);
            }
            contract_addr
        }

        #[ink(message)]
        pub fn user_multisig(&self,user:AccountId) -> Vec<AccountId> {
            let list =  self.user_multisign.get(&user).unwrap().clone();
            list
        }
    }

    // #[cfg(test)]
    // mod tests {


    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;

    //     /// Imports `ink_lang` so we can use `#[ink::test]`.
    //     use ink_lang as ink;


    //     #[ink::test]
    //     fn init_works() {
    //     }
    // }
}