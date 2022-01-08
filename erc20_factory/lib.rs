
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

pub use self::erc20_factory::{
    Erc20Factory,
};
#[ink::contract]
mod erc20_factory{
    use alloc::string::String;
    use erc20::Erc20;
    use ink_prelude::vec::Vec;
    use core::ptr::null;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
        },
        traits::{
            PackedLayout,
            SpreadLayout,
        },
    };
    const RENT_VALUE: u128 = 1000 * 1_000_000_000_000;

    #[derive(Debug, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct ContractInstance{
        erc20:Option<Erc20>,
    }
    
    #[ink(storage)]
    pub struct Erc20Factory {
        owner: AccountId,
        index: u64,
        token: StorageHashMap<u64,AccountId>,
        symbol_token: StorageHashMap<String, u64>,
        // erc_ins: Erc20,
    }

    impl Erc20Factory {
        
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Default::default(),
                index: 0,
                token: StorageHashMap::new(),
                symbol_token: StorageHashMap::new(),
                // erc_ins: Default::default(),
            }
        }

        // #[ink(message)]
        // pub fn erc_ins_fn(&self) -> String {
        //     self.erc_ins.name()
        // }

        
        pub fn mint_token(&mut self,erc20_hash: Hash,version: u32,name:String,symbol:String ,initial_supply:u64,adr: AccountId,decimals:u8)->bool{
            let salt=version.to_le_bytes();
            let instance_params=Erc20::new(initial_supply.into(),name.clone(),symbol.clone(),decimals,adr)
            .endowment(RENT_VALUE)
            .code_hash(erc20_hash)
            .salt_bytes(salt)
            .params();
            let instance_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = instance_result.expect("failed at instantiating the `ERC20` contract");
            self.token.insert(self.index,contract_addr);
            self.index += 1;
            true
        }

        #[ink(message)]
        pub fn get_token(&self,index: u64) -> AccountId {
            self.token.get(&index).unwrap().clone()
        }
        
        #[ink(message)]
        pub fn get_Block(&self) -> Timestamp {
            self.env().block_timestamp()
        }
    }
}
