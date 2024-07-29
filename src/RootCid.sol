// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8;

import "@openzeppelin/contracts/access/AccessControl.sol";

/// @title Root CID Contract
/// @author Alex Miller
/// @notice This contract is a simple encrypted share contract
///   for a demo of how to post encrypted data to the blockchain.

contract RootCid is AccessControl {
  // We're opting for 512 bit Cids, so we'll need two bytes32
  bytes32[2] public cid;
  // Map from addresses to their share.
  //  We're AES key wrapping an AES-256 key, 
  //   so we'll need two 20 bytes
  Mapping(address => bytes20[2]) public shares;
  // And we'll set up a distributor role. It's not relevant
  //  now but may be in the future.
  bytes32 public constant DISTRIBUTOR_ROLE = keccak256("DISTRIBUTOR_ROLE");

  constructor(bytes32[2] memory _cid) {
    _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    _grantRole(DISTRIBUTOR_ROLE, msg.sender);
    cid = _cid;
  }
  
  function claim() public view returns (bytes20[2] memory) {
    return shares[msg.sender];
  }

  // Set the CID of the blog - restricted to owner
  function share(
    bytes20[2] memory secret,
    recipient address
  ) public {
    require(hasRole(DISTRIBUTOR_ROLE, msg.sender));
    shares[recipient] = secret;
  }
}
