/// using heap start
use std::cmp::Ordering;
use std::collections::binary_heap::BinaryHeap;
use std::collections::HashMap;

//struct Node(i32, usize);

#[derive(Debug)]
struct Node {
    freq: i32,
    val: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        self.freq.cmp(&other.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        //        self.freq == other.freq
        self.val == other.val
    }
}

impl Eq for Node {}

/// using heap end

struct Solution;

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let mut hash = HashMap::new();
        for code in barcodes {
            if hash.contains_key(&code) {
                hash.insert(code, hash.get(&code).unwrap() + 1);
            } else {
                hash.insert(code, 1);
            }
        }

        let mut heap = BinaryHeap::new();
        for (k, v) in hash.iter() {
            heap.push(Node { val: *k, freq: *v });
        }
        let mut prev = Node { val: 0, freq: -1 };
        while !heap.is_empty() {
            let mut top = heap.pop().unwrap();
            res.push(top.val);

            if prev.freq > 0 {
                heap.push(prev);
            }
            top.freq -= 1;
            prev = top;
            //            dbg!(&heap, &res);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q1054() {}
}
