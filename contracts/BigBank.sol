// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "Bank.sol";

contract BigBank is Bank {
    address public immutable owner;
    modifier onlyOwner() {
        require(msg.sender == owner, "Forbidden");
        _;
    }

    modifier lowDeposit() {
        require(msg.value > 1000000000000000, "LowDeposit");
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    function updateBankAdmin(address newAdmin) external onlyOwner {
        admin = newAdmin;
    }

    function deposit() internal override lowDeposit {
        super.deposit();
    } 
}

