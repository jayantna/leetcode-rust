use std::fmt::Display;
use std::{ffi::os_str, usize};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub value: i32, // 4 bytes
    // Option<> to point None to last node
    // Box<> is smart pointer used to solve infinite size problem.
    // compiler must know exactly how much space a struct takes up on the stack at compile time.
    // Instead of memory on the stack, box assigns memory on the heap.
    // A Box<T> is a pointer to memory on the heap, which is 8 bytes
    // This way compiler knows the fixed size 13 bytes of ListNode.
    pub next: Option<Box<ListNode>>, //8 bytes on 64bit system.
                                     // Box owns the data it points to. When the Box goes out of scope, Rust’s ownership system automatically:
                                     // Calls the destructor (if any) for the data.
                                     // Deallocates the memory on the heap.
}



impl ListNode {
    // There is no self in function parameter because there is no instance of ListNode created earlier.
    // If new requires self, it would already need to have a ListNode instance. (Chicken and Egg problem).
    // This type of function without self are called Assiciated Functions. (One with self are called methods which requires instance of ListNode).
    pub fn new(value: i32) -> ListNode {
        ListNode {
            value: value,
            next: None,
        }
    }

    fn len(&self) -> i32 {
        let mut count = 0;
        let mut curr = Some(self);
        while let Some(node) = curr {
            count += 1;
            curr = node.next.as_deref();
        }
        count
    }

    fn get(&self, index: usize) -> Option<i32> {
        let mut curr = Some(self);
        let mut current_index: usize = 0;
        while let Some(node) = curr {
            if (current_index == index) {
                return Some(node.value);
            } else {
                curr = node.next.as_deref();
                current_index += 1;
            }
        }
        None
    }

    fn head(&self) -> Option<i32> {
        return ListNode::get(&self, 0);
    }

    
}

pub fn run() {

}
