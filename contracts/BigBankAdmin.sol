// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "interface/IBank.sol";

contract BigBankAdmin {
    receive() external payable { }
    function adminWithdraw(IBank bank, uint256 amount) public payable  {
        bank.withdraw(amount, payable(address(this)));
    }
}
