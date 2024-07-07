// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.26;

import {Test, console} from "forge-std/Test.sol";
import {Counter, TransferOwnership} from "../src/Counter.sol";

contract CounterTest is Test {
    Counter public counter;

    function setUp() public {
        counter = new Counter(0);
    }

    function test_Increment() public {
        counter.inc();
        assertEq(counter.getCount(), 1);
    }

    function testFuzz_SetNumber(uint256 x) public {
        counter.setCount(x);
        assertEq(counter.getCount(), x);
    }

    function testFuzz_ChangeOwner(address newOwner) public {
        vm.expectEmit();
        emit TransferOwnership(address(this), newOwner);

        counter.changeOwner(newOwner);

        assertEq(newOwner, counter.getOwner());
    }
}
