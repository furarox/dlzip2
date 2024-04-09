use std::collections::{VecDeque, HashMap};

const HUFFMAN_MARKER: usize = 259;

#[derive(Debug)]

struct HuffmanTreeNode {
    symbol: Option<usize>,
    occurence: Occurence,
    left_child: Option<Box<HuffmanTreeNode>>,
    right_child: Option<Box<HuffmanTreeNode>>,
}

impl HuffmanTreeNode {
    fn merge(h1: Self, h2: Self) -> Self {
        HuffmanTreeNode {
            symbol: Option::None,
            occurence: Occurence {
                int: h1.occurence.int + h2.occurence.int,
                finite: true,
            },
            left_child: Some(Box::new(h1)),
            right_child: Some(Box::new(h2)),
        }
    }

    fn new() -> Self {
        HuffmanTreeNode {
            symbol: None,
            occurence: Occurence::new(0),
            left_child: None,
            right_child: None
        }
    }
}

#[derive(Debug)]
struct Occurence {
    int: u64,
    finite: bool,
}

impl Occurence {
    fn new(number: u64) -> Occurence {
        Occurence {
            int: number,
            finite: true,
        }
    }

    fn infinite() -> Occurence {
        Occurence {
            int: 0,
            finite: false,
        }
    }

    fn copy(&self) -> Occurence {
        Occurence {
            int: self.int,
            finite: self.finite,
        }
    }
    fn less(occur1: &Occurence, occur2: &Occurence) -> bool {
        if occur1.int < occur2.int && occur1.finite {
            return true;
        } else {
            return false;
        }
    }
}

struct PQueue {
    queue: VecDeque<HuffmanTreeNode>,
}

impl PQueue {
    fn new(size: usize) -> Self {
        PQueue {
            queue: VecDeque::with_capacity(size),
        }
    }

    fn min(&mut self) -> HuffmanTreeNode {
        // On extraie le premier élement
        let result = self.queue.pop_front();
        let i = self.queue.len();

        if i == 0 {
            return result.unwrap();
        }

        // On met le dernier élément au début de la queue, puis on effectue les permutations nécessaires
        let last = self.queue.pop_back().unwrap();
        self.queue.push_front(last);

        let mut j: usize = 0;
        let mut current_node_occurence = self.queue[j].occurence.copy();

        // We check if children exist, else we create child with infinite occurence
        let mut fg_occurence: Occurence;
        let mut fd_occurence: Occurence;

        if 2 * j + 1 < i {
            fg_occurence = self.queue[2 * j + 1].occurence.copy();
        } else {
            let infinite_occurence = Occurence::infinite();
            fg_occurence = infinite_occurence;
        }
        if 2 * j + 2 < i {
            fd_occurence = self.queue[2 * j + 2].occurence.copy();
        } else {
            let infinite_occurence = Occurence::infinite();
            fd_occurence = infinite_occurence;
        }

        // Now we loop while our current node is bigger than one of it's children
        while Occurence::less(&fg_occurence, &current_node_occurence)
            || Occurence::less(&fd_occurence, &current_node_occurence)
        {
            if Occurence::less(&fg_occurence, &current_node_occurence) {
                self.queue.swap(j, 2 * j + 1);
                j = 2 * j + 1;
            } else if Occurence::less(&fd_occurence, &current_node_occurence) {
                self.queue.swap(j, 2 * j + 2);
                j = 2 * j + 2;
            }

            current_node_occurence = self.queue[j].occurence.copy();
            if 2 * j + 1 < i {
                fg_occurence = self.queue[2 * j + 1].occurence.copy();
            } else {
                let infinite_occurence = Occurence::infinite();
                fg_occurence = infinite_occurence;
            }

            if 2 * j + 2 < i {
                fd_occurence = self.queue[2 * j + 2].occurence.copy();
            } else {
                let infinite_occurence = Occurence::infinite();
                fd_occurence = infinite_occurence;
            }
        }

        result.unwrap()
    }

    fn len(&self) -> usize {
        self.queue.len()
    }

    fn insert(&mut self, node: HuffmanTreeNode) {
        let node_occurence = node.occurence.copy();
        self.queue.push_back(node);

        let mut index_node = self.len() - 1;
        let index_pred = ((index_node as isize) - 1) / 2;
        let mut index_pred = index_pred as usize;

        while Occurence::less(&node_occurence, &self.queue[index_pred].occurence) {
            self.queue.swap(index_node, index_pred);
            index_node = index_pred;
            index_pred = ((index_pred as isize - 1) / 2) as usize;
        }
    }
}

struct Huffman {
    tree: Option<Box<HuffmanTreeNode>>,
    codes: HashMap<usize, Vec<u8>>,
}

fn count_occurence(content: &Vec<usize>, occurence: &mut [u32; 260]) {
    for el in content.iter() {
        occurence[*el as usize] += 1;
    }
}

impl Huffman {
    fn new() -> Huffman {
        Huffman {
            tree: None,
            codes: HashMap::new(),
        }
    }

    fn build_tree(&mut self, content: &mut Vec<usize>) {
        content.push(HUFFMAN_MARKER);

        let mut occurence = [0; 260];
        count_occurence(content, &mut occurence);

        let mut queue = PQueue::new(260);

        for (code, occur) in occurence.iter().enumerate() {
            if *occur > 0 {
                queue.insert(HuffmanTreeNode {
                    symbol: Some(code as usize),
                    occurence: Occurence::new(*occur as u64),
                    left_child: None,
                    right_child: None,
                });
            }
        }

        while queue.len() > 1 {
            let min1 = queue.min();
            let min2 = queue.min();
            let new_node = HuffmanTreeNode::merge(min1, min2);
            queue.insert(new_node);
        }

        self.tree = Some(Box::new(queue.min()));
    }

    fn build_codemap_rec(node: &HuffmanTreeNode, code: Vec<u8>, codemap: &mut HashMap<usize, Vec<u8>>) {
        if node.left_child.is_some() {
            let mut code_left = code.clone();
            code_left.push(0);
            Huffman::build_codemap_rec(
                node.left_child.as_ref().unwrap().as_ref(),
                code_left,
                codemap,
            );
        }

        if node.right_child.is_some() {
            let mut code_right = code.clone();
            code_right.push(1);
            Huffman::build_codemap_rec(
                node.right_child.as_ref().unwrap().as_ref(),
                code_right,
                codemap,
            );
        }

        if node.symbol.is_some() {
            let index = *(node.symbol.as_ref().unwrap()) as usize;
            codemap.insert(index, code);
        }
    }

    fn build_codemap(&mut self) {
        let code: Vec<u8> = Vec::new();
        let mut codemap: HashMap<usize, Vec<u8>> = HashMap::new();

        let root = self.tree.as_ref().unwrap();
        Huffman::build_codemap_rec(root, code, &mut codemap);

        self.codes = codemap;
    }

    fn encode(&mut self, content: &mut Vec<usize>) -> Vec<u8> {
        self.build_tree(content);
        self.build_codemap();
        self.build_canonical_codemap();

        let mut result: Vec<u8> = Vec::new();

        let mut c = 0;
        let mut tmp: u8 = 0;

        for element in content.iter() {
            for &bit in self.codes[element as &usize].iter() {
                tmp = tmp << 1;
                tmp = tmp | bit;
                c += 1;

                if c == 8 {
                    result.push(tmp);
                    tmp = 0;
                    c = 0;
                }
            }
        }

        if c > 0 {
            tmp = tmp << (8 - c);
            result.push(tmp);
        }
        let codes = self.canonical_diffs();
        let mut codes = split_symbol(&codes);
        
        let code_len = codes.len();
        let first_bit_len = 0xFF00 & code_len;
        let first_bit_len = (first_bit_len >> 8) as u8;
        let second_bit_len = 0xFF & code_len as u8;
        codes.insert(0, second_bit_len);
        codes.insert(0, first_bit_len);

        codes.append(&mut result);

        codes
    }

    fn decode(&mut self, mut content: Vec<u8>) -> Vec<usize> {
        let first_bit_len = content[0] as usize;
        let second_bit_len = content[1] as usize;

        let codes_len = second_bit_len + (first_bit_len << 8);
        let codes = &content[2..codes_len+2];
        let can_codes = merge_symbol(codes);
        self.rebuild_tree(&can_codes);
        content.drain(0..codes_len+2);

        let mut result = Vec::new();

        let mut node = self.tree.as_ref().unwrap().as_ref();

        for mut element in content.iter().cloned() {
            for _ in 0..8 {
                if element & 0x80 == 0 {
                    node = node.left_child.as_ref().unwrap().as_ref();
                } else if element & 0x80 == 0x80 {
                    node = node.right_child.as_ref().unwrap().as_ref();
                }

                if node.symbol.is_some() {
                    let symbol = node.symbol.unwrap();
                    if symbol == HUFFMAN_MARKER {
                        return result;
                    }
                    let symbol = symbol;
                    result.push(symbol);
                    node = self.tree.as_ref().unwrap().as_ref();
                }

                element = element << 1;
            }
        }

        panic!("Didn't found huffman marker");
    }

    fn canonical_diffs(&self) -> Vec<usize> {
        let mut list_length: Vec<(usize, usize)> = Vec::new();

        for (&symbol, code) in self.codes.iter() {
            list_length.push((symbol, code.len()))
        }

        list_length.sort_by(|a, b| a.1.cmp(&b.1));

        let mut res: Vec<usize> = Vec::with_capacity(2 * list_length.len());
        let mut last_length = 0;
        for &(symbol, len) in list_length.iter() {
            res.push(symbol);
            if len > last_length {
                res.push(len - last_length);
                last_length = len;
            } else {
                res.push(0);
            }
        }

        res
    }

    fn build_canonical_codemap(&mut self) {
        let can_diffs = self.canonical_diffs();
        let mut last_length = 0;
        let mut c = 0;

        for i in 0..can_diffs.len() {
            if i % 2 == 1 {
                continue;
            }
            let symbol = can_diffs[i];
            let length_diff = can_diffs[i+1];

            if length_diff > 0 {
                c = c << (length_diff);
                last_length += length_diff;
            }
            let new_code = binary_list(c, last_length);
            
            self.codes.insert(symbol, new_code);
            c += 1;
        } 
    }

    fn rebuild_tree_rec(node: &mut Box<HuffmanTreeNode>, mut code: Vec<u8>, symbol: usize) {
        if code.is_empty() {
            node.symbol = Some(symbol);
        } else {
            let bit = code.pop().unwrap();

            if bit == 0 {
                if let Some(boxed_node) = node.left_child.as_mut() {
                    Huffman::rebuild_tree_rec(boxed_node, code, symbol);
                } else {
                    let left_child = HuffmanTreeNode::new();
                    node.left_child = Some(Box::new(left_child));
                    Huffman::rebuild_tree_rec(node.left_child.as_mut().unwrap(), code, symbol)
                }
            } else if bit == 1 {
                if let Some(boxed_node) = node.right_child.as_mut() {
                    Huffman::rebuild_tree_rec(boxed_node, code, symbol);
                } else {
                    let right_child = HuffmanTreeNode::new();
                    node.right_child = Some(Box::new(right_child));
                    Huffman::rebuild_tree_rec(node.right_child.as_mut().unwrap(), code, symbol);
                }
            }
        }
    }

    fn rebuild_tree(&mut self, codes: &[usize]) {
        let mut root = Box::new(HuffmanTreeNode::new());
        let mut c = 0;
        let mut last_length = 0;

        for i in 0..codes.len() {
            if i % 2 == 1 {
                continue
            } else {
                let symbol = codes[i];
                let length_diff = codes[i+1];

                if length_diff > 0 {
                    c = c << length_diff;
                    last_length += length_diff;
                }

                let mut code = binary_list(c, last_length);
                code.reverse();
                Huffman::rebuild_tree_rec(&mut root, code, symbol);
                c += 1;
            }
        }

        self.tree = Some(root);
    }
}

fn binary_list(mut x: usize, len: usize) -> Vec<u8> {
    let mut result: Vec<u8> = vec![0; len];
    let mut idx;
    if len == 0 {
        return result;
    } else {
        idx = len;
    }
    let mut pred;

    while x > 0 {
        idx -= 1;
        pred = x / 2;
        result[idx] = (x - 2 * pred) as u8;
        x = pred;
    }

    result
}

fn split_symbol(codes: &Vec<usize>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::with_capacity(codes.len());

    for &el in codes.iter() {
        if el > 255 {
            let r = el % 256;
            res.push(0);
            res.push(r as u8);
        } else {
            res.push(el as u8);
        }
    }

    res
}

fn merge_symbol(codes: &[u8]) -> Vec<usize> {
    let mut res: Vec<usize>  = Vec::new();

    let mut i = 0;
    while i < codes.len() {
        if codes[i] == 0 {
            let r = 256 + codes[i+1] as usize;
            res.push(r);
            res.push(codes[i+2] as usize);
            i = i + 3;
        } else {
            res.push(codes[i] as usize);
            res.push(codes[i+1] as usize);
            i = i + 2;
        }
    }

    res
}

pub fn huffman_encode(mut content: Vec<usize>) -> Vec<u8> {
    let mut huffmantree = Huffman::new();
    huffmantree.encode(&mut content)
}

pub fn huffman_decode(content: Vec<u8>) -> Vec<usize> {
    let mut huffmantree = Huffman::new();
    huffmantree.decode(content)
}

#[cfg(test)]
mod test {
    use super::Huffman;

    #[test]
    pub fn test_huffmantree() {
        let text_content = vec![15, 15, 15, 15, 16, 16, 231, 231, 192, 255];
        let mut huffmantree = Huffman::new();
        let code = huffmantree.encode(&mut text_content.clone());
        let mut n_huffmantree = Huffman::new();
        let _decode = n_huffmantree.decode(code);
        assert_eq!(text_content, _decode);
    }
}