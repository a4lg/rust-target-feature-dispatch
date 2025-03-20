fn sample() {
    let value = { #[cfg(not(all(all(), feature = "dispatch")))] { "fallback" } };
}
