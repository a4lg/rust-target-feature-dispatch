# Testing

# Prerequisites

*   `rustup`  
    The tests under testing workspaces require specific version of Rust
    is installed.  `rustup` automates management / dispatching of various
    toolchains through `rust-toolchain.toml`.
*   Crate: `cargo-make`  
    This crate is the main backend for testing gear.
*   Crate: `cargo-expand`  
    This is required for macro expansion tests through `macrotest`.
*   Crate: `wasmtime-cli`  
    This is used for WebAssembly emulation.
*   UNIX-like environment either:
    *   With packages required to build and install CI tools
        (as in the `tools` task).
    *   With prebuilt CI tool binaries copied under
        `${CARGO_TARGET_DIR}/ci-tools-prefix`.

# Building / Installing CI Tools

For testing, they are required:

*   GNU Binutils (to be specific, `ld` for static linking)
*   QEMU (to emulate various Linux userland configurations)

To build, run `cargo make tools`.

The CI tools will be installed into `${CARGO_TARGET_DIR}/ci-tools-prefix`
(which could be copied to another environment).

Follows are related tasks (which can be invoked like `cargo make [TASK_NAME]`):

*   `tools`  
    Build and install CI tools.
    *   `build-tools-binutils`  
        Build GNU Binutils and install to the specific directory.
        *   `build-tools-binutils-*`  
            Build GNU Binutils and install to the specific directory
            (for specific target).
            This is skipped if the target's `ld` exists on the CI tools
            installation directory.
    *   `build-tools-qemu`  
        Build QEMU and install to the specific directory.
        This is skipped if QEMU part of CI tools is already installed.
*   `clean-tools`  
    Remove CI tools and its build directory.
    *   `clean-tools-bin`  
        Remove CI tools (installation).
    *   `clean-tools-build`  
        Remove CI tools (build directory).
*   `distclean`  
    Remove CI tools along with source code archives and all contents inside
    the build target directory.
*   `download-tools`  
    Download source code archives of CI tools.
    Archives are saved just under the root directory of the repository.
    *   `download-tools-binutils`  
        Download GNU Binutils.
        This is skipped if GNU Binutils part of CI tools is already installed.
    *   `download-tools-qemu`  
        Download QEMU.
        This is skipped if QEMU part of CI tools is already installed.

# Running Test Tasks

Run `cargo make [TASK_NAME]` on the testing workspace for performing
various tasks as shown below:

*   `test`  
    Tests the main crate in various ways.
    *   `test-build`  
        Performs the build test.  Cross-compilation is not available
        due to the limitation of `trybuild`.
    *   `test-expand`  
        Performs the macro expansion test.
        Cross-compilation is performed.
    *   `test-run`  
        Performs cross-compiled emulation tests using various
        configurations and various simulators.
*   `update`  
    Updates reference files to be used on the `test` task.
    Use with care because it does not check whether the updated contents
    are valid.
    *   `update-build`  
        Updates reference files for `test-build`.
    *   `update-expand`  
        Updates reference files for `test-expand`.

# Comments to Notify Rust Toolchain-specific Parts

## `SPECIFIC: ARCH_COMPILER_VARIANT`

This comment follows with a line with a `target_arch` list containing all
architectures supported by current version of the Rust compiler.
