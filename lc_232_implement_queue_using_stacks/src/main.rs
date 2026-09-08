struct MyQueue {
    push_mode: bool,
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            push_mode: true,
            stack1: vec![],
            stack2: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        if !self.push_mode {
            while !self.stack2.is_empty() {
                self.stack1.push(self.stack2.pop().unwrap());
            }
            self.push_mode = true
        }
        self.stack1.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.push_mode {
            while !self.stack1.is_empty() {
                self.stack2.push(self.stack1.pop().unwrap());
            }
            self.push_mode = false;
        }
        self.stack2.pop().expect("empty check")
    }

    fn peek(&mut self) -> i32 {
        if self.push_mode {
            while !self.stack1.is_empty() {
                self.stack2.push(self.stack1.pop().unwrap());
            }
            self.push_mode = false;
        }
        self.stack2.last().expect("empty check").clone()
    }

    fn empty(&mut self) -> bool {
        if self.push_mode {
            self.stack1.is_empty()
        } else {
            self.stack2.is_empty()
        }
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut my_queue = MyQueue::new();
        my_queue.push(1); // queue is: [1]
        my_queue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
        assert_eq!(my_queue.peek(), 1);
        assert_eq!(my_queue.pop(), 1); // queue is [2]
        assert_eq!(my_queue.empty(), false);
    }
}
