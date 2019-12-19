use std::cell::RefCell;
use std::collections::VecDeque;

struct MinStack {
    s: RefCell<VecDeque<i32>>,
    min_stack: RefCell<VecDeque<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            s: RefCell::new(VecDeque::new()),
            min_stack: RefCell::new(VecDeque::from(vec![std::i32::MAX])),
        }
    }

    fn push(&self, x: i32) {
        if x <= *self.min_stack.borrow().back().unwrap() {
            self.min_stack.borrow_mut().push_back(x);
        }
        self.s.borrow_mut().push_back(x);
    }

    fn pop(&self) {
        if self.s.borrow_mut().pop_back().unwrap() == *self.min_stack.borrow().back().unwrap() {
            self.min_stack.borrow_mut().pop_back();
        }
    }

    fn top(&self) -> i32 {
        *self.s.borrow().back().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.borrow().back().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q155() {
        let obj = MinStack::new();
        obj.push(-2);
        obj.push(0);
        obj.push(-3);
        assert_eq!(obj.get_min(), -3);
        obj.pop();
        assert_eq!(obj.get_min(), -2);
        assert_eq!(obj.top(), 0);
    }
}
