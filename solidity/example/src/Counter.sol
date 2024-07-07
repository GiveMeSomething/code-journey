// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Counter {
    uint256 public count;

    function get() public view returns (uint256) {
        return count;
    }

    function inc() public {
        count += 1;
    }

    function desc() public {
        // This function will fail if count = 0, as count is an unsigned integer
        count -= 1;
    }
}
