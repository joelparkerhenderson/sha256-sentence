# SHA256 sentence: Rust with basics

This is a Rust implementation of coding challenge that searches for a sentence that describes its own SHA256 checksum.

[Inspired by @lauriewired post](https://twitter.com/lauriewired/status/1700982575291142594)

[Discussion on Hacker News](https://news.ycombinator.com/item?id=37465086)

This project demonstrates:

* Iteration via the standard library. This is without parallelization.

* Permutation via the itertools crate. This is without replacement.

* A local system time variable. This is without multi-thread safety.
 
Run:

```sh
cargo run --release
```

Output:

```
milliseconds: 3, digest: "0b62c3f1bf41205f419fd37a3e028e65a054d6ba913b10711ef53073e9096c85", starts: "0b62", sentence: "The SHA256 for this sentence begins with: zero, b, six and two."
milliseconds: 54, digest: "0ed8f8c1e8ab4f84ee6503a39a9973e57cc6f7065862ac69c34a6126708b6269", starts: "0ed8f", sentence: "The SHA256 for this sentence begins with: zero, e, d, eight and f."
milliseconds: 8288, digest: "182a7c930b0e5227ff8d24b5f4500ff2fa3ee1a57bd35e52d98c6e24c2749ae0", starts: "182a7c9", sentence: "The SHA256 for this sentence begins with: one, eight, two, a, seven, c and nine."
…
```
