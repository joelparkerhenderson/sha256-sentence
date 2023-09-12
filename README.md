# SHA256 sentence

"The SHA256 for this sentence begins with: one, eight, two, a, seven, c and nine."

This is a Rust implementation of coding challenge that searches for a sentence that describes its own SHA256 checksum.

This repo contains two Rust complete projects.

The [/rust-with-basics](/rust-with-basics) project directory demonstrates:

* Iteration via the standard library. This is without parallelization.

* Permutation via the itertools crate. This is without replacement.

* A local system time variable. This is without thread safety.
 
The [/rust-with-extras](rust-with-extras) project directory demonstrates these improvements:

* Parallel iterations via the `rayon` crate.

* Permutation with replacement via a custom iterator `permutation_with_replacement`.

* A global system time variable via `once_cell` single-initialization thread safety.

[Inspired by @lauriewired post](https://twitter.com/lauriewired/status/1700982575291142594)

[Discussion on Hacker News](https://news.ycombinator.com/item?id=37465086)
