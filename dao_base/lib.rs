#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
pub use self::dao_base::DaoBase;

#[ink::contract]
mod dao_base {

    use alloc::string::String;
    #[ink(storage)]
    pub struct DaoBase {
        owner: AccountId,
        name: String,
        logo: String,
        desc: String,
    }

    impl DaoBase {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                name:String::default(),
                logo:String::default(),
                desc:String::default(),
            }
        }
        #[ink(message)]
        pub fn init_base(&mut self, name: String, logo: String, desc: String) {
            self.set_name(name);
            self.set_logo(logo);
            self.set_desc(desc);
        }
        #[ink(message)]
        pub fn set_name(&mut self, name: String) {
            self.name = String::from(name);
        }
        #[ink(message)]
        pub fn get_name(&self) -> String{
            self.name.clone()
        }
        
        #[ink(message)]
        pub fn set_logo(&mut self, logo: String) {
            self.logo = String::from(logo);
        }
        #[ink(message)]
        pub fn get_logo(&self) -> String{
            self.logo.clone()
        }
        #[ink(message)]
        pub fn set_desc(&mut self, desc: String) {
            self.desc = String::from(desc);
        }
        #[ink(message)]
        pub fn get_desc(&self) -> String{
            self.desc.clone()
        }



    }


    #[cfg(test)]

    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn test_name() {
            let mut base = DaoBase::new();
            base.set_name("DaoBase".to_string());
            let dbg_msg = format!("name is {}", base.get_name());
            ink_env::debug_println!("{}", &dbg_msg );
            assert_eq!(base.get_name(), "DaoBase");
        }

        #[ink::test]
        fn test_logo() {
            let mut base = DaoBase::new();

            base.set_logo("logo.jpg".to_string());

            let dbg_msg = format!("logo is {}", base.get_logo());
            ink_env::debug_println!("{}", &dbg_msg );

            assert_eq!(base.get_logo(), "logo.jpg");
        }

        #[ink::test]
        fn test_desc() {
            let mut base = DaoBase::new();

            base.set_desc("This is DAO description".to_string());

            let dbg_msg = format!("name is {}", base.get_desc());
            ink_env::debug_println!("{}", &dbg_msg );

            assert_eq!(base.get_desc(), "This is DAO description");
        }

       
    }
}
