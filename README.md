# `target_feature_dispatch` (as expression)

It provides target feature dispatch macro for use in the expression position.

Once you write the target feature-specific branches once, this macro
implements both dynamic and static dispatch routines
(normally, those two are written very differently).

This crate is particularly useful on x86 and AArch64 targets.

# When is this crate useful?

If you want to optimize the program depending on the target features and
you want to satisfy both:

*   The final binary is going to be distributed for various environment but
    with low minimum requirements (e.g. x86_64 with only SSE2 enabled) and
*   You want to write a `no_std` crate without dynamic dispatch capabilities,

You will need both static and dynamic dispatching.
However, because the huge difference between static and dynamic dispatching,
you needed to write both and that can be tedious sometimes.

...Until now.  This crate provides a macro to easily write both static and
dynamic dispatch routines depending on the target architecture / features.

For more details of this crate, see [crate's readme](src/README.md).
