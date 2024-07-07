// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.26;

event TransferOwnership(address indexed from, address indexed to);

contract Counter {
    address public owner;
    uint256 public count;
    bool public locked;

    constructor(uint256 _initCount) {
        count = _initCount;
        owner = msg.sender;
    }

    modifier onlyOwner() {
        require(msg.sender == owner);
        _;
    }

    modifier validAddress(address _address) {
        require(_address != address(0), "Not a valid address");
        _;
    }

    modifier noReentrancy() {
        require(!locked, "No reentrancy");

        locked = true;
        _;
        locked = false;
    }

    function changeOwner(
        address _newOwner
    ) public onlyOwner validAddress(_newOwner) {
        emit TransferOwnership(msg.sender, _newOwner);
        owner = _newOwner;
    }

    function getOwner() public view returns (address) {
        return owner;
    }

    function getCount() public view returns (uint256) {
        return count;
    }

    function setCount(uint256 _count) public {
        count = _count;
    }

    function inc() public {
        count += 1;
    }

    function desc() public {
        if (count == 0) {
            return;
        }

        // This function will fail if count = 0, as count is an unsigned integer
        count -= 1;
    }
}
