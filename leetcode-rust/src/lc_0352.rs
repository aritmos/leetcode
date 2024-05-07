#[derive(Default)]
pub struct SummaryRanges {
    ranges: Vec<[i32; 2]>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_num(&mut self, value: i32) {
        if self.ranges.is_empty() {
            self.ranges.push([value, value]);
            return;
        }

        let Err(idx) = self.ranges.binary_search_by_key(&value, |a| a[1]) else {
            return;
        };

        // if the value is within an already existing range, do nothing
        if idx != self.ranges.len() && self.ranges[idx][0] <= value {
            return;
        }

        // value is in between ranges

        let mut is_new_range = true;

        if idx != 0 && self.ranges[idx - 1][1] + 1 == value {
            self.ranges[idx - 1][1] = value;
            is_new_range = false;
        }
        if idx != self.ranges.len() && self.ranges[idx][0] - 1 == value {
            self.ranges[idx][0] = value;
            is_new_range = false;
        }
        if idx != 0 && idx != self.ranges.len() && self.ranges[idx - 1][1] == self.ranges[idx][0] {
            self.ranges[idx - 1][1] = self.ranges[idx][1];
            self.ranges.remove(idx);
        }

        if is_new_range {
            self.ranges.insert(idx, [value, value]);
        }
    }

    pub fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.ranges.iter().map(|a| a.into()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut ranges = SummaryRanges::new();
        ranges.add_num(1);
        println!("{:?}", ranges.get_intervals());
        ranges.add_num(3);
        println!("{:?}", ranges.get_intervals());
        ranges.add_num(2);
        println!("{:?}", ranges.get_intervals());
        ranges.add_num(7);
        println!("{:?}", ranges.get_intervals());
    }
}
