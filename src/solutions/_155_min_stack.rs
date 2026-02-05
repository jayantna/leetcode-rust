use std::i32;

struct StackElement {
    val: i32,
    min: i32,
}

struct MinStack {
    stack: Vec<StackElement>,
    min: i32,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min: i32::MAX,
        }
    }

    fn push(&mut self, val: i32) {
        let elem = StackElement {
            val: val,
            min: self.min,
        };
        self.stack.push(elem);
        self.min = self.min.min(val);
    }

    fn pop(&mut self) {
        let value = self.stack.pop().unwrap();
        self.min = value.min;
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().val
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}
