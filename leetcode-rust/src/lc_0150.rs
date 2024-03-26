pub struct Solution;
use std::ops::{Add, Div, Mul, Sub};

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        let reduce_end = |f: fn(_, _) -> _, stack: &mut Vec<_>| {
            let (o2, o1) = (stack.pop().unwrap(), stack.pop().unwrap());
            stack.push(f(o1, o2));
        };

        for token in tokens {
            match token.as_str() {
                "+" => reduce_end(i32::add, &mut stack),
                "-" => reduce_end(i32::sub, &mut stack),
                "*" => reduce_end(i32::mul, &mut stack),
                "/" => reduce_end(i32::div, &mut stack),
                number => stack.push(number.parse().unwrap()),
            }
        }
        stack.pop().unwrap()
    }
}
