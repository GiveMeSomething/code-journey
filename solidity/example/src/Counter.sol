// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.26;

contract Counter {
    address public immutable OWNER;
    uint256 public count;

    constructor(uint256 _initCount) {
        count = _initCount;
        OWNER = msg.sender;
    }

    function getOwner() public view returns (address) {
        return OWNER;
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
