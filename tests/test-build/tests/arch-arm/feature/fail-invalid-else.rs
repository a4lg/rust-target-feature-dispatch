pub const VALUE: i32 = target_feature_dispatch::target_feature_dispatch! {
    if class("arm") {
        if "sha2" {
            1
        } else {
            // ERROR: reserved for internals.
            @__tgtfeat_dispatch_no_fallback
        }
    } else {
        0
    }
};
fn main() {}
