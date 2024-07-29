// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {Drop} from "../src/Drop.sol";

contract DropTest is Test {
    bytes32 foo = "foo";
    bytes32 bar = "bar";
    bytes32[2] cid = [foo,bar];
    bytes32[2] share = [bar,foo];
    RootCid public root_cid;

    function setUp() public {
        drop = new Drop(cid);
    }

    function testCid() public {
        bytes32[2] memory _cid = root_cid.cid();
        assertEq(_cid[0], cid[0]);
        assertEq(_cid[1], cid[1]);
    }
    function testShare() public {
        root_cid.share(share, this);
        bytes20[2] memory _share = root_cid.claim();
        assertEq(_share[0], share[0]);
        assertEq(_share[1], share[1]);
    }
}
