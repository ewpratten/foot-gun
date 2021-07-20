# foot-gun
[![Crates.io](https://img.shields.io/crates/v/foot-gun)](https://crates.io/crates/foot-gun) 
[![Docs.rs](https://docs.rs/foot-gun/badge.svg)](https://docs.rs/foot-gun) 
[![Build](https://github.com/Ewpratten/foot-gun/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/foot-gun/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/foot-gun/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/foot-gun/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/foot-gun/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/foot-gun/actions/workflows/audit.yml)


The `foot-gun` crate is a joke library inspired by [this twitter thread](https://twitter.com/flukejones/status/1417241932154081294).

This crate provides the following macros:

 - `foot_gun`
 - `here_be_dragons`
 - `beware`
 - `behold`
 - `en_garde`
 - `i_got_this`
 - `hold_my_borrowchk`

All macros are credited to their "inventors" in RustDoc.

## Examples

```rust
foot_gun!({
    // Unsafe code here
});

here_be_dragons!({
    // Unsafe code here
});

beware!({
    // Unsafe code here
});

behold!({
    // Unsafe code here
});

en_garde!({
    // Unsafe code here
});

i_got_this!({
    // Unsafe code here
});

hold_my_borrowchk!({
    // Unsafe code here
});
```