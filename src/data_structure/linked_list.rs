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

impl From<Vec<i32>> for ListNode {
    fn from(v: Vec<i32>) -> Self {
        let mut curr = None;
        for &i in v.iter().rev() {
            let mut new_node = ListNode::new(i);
            new_node.next = curr;
            curr = Some(Box::new(new_node));
        }
        *curr.expect("Cannot convert an empty vector to a ListNode")
    }
}

impl From<ListNode> for Vec<i32> {
    fn from(node: ListNode) -> Self {
        let mut result = Vec::new();
        let mut curr: Option<Box<ListNode>> = Some(Box::new(node));
        // Notice the Linked list start Without Option<ListNode> Type
        // We need to wrap it with Some to make it similar with inner nodes.
        while let Some(n) = curr {
            // println!("Heap address: {:p}", n);
            result.push(n.value);
            curr = n.next;
        }
        result
    }
}
impl Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut curr = Some(self);
        while let Some(node) = curr {
            write!(f, "{} -> ", node.value)?;
            curr = node.next.as_deref();
        }
        write!(f, "None")
    }
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

    fn push_front(&mut self, value: i32) {
        // Memomory swapping
        // std::mem::replace(dest, src) does three things in one atomic step:
        // 1. Reads the value out of dest (self).
        // 2. Writes the src (ListNode::new(value)) into that same memory location.
        // 3. Returns the original value that was in dest.
        let old_node = std::mem::replace(self, ListNode::new(value));
        self.next = Some(Box::new(old_node));
    }
    fn pop_front(&mut self) -> Option<i32> {
        // Step 1: Check if there is a next node to replace the current one.
        // .take() moves the Box out of the Option, leaving None behind. example below:
        // let mut my_option = Some("hello".to_string());
        // let taken_value = my_option.take();
        // println!("Taken value: {:?}", taken_value);     // Output: Taken value: Some("hello")
        // println!("My option after take: {:?}", my_option); // Output: My option after take: None
        if let Some(boxed_next) = self.next.take() {
            // Step 2: Unbox the next node so we have the raw ListNode struct.
            let next_node = *boxed_next;
            // Step 3: Swap 'self' with 'next_node'.
            // 'self' gets overwritten with the data from the next node.
            // 'old_head' receives the data that was just in 'self'.
            let old_head = std::mem::replace(self, next_node);
            // Step 4: Return the value of the old head.
            Some(old_head.value)
        } else {
            // Edge Case: The list has only one node.
            // Since 'self' is a struct and not an Option, we cannot delete the
            // final node to make the list truly "empty" (Size 0).
            None
        }
    }

    /// Inserts a new node at the specified index.
    /// This uses `std::mem::replace` to swap the existing node with the new one,
    /// and then links the old node as the 'next' child of the new node.
    fn push_at(&mut self, index: usize, value: i32) {
        let mut curr = Some(self);
        let mut count = 0;
        while let Some(node) = curr {
            if count == index {
                // std::mem::replace allows us to move out the value of 'node' (the old node at this index)
                // and replace it with a new ListNode, all while staying within the borrow checker's rules.
                let node_at_index = std::mem::replace(node, ListNode { value, next: None });

                // Now we link the old node that was previously at this position as the child of the new node.
                node.next = Some(Box::new(node_at_index));
            }
            // Move to the next node by taking a mutable reference to the inner value of the Option Box.
            curr = node.next.as_deref_mut();
            count += 1;
        }
    }

    /// Removes a node at the specified index and returns its value.
    /// It swaps the content of the target node with its successor (next node).
    fn pop_at(&mut self, index: usize) -> Option<i32> {
        let mut curr = Some(self);
        let mut count = 0;
        while let Some(node) = curr {
            if count == index {
                // To "delete" a node in this struct (where self is the head and not an Option),
                // we take its next neighbor and move its contents into the current position.
                if let Some(boxed_next) = node.next.take() {
                    let next_node = *boxed_next;
                    // Replace the current node's data with the next node's data,
                    // effectively "deleting" the current node by overwriting it with the next one.
                    let old_node = std::mem::replace(node, next_node);
                    return Some(old_node.value);
                }
            }
            curr = node.next.as_deref_mut();
            count += 1
        }
        None
    }

}

pub fn run() {

}
