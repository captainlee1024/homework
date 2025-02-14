// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "interface/IBank.sol";

// Bank 基础接口
contract Bank is IBank {
    // 管理员, 合约创建者, 部署合约时初始化, 不可修改
    address public admin;
    // user balance
    mapping(address => uint256) public ledgers;
    // rankThree
    address[3] public rankThree;

    modifier onlyAdmin() {
        require(msg.sender == admin);
        _;
    }

    constructor() payable  {
        admin = msg.sender;
    }

		// BigBank中重写deposit时添加BigBank中的存款额度限时modifier就可以实现作业功能
    receive() external payable virtual  {
        deposit();
    }

    function deposit() internal virtual {
        ledgers[msg.sender] = ledgers[msg.sender] + msg.value;
        updagteRank(msg.sender);
    }

    function updagteRank(address account) internal {
        uint256 topOneBalance = ledgers[rankThree[0]];
        uint256 topTwoBalance = ledgers[rankThree[1]];
        uint256 topThreeBalance = ledgers[rankThree[2]];

        uint256 newBalance = ledgers[account];

        if (newBalance > topOneBalance) {
            rankThree[2] = rankThree[1];
            rankThree[1] = rankThree[0];
            rankThree[0] = account;
        } else if ((account != rankThree[0]) && (newBalance > topTwoBalance)) {
            if (rankThree[1] != account) {
                rankThree[2] = rankThree[1];
                rankThree[1] = account;
            }
        } else if ((account != rankThree[0]) && (account != rankThree[1]) && (newBalance > topThreeBalance)) {
            rankThree[2] = account;
        }
    }

    function withdraw(uint256 amount, address payable to) public payable onlyAdmin {
        // require(ledgers[to] >= amount);
        // ledgers[to] = ledgers[to] - amount;
        to.transfer(amount);
    }

    function getRank()
        public
        view
        returns (
            address,
            uint256,
            address,
            uint256,
            address,
            uint256
        )
    {
        return (
            rankThree[0],
            ledgers[rankThree[0]],
            rankThree[1],
            ledgers[rankThree[1]],
            rankThree[2],
            ledgers[rankThree[2]]
        );
    }
}

