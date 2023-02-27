use std::collections::VecDeque;

pub struct Queue {
    arr: VecDeque<String>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            arr: VecDeque::<String>::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.arr.len()
    }

    pub fn enqueue(&mut self, message: String) {
        self.arr.push_back(message);
    }

    pub fn dequeue(&mut self) -> Option<String> {
        self.arr.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue() {
        let mut queue = Queue::new();
        queue.enqueue("This is a test message".to_string());
        assert!(queue.len() == 1);
    }

    #[test]
    fn test_dequeue() {
        let mut queue = Queue::new();
        let first_message = "This is the first message";
        let second_message = "This is the second message";
        queue.enqueue(first_message.to_string());
        queue.enqueue(second_message.to_string());

        assert!(queue.dequeue().unwrap() == first_message);
        assert!(queue.dequeue().unwrap() == second_message);
    }
}
