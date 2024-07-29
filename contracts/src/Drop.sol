// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8;

import "@openzeppelin/contracts/access/AccessControl.sol";

/// @title Drop Contract
/// @author Alex Miller

contract Drop is AccessControl {
  bytes32[2] public cid;
  mapping(address => bytes20[2]) public shares;
  bytes32 public constant DISTRIBUTOR_ROLE = keccak256("DISTRIBUTOR_ROLE");

  constructor(bytes32[2] memory _cid) {
    _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    _grantRole(DISTRIBUTOR_ROLE, msg.sender);
    cid = _cid;
  }

  function cid() public view returns (bytes32[2] memory) {
    return cid;
  }
  
  function claim() public view returns (bytes20[2] memory) {
    return shares[msg.sender];
  }

  // Set the CID of the blog - restricted to owner
  function share(bytes20[2] memory share, address recipient) public {
    require(hasRole(DISTRIBUTOR_ROLE, msg.sender));
    shares[recipient] = share;
  }
}
