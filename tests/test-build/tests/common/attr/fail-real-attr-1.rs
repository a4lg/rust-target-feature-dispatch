pub const VALUE: i32 = target_feature_dispatch::target_feature_dispatch! {
    // Regular cfg attribute but this position only allows pseudo-attributes
    // parsed by the macro.
    #[cfg(all())]
    if (()) {
        (1)
    } else {
        0
    }
};
fn main() {}
