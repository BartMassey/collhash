use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A newtype for a slice.
struct H<'a, T>(&'a [T]);

// Hash implementation for our newtype.
impl<'a, T: Hash> Hash for H<'a, T> {
    fn hash<HH: Hasher>(&self, state: &mut HH) {
        // If enabled, hash in the slice length.
        #[cfg(feature = "hash_len")]
        self.0.len().hash(state);

        // Hash in each element of the slice.
        for e in self.0 {
            e.hash(state);
        }
    }
}

/// Compute the hash of some slice by first wrapping it in
/// our newtype [H].
fn hash<T: Hash>(t: &[T]) -> u64 {
    let mut s = DefaultHasher::new();
    H(t).hash(&mut s);
    s.finish()
}

fn main() {
    // Since all examples should compare unequal this should
    // always print `false` (with very high probability).
    fn run_eg<T: Hash>(eg: &[(&[T], &[T])]) {
        for (l, r) in eg {
            println!("{}", hash(l) == hash(r));
        }
    }

    // Examples with string slices such that the total
    // string is the same.
    run_eg(&[
        (["he", "llo"].as_ref(), ["hello"].as_ref()),
        (["he", "llo"].as_ref(), ["hel", "lo"].as_ref()),
        ([].as_ref(), [""].as_ref()),
        ([""].as_ref(), ["", ""].as_ref()),
    ]);

    // Examples with unit slices.
    run_eg(&[
        (&[].as_ref(), &[()].as_ref()),
        (&[()].as_ref(), &[(), ()].as_ref()),
    ]);
}
