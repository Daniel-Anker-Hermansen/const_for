use const_for::*;

macro_rules! validate_loop {
    (@impl $($loop:tt)*) => {
        let mut c_values_hit = Vec::new();
        const_for!(i in $($loop)* => {
            c_values_hit.push(i);
        });

        let mut r_values_hit = Vec::new();
        for i in $($loop)* {
            r_values_hit.push(i);
        };

        assert!(c_values_hit == r_values_hit);
    };

    ($step: expr, $($loop:tt)*) => {
        validate_loop!(@impl ($($loop)*).step_by(1));
        validate_loop!(@impl ($($loop)*).step_by(1).rev());
        validate_loop!(@impl ($($loop)*).rev().step_by(1));
    };

    ($($loop:tt)*) => {
        validate_loop!(@impl $($loop)*);
        validate_loop!(@impl ($($loop)*).rev());
        
        validate_loop!(1, $($loop)*);
        validate_loop!(2, $($loop)*);
        validate_loop!(3, $($loop)*);
        validate_loop!(8, $($loop)*);
        validate_loop!(15, $($loop)*);
        validate_loop!(17, $($loop)*);
        validate_loop!(45, $($loop)*);
        validate_loop!(150, $($loop)*);
    };
}

#[allow(unused_parens)]
#[test]
fn equivalent_to_regular_for() {
    validate_loop!(-10..10);
    validate_loop!(0..10);
    validate_loop!(-10..10);
    validate_loop!((0..10));
    validate_loop!(100..10);
    validate_loop!(-15..-12);
    validate_loop!(-14..0);
    validate_loop!(-100..-50);
    validate_loop!(-14..200);
    validate_loop!(1..11110);
}

#[test]
const fn available_in_const() {
    let mut a = 0;

    const_for!(i in 0..25 => {
        a += 1
    });
    const_for!(i in (0..25).rev() => {
        a += 1
    });
    const_for!(i in (0..100).step_by(2) => {
        a += 1
    });

    assert!(a == 100)
}