use crate::bwt::BWT_MARKER;

struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, elem: T) {
        let new_node = Node {
            element: elem,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    #[allow(dead_code)]
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.element
        })
    }

    #[allow(dead_code)]
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }
}

impl List<usize> {
    fn find_npi(&mut self, symbol: usize) -> usize {
        let mut idx = 0;
        let mut current_node = self.head.as_mut().unwrap().as_mut();

        if current_node.element == symbol {
            return idx;
        }

        idx += 1;

        while current_node.next.as_ref().unwrap().element != symbol {
            current_node = current_node.next.as_mut().unwrap();
            idx += 1
        }

        let mut next = current_node.next.take().unwrap();
        current_node.next = next.next;
        next.next = self.head.take();
        self.head = Some(next);

        idx
    }

    fn find_np(&mut self, idx: usize) -> usize {
        let mut current_node = self.head.as_mut().unwrap().as_mut();

        if idx == 0 {
            return current_node.element;
        }

        let mut c = 1;
        while c < idx {
            current_node = current_node.next.as_mut().unwrap();
            c += 1;
        }

        let mut next = current_node.next.take().unwrap();
        current_node.next = next.next;
        next.next = self.head.take();
        self.head = Some(next);

        self.head.as_ref().unwrap().element
    }
}

pub fn mtf_encode(text_content: Vec<usize>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::with_capacity(text_content.len());
    let mut list = List::new();

    for symbol in (0..BWT_MARKER + 1).rev() {
        list.push(symbol)
    }

    for &symbol in text_content.iter() {
        let idx = list.find_npi(symbol);
        result.push(idx);
    }

    result
}

pub fn mtf_decode(text_content: Vec<usize>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::with_capacity(text_content.len());
    let mut list = List::new();

    for symbol in (0..BWT_MARKER + 1).rev() {
        list.push(symbol)
    }

    for &symbol in text_content.iter() {
        let idx = list.find_np(symbol);
        result.push(idx);
    }

    result
}

#[cfg(test)]
mod test {
    use super::{mtf_encode, mtf_decode, BWT_MARKER};

    #[test]
    pub fn test_mtf() {
        let mut text_content = vec![15, 15, 15, 15, 16, 16, 231, 231, 192, 255];
        text_content.push(BWT_MARKER);
        println!("{:?}", text_content);
        let _code = mtf_encode(text_content.clone());
        println!("{:?}", _code);
        let _decode = mtf_decode(_code);
        println!("{:?}", _decode);
        assert_eq!(text_content, _decode);
    }
}
