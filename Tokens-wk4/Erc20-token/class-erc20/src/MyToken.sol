//SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IERC20} from "./interfaces/IERC20.sol";

contract MyToken is IERC20 {
    //metadata
    string private _name;
    string private _symbol;
    uint8 private immutable _decimals;

    //--core state ---
    uint256 private _totalSupply;
    mapping(address => uint256) private _balance;
    mapping(address => mapping(address => uint256)) private _allowances;

    //--core reads --
    function totalSupply() public view returns (uint256) {
        return _totalSupply;
    }

    function balanceOf(address owner) public view returns (uint256) {
        return _balance[owner];
    }

    function allowance(address owner, address spender) public view returns (uint256) {
        return _allowances[owner][spender];
    }
}
