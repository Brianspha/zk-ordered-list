// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol"; // auto-generated contract after running `cargo build`.

contract OrderedList {
    /// @notice RISC Zero verifier contract address.
    IRiscZeroVerifier public immutable verifier;

    /// @notice Image ID of the only zkVM binary to accept verification from.
    ///         The image ID is similar to the address of a smart contract.
    ///         It uniquely represents the logic of that guest program,
    ///         ensuring that only proofs generated from a pre-defined guest program
    ///         (in this case, checking if a number is even) are considered valid.
    bytes32 public constant imageId = ImageID.ORDERED_LIST_ID;

    uint256[] public list;
    
    constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
    }
    function set(uint256[] memory numbers, bytes calldata seal) public {
        bytes memory journal = abi.encode(numbers);
        verifier.verify(seal, imageId, sha256(journal));
        list = numbers;
    }
    function get() public view returns (uint256[] memory) {
        return list;
    }
}
