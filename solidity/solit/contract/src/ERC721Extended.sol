// SPDX-License-Identifier: MIT
pragma solidity >=0.8.26;

import {ERC721} from "./ERC721.sol";
import {ERC20} from "./ERC20.sol";

contract ERC721Extended is ERC721 {
    // COUNTER STORAGE

    uint256 _counter;

    // EXTENDED ERC-721 STORAGE

    mapping(address => uint256) paymentMethods;

    constructor(
        string memory _name,
        string memory _symbol
    ) ERC721(_name, _symbol) {}

    // ERC-20 PAYMENT LOGIC
    function addPayment(address erc20Contract, uint256 price) public onlyOwner {
        require(price > 0, "NO FREE TOKEN");
        paymentMethods[erc20Contract] = price;
    }

    function removePayment(address erc20Contract) public onlyOwner {
        delete paymentMethods[erc20Contract];
    }

    // COUNTER LOGIC
    function current() public view returns (uint256) {
        return _counter;
    }

    // Overflow not realitically possible
    function increment() internal {
        unchecked {
            _counter += 1;
        }
    }

    function decrement() internal {
        require(_counter > 0, "Decrement underflow");

        unchecked {
            _counter -= 1;
        }
    }

    // CUSTOM MINT LOGIC

    function mint(address payment) public returns (bool) {
        require(paymentMethods[payment] > 0, "Invalid payment method");

        ERC20 erc20Contract = ERC20(payment);

        // Check allowance for this contract
        _checkAllowance(erc20Contract);

        // Safe transfer to this contract
        uint256 price = paymentMethods[payment];
        uint8 tokenDecimals = erc20Contract.decimals();
        bool result = erc20Contract.transferFrom(
            msg.sender,
            address(this),
            _tokenPrice(price, tokenDecimals)
        );

        require(result == true, "Transfer failed");

        // Mint
        increment();
        _safeMint(msg.sender, _counter);

        return true;
    }

    // Check allowance against NFT price
    function _checkAllowance(ERC20 tokenContract) internal view returns (bool) {
        uint256 price = paymentMethods[address(tokenContract)];
        uint256 allowed = tokenContract.allowance(msg.sender, address(this));
        uint8 tokenDecimals = tokenContract.decimals();

        require(allowed >= _tokenPrice(price, tokenDecimals));

        return true;
    }

    function _tokenPrice(
        uint256 price,
        uint8 decimals
    ) internal pure returns (uint256) {
        return price * (10 ** decimals);
    }
}
