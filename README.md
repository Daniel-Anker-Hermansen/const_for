# Const for loops

[![GitHub](https://img.shields.io/badge/GitHub-black?logo=github)](https://github.com/JENebel/const_for)
[![crates.io](https://img.shields.io/crates/v/const_for?logo=rust&logoColor=b7410e)](http://crates.io/crates/const_for)
[![Docs](https://img.shields.io/docsrs/const_for/latest?logo=Docs.rs)](https://docs.rs/const_for/latest)

## Introduction

Regular for loops are not allowed in const contexts, because it relies on iterators, which are not available in const.\
This is rather annoying when writing const functions, as you need to write custom for loops using 'loop' or 'while'.

This crate provides a macro implementation of a for loop over a range that is usable in const contexts.\
The aim is to imitate a regular for loop as closely as possible. It handles break and continue correctly, and the variable is immutable in the body.\
To make the for loop as versatile as possible, it comes with macro variants to handle .rev() and .step_by(x), which imitates the respective function calls.
This is necessary, as normally they depend on non-const iterators. But they can be used here with identical syntax.

The main restriction is that the macro only supports standard(exclusive) ranges, eg. 0..10 and -5..5, but not ..5 or 0..=10. This is mostly a limit of current stable Rust, and wont be possible without using nightly before #![feature(const_range_bounds)] becomes stable.

```rust
# use const_for::*;
let mut a = 0;
ctfor!(i in 0..5 => {
    a += i
});
assert!(a == 10)
```

This is equivalent to the following regular for loop, except it is usable in const context.

```rust
let mut a = 0;
for i in 0..5 {
    a += i
}
assert!(a == 10)
```

## Custom step size

A custom step size can be set:

```rust
# use const_for::*;
let mut v = Vec::new();
ctfor!(i in (0..5).step_by(2) => {
    v.push(i)
});
assert!(v == vec![0, 2, 4])
```

The loop behaves as if the function was called on the range, but it is implemented by a macro.\
It is equivalent to the following non-const loop:

```rust
let mut v = Vec::new();
for i in (0..5).step_by(2) {
    v.push(i)
}
assert!(v == vec![0, 2, 4])
```

## Reversed

Iteration can be reversed:

```rust
# use const_for::*;
let mut v = Vec::new();
ctfor!(i in (0..5).rev() => {
    v.push(i)
});
assert!(v == vec![4, 3, 2, 1, 0])
```

The loop behaves as if the function was called on the range, but it is implemented by a macro.\
It is equivalent to the following non-const loop:

```rust
# use const_for::*;
let mut v = Vec::new();
for i in (0..5).rev() {
    v.push(i)
}
assert!(v == vec![4, 3, 2, 1, 0])
```

## Reversed and custom step size

It is possible to combine rev and step_by, but each can only be appended once. So the following two examples are the only legal combinations.

```rust
# use const_for::*;
// Reverse, then change step size
let mut v = Vec::new();
ctfor!(i in (0..10).rev().step_by(4) => {
    v.push(i)
});
assert!(v == vec![9, 5, 1]);

// Change step size, then reverse
let mut v = Vec::new();
ctfor!(i in (0..10).step_by(4).rev() => {
    v.push(i)
});
assert!(v == vec![8, 4, 0])
```

### Real world example

Here is an example of how this crate helped make some actual code much nicer and readable.

The code was taken (and edited a bit for clarity) from the [Cadabra](https://github.com/JENebel/Cadabra/blob/master/prepare_constants.rs) chess engine.

Before:

```rust
const fn gen_white_pawn_attacks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            let index = (rank*8+file) as usize;
            if file != 7 { masks[index] |= (1 << index) >> 7 as u64 }
            if file != 0 { masks[index] |= (1 << index) >> 9 as u64 }

            file += 1;
        }
        rank += 1;
    }

    masks
}
```

After:

```rust
# use const_for::*;
const fn gen_white_pawn_attacks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    ctfor!(rank in 0..8 => {
        ctfor!(file in 0..8 => {
            let index = (rank*8+file) as usize;
            if file != 7 { masks[index] |= (1 << index) >> 7 as u64 }
            if file != 0 { masks[index] |= (1 << index) >> 9 as u64 }
        })
    });

    masks
}
```
