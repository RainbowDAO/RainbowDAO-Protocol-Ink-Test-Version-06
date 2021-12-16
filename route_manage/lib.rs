#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
pub use self::route_manage::{
    RouteManage,
};

#[ink::contract]
mod route_manage {

    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{collections::HashMap as StorageHashMap, };
    #[ink(storage)]
    pub struct RouteManage {
        owner:AccountId,
        index:u64,
        route_map:StorageHashMap<String,AccountId>,
    }

    impl RouteManage {
        #[ink(constructor)]
        pub fn new() -> Self {

            let instance = Self {
                owner:Self::env().caller(),
                index: 0,
                route_map : StorageHashMap::new(),
            };
            instance
        }
        fn only_core(&self,sender:AccountId) {
            assert_eq!(self.owner, sender);
        }

        #[ink(message)]
        pub fn add_route(&mut self, name: String,value:AccountId) -> bool {
            // self.only_core(Self::env().caller());
            assert_eq!(self.index + 1 > self.index, true);
            self.route_map.insert(name,value);
            self.index += 1;
            true
        }

        // #[ink(message)]
        // pub fn list_route(&self) -> Vec<String> {
        //     let mut route_vec = Vec::new();
        //     let mut iter = self.route_map.values();
        //     let mut route = iter.next();
        //     while route.is_some() {
        //         route_vec.push(route.unwrap().clone());
        //         route = iter.next();
        //     }
        //     route_vec
        // }

        #[ink(message)]
        pub fn query_route_by_name(&self, name: String) -> AccountId {
            self.route_map.get(&name).unwrap().clone()
        }
        #[ink(message)]
        pub fn change_route(&mut self,name:String,value:AccountId) -> bool {
            // self.only_core(Self::env().caller());
            self.route_map[&name] = value;
            true
        }
    }

}
