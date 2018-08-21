# synacor_rs
⚙ Implementation of the Synacor Challenge virtual machine in Rust

More information about the Synacor challenge can be found at [its website](https://challenge.synacor.com/). Additionally, I wrote a [blog post](https://benjamincongdon.me/blog/2016/12/18/Taking-on-the-Synacor-Challenge/) about my experience working on the challenge, and have also published my initial [python implementation](https://github.com/bcongdon/synacor_challenge) of the VM.

## Usage

```
cargo build --release
./target/release/synacor_rs [CHALLENGE_BIN]
```

### Hacks

The `hack_teleporter` command can be used at any time, but should only be used upon receiving the teleporter. This command alters specific locations in memory (and in registers) to "fix" the teleporter.

## Benchmark

`synacor_rs` (Rust Implementation):
```
Time (mean ± σ):      57.3 ms ±   5.1 ms

Range (min … max):    52.4 ms …  76.5 ms
```

`synacor_challenge` ([Python Implementation](https://github.com/bcongdon/synacor_challenge)):
```
Time (mean ± σ):     19.916 s ±  0.264 s

Range (min … max):   19.165 s … 20.028 s
```

Benchmarks were perfomed by measuring the time it takes to do a full walkthrough of all the puzzles using the `./walkthrough.sh` script (which simulates user input via `expect(1)`).

Neither of these implementaitons were particularly optimization-focussed, though the Python implementation has some low-hanging fruit for optimizations that would likely make it somewhat more competitive. (For example, by reducing reliance on `dict`s so as to reduce the amount of time wasted by hashing).

That being said, the naive Rust implementation is ~300 times faster. ⚡😄
