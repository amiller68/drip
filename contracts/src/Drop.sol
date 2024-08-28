// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8;

import "@openzeppelin-contracts-5.0.2/access/AccessControl.sol";

/// @title Drop Contract
/// @author Alex Miller

contract Drop is AccessControl {
  bytes32[2] public root;
  mapping(address => bytes20[4]) public shares;
  bytes32 public constant DISTRIBUTOR_ROLE = keccak256("DISTRIBUTOR_ROLE");

  constructor(
    bytes32[2] memory _cid,
    bytes20[4] memory _share
  ) {
    _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    _grantRole(DISTRIBUTOR_ROLE, msg.sender);
    root = _cid;
    shares[msg.sender] = _share;
  }
  
  function cid() public view returns (bytes32[2] memory) {
    return root;
  }

  function claim() public view returns (bytes20[4] memory) {
    return shares[msg.sender];
  }

  // Set the CID of the blog - restricted to owner
  function shareWith(bytes20[4] memory _share, address recipient) public {
    require(hasRole(DISTRIBUTOR_ROLE, msg.sender));
    shares[recipient] = _share;
  }
}
