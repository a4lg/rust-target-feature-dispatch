// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (C) 2025 Tsukasa OI <floss_rust@irq.a4lg.com>.

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process;
use std::process::Command;

use clap::{Parser, Subcommand};

/// Command line arguments.
#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true
)]
struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

/// Subcommands.
#[derive(Debug, Subcommand)]
enum SubCommands {
    /// Updates package information.
    ///
    /// This subcommand extracts package information from
    /// `src/Cargo.toml` and reflects into following files:
    ///
    /// *   `tests/Cargo.toml`
    /// *   `tests-unstable/Cargo.toml`
    /// *   `test-utils/Cargo.toml`
    UpdatePackageInfo,

    /// Updates dependencies on all workspaces.
    ///
    /// This subcommand just runs `cargo update` on every workspaces.
    UpdateDeps,
}

/// Removes all Cargo-related environment variables.
///
/// To avoid reflecting *this* workspace's values,
/// this command needs to remove all Cargo-related environment variables.
fn remove_cargo_env() {
    let names: Vec<OsString> = env::vars_os()
        .map(|(name, _)| name)
        .filter(|name| name.to_string_lossy().starts_with("CARGO"))
        .collect();
    unsafe {
        names.iter().for_each(|name| env::remove_var(name));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    const SUB_WORKSPACES: &[&str] = &["tests", "tests-unstable", "test-utils"];
    let root = {
        let mut paths = Vec::<PathBuf>::new();
        let _ = env::var_os("CARGO_MANIFEST_DIR").inspect(|path| {
            // "${CARGO_MANIFEST_DIR}/../.."
            let mut path = PathBuf::from(path);
            if !path.pop() || !path.pop() {
                return;
            }
            paths.push(path);
        });
        let _ = env::current_dir().inspect(|path| {
            // "${PWD}/.."
            let mut path = path.clone();
            if !path.pop() {
                return;
            }
            paths.push(path);
        });
        let _ = env::current_dir().inspect(|path| {
            // "${PWD}/../.."
            let mut path = path.clone();
            if !path.pop() || !path.pop() {
                return;
            }
            paths.push(path);
        });
        let mut path: Option<PathBuf> = None;
        for test_path in paths {
            if test_path
                .join("src/docs/target_feature_dispatch.md")
                .is_file()
            {
                path = Some(test_path);
                break;
            }
        }
        path.expect("target_feature_dispatch repository path not found")
            .canonicalize()
    }?;
    remove_cargo_env();
    let cli = Cli::parse();
    match cli.subcommand {
        SubCommands::UpdatePackageInfo => {
            // Read contents
            let mut contents = String::new();
            File::open(root.join("src/Cargo.toml"))?.read_to_string(&mut contents)?;
            let doc = contents.parse::<toml_edit::DocumentMut>()?;
            drop(contents);
            let package = &doc["package"];
            let version = &package["version"];
            let license = &package["license"];
            let authors = &package["authors"];
            let homepage = &package["homepage"];
            let repository = &package["repository"];

            for &workspace in SUB_WORKSPACES {
                // Update workspace's Cargo.toml.
                let path = root.join(workspace).join("Cargo.toml");
                let mut contents = String::new();
                File::open(&path)?.read_to_string(&mut contents)?;
                let mut doc = contents.parse::<toml_edit::DocumentMut>()?;
                drop(contents);
                doc["workspace"]["package"]["version"] = version.clone();
                doc["workspace"]["package"]["license"] = license.clone();
                doc["workspace"]["package"]["authors"] = authors.clone();
                doc["workspace"]["package"]["homepage"] = homepage.clone();
                doc["workspace"]["package"]["repository"] = repository.clone();
                let members = &doc["workspace"]["members"];
                File::create(&path)?.write_all(doc.to_string().as_bytes())?;

                /*
                    Update dependencies of:
                    *   target-feature-dispatch
                    *   Workspace members
                */
                let path = root.join(workspace);
                env::set_current_dir(&path)?;
                let mut cmd = Command::new("cargo");
                cmd.args(["update", "target-feature-dispatch"]);
                cmd.args(
                    members
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|x| {
                            let member = x.as_str().unwrap();
                            let path = path.join(member).join("Cargo.toml");
                            let mut contents = String::new();
                            File::open(&path)?.read_to_string(&mut contents)?;
                            let doc = contents.parse::<toml_edit::DocumentMut>()?;
                            Ok(doc["package"]["name"].as_str().unwrap().to_owned())
                        })
                        .collect::<Result<Vec<_>, Box<dyn Error>>>()?,
                );
                let is_success = cmd.status()?.success();
                if !is_success {
                    eprintln!("Failed to update workspace `{workspace}'");
                    process::exit(1);
                }
            }

            // Update the main workspace.
            env::set_current_dir(&root)?;
            let is_success = Command::new("cargo")
                .args(["update", "target-feature-dispatch"])
                .status()?
                .success();
            if !is_success {
                eprintln!("Failed to update main workspace");
                process::exit(1);
            }
        }
        SubCommands::UpdateDeps => {
            for &workspace in SUB_WORKSPACES.iter().chain(&["."]) {
                let path = root.join(workspace);
                env::set_current_dir(path)?;
                let is_success = Command::new("cargo").arg("update").status()?.success();
                if !is_success {
                    eprintln!("Failed to update workspace `{workspace}'");
                    process::exit(1);
                }
            }
        }
    }
    Ok(())
}
