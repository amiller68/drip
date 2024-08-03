// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {Drop} from "../src/Drop.sol";

contract DropTest is Test {
    bytes32 foo = "foo";
    bytes32 bar = "bar";
    bytes20 biz = "biz";
    bytes20 baz = "baz";
    bytes32[2] cid = [foo,bar];
    bytes20[2] share = [biz,baz];
    Drop public drop;

    function setUp() public {
        drop = new Drop(cid);
    }

    function testCid() public view {
        bytes32[2] memory _cid = drop.cid();

        assertEq(_cid[0], cid[0]);
        assertEq(_cid[1], cid[1]);
    }
    function testShare() public {
        drop.shareWith(share, address(this));
        bytes20[2] memory _share = drop.claim();
        assertEq(_share[0], share[0]);
        assertEq(_share[1], share[1]);
    }
}
