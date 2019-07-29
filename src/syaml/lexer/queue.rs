use std::clone::Clone;

/// Simple queue data structure
struct Queue<T>  {
    buffer: Vec<T>,
}

impl <T> Queue<T> {


    /// Creates the queue empty
    fn build() -> Queue<T> {
        Queue {
            buffer: Vec::new(),
        }
    }

    /// Returns the size of the queue
    fn size(&self) -> usize {
        // self.tail - self.head
        self.buffer.len()
    }

    /// Append an element to the queue incrementing its size
    fn append(&mut self, x: T) -> () {
        self.buffer.push(x);
    }

    /// Reduces the size of the queue to a certain number of elements
    fn reduce_to(&mut self, new_size: usize) -> () {
        if new_size < self.buffer.len() {
            /*
            while self.buffer.len() > new_size {
                self.buffer.pop();
            }
            */
            unsafe { self.buffer.set_len(new_size) }
        }

    }

    /// Returns the first element in the queue
    fn dequeue(&mut self) -> T {
        self.buffer.remove(0)
    }

    fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}


#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn queue_ops() {
        let mut q = Queue::build();
        q.append(1);
        q.append(2);

        assert_eq!(q.size(), 2);

        assert_eq!(q.dequeue(), 1);
        assert_eq!(q.dequeue(), 2);

        assert_eq!(q.size(), 0);

        assert!(q.is_empty());

        q.append(1);
        q.append(2);
        q.append(3);
        q.append(4);
        q.dequeue();

        q.reduce_to(2);

        assert_eq!(q.size(), 2);

        assert_eq!(q.dequeue(), 2);
        assert_eq!(q.dequeue(), 3);

    }
}