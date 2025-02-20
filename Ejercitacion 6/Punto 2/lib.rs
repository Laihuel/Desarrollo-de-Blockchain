#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod personal_wallet {
    use ink::prelude::string::ToString;
    use ink::prelude::string::String;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct PersonalWallet {
        owner: AccountId,
        balance: Balance,
    }

    #[ink(event)]
    pub struct CoinReceived {
        #[ink(topic)]
        sender: AccountId,
        amount: Balance,
    }

    impl PersonalWallet {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self { owner : caller,
                    balance : 0 }
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) {
            let caller = self.env().caller();
            let amount = self.env().transferred_value();

            self.balance = self.balance.checked_add(amount).ok_or("Desbordamiento en el balance").unwrap();

            self.env().emit_event(CoinReceived { sender: caller, amount });
        }

        #[ink(message)]
        pub fn send_coin(&mut self, to: AccountId, amount: Balance) -> Result<(), String> {
            let caller = self.env().caller();

            // Verificar si el llamador es el propietario
            if caller != self.owner {
                return Err("Caller is not owner".into());
            }

            // Verificar si el contrato tiene suficiente saldo
            if self.balance < amount {
                return Err("Balance insuficiente".into());
            }

            // Intentar la transferencia
            self.env()
                .transfer(to, amount)
                .map_err(|_e| "Transfer failed".to_string())?;

            // Actualizar el balance del contrato
            self.balance = self.balance.checked_sub(amount).ok_or("Subdesbordamiento en el balance").unwrap();

            Ok(())
        }



        #[ink(message)]
        pub fn get_balance(&self) -> Balance {
            self.balance
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
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Configurar el balance inicial del contrato en 0
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.alice, 0);
            let simple_counter = PersonalWallet::new();
            assert_eq!(simple_counter.get_balance(), 0);
        }

        #[ink::test]
        fn test_deposit_from_owner() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Vaciar el balance de Alice (no es necesario pero lo puedes hacer para estar seguro)
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.alice, 0);
            let mut wallet = PersonalWallet::new();
            assert_eq!(wallet.get_balance(), 0);

            // Cargar fondos a Alice
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.alice, 9000000);

            // Simular transferencia de 2000000 unidades al contrato
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(2000000);

            // Establecer a Alice como el "caller" para llamar a la función `deposit`
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            // Llamar a la función `deposit` para recibir los fondos
            wallet.deposit();
            
            // Verificar el balance del contrato
            assert_eq!(wallet.get_balance(), 2000000);
        }


        
        #[ink::test]
        fn test_deposit_from_non_owner() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            // Configurar el balance inicial del contrato en 0
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.alice, 0);
            let mut wallet = PersonalWallet::new();
            
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(50);
            wallet.deposit();
            
            assert_eq!(wallet.get_balance(), 50);
        }

        #[ink::test]
        fn test_send_coin_from_owner() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            // Configurar el balance inicial del contrato en 0
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.alice, 0);
            let mut wallet = PersonalWallet::new();

            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.alice, 1000000);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
            wallet.deposit();
            
            let result = wallet.send_coin(accounts.bob, 50);
            assert!(result.is_ok());
            assert_eq!(wallet.get_balance(), 50);
        }

        #[ink::test]
        fn test_send_coin_from_non_owner() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            // Configurar el balance inicial del contrato en 0
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.alice, 0);
            let mut wallet = PersonalWallet::new();
            
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
            wallet.deposit();
            
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            let result = wallet.send_coin(accounts.charlie, 50);
            assert!(result.is_err());
            assert_eq!(wallet.get_balance(), 100);
        }

        #[ink::test]
        fn test_send_coin_transfer_failure() {

            /* 
                En realidad este test lo hice tratando de generar que la transacción falle y valla
                por la linea: .map_err(|e| format!("Transfer failed: {:?}", e))?;
                Que es la que tarpaulin me marca que me falta recorrer para llegar
                al 100% de coverage, pero no lo logre
            */


            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Configurar el balance inicial del contrato en 0
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.alice, 0);
            // Crear el contrato con saldo suficiente para realizar la transferencia
            let mut wallet = PersonalWallet::new();
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.alice, 1000000);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            wallet.deposit(); // Depositar 1000 en el contrato

            // Intentar realizar una transferencia de 500, que debe ser exitosa
            let result = wallet.send_coin(accounts.bob, 500);

            // Aquí verificamos si la transferencia fue exitosa
            assert_eq!(result, Ok(()));

            // Intentar realizar una transferencia con un monto mayor que el saldo (esto debería fallar)
            let result_fail = wallet.send_coin(accounts.bob, 2000);

            // Verificamos que el error proviene de la transferencia fallida
            assert_eq!(result_fail, Err("Balance insuficiente".to_string()));
        }


        
    }    


    
}
