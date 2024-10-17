/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM NOT DONE

use std::cmp::Ord;
use std::default::Default;

impl<T> Heap<T>
where
    T: Default,
{
    pub fn add(&mut self, value: T) {
        // 将新元素添加到堆末尾
        self.items.push(value);
        self.count += 1;

        // 上浮新元素到正确位置，保持堆的性质
        self.swim(self.count);
    }

    // 上浮操作：新添加的元素如果比父节点更符合堆的顺序条件，则上浮
    fn swim(&mut self, mut idx: usize) {
        // 当idx大于1，且当前元素符合上浮条件时，与父节点交换
        while idx > 1 && (self.comparator)(&self.items[idx], &self.items[self.parent_idx(idx)]) {
            self.items.swap(idx, self.parent_idx(idx));
            idx = self.parent_idx(idx);
        }
    }

    pub fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // 将堆顶元素与最后一个元素交换，并移除最后一个元素
        self.items.swap(1, self.count);
        let result = self.items.pop();
        self.count -= 1;

        // 下沉堆顶元素到正确位置，保持堆的性质
        self.sink(1);

        result
    }

    // 下沉操作：如果父节点比子节点更符合堆的顺序条件，进行下沉
    fn sink(&mut self, mut idx: usize) {
        // 当存在子节点时
        while self.children_present(idx) {
            // 找到最符合堆顺序条件的子节点
            let mut child = self.left_child_idx(idx);
            if child < self.count && (self.comparator)(&self.items[child + 1], &self.items[child]) {
                child += 1;
            }

            // 如果当前元素已经比最符合堆顺序的子节点更合适，则停止下沉
            if !(self.comparator)(&self.items[child], &self.items[idx]) {
                break;
            }

            // 否则与子节点交换位置，继续下沉
            self.items.swap(idx, child);
            idx = child;
        }
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        if right <= self.count && (self.comparator)(&self.items[right], &self.items[left]) {
            return right;
        }

        left
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}