#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod votation_system {
    use ink::storage::Mapping;
    use ink::scale::{Encode, Decode};
    use ink::prelude::vec::Vec;
    use scale_info::TypeInfo;

    #[ink(event)]
    pub struct VotationFinished {
        #[ink(topic)]
        winner: i32,
    }

    #[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, Debug)]
    pub struct Candidate {
        pub id: i32,
        pub votes: i32,
    }

    #[ink(storage)]
    pub struct VotationSystem {
        owner: AccountId,
        candidates: Vec<Candidate>,
        whitelist: Mapping<AccountId, bool>,
        has_voted: Mapping<AccountId, bool>,
        votation_finished: bool,
        winner: i32,
    }

    impl VotationSystem {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self { 
                owner : caller,
                candidates: Vec::new(),
                whitelist: Mapping::new(),
                has_voted: Mapping::new(),
                votation_finished: false,
                winner: -1,                
            }
        }

        fn ensure_owner(&self) {
            assert_eq!(self.env().caller(), self.owner, "Only owner can call this function");
        }

        #[ink(message)]
        pub fn vote(&mut self, id: i32) {
            let caller = self.env().caller();
            assert!(self.whitelist.get(caller).unwrap_or(false), "Caller is not on the white list");
            assert!(!self.has_voted.get(caller).unwrap_or(false), "Caller has voted already");
            assert!(!self.votation_finished, "Votation has finished already");

            for candidate in &mut self.candidates {
                if candidate.id == id {
                    candidate.votes = candidate.votes
                                        .checked_add(1)
                                        .expect("Overflow en increase_number");
                    self.has_voted.insert(caller, &true);
                    break;
                }
            }
        }

        #[ink(message)]
        pub fn add_candidate(&mut self, id: i32) {
            self.ensure_owner();
            self.candidates.push(Candidate { id, votes: 0 });
        }

        #[ink(message)]
        pub fn add_voter(&mut self, voter: AccountId) {
            self.ensure_owner();
            self.whitelist.insert(voter, &true);
            self.has_voted.insert(voter, &false);
        }

        #[ink(message)]
        pub fn finish_votation(&mut self) {
            self.ensure_owner();
            assert!(!self.votation_finished, "Votation has finished already");

            let mut max_votes = 0;
            let mut winning_candidate_id = -1;

            for candidate in &self.candidates {
                if candidate.votes > max_votes {
                    max_votes = candidate.votes;
                    winning_candidate_id = candidate.id;
                }
            }

            self.winner = winning_candidate_id;
            self.votation_finished = true;
            self.env().emit_event(VotationFinished { winner: self.winner });
        }


    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink::env::{test, DefaultEnvironment};

        #[ink::test]
        fn basic_votation_flow() {
            // 1. Crear el contrato y verificar el owner
            let accounts = test::default_accounts::<DefaultEnvironment>();
            test::set_caller::<DefaultEnvironment>(accounts.alice);
            let mut contract = VotationSystem::new();
            assert_eq!(contract.owner, accounts.alice);

            // 2. Agregar candidatos
            contract.add_candidate(1);
            contract.add_candidate(2);

            // 3. Agregar votantes
            contract.add_voter(accounts.bob);
            contract.add_voter(accounts.charlie);
            contract.add_voter(accounts.django);

            // 4. Votaciones
            test::set_caller::<DefaultEnvironment>(accounts.bob);
            contract.vote(1);

            test::set_caller::<DefaultEnvironment>(accounts.charlie);
            contract.vote(2);

            test::set_caller::<DefaultEnvironment>(accounts.django);
            contract.vote(1);

            // 5. Finalizar votación
            test::set_caller::<DefaultEnvironment>(accounts.alice);
            contract.finish_votation();

            // 6. Verificar que la votación terminó y que el ganador es el candidato 1
            assert!(contract.votation_finished);
            assert_eq!(contract.winner, 1);
        }

        #[ink::test]
        #[should_panic(expected = "Caller is not on the white list")]
        fn vote_not_whitelisted() {
            let accounts = test::default_accounts::<DefaultEnvironment>();
            let mut contract = VotationSystem::new();
            contract.add_candidate(1);
            test::set_caller::<DefaultEnvironment>(accounts.bob);
            contract.vote(1);
        }

        #[ink::test]
        #[should_panic(expected = "Caller has voted already")]
        fn vote_twice() {
            let accounts = test::default_accounts::<DefaultEnvironment>();
            let mut contract = VotationSystem::new();
            contract.add_candidate(1);
            contract.add_voter(accounts.bob);
            test::set_caller::<DefaultEnvironment>(accounts.bob);
            contract.vote(1);
            contract.vote(1);
        }

        #[ink::test]
        #[should_panic(expected = "Only owner can call this function")]
        fn non_owner_finishes_votation() {
            let accounts = test::default_accounts::<DefaultEnvironment>();
            let mut contract = VotationSystem::new();
            test::set_caller::<DefaultEnvironment>(accounts.bob);
            contract.finish_votation();
        }

    }

}
