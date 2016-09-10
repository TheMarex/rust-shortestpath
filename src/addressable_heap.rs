
//! Implements a heap that stores a handle for each elements and allows modification
//! of the key associated witht the handle
pub trait AddressableHeap<Key> {
    type Handle;

    fn len(&self) -> usize;
    fn min(&self) -> Option<Self::Handle>;
    fn min_key(&self) -> Option<Key>;
    fn push(&mut self, Self::Handle, Key);
    fn pop(&mut self) -> Option<Self::Handle>;
    fn decrease(&mut self, Self::Handle, Key);
}

pub struct AddressableBinaryHeap<Key> {
    binary_tree: Vec<(Key, u32)>,
    handle_to_index: Vec<u32>
}

impl<Key: Ord> AddressableBinaryHeap<Key> {
    fn new(num_handles: usize) -> AddressableBinaryHeap<Key> {
        let mut handle_to_index : Vec<u32> = Vec::new();
        handle_to_index.resize(num_handles, u32::max_value());
        AddressableBinaryHeap {binary_tree: vec![], handle_to_index: handle_to_index}
    }

    fn update_handle(&mut self, idx: usize) {
        let handle = self.binary_tree[idx as usize].1;
        self.handle_to_index[handle as usize] = idx as u32;
    }

    // left child = (parent+1)*2
    // right child = (parent+1)*2 + 1
    fn heap_up(&mut self, start_idx: usize) {
        for idx in (start_idx..self.binary_tree.len()).rev() {
            let parent_idx = idx / 2 - 1;
            if self.binary_tree[idx].0 > self.binary_tree[parent_idx].0 {
                self.binary_tree.as_mut_slice().swap(idx, parent_idx);
                self.update_handle(idx);
                self.update_handle(parent_idx);
            }
        }
    }

    // swaps down the hole created by removing the top at idx
    fn heap_down(&mut self, idx: usize) {
        let mut parent_idx = idx;
        let mut left_idx = (parent_idx+1)*2;
        let mut right_idx = left_idx + 1;

        while right_idx < self.binary_tree.len() {
            let min_idx = if self.binary_tree[right_idx] < self.binary_tree[left_idx] {
                right_idx
            } else {
                left_idx
            };
            self.binary_tree.as_mut_slice().swap(parent_idx as usize, min_idx as usize);
            // update handle of child that used to be at min_idx
            self.update_handle(parent_idx);

            parent_idx = min_idx;
            left_idx = (parent_idx+1)*2;
            right_idx = left_idx + 1;
        }

        // parent_idx could still be a half-leaf
        if left_idx < self.binary_tree.len() {
            self.binary_tree.as_mut_slice().swap(parent_idx, left_idx);
            // update handle of child that used to be at left_idx
            self.update_handle(parent_idx);
        }

        // we transported the hole down to the last element, now we cut off the hole
        let new_len = self.binary_tree.len() - 1;
        self.binary_tree.truncate(new_len);
    }
}

impl<Key: Ord + Copy> AddressableHeap<Key> for AddressableBinaryHeap<Key> {
    type Handle = u32;

    fn len(&self) -> usize {
        self.binary_tree.len()
    }

    fn min_key(&self) -> Option<Key> {
        self.binary_tree.first().map(|top| top.0)
    }

    fn min(&self) -> Option<Self::Handle> {
        self.binary_tree.first().map(|top| top.1)
    }

    fn push(&mut self, h: Self::Handle, k: Key) {
        let tree_idx = self.binary_tree.len();
        self.handle_to_index[h as usize] = tree_idx as u32;
        self.binary_tree.push((k, h));
        self.heap_up(tree_idx);
    }

    fn pop(&mut self) -> Option<Self::Handle> {
        if self.binary_tree.is_empty() {
            return None;
        }

        let handle = self.min();
        self.heap_down(0);
        return handle;
    }

    fn decrease(&mut self, handle: Self::Handle, k: Key) {
        let idx = self.handle_to_index[handle as usize];
        if idx == u32::max_value() {
            panic!("Handle {} is was not inserted yet", handle);
        }

        // update the key
        self.binary_tree[idx as usize].0 = k;
        self.heap_up(idx as usize);
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
        assert_eq!(h.min_key(), None);
        assert_eq!(h.pop(), None);
    }
}
