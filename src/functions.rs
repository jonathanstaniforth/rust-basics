/// Create a function that returns a value making the test pass.
/// Docs link: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
///
/// # Examples
///
/// ```
/// let number = rust_basics::functions::create_function();
/// assert_eq!(10, number);
/// ```
pub fn create_function() -> i32 {
    add(5, 5)
}

