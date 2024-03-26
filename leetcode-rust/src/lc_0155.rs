// Min Stack
#![allow(unused)]

struct MinStack {
    stack: Vec<(i32, i32)>, // (val, min)
}

impl MinStack {
    fn new() -> Self {
        MinStack { stack: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        let min = match self.stack.last() {
            Some(&(_, min)) => std::cmp::min(val, min),
            _ => val,
        };
        self.stack.push((val, min))
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}
