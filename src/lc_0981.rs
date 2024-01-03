#![allow(unused)]

struct TimeMap {
    map: std::collections::HashMap<String, Vec<(i32, String)>>,
}

// sets only have incremental timestamps, so the Vecs are always sorted by timestamp
// we can then use binary search to get values
impl TimeMap {
    fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((timestamp, value));
    }

    fn get(&mut self, key: String, timestamp: i32) -> String {
        let Some(vec) = self.map.get(&key) else {
            return String::new();
        };
        match vec.binary_search_by_key(&timestamp, |&(t, _)| t) {
            Ok(i) => vec[i].1.clone(),
            Err(i) if i != 0 => vec[i - 1].1.clone(),
            _ => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        let mut time_map = TimeMap::new();
        time_map.set("foo".to_owned(), "bar".to_owned(), 1);
        assert_eq!(&time_map.get("foo".to_owned(), 1), "bar");
        assert_eq!(&time_map.get("foo".to_owned(), 3), "bar");
        time_map.get("foo".to_owned(), 3);
        time_map.set("foo".to_owned(), "bar2".to_owned(), 4);
        assert_eq!(&time_map.get("foo".to_owned(), 4), "bar2");
        assert_eq!(&time_map.get("foo".to_owned(), 5), "bar2");
    }
}
