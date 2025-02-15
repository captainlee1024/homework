// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;


contract BaseERC20 {
    string public name; 
    string public symbol; 
    uint8 public decimals; 

    uint256 public totalSupply; 

    mapping (address => uint256) balances; 

    mapping (address => mapping (address => uint256)) allowances; 

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    constructor() {
        name = "BaseERC20";  
        symbol = "BERC20"; 
        decimals = 18;
        totalSupply = 100000000000000000000000000;
        balances[msg.sender] = totalSupply;  
    }

    function balanceOf(address _owner) public view returns (uint256 balance) {
        return balances[_owner];
    }

    // transfer(address,uint256)
    function transfer(address _to, uint256 _value) public returns (bool success) {
        require(balances[msg.sender] >= _value, "ERC20: transfer amount exceeds balance");

        bool _success = transferFrom(msg.sender, _to, _value);
        require(_success);

        emit Transfer(msg.sender, _to, _value);  
        return true;   
    }

    function transferFrom(address _from, address _to, uint256 _value) public returns (bool success) {
        // if ((msg.sender != _from) && (allowances[_from][msg.sender] >= 0)) {
        if (msg.sender != _from) {
            require(allowances[_from][msg.sender] >= _value, "ERC20: transfer amount exceeds allowance");
            allowances[_from][msg.sender] -= _value;
        }
        
        require(balances[_from] >= _value, "ERC20: transfer amount exceeds balance");

        balances[_from] -= _value;
        balances[_to] += _value;
        
        emit Transfer(_from, _to, _value); 
        return true; 
    }

    function approve(address _spender, uint256 _value) public returns (bool success) {
        // require(balances[msg.sender] >= (allowances[msg.sender][_spender] + _value), "ERC20: approve amount exceeds balance");
        allowances[msg.sender][_spender] += _value;

        emit Approval(msg.sender, _spender, _value); 
        return true; 
    }

    function allowance(address _owner, address _spender) public view returns (uint256 remaining) {   
        return allowances[_owner][_spender];
    }

    // receive() external payable { }
}


contract TokenBank {
    // 用户地址 -> (代币地址 -> 余额): 不同发行方的ERC20代币价值不同，使用该代币地址进行区分
    mapping(address => mapping(address => uint256)) public balances;

    constructor()  {

    }
    receive() external payable { }

    // Deposit 通过 delegatecall 调用 BaseERC20 的 transfer 函数
    // function deposit(BaseERC20 baseERC20, uint256 amount) public returns(bool) {
    //     bytes memory payload = abi.encodeWithSignature("transfer(address,uint256)", address(this), amount);
    //     (bool success, bytes memory result) = address(baseERC20).delegatecall(payload);
    //     require(success, "TokenBank.deposit: delegatecall failed");
    //     require(abi.decode(result, (bool)), "TokenBank.deposit: delegatecall transfer failed");

    //     // update bank data
    //     balances[msg.sender][address(baseERC20)] += amount;

    //     return true;
    // }

    // TODO: 为什么transfer方法使用 delegatecall成功不了，但是call调用withdraw方法可以
    // 当前实现为调用transferFrom方法, 需要先approve
    function deposit(BaseERC20 baseERC20, uint256 amount) public returns(bool) {
        baseERC20.transferFrom(msg.sender, address(this), amount);

        // 更新 bank 数据
        balances[msg.sender][address(baseERC20)] += amount;
        return true;
    }

    function withdraw(address baseERC20, uint256 amount) public returns(bool) {
        require(balances[msg.sender][baseERC20] >= amount, "TokanBank: withdraw failed");
        bytes memory payload = abi.encodeWithSignature("transfer(address,uint256)", msg.sender, amount);
        (bool success, bytes memory result) = baseERC20.call(payload);
        require(success, "TokenBank.withdraw: call failed");
        require(abi.decode(result, (bool)), "TokenBank.withdraw: call transfer failed");

        // update bank data
        balances[msg.sender][address(baseERC20)] -= amount;

        return true;
    }
}