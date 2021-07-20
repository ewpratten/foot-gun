//! The `foot-gun` crate is a joke library inspired by [this twitter thread](https://twitter.com/flukejones/status/1417241932154081294)

/// The `foot_gun` macro for unsafe code.
///
/// Inspired by a comment from [@flukejones](https://twitter.com/flukejones/status/1417241932154081294)
///
/// ## Usage
/// ```
/// foot_gun!({
///    // Unsafe code here
/// }
/// ```
#[macro_export]
macro_rules! foot_gun {
    ($e:block) => {
        unsafe { $e }
    };
}

/// The `here_be_dragons` macro for unsafe code.
///
/// Inspired by a comment from [@ekuber](https://twitter.com/ekuber/status/1417243264315387908)
///
/// ## Usage
/// ```
/// here_be_dragons!({
///    // Unsafe code here
/// }
/// ```
#[macro_export]
macro_rules! here_be_dragons {
    ($e:block) => {
        unsafe { $e }
    };
}

/// The `beware` macro for unsafe code.
///
/// Inspired by a comment from [@NikolaiVazquez](https://twitter.com/NikolaiVazquez/status/1417253654508212225)
///
/// ## Usage
/// ```
/// beware!({
///    // Unsafe code here
/// }
/// ```
#[macro_export]
macro_rules! beware {
    ($e:block) => {
        unsafe { $e }
    };
}

/// The `behold` macro for unsafe code.
///
/// Inspired by a comment from [@fasterthanlime](https://twitter.com/fasterthanlime/status/1417370585072611352)
///
/// ## Usage
/// ```
/// behold!({
///    // Unsafe code here
/// }
/// ```
#[macro_export]
macro_rules! behold {
    ($e:block) => {
        unsafe { $e }
    };
}

/// The `en_garde` macro for unsafe code.
///
/// Inspired by a comment from [@redtwitdown](https://twitter.com/redtwitdown/status/1417372091599314946)
///
/// ## Usage
/// ```
/// en_garde!({
///    // Unsafe code here
/// }
/// ```
#[macro_export]
macro_rules! en_garde {
    ($e:block) => {
        unsafe { $e }
    };
}

/// The `i_got_this` macro for unsafe code.
///
/// Inspired by a comment from [@algo_luca](https://twitter.com/algo_luca/status/1417397666225561600)
///
/// ## Usage
/// ```
/// i_got_this!({
///    // Unsafe code here
/// }
/// ```
#[macro_export]
macro_rules! i_got_this {
    ($e:block) => {
        unsafe { $e }
    };
}

/// The `hold_my_borrowchk` macro for unsafe code.
///
/// Inspired by a comment from [@redtwitdown](https://twitter.com/redtwitdown/status/1417398144585715713)
///
/// ## Usage
/// ```
/// hold_my_borrowchk!({
///    // Unsafe code here
/// }
/// ```
#[macro_export]
macro_rules! hold_my_borrowchk {
    ($e:block) => {
        unsafe { $e }
    };
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[allow(unused_unsafe)]
    pub fn test_compile() {
        foot_gun!({ println!("Hello") });
        here_be_dragons!({ println!("Hello") });
        beware!({ println!("Hello") });
        behold!({ println!("Hello") });
        en_garde!({ println!("Hello") });
        i_got_this!({ println!("Hello") });
        hold_my_borrowchk!({ println!("Hello") });
    }
}
