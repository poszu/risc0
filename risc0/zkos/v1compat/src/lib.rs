// Copyright 2025 RISC Zero, Inc.
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

#[cfg(not(target_os = "zkvm"))]
pub const V1COMPAT_ELF: &[u8] = include_bytes!("../elfs/v1compat.elf");

#[cfg(not(target_os = "zkvm"))]
pub const V1COMPAT_V2_KERNEL_ID: &[u8] = include_bytes!("../elfs/v1compat.kid");