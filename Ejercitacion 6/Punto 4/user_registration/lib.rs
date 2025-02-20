#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod user_registration {

    use ink::storage::Mapping;
    use ink::prelude::string::String; // Importa el tipo String compatible con no_std

    #[ink(storage)]
    pub struct UserRegistration {
        users: Mapping<String, AccountId>,
        addresses: Mapping<AccountId, String>,
    }

    impl UserRegistration {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                users: Mapping::default(),
                addresses: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn register_user(&mut self, user_name: String) {
            let caller = self.env().caller();
            
            assert!(self.users.get(&user_name).is_none(), "User exists already");
            assert!(self.addresses.get(&caller).is_none(), "User already has a user name");
            
            self.users.insert(&user_name, &caller);
            self.addresses.insert(&caller, &user_name);
        }

        #[ink(message)]
        pub fn get_user_name(&self, user_address: AccountId) -> Option<String> {
            self.addresses.get(&user_address)
        }

    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink::env::test;

        #[ink::test]
        fn test_register_user() {
            let mut contract = UserRegistration::new();
            let user = String::from("Alice"); // Usa String::from para crear una cadena
            contract.register_user(user.clone());
            let caller = test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            assert_eq!(contract.get_user_name(caller), Some(user));
        }

        #[ink::test]
        #[should_panic(expected = "User exists already")]
        fn test_register_existing_user() {
            let mut contract = UserRegistration::new();
            let user = String::from("Alice"); // Usa String::from para crear una cadena
            contract.register_user(user.clone());
            contract.register_user(user); // Esto debería fallar
        }

        #[ink::test]
        fn test_get_nonexistent_user() {
            let contract = UserRegistration::new();
            let random_address = AccountId::from([0x02; 32]);
            assert_eq!(contract.get_user_name(random_address), None);
        }
    }
}