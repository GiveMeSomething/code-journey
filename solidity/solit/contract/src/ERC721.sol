// SPDX-License-Identifier: MIT
pragma solidity >=0.8.26;

import {ERC721TokenReceiver} from "./ERC721TokenReceiver.sol";

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

    modifier mustBeSafe(
        address _address,
        uint256 tokenId,
        bytes memory data
    ) {
        _;

        // Require _address to be not be a contract, or
        // Be a contract and adhere to ERC721TokenReceiver interface/abstract contract
        require(
            _address.code.length == 0 ||
                ERC721TokenReceiver(_address).onERC721Received(
                    msg.sender,
                    address(0),
                    tokenId,
                    data
                ) ==
                ERC721TokenReceiver.onERC721Received.selector,
            "UNSAFE_RECIPIENT"
        );
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

    // ERC721 APPROVAL STORAGE
    mapping(uint256 => address) public getApproved;

    mapping(address => mapping(address => bool)) public isApprovedForAll;

    constructor(string memory _name, string memory _symbol) {
        name = _name;
        symbol = _symbol;

        owner = msg.sender;
    }

    // Authorize another address to use one's token
    function approve(address spender, uint256 tokenId) public {
        address tokenOwner = _ownerOf[tokenId];
        require(
            msg.sender == tokenOwner || isApprovedForAll[owner][msg.sender],
            "NOT_AUTHORIZED"
        );

        getApproved[tokenId] = spender;

        emit Approval(tokenOwner, spender, tokenId);
    }

    // Authorize another address to use all of one's tokens
    function setApprovalForAll(address operator, bool approved) public {
        isApprovedForAll[msg.sender][operator] = approved;

        emit ApprovalForAll(msg.sender, operator, approved);
    }

    // Transfer logic
    function transferFrom(
        address from,
        address to,
        uint256 tokenId
    ) public validAddress(to) {
        require(from == _ownerOf[tokenId], "NOT_AUTHORIZED");

        // Is owner of the token
        // Is authorized to use the token
        // Is authorized by the "from" to use all of their tokens
        require(
            msg.sender == from ||
                msg.sender == getApproved[tokenId] ||
                isApprovedForAll[from][msg.sender]
        );

        unchecked {
            _balanceOf[from]--;
            _balanceOf[to]++;
        }

        // Change token owner
        _ownerOf[tokenId] = to;

        // Revoke approval of the current token
        delete getApproved[tokenId];

        emit Transfer(from, to, tokenId);
    }

    function safeTransferFrom(
        address from,
        address to,
        uint256 tokenId
    ) public mustBeSafe(to, tokenId, "") {
        transferFrom(from, to, tokenId);
    }

    function safeTransferFrom(
        address from,
        address to,
        uint256 tokenId,
        bytes memory data
    ) public mustBeSafe(to, tokenId, data) {
        transferFrom(from, to, tokenId);
    }
}
