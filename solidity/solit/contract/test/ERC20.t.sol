// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.26;

import {Test, console} from "forge-std/Test.sol";
import {ERC20} from "../src/ERC20.sol";

contract ERC20721Test is Test {
    ERC20 public token;

    address alice = vm.addr(0x1);
    address bob = vm.addr(0x2);

    // Init 2 account Bob + Alice

    function setUp() public {
        token = new ERC20("TastyFish", "TAFI", 18);
    }

    function testName() public view {
        assertEq("TastyFish", token.name());
    }

    function testSymbol() public view {
        assertEq("TAFI", token.symbol());
    }

    function testDecimals() public view {
        assertEq(18, token.decimals());
    }

    function testMint() public {
        token.mint(alice, 2e18);
        assertEq(token.balanceOf(alice), 2e18);
    }

    function testBurn() public {
        address contractAddress = address(this);

        token.mint(contractAddress, 10e18);
        assertEq(token.balanceOf(contractAddress), 10e18);

        token.burn(8e18);
        assertEq(token.balanceOf(contractAddress), 2e18);
        assertEq(token.totalSupply(), 2e18);
    }

    function testTransfer() public {
        // Mint for Alice
        token.mint(alice, 10e18);
        assertEq(token.balanceOf(alice), 10e18);

        // Set msg sender to Alice
        vm.prank(alice);

        token.transfer(bob, 5e18);
        assertEq(token.balanceOf(alice), 5e18);
        assertEq(token.balanceOf(bob), 5e18);
    }

    function testFailMintToZero() external {
        token.mint(address(0), 10e18);
    }

    function testFailBurnInsufficientBalance() external {
        // Mint for Alice
        token.mint(alice, 2e18);
        assertEq(token.balanceOf(alice), 2e18);

        // Set msg sender to Alice
        vm.prank(alice);

        token.burn(3e18);
    }

    function testFailTransferFromZeroAddress() external {
        // Mint for Alice
        token.mint(alice, 10e18);
        assertEq(token.balanceOf(alice), 10e18);

        // Set msg sender to Alice
        vm.prank(alice);

        // Alice burn => transfer to address(0)
        token.burn(10e18);

        // Act as address(0)
        vm.prank(address(0));
        token.transfer(alice, 1e18);
    }

    function testFailTransferInsufficientBalance() external {
        // Mint for Alice
        token.mint(alice, 2e18);
        assertEq(token.balanceOf(alice), 2e18);

        // Set msg sender to Alice
        vm.prank(alice);

        token.transfer(bob, 3e18);
    }
}
