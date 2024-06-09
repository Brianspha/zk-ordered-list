// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::Read;

use alloy_primitives::U256;
use alloy_sol_types::SolValue;
use risc0_zkvm::guest::env;

fn main() {
    // Read the input data for this application.
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    let list = <Vec<U256>>::abi_decode(&input_bytes, true).unwrap();
    
    let mut index: usize = 0;
    let mut ordered: bool = list.len() > 0;
    while index < (list.len() - 2).try_into().unwrap() {
        let a: U256 = list[index];
        let b: U256 = list[index + 1];
        ordered = a >= b;
        if !ordered {
            break;
        }
        index += 1;
    }

    // Run the computation.
    // In this case, asserting that the numbers are ordered
    assert!(ordered, "list is not ordered");
    print!("list ordered? {:?}\n\n", ordered.clone());
    // Commit the journal that will be received by the application contract.
    // Journal is encoded using Solidity ABI for easy decoding in the app contract.
    env::commit_slice(list.abi_encode().as_slice());
}
