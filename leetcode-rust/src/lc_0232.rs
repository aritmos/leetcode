// implement queue using stacks
// TODO
use std::collections::VecDeque;

#[derive(Default)]
pub struct MyQueue {
    vals: VecDeque<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        MyQueue {
            vals: VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.vals.push_back(x);
    }

    pub fn pop(&mut self) -> i32 {
        self.vals.pop_front().unwrap()
    }

    pub fn peek(&mut self) -> i32 {
        *self.vals.front().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.vals.is_empty()
    }
}

#[cfg(test)]
mod lc_232 {
    use super::*;

    #[test]
    fn eg1() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 2);
        assert_eq!(queue.pop(), 1);
        assert!(!queue.empty());
    }
}
