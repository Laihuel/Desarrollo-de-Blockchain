//SPDX-License-Identifier: UNLICENSED

pragma solidity >=0.7.0 <0.9.0;


contract VotationSystem {

    event VotationFinished(int winner);

    address private owner;

    struct Candidate {
        int id;
        int votes;
    }

    // Lista dinámica de candidatos
    Candidate[] public candidates;

    mapping(address => bool) public whitelist;
    mapping(address => bool) public hasVoted;

    bool public votationFinished = false;

    int public winner;


    // modifier to check if caller is owner
    modifier isOwner() {
        // If the first argument of 'require' evaluates to 'false', execution terminates and all
        // changes to the state and to Ether balances are reverted.
        // This used to consume all gas in old EVM versions, but not anymore.
        // It is often a good idea to use 'require' to check if functions are called correctly.
        // As a second argument, you can also provide an explanation about what went wrong.
        require(msg.sender == owner, "Caller is not owner");
        _;
    }

    modifier isAllowed() {
        require(whitelist[msg.sender], "Caller is not on the white list");
        require(!hasVoted[msg.sender], "Caller has voted already");
        _;
    }

    modifier hasntFinished() {
        require(!votationFinished, "Votation has finished already");
        _;
    }


    /**
     * @dev Set contract deployer as owner
     */
    constructor() {
        owner = msg.sender; // 'msg.sender' is sender of current call, contract deployer for a constructor
    }

    // Votar por un candidato dado su ID
    function vote(int id) public isAllowed hasntFinished {
        for (uint i = 0; i < candidates.length; i++) {
            if (candidates[i].id == id) {
                candidates[i].votes++;
                hasVoted[msg.sender] = true;  // Marca que el votante ya ha votado
                break;
            }
        }
    }

    // Añadir un nuevo candidato
    function addCandidate(int _id) public isOwner {
        candidates.push(Candidate(_id, 0));  // Añade un candidato con 0 votos
    }

    function addVoter(address _address) public isOwner {
        whitelist[_address] = true;
        hasVoted[_address] = false;
    }

    function finishVotation() public isOwner hasntFinished {
        votationFinished = true;

        int maxVotes = -1;
        int winningCandidateID = 0;
        
        // Encuentra el candidato con más votos
        for (uint i = 0; i < candidates.length; i++) {
            if (candidates[i].votes > maxVotes) {
                maxVotes = candidates[i].votes;
                winningCandidateID = candidates[i].id;
            }
        }

        winner = winningCandidateID;

        emit VotationFinished(winner); // Emite el evento cuando se finaliza la votación

    }
    
}