/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/
// I AM NOT DONE

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue::new()
    }
}

pub struct MyStack<T> {
    q1: Queue<T>, // 主队列，负责存储元素
    q2: Queue<T>, // 辅助队列，用于帮助pop操作
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::new(),
            q2: Queue::new(),
        }
    }

    // 将元素推入栈中
    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
    }

    // 从栈中弹出元素
    pub fn pop(&mut self) -> Result<T, &str> {
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }

        // 将 q1 中的元素除最后一个以外，全部移到 q2
        while self.q1.size() > 1 {
            let front = self.q1.dequeue().unwrap();
            self.q2.enqueue(front);
        }

        // 弹出 q1 中的最后一个元素
        let popped_value = self.q1.dequeue().unwrap();

        // 交换 q1 和 q2，使 q1 始终为存储元素的队列
        std::mem::swap(&mut self.q1, &mut self.q2);

        Ok(popped_value)
    }

    // 检查栈是否为空
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}