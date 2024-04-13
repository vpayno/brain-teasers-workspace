# brain-teasers-workspace

Just read `Rust Brain Teasers` and I needed a place to play with the code.
Also using this workspace to try the brain teasers in other languages.

## Brain Teasers

### Three and a Bit

```text
$ cargo run --bin three_and_a_bit
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/three_and_a_bit three_and_a_bit`
Wanted: 3.4028236
   Got: 3.4028237
```

### Non-Standard Input

```text
$ cargo run --bin non-std-input
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/non-std-input`

Type your answer and press enter.
What is 3+2?
5

Response: "5\n"
Trimmed: "5"

Correct!

$ cargo run --bin non-std-input
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/non-std-input`

Type your answer and press enter.
What is 3+2?
7

Response: "7\n"
Trimmed: "7"

Incorrect!
```
