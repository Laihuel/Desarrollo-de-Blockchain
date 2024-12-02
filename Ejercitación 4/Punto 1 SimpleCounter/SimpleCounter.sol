//SPDX-License-Identifier: UNLICENSED

pragma solidity >=0.7.0 <0.9.0;

contract SimpleCounter {

    address private owner;

    uint256 number;

    mapping(address => bool) public whiteList;

    // event for EVM logging
    event NewValue(address indexed sender, uint256 newNumber);

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

    modifier isInWhiteList() {
        require(whiteList[msg.sender], "Caller is not in the white list");
        _;
    }

    modifier notToNegative() {
        require(number > 0, "Counter cannot be negative");
        _;
    }

    /**
     * @dev Set contract deployer as owner
     */
    constructor() {
        owner = msg.sender; // 'msg.sender' is sender of current call, contract deployer for a constructor
        addToWhiteList(msg.sender);
    }

    // Función para agregar direcciones a la whitelist
    function addToWhiteList(address _address) public isOwner {
        whiteList[_address] = true;
    }

    // Función para quitar direcciones de la whitelist
    function removeFromWhiteList(address _address) public isOwner {
        whiteList[_address] = false;
    }

    //Función para incrementar el contador
    function increaseNumber() public isInWhiteList {
        number++;
        emit NewValue(msg.sender, number);
    }

    //Función para decrementar el contador
    function decreaseNumber() public isInWhiteList notToNegative() {
        number--;
        emit NewValue(msg.sender, number);
    }

    // Función para leer el número
    function retrieveNumber() public view isInWhiteList returns (uint256) {
        return number;
    }

}