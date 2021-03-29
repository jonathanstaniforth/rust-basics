pub mod static_types {
    /// Create an integer type variable.
    /// Docs link: https://doc.rust-lang.org/book/ch03-02-data-types.html
    ///
    /// # Examples
    ///
    /// ```
    /// let integer = rust_basics::types::static_types::create_integer_type();
    /// assert_eq!(5, integer);
    /// ```
    pub fn create_integer_type() -> i32 {
        

        integer
    }

    /// Create a float type variable.
    /// Docs link: https://doc.rust-lang.org/book/ch03-02-data-types.html
    ///
    /// # Examples
    ///
    /// ```
    /// let float = rust_basics::types::static_types::create_float_type();
    /// assert_eq!(5.0, float);
    /// ```
    pub fn create_float_type() -> f32 {
        

        float
    }

    /// Create a boolean type variable.
    /// Docs link: https://doc.rust-lang.org/book/ch03-02-data-types.html
    ///
    /// # Examples
    ///
    /// ```
    /// let boolean = rust_basics::types::static_types::create_boolean_type();
    /// assert_eq!(true, boolean);
    /// ```
    pub fn create_boolean_type() -> bool {
        

        boolean
    }

    /// Create a character type variable.
    /// Docs link: https://doc.rust-lang.org/book/ch03-02-data-types.html
    ///
    /// # Examples
    ///
    /// ```
    /// let character = rust_basics::types::static_types::create_character_type();
    /// assert_eq!('t', character);
    /// ```
    pub fn create_character_type() -> char {
        

        character
    }
}

pub mod compound_types {
    /// Create a tuple type variable.
    /// Docs link: https://doc.rust-lang.org/book/ch03-02-data-types.html
    ///
    /// # Examples
    ///
    /// ```
    /// let tuple = rust_basics::types::compound_types::create_tuple_type();
    /// assert_eq!((5, 5.0, true), tuple);
    /// ```
    pub fn create_tuple_type() -> (i32, f32, bool) {
        

        tuple
    }

    /// Create an array type variable.
    ///
    /// # Examples
    ///
    /// ```
    /// let array = rust_basics::types::compound_types::create_array_type();
    /// assert_eq!([1, 2, 3, 4, 5], array);
    /// ```
    pub fn create_array_type() -> [i32; 5] {
        

        array
    }
}