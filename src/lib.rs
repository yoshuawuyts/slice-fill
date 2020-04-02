//! `Slice::fill` prototype
//!
//! # Examples
//!
//! ```
//! use slice_fill::SliceExt;
//!
//! let mut buf = vec![0; 10];
//! buf.fill(1);
//! assert_eq!(buf, vec![1; 10]);
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]

/// Extension trait for `std::slice`.
pub trait SliceExt<T: Clone> {
    /// Fill a slice with an item.
    fn fill(&mut self, item: T);
}

impl<T: Clone> SliceExt<T> for [T] {
    fn fill(&mut self, item: T) {
        for el in self {
            *el = item.clone();
        }
    }
}

mod test {
    #[test]
    fn fills_vec() {
        use super::SliceExt;
        let mut buf = vec![0usize; 10];
        buf.fill(1);
        assert_eq!(buf, vec![1usize; 10]);
    }

    #[test]
    fn fills_slice() {
        use super::SliceExt;
        let mut buf = vec![0usize; 10];
        buf[0..10].fill(1);
        assert_eq!(buf, vec![1usize; 10]);
    }
}
