// K closest points to the origin
pub struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut heap = std::collections::BinaryHeap::with_capacity(k);
        let distance = |p: &Vec<i32>| p[0] * p[0] + p[1] * p[1];
        let mut it = points.iter().map(distance).enumerate();
        // populate the heap with the first k elements
        for _ in 0..k {
            let (i, d) = it.next().unwrap();
            heap.push((d, i));
        }
        // loop through remaining points keeping the lowest k points
        // or more correctly their indices
        for (i, d) in it {
            if d < heap.peek().unwrap().0 {
                heap.push((d, i));
                heap.pop(); // technically we dont need to pop here, as long as we restrict the
                            // iterator down the line to only the k first elements, but this would use
                            // unecessary memory
            }
        }
        // get k closest points from their indices
        heap.iter().map(|&(_, i)| points[i].clone()).collect()
    }
}
