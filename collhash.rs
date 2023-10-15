use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct H<'a, T>(&'a [T]);

impl<'a, T: Hash> Hash for H<'a, T> {
    fn hash<HH: Hasher>(&self, state: &mut HH) {
        #[cfg(feature = "hash_len")]
        self.0.len().hash(state);
        for e in self.0 {
            e.hash(state);
        }
    }
}

fn hash<T: Hash>(t: &[T]) -> u64 {
    let mut s = DefaultHasher::new();
    H(t).hash(&mut s);
    s.finish()
}

fn main() {
    fn run_eg<T: Hash>(eg: &[(&[T], &[T])]) {
        for (l, r) in eg {
            println!("{}", hash(l) == hash(r));
        }
    }

    run_eg(&[
        (["he", "llo"].as_ref(), ["hello"].as_ref()),
        ([].as_ref(), [""].as_ref()),
        ([""].as_ref(), ["", ""].as_ref()),
    ]);

    run_eg(&[
        (&[].as_ref(), &[()].as_ref()),
        (&[()].as_ref(), &[(), ()].as_ref()),
    ]);
}
