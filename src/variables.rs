/// Create a variable and assign it a value so that the test passes.
/// Docs link: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
///
/// # Examples
///
/// ```
/// let number = rust_basics::variables::assign_variable();
/// assert_eq!(5, number);
/// ```
pub fn assign_variable() -> i32 {
    

    number
}

/// Create a mutable variable and assign it a value so that the test passes.
/// Docs link: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
///
/// # Examples
///
/// ```
/// let number = rust_basics::variables::assign_mutable_variable();
/// assert_eq!(4, number);
/// ```
pub fn assign_mutable_variable() -> i32 {
    

    number = 4;

    number
}

/// Shadow a variable and assign it a value.
/// Docs link: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
///
/// # Examples
///
/// ```
/// let number = rust_basics::variables::shadow_variable();
/// assert_eq!(4, number);
/// ```
pub fn shadow_variable() -> i32 {
    let number = 5;

    

    number
}