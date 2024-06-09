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

//! Generated crate containing the image ID and ELF binary of the build guest.
include!(concat!(env!("OUT_DIR"), "/methods.rs"));

#[cfg(test)]
mod tests {
    use alloy_primitives::U256;
    use alloy_sol_types::SolValue;
    use risc0_zkvm::{default_executor, ExecutorEnv};

    #[test]
    fn proves_ordered_numbers() {
        let numbers: Vec<alloy_primitives::Uint<256, 4>> = vec![
            U256::from(1304),
            U256::from(1303),
            U256::from(1302),
            U256::from(1301),
        ];
        let env = ExecutorEnv::builder()
            .write_slice(&numbers.abi_encode())
            .build()
            .unwrap();

        // NOTE: Use the executor to run tests without proving.
        let session_info: risc0_zkvm::SessionInfo = default_executor()
            .execute(env, super::ORDERED_LIST_ELF)
            .unwrap();
        let x: Vec<alloy_primitives::Uint<256, 4>> =
            <Vec<U256>>::abi_decode(&session_info.journal.bytes, true).unwrap();
        assert_eq!(x, numbers);
    }
    #[test]
    #[should_panic = "list is not ordered"]
    fn proves_unordered_numbers() {
        let numbers: Vec<alloy_primitives::Uint<256, 4>> = vec![
            U256::from(1303),
            U256::from(1304),
            U256::from(1302),
            U256::from(1301),
        ];
        let env = ExecutorEnv::builder()
            .write_slice(&numbers.abi_encode())
            .build()
            .unwrap();
        default_executor()
            .execute(env, super::ORDERED_LIST_ELF)
            .unwrap();
    }
}
