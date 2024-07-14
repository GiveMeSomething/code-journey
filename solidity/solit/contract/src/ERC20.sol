// SPDX-License-Identifier: MIT
pragma solidity >=0.8.26;

contract ERC20 {
    // EVENTS
    event Transfer(address indexed from, address indexed to, uint256 amount);

    event Approval(
        address indexed owner,
        address indexed spender,
        uint256 amount
    );

    // METADATA STORAGE
    string public name;

    string public symbol;

    uint8 public immutable decimals;

    // ERC20 STORAGE
    uint256 public totalSupply;

    mapping(address => uint256) public balanceOf;

    mapping(address => mapping(address => uint256)) public allowance;

    // CUSTOM
    address private owner;

    constructor(string memory _name, string memory _symbol, uint8 _decimals) {
        name = _name;
        symbol = _symbol;
        decimals = _decimals;

        owner = msg.sender;
    }

    function transfer(
        address to,
        uint256 amount
    ) public validAddress(msg.sender) validAddress(to) returns (bool) {
        balanceOf[msg.sender] -= amount;

        unchecked {
            balanceOf[to] += amount;
        }

        emit Transfer(msg.sender, to, amount);

        return true;
    }

    function transferFrom(
        address from,
        address to,
        uint256 amount
    ) public validAddress(to) returns (bool) {
        uint256 allowedAmount = allowance[from][msg.sender];

        require(allowedAmount >= amount, "INSUFFICIENT FUND");

        if (allowedAmount != type(uint256).max) {
            allowance[from][msg.sender] = allowedAmount - amount;
        }

        balanceOf[from] -= amount;

        unchecked {
            balanceOf[to] += amount;
        }

        emit Transfer(from, to, amount);

        return true;
    }

    function approve(
        address spender,
        uint256 amount
    ) public validAddress(spender) returns (bool) {
        allowance[msg.sender][spender] = amount;

        emit Approval(msg.sender, spender, amount);

        return true;
    }

    // OWNER LOGIC

    modifier onlyOwner() {
        require(msg.sender == owner);
        _;
    }

    modifier validAddress(address _address) {
        require(_address != address(0), "Not a valid address");
        _;
    }

    function changeOwner(
        address _newOwner
    ) public onlyOwner validAddress(_newOwner) {
        owner = _newOwner;
    }

    // MINT/BURN LOGIC

    function mint(
        address to,
        uint256 amount
    ) public onlyOwner validAddress(to) {
        totalSupply += amount;

        unchecked {
            balanceOf[to] += amount;
        }

        emit Transfer(address(0), to, amount);
    }

    function burn(uint256 amount) public {
        balanceOf[msg.sender] -= amount;

        unchecked {
            totalSupply -= amount;
        }

        emit Transfer(msg.sender, address(0), amount);
    }
}
