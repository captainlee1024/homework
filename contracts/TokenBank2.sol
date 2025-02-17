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

    error ERC20OnCheckeceived();

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

    function transferAndCall(
        address _to,
        uint256 _value
    ) public virtual returns (bool) {
        if(!transfer(_to, _value)) {
            revert();
        }
        _checkOnErc20Received(msg.sender, msg.sender, _to, _value);
        return true;
    }

    function _checkOnErc20Received(address _msgSender, address _from, address _to, uint256 _amount) private {
        bytes memory payload = abi.encodeWithSignature("onErc20Received(address,address,uint256)", _msgSender, _from, _amount);
        (bool success, bytes memory resultBytes ) = _to.call(payload);
        //abi.decode(result, (bool))
        bool result = abi.decode(resultBytes, (bool));
        require(result, "onErc20Received: execution failed");
        require(success, "ERC20 check received failed");
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
        receive() external payable { }

}


contract TokenBank {
    // 用户地址 -> (代币地址 -> 余额): 不同发行方的ERC20代币价值不同，使用该代币地址进行区分
    mapping(address => mapping(address => uint256)) public balances;
    // 用于限制只支持哪些Token
    mapping(address => bool) public supportsTokenAddr;    
    address immutable bankAdmin;

    receive() external payable { }

    constructor(address _bankAdmin)  {
        bankAdmin = _bankAdmin;
    }
    
    function supportNewToken(address newTokenAddr) public {
        require(msg.sender == bankAdmin, "not admin");
        require(!supportsTokenAddr[newTokenAddr], "already supported");
        supportsTokenAddr[newTokenAddr] = true;
    }

    // onErc20Received(address,address,uint256)
    function onErc20Received(
        address operator,
        address from,
        uint256 amount
    ) public returns (bool success) {
        require(supportsTokenAddr[msg.sender], "OnReceiveERC20 failed, invalid ERC20addr");
        balances[from][msg.sender] += amount;
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
