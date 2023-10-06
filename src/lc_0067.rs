// add binary
pub struct Solution;

impl Solution {
    pub fn add_binary_2ms(a: String, b: String) -> String {
        // work with bytes where: 48 => '0', 49 => '1'
        use std::collections::VecDeque;
        let mut out: VecDeque<u8> = VecDeque::with_capacity(std::cmp::max(a.len(), b.len()));
        let mut a = a.bytes().rev();
        let mut b = b.bytes().rev();
        let mut carry = 0;
        loop {
            let (x, y) = (a.next(), b.next());
            if let (None, None) = (x, y) {
                if carry == 1 {
                    out.push_front(49);
                }
                break;
            } else {
                let x = x.unwrap_or(48);
                let y = y.unwrap_or(48);
                let sum = carry + x + y - 96;
                out.push_front((sum & 1) + 48);
                carry = (sum & 2) >> 1;
            }
        }
        String::from_utf8(out.into()).unwrap()
    }

    // 0ms
    pub fn add_binary(a: String, b: String) -> String {
        let a: Vec<u8> = a.bytes().rev().collect();
        let b: Vec<u8> = b.bytes().rev().collect();
        let mut idx = 0;
        let mut carry = 0;
        let mut out =
            std::collections::VecDeque::with_capacity(std::cmp::max(a.len(), b.len()) + 1);
        while idx < a.len() && idx < b.len() {
            let sum = a[idx] + b[idx] + carry;
            println!("A: {x} {carry} {sum}", x = (sum & 1) + 48);
            out.push_front((sum & 1) + 48);
            carry = (sum & 2) >> 1;
            idx += 1;
        }
        while idx < a.len() {
            let sum = a[idx] + carry;
            println!("B: {x} {carry}", x = (sum & 1) + 48);
            out.push_front((sum & 1) + 48);
            carry = (sum & 2) >> 1;
            idx += 1;
        }
        while idx < b.len() {
            let sum = b[idx] + carry;
            println!("C: {x} {carry}", x = (sum & 1) + 48);
            out.push_front((sum & 1) + 48);
            carry = (sum & 2) >> 1;
            idx += 1;
        }
        if carry != 0 {
            println!("D: 49");
            out.push_front(49);
        }
        String::from_utf8(out.into()).unwrap()
    }
}

#[cfg(test)]
mod test_0067 {
    use super::*;

    #[test]
    fn eg1() {
        let ans = Solution::add_binary("11".to_string(), "1".to_string());
        assert_eq!(ans, "100".to_string());
    }
}
