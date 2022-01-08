#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
pub use self::daoVote::{
    DaoVote
};

#[ink::contract]
mod daoVote {
    
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use erc20::Erc20;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{
            PackedLayout,
            SpreadLayout,
        }
    };

    // #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    // #[cfg_attr(
    // feature = "std",
    // derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    // )]
    // pub struct VoteInfo{
    //     name:String,
    //     number:u128,//voter turnout
    //     vote:u128,//Number of votes cast


    // }
  
    #[ink(storage)]
    pub struct DaoVote {
        owner:AccountId,
        index:u128,
        balance:StorageHashMap<AccountId,u128>, 
        list_vote:StorageHashMap<u64,String>,
        weightVotes:StorageHashMap<u128,u128>,
        vote:u128,
        erc20_address:AccountId,
        pub contract_instance:ContractInstance,
    }
    #[derive(Debug, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct ContractInstance{
        erc20:Option<Erc20>,
    }

    impl DaoVote {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(erc20_addr:AccountId) -> Self {
            let mut list=StorageHashMap::new();
            list.insert(1,String::from("a_currency_one_vote"));
            list.insert(2,String::from("one_man_one_vote"));
            list.insert(3,String::from("weight_vote"));
            list.insert(4,String::from("basic_quadratic_vote"));
            list.insert(5,String::from("weight_quadratic_vote"));
            Self { 
                owner:Self::env().caller(),
                index:0,
                balance:StorageHashMap::new(),
                list_vote:list,
                weightVotes:StorageHashMap::new(),
                vote:0,
                erc20_address:erc20_addr,
                contract_instance:ContractInstance{
                    erc20:None,
                }
             }
        }
        
        ///Instantiate erc20
        #[ink(message)]
        pub fn erc20_ins(&mut self) ->bool{
            let contract_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(self.erc20_address);
            self.contract_instance.erc20 = Some(contract_instance);
            true
        } 
        #[ink(message)]
        pub fn set_weight_vote(&mut self) {
           // let total_balance = Self::env().balance();
           let total_balance=self.balance.get(&self.env().caller()).unwrap().clone();
            self.balance.insert(self.env().caller(),total_balance);
            if total_balance<100{
                self.weightVotes.insert(total_balance,1);
            }
            else if total_balance>100&&total_balance<1000{
                self.weightVotes.insert(total_balance,2);
            }
            else if total_balance>1000{
                self.weightVotes.insert(total_balance,3);
            }
        }

        #[ink(message)]
        pub fn select_vote(&mut self,index:u128) {
            if let index=1{
                self.index+=1;
                self.vote=self.balance.get(&self.owner).unwrap().clone();
            }
            else if index==2{
                self.index+=1;
                self.vote+=self.index;
            }
            else if index==3{
                self.index+=1;
                self.vote+=self.weightVotes.get(&(self.balance.get(&self.owner).unwrap().clone())).unwrap().clone();

            }
            else if index==4{
                self.index+=1;
                self.vote+=(self.balance.get(&self.owner).unwrap().clone())^2/1;

            }
            else if index==5{
                self.index+=1;
                self.vote+=(self.weightVotes.get(&(self.balance.get(&self.owner).unwrap().clone()))).unwrap().clone()^2/1;

            }
            
        }

        // #[ink(message)]
        // pub fn select_vote_info(&self)->VoteInfo{
        //     VoteInfo {
        //         name:self.name.clone(),
        //         number:self.index,
        //         vote:self.vote,
        //     } 
        // }

    }
}