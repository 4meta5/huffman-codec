use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    rc::Rc,
};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("the character cannot be found in the dictionary")]
pub struct CharDNEinDict;

pub struct Codec(pub HashMap<char, Vec<u8>>);

impl Codec {
    pub fn encode(&self, data: &str) -> Result<Vec<u8>, CharDNEinDict> {
        let mut nbits = 0;
        data.chars()
            .map(|c| -> Result<(), CharDNEinDict> {
                if let Some(code) = self.0.get(&c) {
                    nbits += code.len();
                    Ok(())
                } else {
                    Err(CharDNEinDict)
                }
            })
            .collect::<Result<(), CharDNEinDict>>()?;
        let mut ret = Vec::<u8>::with_capacity(nbits);
        data.chars().for_each(|c| {
            let v = self
                .0
                .get(&c)
                .expect("checked existence in first for loop above; qed");
            v.iter().for_each(|bit| ret.push(*bit));
        });
        Ok(ret)
    }
    pub fn decode(&self, data: Vec<u8>) -> String {
        fn reverse(h: &HashMap<char, Vec<u8>>) -> HashMap<Vec<u8>, char> {
            let mut ret = HashMap::new();
            h.iter().for_each(|(k, v)| {
                ret.insert(v.clone(), *k);
            });
            ret
        }
        let code = reverse(&self.0);
        let mut temp = Vec::<u8>::new();
        let mut ret = String::new();
        data.into_iter().for_each(|b| {
            temp.push(b);
            if let Some(c) = code.get(&temp) {
                ret.push(*c);
                temp.clear();
            }
        });
        ret
    }
}

#[derive(Eq, Debug, Clone)]
struct Tree {
    count: i32,
    value: Option<char>,
    left: Option<Rc<Tree>>,
    right: Option<Rc<Tree>>,
}

impl Ord for Tree {
    fn cmp(&self, other: &Tree) -> Ordering {
        (self.count).cmp(&(other.count))
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Tree) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Tree) -> bool {
        self.count == other.count
    }
}

impl Tree {
    fn new(value: char, count: i32) -> Rc<Tree> {
        let t = Tree {
            count,
            value: Some(value),
            left: None,
            right: None,
        };

        Rc::new(t)
    }

    fn merge(tree_smaller: Rc<Tree>, tree_larger: Rc<Tree>) -> Rc<Tree> {
        let t = Tree {
            count: tree_smaller.count + tree_larger.count,
            value: None,
            left: Some(tree_smaller),
            right: Some(tree_larger),
        };

        Rc::new(t)
    }
}

pub fn frequency(n: &str) -> HashMap<char, i32> {
    let mut output: HashMap<char, i32> = HashMap::new();
    n.chars().for_each(|c| {
        let new = if let Some(o) = output.get(&c) {
            o + 1i32
        } else {
            1i32
        };
        output.insert(c, new);
    });
    output
}

pub fn huffman_codes(data: &str) -> HashMap<char, Vec<u8>> {
    fn map_to_heap(map: HashMap<char, i32>) -> BinaryHeap<Rc<Tree>> {
        let mut heap = BinaryHeap::new();
        map.into_iter().for_each(|(l, c)| {
            let t = Tree::new(l, c);
            heap.push(t);
        });
        heap
    }
    fn heap_to_tree(mut heap: BinaryHeap<Rc<Tree>>) -> Rc<Tree> {
        while heap.len() > 1 {
            let (t1, t2) = (heap.pop().unwrap(), heap.pop().unwrap());
            heap.push(Tree::merge(t1, t2));
        }
        heap.pop().unwrap()
    }
    fn tree_to_codes(
        root: &Option<Rc<Tree>>,
        prefix: Vec<u8>,
        mut map: HashMap<char, Vec<u8>>,
    ) -> HashMap<char, Vec<u8>> {
        if let Some(ref tree) = *root {
            match tree.value {
                Some(t) => {
                    map.insert(t, prefix);
                }
                None => {
                    let (mut prefix_l, mut prefix_r) = (prefix.clone(), prefix);
                    prefix_l.push(1u8);
                    let map = tree_to_codes(&tree.left, prefix_l, map);
                    prefix_r.push(0u8);
                    return tree_to_codes(&tree.right, prefix_r, map);
                }
            }
        }
        map
    }
    let f_map = frequency(data);
    let heap = map_to_heap(f_map);
    let tree = heap_to_tree(heap);
    tree_to_codes(&Some(tree), Vec::new(), HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frequency_works() {
        let a = "aaaabbbcccddddabababa";

        let res_fn = frequency(a);
        let mut res: HashMap<char, i32> = HashMap::new();
        res.insert('a', 8);
        res.insert('b', 6);
        res.insert('c', 3);
        res.insert('d', 4);

        assert_eq!(res_fn, res);
    }

    #[test]
    fn decoding_works() {
        let a = "aaaabbbcccddddaaabababr";
        let a1 = "abracadabra";
        let codec = Codec(huffman_codes(a));
        let encoded = codec.encode(a1).unwrap();
        let decoded = codec.decode(encoded);

        assert_eq!(a1, decoded);

        let a2 = "abcdr";

        let encoded = codec.encode(a2).unwrap();
        let decoded = codec.decode(encoded);

        assert_eq!(a2, decoded);

        let a3 = "x";
        assert!(codec.encode(a3).is_err());
    }
}
