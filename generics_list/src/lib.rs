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

        if self.head.is_none() {
            self.head = Some(new_node);
            return;
        }

        let current = self.head.take().unwrap();
        let new_node = Node {
            value: new_node.value,
            next: Some(Box::new(current)),
        };

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) {
        if self.head.is_none() {
            return;
        }

        let current = self.head.take().unwrap();

        if let Some(next_node) = current.next {
            self.head = Some(*next_node);
        }
    }

    pub fn len(&self) -> usize {
        fn count_nodes<T>(node: &Option<Node<T>>) -> usize {
            match node {
                None => 0,
                Some(n) => {
                    let mut count = 1;
                    let mut current = &n.next;
                    
                    // Count nodes in the chain
                    while let Some(boxed_node) = current {
                        count += 1;
                        current = &boxed_node.next;
                    }
                    
                    count
                }
            }
        }
        
        count_nodes(&self.head)
    }
}