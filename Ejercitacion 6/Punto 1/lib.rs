#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod simple_counter {
    use ink::storage::Mapping;


    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct SimpleCounter {
        /// Stores a single `bool` value on the storage.
        number: u128,
        owner: AccountId,
        white_list: Mapping<AccountId, bool>,
    }

    #[ink(event)]
    pub struct NewValue {
        #[ink(topic)]
        sender: AccountId,
        new_number: u128,
    }

    impl SimpleCounter {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {  number: 0,
                    owner: caller,
                    white_list: Mapping::new(), 
                }
        }


        #[ink(message)]
        pub fn add_to_white_list(&mut self, address: AccountId) {
            self.ensure_owner();
            self.white_list.insert(address, &true);
        }

        #[ink(message)]
        pub fn remove_from_white_list(&mut self, address: AccountId) {
            self.ensure_owner();
            self.white_list.insert(address, &false);
        }

        #[ink(message)]
        pub fn increase_number(&mut self) {
            self.ensure_white_list();
            self.number = self
                .number
                .checked_add(1)
                .expect("Overflow en increase_number");
            self.env().emit_event(NewValue {
                sender: self.env().caller(),
                new_number: self.number,
            });
        }

        #[ink(message)]
        pub fn decrease_number(&mut self) {
            self.ensure_white_list();
            self.number = self
                .number
                .checked_sub(1)
                .expect("Underflow en decrease_number");
            self.env().emit_event(NewValue {
                sender: self.env().caller(),
                new_number: self.number,
            });
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn retrieve_number(&self) -> u128 {
            self.number
        }


        fn ensure_owner(&self){
            assert_eq!(self.env().caller(), self.owner,
                "Caller is not owner");
        }

        fn ensure_white_list(&self){
            assert!(&self.white_list.get(&self.env().caller()).unwrap_or(false),
                "Caller is not in the white list");
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

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let simple_counter = SimpleCounter::new();
            assert_eq!(simple_counter.retrieve_number(), 0);
        }

        #[ink::test]
        fn owner_can_add_to_whitelist() {
            // Obtenemos las cuentas por defecto.
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            // Establecemos a `alice` como el owner.
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            let mut contract = SimpleCounter::new();

            // El owner (alice) agrega a `bob` a la whitelist.
            contract.add_to_white_list(accounts.bob);

            // Verificamos que `bob` esté en la whitelist.
            let is_whitelisted = contract.white_list.get(&accounts.bob).unwrap_or(false);
            assert_eq!(is_whitelisted, true, "El owner debería poder agregar a alguien a la whitelist");
        }
        
        #[ink::test]
        fn owner_can_remove_from_whitelist() {
            // Obtenemos las cuentas por defecto.
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            // Establecemos a `alice` como el owner.
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            let mut contract = SimpleCounter::new();

            // El owner agrega a `bob` a la whitelist.
            contract.add_to_white_list(accounts.bob);
            // Verificamos que `bob` esté en la whitelist.
            let is_whitelisted = contract.white_list.get(&accounts.bob).unwrap_or(false);
            assert_eq!(is_whitelisted, true, "bob debería estar en la whitelist");

            // El owner remueve a `bob` de la whitelist.
            contract.remove_from_white_list(accounts.bob);
            // Verificamos que `bob` ya no esté en la whitelist.
            let is_whitelisted = contract.white_list.get(&accounts.bob).unwrap_or(false);
            assert_eq!(is_whitelisted, false, "bob ya no debería estar en la whitelist");
        }

        #[ink::test]
        #[should_panic(expected = "Caller is not owner")]
        fn only_owner_can_add_to_whitelist() {
            // Obtenemos cuentas de prueba.
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            // Establecemos a `alice` como el caller, que será el owner.
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            let mut contract = SimpleCounter::new();

            // Ahora, cambiamos el caller a `bob`, que no es el owner.
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            // Como `bob` no es owner, se debe generar un panic al intentar agregar a alguien a la whitelist.
            contract.add_to_white_list(accounts.eve);
        }

        #[ink::test]
        #[should_panic(expected = "Caller is not owner")]
        fn only_owner_can_remove_from_whitelist() {
            // Obtenemos cuentas de prueba
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            // Establecemos a `alice` como el owner
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            let mut contract = SimpleCounter::new();

            // Para asegurarnos que hay algo en la whitelist, lo agregamos desde el owner
            contract.add_to_white_list(accounts.eve);

            // Cambiamos el caller a `bob`, que no es owner
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            // Al intentar remover de la whitelist, se debe generar un panic ya que `bob` no es owner
            contract.remove_from_white_list(accounts.eve);
        }

        #[ink::test]
        fn increase_and_decrease_work() {
            use ink::env::test;

            // Obtenemos las cuentas de prueba, donde `alice` será el owner.
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            // Establecemos a `alice` como caller y, por ende, owner.
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            let mut contract = SimpleCounter::new();

            // Para poder llamar a `increase_number` y `decrease_number`,
            // debemos asegurarnos de que el caller (alice) esté en la whitelist.
            contract.add_to_white_list(accounts.alice);

            // Verificamos que el número inicial sea 0.
            assert_eq!(contract.retrieve_number(), 0);

            // Llamamos a increase_number y verificamos que el número aumente a 1.
            contract.increase_number();
            assert_eq!(contract.retrieve_number(), 1);

            // Llamamos a decrease_number y verificamos que el número regrese a 0.
            contract.decrease_number();
            assert_eq!(contract.retrieve_number(), 0);
        }

        #[ink::test]
        #[should_panic(expected = "Caller is not in the white list")]
        fn increase_only_whitelisted() {
            use ink::env::test;

            // Obtenemos las cuentas por defecto.
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            // Configuramos al owner (alice) y creamos el contrato.
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            let mut contract = SimpleCounter::new();

            // No agregamos a bob a la whitelist.
            // Cambiamos el caller a bob, que no está en la whitelist.
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            // Se espera que al intentar aumentar el contador se genere un panic
            // con el mensaje "Caller is not in the white list".
            contract.increase_number();
        }

        #[ink::test]
        #[should_panic(expected = "Caller is not in the white list")]
        fn decrease_only_whitelisted() {
            use ink::env::test;

            // Obtenemos las cuentas por defecto.
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            // Configuramos al owner (alice) y creamos el contrato.
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            let mut contract = SimpleCounter::new();

            // No agregamos a bob a la whitelist.
            // Cambiamos el caller a bob, que no está en la whitelist.
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            // Se espera que al intentar disminuir el contador se genere un panic
            // con el mensaje "Caller is not in the white list".
            contract.decrease_number();
        }

    }    
}
