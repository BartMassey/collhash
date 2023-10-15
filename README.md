# collhash: demo of collection hashing w/ and w/o length
Bart Massey 2023

This little code snippet shows why it is necessary to hash
in the length when hashing a collection.

When run without any feature flags, this program prints

     false
     false
     false
     true
     true

The first three `false`s indicates that it is apparently not
necessary to hash in the length for arrays of `str`s
(probably because the length of each `str` is hashed in
anyway). However, the fourth and fifth `true` indicate that
different slices of `unit` hash identically (probably
because each of these is a zero-sized type and thus no
actual hashing is performed).

Run with `--features=hash_len` to show that hashing in the
slice length fixes the problem.
