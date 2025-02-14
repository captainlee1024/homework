// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// 功能：
// 1. 钱包可以直接转账 -> 实现receive函数
// 2. 获取存款前三的用户
interface IBank {
    function withdraw(uint256 amount, address payable to) external payable ;
    // function deposit() external payable;
    function getRank()
        external
        view
        returns (
            address,
            uint256,
            address,
            uint256,
            address,
            uint256
        );

}
