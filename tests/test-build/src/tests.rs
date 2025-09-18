// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (C) 2025 Tsukasa OI <floss_rust@irq.a4lg.com>.

#![cfg(test)]

#[cfg(not(feature = "enable"))]
#[test]
fn fail() {
    panic!("This test program must be pre-configured using cargo-make files.");
}

#[test]
fn common() {
    let tests = trybuild::TestCases::new();
    tests.pass("tests/common/*/pass-*.rs");
    tests.compile_fail("tests/common/*/expect-fail-*.rs");
    tests.compile_fail("tests/common/*/fail-*.rs");
    tests.compile_fail("tests/common/*/warn-*.rs");
}

#[cfg(any(target_arch = "arm", target_arch = "aarch64", target_arch = "arm64ec"))]
#[test]
fn arch_arm() {
    let tests = trybuild::TestCases::new();
    tests.pass("tests/arch-arm/*/pass-*.rs");
    tests.compile_fail("tests/arch-arm/*/expect-fail-*.rs");
    tests.compile_fail("tests/arch-arm/*/fail-*.rs");
    tests.compile_fail("tests/arch-arm/*/warn-*.rs");
}

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
#[test]
fn arch_riscv() {
    let tests = trybuild::TestCases::new();
    tests.pass("tests/arch-riscv/*/pass-*.rs");
    tests.compile_fail("tests/arch-riscv/*/expect-fail-*.rs");
    tests.compile_fail("tests/arch-riscv/*/fail-*.rs");
    tests.compile_fail("tests/arch-riscv/*/warn-*.rs");
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[test]
fn arch_x86() {
    let tests = trybuild::TestCases::new();
    tests.pass("tests/arch-x86/*/pass-*.rs");
    tests.compile_fail("tests/arch-x86/*/expect-fail-*.rs");
    tests.compile_fail("tests/arch-x86/*/fail-*.rs");
    tests.compile_fail("tests/arch-x86/*/warn-*.rs");
}
