// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test, console} from "forge-std/Test.sol";
import {ERC20} from "../src/ERC20.sol";

contract ERC20Test is Test {
    ERC20 public token;

    address alice = vm.addr(0x1);
    address bob = vm.addr(0x2);

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

    function testApprove() public {
        vm.prank(alice);

        token.approve(bob, type(uint256).max);

        assertEq(token.allowance(alice, bob), type(uint256).max);
    }

    function testTransferFrom() public {
        // Init fund for Alice
        token.mint(alice, 10e18);

        assertEq(token.balanceOf(alice), 10e18);
        assertEq(token.totalSupply(), 10e18);

        // Alice approve Bob to use all tokens
        vm.prank(alice);

        token.approve(bob, 10e18);

        assertEq(token.allowance(alice, bob), 10e18);

        // Try transferFrom Alice
        vm.prank(bob);

        assertTrue(token.transferFrom(alice, address(this), 3e18));
        assertEq(token.balanceOf(alice), 7e18);
        assertEq(token.balanceOf(address(this)), 3e18);
        assertEq(token.allowance(alice, bob), 7e18);

        vm.prank(bob);
        assertTrue(token.transferFrom(alice, bob, 7e18));
        assertEq(token.balanceOf(alice), 0);
        assertEq(token.balanceOf(bob), 7e18);
        assertEq(token.allowance(alice, bob), 0);
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

    function testFailApproveWithZeroAllowance() external {
        vm.prank(alice);

        token.approve(address(bob), 0);
    }

    function testFailTransferFromNotApproved() external {
        token.mint(address(this), 10e18);
        token.transferFrom(address(this), alice, 10e18);
    }

    function testFailTransferFromInsufficientFund() external {
        token.mint(address(this), 10e18);
        token.approve(alice, 10e18);

        vm.prank(alice);
        token.transferFrom(address(this), bob, 20e18);
    }

    function testFuzzMint(address to, uint256 amount) external {
        vm.assume(to != address(0));
        token.mint(to, amount);

        assertEq(amount, token.balanceOf(to));
        assertEq(amount, token.totalSupply());
    }

    function testFuzzBurn(
        address from,
        uint256 mintAmount,
        uint256 burnAmount
    ) external {
        vm.assume(from != address(0));

        // Limit burn amount to mint amount (burnAmount <= mintAmount always)
        burnAmount = bound(burnAmount, 0, mintAmount);

        token.mint(from, mintAmount);

        vm.prank(from);
        token.burn(burnAmount);

        assertEq(token.balanceOf(from), mintAmount - burnAmount);
        assertEq(token.totalSupply(), mintAmount - burnAmount);
    }

    function testFuzzTransfer(address to, uint256 amount) external {
        vm.assume(to != address(0));
        vm.assume(address(this) != to);

        token.mint(address(this), amount);
        bool transfered = token.transfer(to, amount);
        assertTrue(transfered);

        assertEq(token.balanceOf(address(this)), 0);
        assertEq(token.balanceOf(to), amount);
        assertEq(token.totalSupply(), amount);
    }

    function testFuzzApprove(
        address from,
        address to,
        uint256 amount
    ) external {
        // Setup
        vm.assume(from != address(0));
        vm.assume(to != address(0));
        vm.assume(from != to);
        vm.assume(amount > 0);

        vm.prank(from);

        token.approve(to, amount);

        assertEq(token.allowance(from, to), amount);
    }

    function testFuzzTransferFrom(
        address owner,
        address spender,
        address receiver,
        uint256 amount
    ) external {
        vm.assume(
            owner != address(0) &&
                spender != address(0) &&
                receiver != address(0)
        );
        vm.assume(owner != spender && owner != receiver && spender != receiver);
        vm.assume(amount > 0 && amount != type(uint256).max);

        token.mint(owner, amount);

        assertEq(token.balanceOf(owner), amount);
        assertEq(token.totalSupply(), amount);

        // Alice approve Bob to use all tokens
        vm.prank(owner);

        token.approve(spender, amount);

        assertEq(token.allowance(owner, spender), amount);

        // Try transferFrom Alice
        vm.prank(spender);

        assertTrue(token.transferFrom(owner, receiver, amount));
        assertEq(token.balanceOf(owner), 0);
        assertEq(token.balanceOf(receiver), amount);
        assertEq(token.allowance(owner, spender), 0);
    }
}
