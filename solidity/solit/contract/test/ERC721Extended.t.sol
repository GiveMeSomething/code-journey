// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test, console} from "forge-std/Test.sol";
import {ERC20} from "../src/ERC20.sol";
import {ERC721Extended} from "../src/ERC721Extended.sol";

contract ERC721ExtendedTest is Test {
    ERC20 public erc20Token;

    ERC721Extended public erc721Token;

    address alice = address(0x1);

    function setUp() public {
        erc20Token = new ERC20("TastyFish", "TAFI", 18);

        erc721Token = new ERC721Extended("TastyFish", "TastyNFT");

        // Set mint price to 1 TAFI
        erc721Token.addPayment(address(erc20Token), 1e18);
    }

    // Test contract setup
    function testName() public view {
        assertEq("TastyFish", erc721Token.name());
    }

    function testSymbol() public view {
        assertEq("TastyNFT", erc721Token.symbol());
    }

    function testCheckAllowance() external {
        // Mint for alice
        erc20Token.mint(address(this), 1e18);

        assertEq(erc20Token.balanceOf(address(this)), 1e18);
        assertEq(erc20Token.totalSupply(), 1e18);

        // Approve fund for erc721
        assertTrue(erc20Token.approve(address(erc721Token), 1e18));

        // Check allowance
        assertTrue(erc721Token.checkAllowance(erc20Token));
    }
}
