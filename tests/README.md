# `target-feature-dispatch` (Tests; Stable)

This directory contains a workspace to test the main crate.

This workspace is separate from the main one to split MSRV and various
configurations between the main crate and testing crates (because MSRV-aware
resolver adjusts the dependencies to the *lowest* MSRV in the workspace which is
not an expected behavior here and we have to configure some of target-specific
settings).

See [testing.md](../test-utils/doc/testing.md) for details.
