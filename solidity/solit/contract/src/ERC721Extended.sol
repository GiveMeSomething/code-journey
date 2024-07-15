// SPDX-License-Identifier: MIT
pragma solidity >=0.8.26;

import {ERC721} from "./ERC721.sol";
import {ERC20} from "./ERC20.sol";

contract ERC721Extended is ERC721 {
    // COUNTER STORAGE

    uint256 _counter;

    // EXTENDED ERC-721 STORAGE

    // This struct will support payment in decimals
    // ERC20 price = price * (10 ** (ERC20 decimals - decimals))
    struct Payment {
        uint256 price;
        uint8 decimals;
    }

    mapping(address => Payment) paymentMethods;

    constructor(
        string memory _name,
        string memory _symbol
    ) ERC721(_name, _symbol) {}

    // ERC-20 PAYMENT LOGIC
    function addPayment(
        address erc20Contract,
        uint256 price,
        uint8 decimals
    ) public onlyOwner {
        require(price > 0, "NO FREE TOKEN");

        ERC20 tokenContract = ERC20(erc20Contract);
        require(decimals <= tokenContract.decimals());

        paymentMethods[erc20Contract] = Payment({
            price: price,
            decimals: decimals
        });
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
        require(paymentMethods[payment].price > 0, "Invalid payment method");

        ERC20 erc20Contract = ERC20(payment);

        // Check allowance for this contract
        checkAllowance(erc20Contract);

        // Safe transfer to this contract
        uint256 price = paymentMethods[payment].price;
        uint8 priceDecimals = paymentMethods[payment].decimals;
        uint8 tokenDecimals = erc20Contract.decimals();
        bool result = erc20Contract.transferFrom(
            msg.sender,
            address(this),
            _tokenPrice(price, priceDecimals, tokenDecimals)
        );

        require(result == true, "Transfer failed");

        // Mint
        increment();
        _safeMint(msg.sender, _counter);

        return true;
    }

    // Check allowance against NFT price
    function checkAllowance(ERC20 tokenContract) public view returns (bool) {
        Payment memory paymentMethod = paymentMethods[address(tokenContract)];
        uint256 price = paymentMethod.price;
        uint8 priceDecimals = paymentMethod.decimals;

        uint256 allowed = tokenContract.allowance(msg.sender, address(this));
        uint8 tokenDecimals = tokenContract.decimals();

        require(allowed >= _tokenPrice(price, priceDecimals, tokenDecimals));

        return true;
    }

    function _tokenPrice(
        uint256 price,
        uint8 priceDecimals,
        uint8 decimals
    ) internal pure returns (uint256) {
        return price * (10 ** (decimals - priceDecimals));
    }
}
