#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod message_wall {
    use ink::prelude::{
        string::String,
        vec::Vec,
        format,
    };
    use ink::storage::Mapping;

    use ink::env::{
        call::{build_call, ExecutionInput, Selector},
        DefaultEnvironment,
    };


    #[ink(storage)]
    pub struct MessageWall {
        user_contract: AccountId,
        message_count: u32, // Contador para el número de mensajes
        user_names: Mapping<u32, String>, // Mapping para guardar nombres de usuario
        message_texts: Mapping<u32, String>, // Mapping para guardar textos de mensajes
    }

    impl MessageWall {
        #[ink(constructor)]
        pub fn new(user_contract: AccountId) -> Self {
            let mut user_names = Mapping::default();
            let mut message_texts = Mapping::default();
            let mut message_count: u32 = 0; // Especificamos el tipo u32

            // Agregamos dos mensajes por defecto
            user_names.insert(message_count, &String::from("Admin"));
            message_texts.insert(message_count, &String::from("Welcome to the message wall!"));
            message_count = message_count.checked_add(1).expect("Overflow al incrementar message_count");

            user_names.insert(message_count, &String::from("Admin"));
            message_texts.insert(message_count, &String::from("Feel free to leave a message!"));
            message_count = message_count.checked_add(1).expect("Overflow al incrementar message_count");

            Self {
                user_contract,
                message_count,
                user_names,
                message_texts,
            }
        }

        // Hacemos `is_registered` privado
        fn is_registered(&self) -> bool {
            let caller = self.env().caller();

            // En modo de prueba, simulamos la respuesta del otro contrato
            #[cfg(test)]
            {
                if caller == ink::env::test::default_accounts::<DefaultEnvironment>().alice {
                    return true; // Simulamos que Alice está registrada
                } else {
                    return false; // Simulamos que otros usuarios no están registrados
                }
            }

            // En producción, llamamos al otro contrato
            #[cfg(not(test))]
            {
                /*
                true
                */
                
                let result: Option<Vec<u8>> = ink::env::call::build_call::<DefaultEnvironment>()
                    .call(self.user_contract)
                    .exec_input(
                        ink::env::call::ExecutionInput::new(ink::env::call::Selector::new(ink::selector_bytes!("get_user_name")))
                            .push_arg(caller),
                    )
                    .returns::<Option<Vec<u8>>>()
                    .invoke();
                result.is_some()
                
                /*
                let caller = self.env().caller();
                let result: Option<Vec<u8>> = ink::env::call::build_call::<DefaultEnvironment>()
                    .call(self.user_contract)
                    .exec_input(
                        ink::env::call::ExecutionInput::new(ink::env::call::Selector::new(ink::selector_bytes!("get_user_name")))
                            .push_arg(caller),
                    )
                    .returns::<Option<Vec<u8>>>()
                    .invoke();
                result.is_some()

                
                let result: Option<Vec<u8>> = ink::env::call::build_call::<DefaultEnvironment>()
                    .call(self.user_contract)
                    .exec_input(
                        ink::env::call::ExecutionInput::new(ink::env::call::Selector::new(ink::selector_bytes!("get_user_name")))
                            .push_arg(caller),
                    )
                    .returns::<Option<Vec<u8>>>()
                    .invoke();
                result.is_some()
                */
                
            }
        }

        #[ink(message)]
        pub fn get_last_message(&self) -> String {
            if self.message_count == 0 {
                return String::from("No messages available.");
            }

            let last_message_number = self.message_count.checked_sub(1).expect("Underflow al obtener el último mensaje");

            let user_name = self.user_names.get(last_message_number).unwrap_or_default();
            let message_text = self.message_texts.get(last_message_number).unwrap_or_default();

            format!("Message #{}: {} - {}", last_message_number, user_name, message_text)
        }
        
        #[ink(message)]
        pub fn get_last_10_messages(&self) -> Vec<String> {
            let mut messages = Vec::new();

            // Calculamos el índice inicial para los últimos 10 mensajes
            let start_index = if self.message_count > 10 {
                self.message_count.checked_sub(10).expect("Underflow al calcular el índice inicial")
            } else {
                0
            };

            // Iteramos sobre los últimos 10 mensajes (o menos si no hay suficientes)
            for i in start_index..self.message_count {
                let user_name = self.user_names.get(i).unwrap_or_default();
                let message_text = self.message_texts.get(i).unwrap_or_default();
                messages.push(format!("Message #{}: {} - {}", i, user_name, message_text));
            }

            messages
        }
            

        #[ink(message)]
        pub fn post_message(&mut self, user_name: String, message_text: String) {
            // Verificamos si el caller está registrado
            if !self.is_registered() {
                panic!("Caller is not registered.");
            }

            // Insertamos el nuevo mensaje
            self.user_names.insert(self.message_count, &user_name);
            self.message_texts.insert(self.message_count, &message_text);

            // Incrementamos el contador de mensajes
            self.message_count = self.message_count.checked_add(1).expect("Overflow al incrementar message_count");
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test;

        #[ink::test]
        fn test_is_registered() {
            // Configuramos las cuentas de prueba
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let user_contract = accounts.bob; // Usamos `bob` como el contrato de usuario
            let contract = MessageWall::new(user_contract);

            // Simulamos que el usuario "Alice" está registrado
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert!(contract.is_registered(), "Alice debería estar registrada");

            // Simulamos que otro usuario no está registrado
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.charlie);
            assert!(!contract.is_registered(), "Charlie no debería estar registrado");
        }

        #[ink::test]
        fn test_get_last_message() {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let user_contract = accounts.bob;
            let contract = MessageWall::new(user_contract);

            // Verificamos que el último mensaje sea el segundo mensaje cargado en el constructor
            let last_message = contract.get_last_message();
            assert!(last_message.contains("Feel free to leave a message!"));
        }

        #[ink::test]
        fn test_get_last_10_messages() {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let user_contract = accounts.bob;
            let mut contract = MessageWall::new(user_contract);

            // Agregamos más mensajes para probar la función
            for i in 2..15 {
                contract.user_names.insert(i, &String::from("User"));
                contract.message_texts.insert(i, &String::from(format!("Message {}", i)));
                contract.message_count += 1;
            }

            // Obtenemos los últimos 10 mensajes
            let last_10_messages = contract.get_last_10_messages();

            // Verificamos que hay 10 mensajes
            assert_eq!(last_10_messages.len(), 10);

            // Verificamos que el primer mensaje en la lista es el mensaje #5
            assert!(last_10_messages[0].contains("Message 5"));
        }

        #[ink::test]
        fn test_post_message() {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let user_contract = accounts.bob;
            let mut contract = MessageWall::new(user_contract);

            // Simulamos que el usuario "Alice" está registrado
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            // Publicamos un mensaje
            contract.post_message(String::from("Alice"), String::from("Hello, world!"));

            // Verificamos que el mensaje se haya agregado correctamente
            let last_message = contract.get_last_message();
            assert!(last_message.contains("Hello, world!"));
        }

        #[ink::test]
        #[should_panic(expected = "Caller is not registered.")]
        fn test_post_message_unregistered_user() {
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let user_contract = accounts.bob;
            let mut contract = MessageWall::new(user_contract);

            // Simulamos que el usuario "Charlie" no está registrado
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.charlie);

            // Intentamos publicar un mensaje (debería fallar)
            contract.post_message(String::from("Charlie"), String::from("I should not be able to post."));
        }
    }
}