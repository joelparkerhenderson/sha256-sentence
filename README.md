# SHA256 sentence

"The SHA256 for this sentence begins with: one, eight, two, a, seven, c and nine."

[Inspired by @lauriewired post](https://twitter.com/lauriewired/status/1700982575291142594)

[Inspired by @humblehack post](https://twitter.com/humblehack/status/1088982929940848647)

[Discussion on Hacker News](https://news.ycombinator.com/item?id=37465086)

This repo has Rust implementations of a coding challenge that searches for a sentence that describes its own SHA256 checksum.

This repo contains two Rust complete projects.

## Rust with basics

The [/rust-with-basics](/rust-with-basics) project directory demonstrates:

* Iteration via the standard library. This is without parallelization.

* Permutation via the itertools crate. This is without replacement.

* A local system time variable. This is without multi-thread safety.

Output:

```
milliseconds: 3, digest: "0b62c3f1bf41205f419fd37a3e028e65a054d6ba913b10711ef53073e9096c85", starts: "0b62", sentence: "The SHA256 for this sentence begins with: zero, b, six and two."
milliseconds: 54, digest: "0ed8f8c1e8ab4f84ee6503a39a9973e57cc6f7065862ac69c34a6126708b6269", starts: "0ed8f", sentence: "The SHA256 for this sentence begins with: zero, e, d, eight and f."
milliseconds: 8288, digest: "182a7c930b0e5227ff8d24b5f4500ff2fa3ee1a57bd35e52d98c6e24c2749ae0", starts: "182a7c9", sentence: "The SHA256 for this sentence begins with: one, eight, two, a, seven, c and nine."
``````

## Rust with extras

The [/rust-with-extras](rust-with-extras) project directory demonstrates:

* Parallel iteration via the `rayon` crate.

* Permutation with replacement via Rosetta code `permutation_with_replacement`.

* A global system time variable via `once_cell` single-initialization multi-thread safety.

Output:

```
milliseconds: 14, digest: "0b62c3f1bf41205f419fd37a3e028e65a054d6ba913b10711ef53073e9096c85", starts: "0b62", sentence: "The SHA256 for this sentence begins with: zero, b, six and two."
milliseconds: 698, digest: "f0dfc81ccecc48f6ee0a50ced43f3abe78d5c4ab2ccaa4f582d93d103ff7db90", starts: "f0dfc", sentence: "The SHA256 for this sentence begins with: f, zero, d, f and c."
milliseconds: 823, digest: "0ed8f8c1e8ab4f84ee6503a39a9973e57cc6f7065862ac69c34a6126708b6269", starts: "0ed8f", sentence: "The SHA256 for this sentence begins with: zero, e, d, eight and f."
milliseconds: 3077, digest: "443bd2f109b49619afa402953babe11b0d95eac073129a04ce89c37ca5c3e253", starts: "443bd2", sentence: "The SHA256 for this sentence begins with: four, four, three, b, d and two."
milliseconds: 125985, digest: "182a7c930b0e5227ff8d24b5f4500ff2fa3ee1a57bd35e52d98c6e24c2749ae0", starts: "182a7c9", sentence: "The SHA256 for this sentence begins with: one, eight, two, a, seven, c and nine."
…
```
