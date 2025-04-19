#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Node {
            value,
            next: None,
        };

        if let Some(current_head) = self.head.take() {
            let mut new_head = Node {
                value: new_node.value,
                next: Some(Box::new(current_head)),
            };

            self.head = Some(new_head);
        } else {
            self.head = Some(new_node);
        }
    }

    pub fn pop(&mut self) {
        if let Some(head) = self.head.take() {
            self.head = head.next.map(|boxed_node| *boxed_node);
        }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;

        while let Some(node) = current {
            count += 1;
            current = &node.next.as_ref().map(|boxed_node| &**boxed_node);
        }

        count
    }
}