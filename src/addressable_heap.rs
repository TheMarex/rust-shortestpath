use std::fmt::Display;

// Implements a heap that stores a handle for each elements and allows modification
// of the key associated witht the handle
pub trait AddressableHeap<Key> {
    type Handle;

    fn len(&self) -> usize;
    fn min(&self) -> Option<(Self::Handle, Key)>;
    fn push(&mut self, Self::Handle, Key);
    fn pop(&mut self) -> Option<(Self::Handle, Key)>;
    // if key is bigger than the current key this is noop
    fn decrease(&mut self, Self::Handle, Key);
    fn in_heap(&self, Self::Handle) -> bool;
}

#[derive(Clone, Copy)]
struct BinaryHeapElement<Key: Copy> {
    key: Key,
    handle: u32
}

pub struct AddressableBinaryHeap<Key: Copy> {
    binary_tree: Vec<BinaryHeapElement<Key>>,
    handle_to_index: Vec<u32>
}

impl<Key: Copy + Ord + Display> AddressableBinaryHeap<Key> {
    pub fn new(num_handles: usize) -> AddressableBinaryHeap<Key> {
        let mut handle_to_index : Vec<u32> = Vec::new();
        handle_to_index.resize(num_handles, u32::max_value());
        AddressableBinaryHeap {binary_tree: vec![], handle_to_index: handle_to_index}
    }

    fn update_handle(&mut self, index: usize) {
        let handle = self.binary_tree[index as usize].handle;
        self.handle_to_index[handle as usize] = index as u32;
    }

    fn dump_heap(&self) {
        println!("### binary_tree:");
        for ref elem in &self.binary_tree {
            println!("{}, {}", elem.key, elem.handle);
        }

        println!("### handle_to_index:");
        for (h, v) in self.handle_to_index.iter().enumerate() {
            println!("{} -> {}", h, v);
        }
    }

    fn parent(&self, index: usize) -> usize {
        (index+1) / 2 - 1
    }

    fn left_child(&self, index: usize) -> usize {
        (index+1)*2 - 1
    }

    fn right_child(&self, index: usize) -> usize {
        (index+1)*2
    }

    // left child = (parent+1)*2
    // right child = (parent+1)*2 + 1
    fn heap_up(&mut self, start_index: usize) {
        if start_index < 1 {
            return;
        }

        let mut index = start_index;
        let mut parent_index = self.parent(index);
        while self.binary_tree[index].key < self.binary_tree[parent_index].key {
            self.binary_tree.as_mut_slice().swap(index, parent_index);
            self.update_handle(index);
            self.update_handle(parent_index);

            if parent_index > 0 {
                index = parent_index;
                parent_index = self.parent(index);
            } else {
                break;
            }
        }
    }

    // swaps down the hole created by removing the top at index
    fn heap_down(&mut self, index: usize) {
        let mut parent_index = index;
        let mut left_index = self.left_child(parent_index);
        let mut right_index = self.right_child(parent_index);

        while right_index < self.binary_tree.len() {
            let min_index = if self.binary_tree[right_index].key < self.binary_tree[left_index].key {
                right_index
            } else {
                left_index
            };
            // sub-heap at parent_index is a valid heap, we are finished
            if self.binary_tree[min_index].key >= self.binary_tree[parent_index].key {
                return;
            }

            self.binary_tree.as_mut_slice().swap(parent_index as usize, min_index as usize);
            self.update_handle(parent_index);
            self.update_handle(min_index);

            parent_index = min_index;
            left_index = self.left_child(parent_index);
            right_index = self.right_child(parent_index);
        }

        // parent_index could still be a half-leaf
        if left_index < self.binary_tree.len() && self.binary_tree[parent_index].key > self.binary_tree[left_index].key {
            self.binary_tree.as_mut_slice().swap(parent_index, left_index);
            self.update_handle(parent_index);
            self.update_handle(left_index);
        }
    }
}

impl<Key: Ord + Copy + Display> AddressableHeap<Key> for AddressableBinaryHeap<Key> {
    type Handle = u32;

    fn len(&self) -> usize {
        self.binary_tree.len()
    }

    fn min(&self) -> Option<(Self::Handle, Key)> {
        self.binary_tree.first().map(|top| (top.handle, top.key))
    }

    fn push(&mut self, h: Self::Handle, k: Key) {
        let tree_index = self.binary_tree.len();
        self.handle_to_index[h as usize] = tree_index as u32;
        self.binary_tree.push(BinaryHeapElement {handle: h, key: k});
        self.heap_up(tree_index);
    }

    fn pop(&mut self) -> Option<(Self::Handle, Key)> {
        if self.binary_tree.is_empty() {
            return None;
        }

        if self.binary_tree.len() > 1 {
            let element = self.binary_tree.swap_remove(0);
            self.update_handle(0);
            self.heap_down(0);
            Some((element.handle, element.key))
        } else {
            let element = self.binary_tree[0];
            self.binary_tree.clear();
            Some((element.handle, element.key))
        }
    }

    fn decrease(&mut self, handle: Self::Handle, k: Key) {
        let index = self.handle_to_index[handle as usize];
        if index == u32::max_value() {
            panic!("Handle {} is was not inserted yet", handle);
        }

        if self.binary_tree[index as usize].key <= k {
            return;
        }

        // update the key
        self.binary_tree[index as usize].key = k;
        self.heap_up(index as usize);
    }

    fn in_heap(&self, handle: Self::Handle) -> bool {
        self.handle_to_index[handle as usize] != u32::max_value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty() {
        let mut h : AddressableBinaryHeap<u32> = AddressableBinaryHeap::new(10);
        assert_eq!(h.len(), 0);
        assert_eq!(h.min(), None);
        assert_eq!(h.pop(), None);
    }

    #[test]
    fn push_ascending() {
        let mut h : AddressableBinaryHeap<u32> = AddressableBinaryHeap::new(10);
        h.push(5, 0);
        h.push(6, 1);
        h.push(0, 2);
        h.push(9, 3);
        assert_eq!(h.min(), Some((5, 0)));
        h.pop();
        assert_eq!(h.min(), Some((6, 1)));
        h.pop();
        assert_eq!(h.min(), Some((0, 2)));
        h.pop();
        assert_eq!(h.min(), Some((9, 3)));
        h.pop();
        assert_eq!(h.min(), None);
    }

    #[test]
    fn push_descending() {
        let mut h : AddressableBinaryHeap<u32> = AddressableBinaryHeap::new(10);
        h.push(5, 3);
        h.push(6, 2);
        h.push(0, 1);
        h.push(9, 0);
        assert_eq!(h.min(), Some((9, 0)));
        h.pop();
        assert_eq!(h.min(), Some((0, 1)));
        h.pop();
        assert_eq!(h.min(), Some((6, 2)));
        h.pop();
        assert_eq!(h.min(), Some((5, 3)));
        h.pop();
        assert_eq!(h.min(), None);
    }

    #[test]
    fn push_mixed() {
        let mut h : AddressableBinaryHeap<u32> = AddressableBinaryHeap::new(10);
        h.push(5, 3);
        h.push(6, 2);
        h.push(0, 0);
        h.push(9, 1);
        h.push(8, 4);
        assert_eq!(h.min(), Some((0, 0)));
        h.pop();
        assert_eq!(h.min(), Some((9, 1)));
        h.pop();
        assert_eq!(h.min(), Some((6, 2)));
        h.pop();
        assert_eq!(h.min(), Some((5, 3)));
        h.pop();
        assert_eq!(h.min(), Some((8, 4)));
        h.pop();
        assert_eq!(h.min(), None);
    }

    #[test]
    fn decrease_key() {
        let mut h : AddressableBinaryHeap<u32> = AddressableBinaryHeap::new(10);
        h.push(5, 4);
        h.push(6, 3);
        h.push(0, 2);
        h.push(9, 1);
        assert_eq!(h.min(), Some((9, 1)));
        // should be a noop
        h.decrease(9, 5);
        assert_eq!(h.min(), Some((9, 1)));
        h.decrease(5, 0);
        assert_eq!(h.min(), Some((5, 0)));
        h.pop();
        assert_eq!(h.min(), Some((9, 1)));
        h.decrease(6, 0);
        assert_eq!(h.min(), Some((6, 0)));
        h.pop();
        assert_eq!(h.min(), Some((9, 1)));
        h.decrease(9, 0);
        assert_eq!(h.min(), Some((9, 0)));
        h.pop();
        assert_eq!(h.min(), Some((0, 2)));
    }


}
