//! # Const for
//! [![GitHub](https://img.shields.io/badge/GitHub-black?logo=github)](https://github.com/JENebel/const_for)
//! [![crates.io](https://img.shields.io/crates/v/const_for?logo=rust&logoColor=b7410e)](http://crates.io/crates/const_for)
//! [![Docs](https://img.shields.io/docsrs/const_for/latest?logo=Docs.rs)](https://docs.rs/const_for/latest)
//! 
//! Regular for loops are not allowed in const contexts, because it relies on iterators, which are not available in const.\
//! This is rather annoying when writing const functions, as you need to write custom for loops using 'loop' or 'while'.
//! 
//! This crate provides a macro implementation of a for loop over a range that is usable in const contexts.\
//! The aim is to imitate a regular for loop as closely as possible. It handles break and continue correctly, and the variable is immutable in the body.\
//! To make the for loop as versatile as possible, it comes with macro variants to handle .rev() and step_by(x), which imitates the respective function calls.
//! This is necessary, as normally they depend on non-const iterators. But they can be used here with identical syntax.
//! 
//! The main restriction is that the macro only supports standard(exclusive) ranges, eg. 0..10 and -5..5, but not ..5 or 0..=10. This is mostly a limit of current stable Rust, and wont be possible without using nightly before #![feature(const_range_bounds)] becomes stable.
//! 
//! ```
//! # use const_for::*;
//! let mut a = 0;
//! const_for!(i in 0..5 => {
//!     a += i
//! });
//! assert!(a == 10)
//! ```
//! 
//! This is equivalent to the following regular for loop, except it is usable in const context.
//! 
//! ```
//! # use const_for::*;
//! let mut a = 0;
//! for i in 0..5 {
//!     a += i
//! }
//! assert!(a == 10)
//! ```
//! 
//! ## Custom step size
//! 
//! A custom step size can be set:
//! 
//! ```
//! # use const_for::*;
//! let mut v = Vec::new();
//! const_for!(i in (0..5).step_by(2) => {
//!     v.push(i)
//! });
//! assert!(v == vec![0, 2, 4])
//! ```
//! 
//! The loop behaves as if the function was called on the range, but it is implemented by a macro.\
//! It is equivalent to the following non-const loop:
//! 
//! ```
//! # use const_for::*;
//! let mut v = Vec::new();
//! for i in (0..5).step_by(2) {
//!     v.push(i)
//! }
//! assert!(v == vec![0, 2, 4])
//! ```
//! 
//! ## Reversed
//! 
//! Iteration can be reversed:
//! 
//! ```
//! # use const_for::*;
//! let mut v = Vec::new();
//! const_for!(i in (0..5).rev() => {
//!     v.push(i)
//! });
//! assert!(v == vec![4, 3, 2, 1, 0])
//! ```
//! 
//! The loop behaves as if the function was called on the range, but it is implemented by a macro.\
//! It is equivalent to the following non-const loop:
//! 
//! ```
//! # use const_for::*;
//! let mut v = Vec::new();
//! for i in (0..5).rev() {
//!     v.push(i)
//! }
//! assert!(v == vec![4, 3, 2, 1, 0])
//! ```
//! 
//! ## Reversed and custom step size
//! 
//! It is possible to combine rev and step_by, but each can only be appended once. So the following two examples are the only legal combinations.
//! 
//! ```
//! # use const_for::*;
//! // Reverse, then change step size
//! let mut v = Vec::new();
//! const_for!(i in (0..10).rev().step_by(4) => {
//!     v.push(i)
//! });
//! assert!(v == vec![9, 5, 1]);
//! 
//! // Change step size, then reverse
//! let mut v = Vec::new();
//! const_for!(i in (0..10).step_by(4).rev() => {
//!     v.push(i)
//! });
//! assert!(v == vec![8, 4, 0])
//! ```
//! 
//! ## Notes
//! 
//! You can use mutable and wildcard variables as the loop variable, and they act as expected.
//! 
//! ```
//! // Mutable variable
//! # use const_for::*;
//! let mut v = Vec::new();
//! const_for!(mut i in (0..4) => {
//!     i *= 2;
//!     v.push(i)
//! });
//! assert!(v == vec![0, 2, 4, 6]);
//! 
//! // Wildcard variable
//! let mut a = 0;
//! const_for!(_ in 0..5 => 
//!    a += 1
//! );
//! assert!(a == 5)
//! ```
//! 
//! The body of the loop can be any statement. This means that the following is legal, even though it is not in a regular for loop.
//! 
//! ```
//! # use const_for::*;
//! let mut a = 0;
//! const_for!(_ in 0..5 => a += 1);
//! 
//! unsafe fn unsafe_function() {}
//! const_for!(_ in 0..5 => unsafe {
//!    unsafe_function()
//! });
//! ```
//! 
//! ### Real world example
//! 
//! Here is an example of how this crate helped make some actual code much nicer and readable.
//! 
//! The code was taken (and edited a bit for clarity) from the [Cadabra](https://github.com/JENebel/Cadabra/) chess engine.
//! 
//! Before:
//! 
//! ```
//! const fn gen_white_pawn_attacks() -> [u64; 64] {
//!     let mut masks = [0; 64];
//!     
//!     let mut rank: u8 = 0;
//!     while rank < 8 {
//!         let mut file: u8 = 0;
//!         while file < 8 {
//!             let index = (rank*8+file) as usize;
//!             if file != 7 { masks[index] |= (1 << index) >> 7 as u64 }
//!             if file != 0 { masks[index] |= (1 << index) >> 9 as u64 }
//! 
//!             file += 1;
//!         }
//!         rank += 1;
//!     }
//! 
//!     masks
//! }
//! ```
//! 
//! After:
//! 
//! ```
//! # use const_for::*;
//! const fn gen_white_pawn_attacks() -> [u64; 64] {
//!     let mut masks = [0; 64];
//!     
//!     const_for!(rank in 0..8 => {
//!         const_for!(file in 0..8 => {
//!             let index = (rank*8+file) as usize;
//!             if file != 7 { masks[index] |= (1 << index) >> 7 as u64 }
//!             if file != 0 { masks[index] |= (1 << index) >> 9 as u64 }
//!         })
//!     });
//! 
//!     masks
//! }
//! ```


/// A for loop that is usable in const contexts.
/// 
/// It aims to work exactly like a normal for loop over a standard exclusive range, eg. 0..10 or -5..5.\
/// Unfortunately it doesn't support other types of ranges like ..10 or 2..=10.\
/// So generally just use it like a regular for loop.
/// 
/// .rev() and .step_by(x) is implemented via macros instead of the non-const iter trait,
/// and makes the loop behave as expected.
/// 
/// # Examples
/// ```
/// # use const_for::*;
/// let mut a = 0;
/// const_for!(i in 0..5 => {
///     a += i
/// });
/// assert!(a == 10)
/// ```
/// 
/// This is equivalent to the following regular for loop, except it is usable in const context.
/// ```
/// # use const_for::*;
/// let mut a = 0;
/// for i in 0..5 {
///     a += i
/// }
/// assert!(a == 10)
/// ```
/// 
/// ## Custom step size
/// 
/// A custom step size can be set:
/// ```
/// # use const_for::*;
/// let mut v = Vec::new();
/// const_for!(i in (0..5).step_by(2) => {
///     v.push(i)
/// });
/// assert!(v == vec![0, 2, 4])
/// ```
/// The loop behaves as if the function was called on the range, including requiring a usize, but it is implemented by a macro.
/// 
/// ## Reversed
/// 
/// Iteration can be reversed:
/// ```
/// # use const_for::*;
/// let mut v = Vec::new();
/// const_for!(i in (0..5).rev() => {
///     v.push(i)
/// });
/// assert!(v == vec![4, 3, 2, 1, 0])
/// ```
/// The loop behaves as if the function was called on the range, but it is implemented by a macro.
/// 
/// ## Reversed and custom step size
/// 
/// It is possible to combine rev and step_by, but each can only be appended once. So the following two examples are the only legal combinations.
/// ```
/// # use const_for::*;
/// // Reverse, then change step size
/// let mut v = Vec::new();
/// const_for!(i in (0..10).rev().step_by(4) => {
///     v.push(i)
/// });
/// assert!(v == vec![9, 5, 1]);
/// 
/// // Change step size, then reverse
/// let mut v = Vec::new();
/// const_for!(i in (0..10).step_by(4).rev() => {
///     v.push(i)
/// });
/// assert!(v == vec![8, 4, 0])
/// ```
/// 
/// ## Notes
/// 
/// You can use mutable and wildcard variables as the loop variable, and they act as expected.
/// 
/// ```
/// // Mutable variable
/// # use const_for::*;
/// let mut v = Vec::new();
/// const_for!(mut i in (0..4) => {
///     i *= 2;
///     v.push(i)
/// });
/// assert!(v == vec![0, 2, 4, 6]);
/// 
/// // Wildcard variable
/// let mut a = 0;
/// const_for!(_ in 0..5 => 
///    a += 1
/// );
/// assert!(a == 5)
/// ```
/// 
/// The body of the loop can be any statement. This means that the following is legal, even though it is not in a regular for loop.
/// 
/// ```
/// # use const_for::*;
/// let mut a = 0;
/// const_for!(_ in 0..5 => a += 1);
/// 
/// unsafe fn unsafe_function() {}
/// const_for!(_ in 0..5 => unsafe {
///    unsafe_function()
/// });
/// ```

#[macro_export]
macro_rules! rev {
    ($rev:ident, rev) => {
        $rev = !$rev;
    };
    ($rev:ident, $_:ident) => {

    };
}

#[macro_export]
macro_rules! is_rev {
    ($rev:ident, $first_adapter:ident, $($adapter:ident), *) => {
        rev!($rev, $first_adapter);
        is_rev!($rev, $($adapter, ) *);
    };
    ($rev:ident, $first_adapter:ident) => {
        rev!($rev, $first_adapter);
    };
    ($rev:ident, ) => {

    };
}

#[macro_export]
macro_rules! adapter {
    ($inner:expr, $exhausted:ident, $outer:expr, rev()) => {
        $inner
    };
    ($inner:expr, $exhausted:ident, $outer:expr, map($arg:expr)) => {
        ($arg)($inner)
    };
    ($inner:expr, $exhausted:ident, $outer:expr, filter($arg:expr)) => {
        loop {
            if $exhausted {
                $outer;
            }
            let val = $inner;
            if ($arg)(&val) {
                break val;
            }
        }
    };
    ($inner:expr, $exhausted:ident, $outer:expr, step_by($arg:expr)) => {
        {
            let mut count = $arg;
            let val = $inner;
            while count > 1 {
                $inner;
                count -= 1;
            }
            val
        }
    }
}

#[macro_export]
macro_rules! adapters {
    ($inner:expr, $exhausted:ident, $outer:expr, $first_adapter:ident($($first_arg:expr), *), $($adapter:ident($($arg:expr), *), )*) => {
        {
            adapters!(adapter!($inner, $exhausted, $outer, $first_adapter($($first_arg), *)), $exhausted, $outer, $($adapter($($arg), *), )*)
        }
    };
    ($inner:expr, $exhausted:ident, $outer:expr, ) => {
        $inner
    }
}

#[macro_export]
macro_rules! next {
    ($start:ident, $end:ident, $outer:expr, $($adapter:ident($($arg:expr), *), )*) => {
        {
            #[allow(unused_mut)]
            let mut rev = false;
            let mut __exhausted = false;
            is_rev!(rev, $($adapter), *);

            adapters!({
            let val = if rev {
                $end -= 1;
                $end
            }
            else {
                let val = $start;
                $start += 1;
                val
            };
            if $start == $end {
                __exhausted = true;
            }
            val
            }, __exhausted, $outer, $($adapter($($arg), *), )*)
        }
    };
    ($start:ident, $end:ident, $outer:expr, ) => {
        {
            let val = $start;
            $start += 1;
            val
        }
    };
}

#[macro_export]
macro_rules! const_for {
    ($var:pat_param in ($range:expr)$(.$adapter:ident($($arg:expr), *))* => $body:expr) => {
        {
            let mut __start = $range.start;
            let mut __end = $range.end;
            let mut __outer = false;
            '__outer: while __start < __end {
                let $var = $crate::next!(__start, __end, { break '__outer; }, $($adapter($($arg), *), )*);
                {
                    $body
                }
            }
        }
    };
    ($var:pat_param in $range:expr => $body:expr) => {
        $crate::const_for!($var in ($range) => $body);
    };
}

#[macro_export]
macro_rules! const_for2 {
    ($var:pat_param in ($range:expr).step_by($step:expr) => $body:stmt) => {
        {
            let _: usize = $step;
            let mut __ite = $range.start;
            let __end = $range.end;
            let mut __is_first = true;
            let __step = $step;

            loop {
                if !__is_first {
                    __ite += __step
                }
                __is_first = false;

                let $var = __ite;

                if __ite >= __end {
                    break
                }

                $body
            }
        }
    };

    ($var:pat_param in ($range:expr).rev().step_by($step:expr) => $body:stmt) => {
        {
            let _: usize = $step;
            let mut __ite = $range.end - 1;
            let __start = $range.start;
            let mut __is_first = true;
            let __step = $step;

            loop {
                if !__is_first {
                    __ite -= __step
                }
                __is_first = false;

                let $var = __ite;

                if __ite < __start {
                    break
                }

                $body
            }
        }
    };

    ($var:pat_param in ($range:expr).rev() => $body:stmt) => {
        const_for!($var in ($range).rev().step_by(1) => $body)
    };

    ($var:pat_param in ($range:expr).step_by($step:expr).rev() => $body:stmt) => {
        const_for!($var in ($range.start..$range.end - ($range.end - $range.start - 1) % $step).rev().step_by($step) => $body)
    };

    ($var:pat_param in $range:expr => $body:stmt) => {
        const_for!($var in ($range).step_by(1) => $body)
    };
}

#[cfg(never)]
#[cfg(test)]
mod test {
    
    #[test]
    fn rev() {
        let expected: Vec<u64> = (0..10).rev().collect();
        let mut actual = Vec::new();
        const_for2!(i in (0..10).rev() => actual.push(i));
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn map() {
        fn f(v: u64) -> u64 {
            v * 3 + 1
        }
        let expected: Vec<u64> = (0..10).map(f).collect();
        let mut actual = Vec::new();
        const_for2!(i in (0..10).map(f) => actual.push(i));
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn filter() {
        fn f(v: &u64) -> bool {
            v % 2 == 1
        }
        let expected: Vec<u64> = (0..10).filter(f).collect();
        let mut actual = Vec::new();
        const_for2!(i in (0..10).filter(f) => actual.push(i));
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn filter_exhaust() {
        fn f(v: &u64) -> bool {
            *v < 5
        }
        let expected: Vec<u64> = (0..10).filter(f).collect();
        let mut actual = Vec::new();
        const_for2!(i in (0..10).filter(f) => actual.push(i));
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn step_by() {
        let expected: Vec<u64> = (0..10).step_by(2).collect();
        let mut actual = Vec::new();
        const_for2!(i in (0..10).step_by(2) => actual.push(i));
        assert_eq!(expected, actual);
    }

    #[test]
    fn map_rev() {
        fn f(v: u64) -> u64 {
            v * 3 + 1
        }
        let expected: Vec<u64> = (0..10).map(f).rev().collect();
        let mut actual = Vec::new();
        const_for2!(i in (0..10).map(f).rev() => actual.push(i));
        assert_eq!(expected, actual);
    }
}
