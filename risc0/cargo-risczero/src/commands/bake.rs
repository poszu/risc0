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

use std::path::Path;

use anyhow::Result;
use cargo_metadata::Package;
use clap::Parser;
use risc0_build::{DockerOptions, GuestOptions, ImageIdKind};
use risc0_zkvm::sha::Digest;

/// `cargo risczero bake`
#[derive(Parser)]
pub struct BakeCommand {
    #[command(flatten)]
    manifest: clap_cargo::Manifest,

    #[command(flatten)]
    workspace: clap_cargo::Workspace,

    #[command(flatten)]
    features: clap_cargo::Features,

    ///  Run compilation using a Docker container for reproducible builds.
    #[arg(long, default_value_t = false)]
    pub docker: bool,
}

impl BakeCommand {
    pub fn run(&self) -> Result<()> {
        let mut meta_cmd = self.manifest.metadata();
        let meta_cmd = self.features.forward_metadata(&mut meta_cmd);
        let meta = meta_cmd.exec()?;

        let target_dir = meta.target_directory.as_std_path().join("guest");

        let (included, _excluded) = self.workspace.partition_packages(&meta);
        for pkg in included {
            if let Some(_risc0) = pkg.metadata.get("risc0") {
                if pkg.targets.iter().any(|x| x.is_bin()) {
                    self.bake_target(pkg, &target_dir)?;
                }
            }
        }

        Ok(())
    }

    fn bake_target(&self, pkg: &Package, target_dir: &Path) -> Result<()> {
        let use_docker = if self.docker {
            Some(DockerOptions {
                root_dir: Some(std::env::current_dir()?),
            })
        } else {
            None
        };

        let options = GuestOptions {
            features: self.features.features.clone(),
            use_docker,
        };

        let elfs_dir = pkg
            .manifest_path
            .as_std_path()
            .parent()
            .unwrap()
            .join("elfs");

        let guests = risc0_build::build_package(pkg, target_dir, options)?;
        for guest in guests {
            let guest_path = guest.path.to_string();
            let src_path = Path::new(&guest_path);
            let file_name = src_path.file_name().unwrap();
            let tgt_path = elfs_dir.join(file_name).with_extension("elf");
            std::fs::create_dir_all(tgt_path.parent().unwrap())?;
            std::fs::copy(src_path, tgt_path)?;

            if guest.image_id != Digest::ZERO {
                let image_id_path = elfs_dir.join(file_name).with_extension("iid");
                std::fs::write(image_id_path, guest.image_id.as_bytes())?
            }

            match guest.v2_image_id {
                ImageIdKind::User(digest) => {
                    let image_id_path = elfs_dir.join(file_name).with_extension("uid");
                    std::fs::write(image_id_path, digest.as_bytes())?
                }
                ImageIdKind::Kernel(digest) => {
                    let image_id_path = elfs_dir.join(file_name).with_extension("kid");
                    std::fs::write(image_id_path, digest.as_bytes())?
                }
            };
        }

        Ok(())
    }
}