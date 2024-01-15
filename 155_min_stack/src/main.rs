struct MinStack {
    stack: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
        }
    }
    fn push(&mut self, val: i32) {
        let min = if self.stack.is_empty() {
            val
        } else {
            std::cmp::min(val, self.get_min())
        };
        self.stack.push((val, min));
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

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

 fn main() {
    let mut min_stack = MinStack::new();
    println!("{:?}", min_stack.push(-2));
    println!("{:?}", min_stack.push(0));
    println!("{:?}", min_stack.push(-3));
    println!("{:?}", min_stack.get_min());
    println!("{:?}", min_stack.pop());
    println!("{:?}", min_stack.top());
    println!("{:?}", min_stack.get_min());
}