pub trait IVectorComparer<T> {
    fn new(vec1: Vec<T>, vec2: Vec<T>) -> Self;
    fn get_differences(&self) -> Vec<T>;
}

#[derive(Debug, Clone)]
pub struct VectorComparer<T> {
    pub vec1: Vec<T>,
    pub vec2: Vec<T>,
}

impl IVectorComparer<String> for VectorComparer<String> {
    /// Initialize a new VectorComparer for type `String`
    ///
    /// # Example
    ///
    /// ```rust
    /// let vector_comparer: VectorComparer<String> = IVectorComparer::<String>::new(vec![], vec![]);
    /// ```
    ///
    /// # Retuns
    ///
    /// A `VectorComparer` that can be used to compare two `Vec` structs of type `String`
    fn new(vec1: Vec<String>, vec2: Vec<String>) -> VectorComparer<String> {
        VectorComparer::<String> { vec1, vec2 }
    }

    /// Get the differences between the two given `Vec` structs of type `String`
    ///
    /// # Example
    ///
    /// ```rust
    /// let differences: Vec<String> = vector_comparer.get_differences();
    /// ```
    ///
    /// # Returns
    ///
    /// A `Vec` struct of type `String` that contains the differences between the two given `Vec` structs of type `String`
    fn get_differences(&self) -> Vec<String> {
        if self.vec1.is_empty() {
            return self.vec2.clone();
        } else if self.vec2.is_empty() {
            return self.vec1.clone();
        }

        let mut diff = vec![];
        for f in &self.vec1 {
            let mut included = false;
            for d in &self.vec2 {
                if f.eq(d) {
                    included = true;
                }
            }

            if !included {
                diff.push(String::from(f));
            }
        }

        for f in &self.vec2 {
            let mut included = false;
            for d in &self.vec1 {
                if f.eq(d) {
                    included = true;
                }
            }

            if !included {
                let n = String::from(f);
                if !diff.contains(&n) {
                    diff.push(n);
                }
            }
        }

        diff
    }
}
