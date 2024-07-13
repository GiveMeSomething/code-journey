// SPDX-License-Identifier: MIT
pragma solidity >=0.8.26;

import {IERC721} from "./interfaces/IERC721.sol";

contract ERC721 {
    // EVENTS
    event Transfer(
        address indexed from,
        address indexed to,
        uint256 indexed tokenId
    );

    event Approval(
        address indexed from,
        address indexed to,
        uint256 indexed tokenId
    );

    event ApprovalForAll(
        address indexed owner,
        address indexed operator,
        bool approved
    );

    // METADATA STORAGE
    string public name;

    string public symbol;

    // MODIFIERS
    modifier onlyOwner() {
        require(msg.sender == owner);

        _;
    }

    modifier validAddress(address _address) {
        require(_address != address(0));

        _;
    }

    // CONTRACT OWNER
    address internal owner;

    function changeOwner(address newOwner) public validAddress(newOwner) {
        owner = newOwner;
    }

    // ERC-721 BALANCE/OWNER STORAGE

    mapping(uint256 => address) internal _ownerOf;

    mapping(address => uint256) internal _balanceOf;

    function ownerOf(uint256 tokenId) public view returns (address tokenOwner) {
        require((tokenOwner = _ownerOf[tokenId]) != address(0), "NOT MINTED");
    }

    function balanceOf(
        address tokenOwner
    ) public view validAddress(tokenOwner) returns (uint256) {
        return _balanceOf[tokenOwner];
    }
}
