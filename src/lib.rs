#![no_std]
#![feature(const_in_array_repeat_expressions)]
extern crate alloc;
use alloc::{
    collections::{binary_heap::BinaryHeap, BTreeMap},
    rc::Rc,
    string::String,
    vec::Vec,
};
use core::cmp::Ordering;

pub fn frequency(n: &str) -> BTreeMap<char, i32> {
    let mut output: BTreeMap<char, i32> = BTreeMap::new();
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

struct Dictionary {
    /* currently a vec must be used because Option<Vec<u8>> doesnt impliment copy */
    #[cfg(not(feature = "nightly-features"))]
    ascii: Vec<Option<Vec<u8>>>,
    #[cfg(feature = "nightly-features")]
    ascii: [Option<Vec<u8>>; 128],

    non_ascii: BTreeMap<char, Vec<u8>>,
}

impl Dictionary {
    #[cfg(not(feature = "nightly-features"))]
    fn new() -> Self {
        Self {
            ascii: (0..128).map(|_| None).collect(),
            non_ascii: Default::default(),
        }
    }
    #[cfg(feature = "nightly-features")]
    fn new() -> Self {
        Self {
            ascii: [None; 128],
            non_ascii: Default::default(),
        }
    }
    fn insert(&mut self, k: char, v: Vec<u8>) {
        let c = k as usize;
        if c < 128 {
            unsafe {
                *self.ascii.get_unchecked_mut(c) = Some(v);
            }
        } else {
            self.non_ascii.insert(k, v);
        }
    }
    fn get(&self, k: &char) -> Option<&Vec<u8>> {
        let c = *k as usize;
        if c < 128 {
            unsafe { self.ascii.get_unchecked(c).as_ref() }
        } else {
            self.non_ascii.get(k)
        }
    }
    fn iter(&self) -> impl Iterator<Item = (char, &'_ Vec<u8>)> + '_ {
        self.ascii
            .iter()
            .enumerate()
            .filter_map(|(index, x)| match *x {
                Some(ref v) => Some((index as u8 as char, v)),
                None => None,
            })
            .chain(self.non_ascii.iter().map(|(k, v)| (*k, v)))
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Codec(Dictionary);

impl Codec {
    pub fn new(s: &str) -> Self {
        fn map_to_heap(map: BTreeMap<char, i32>) -> BinaryHeap<Rc<Tree>> {
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
            mut map: Dictionary,
        ) -> Dictionary {
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
        let f_map = frequency(s);
        let heap = map_to_heap(f_map);
        let tree = heap_to_tree(heap);
        Self(tree_to_codes(
            &Some(tree),
            Default::default(),
            Default::default(),
        ))
    }
    pub fn encode_iterator<I>(&self, it: I) -> Result<Vec<u8>, CharDNEinDict>
    where
        I: Iterator<Item = char> + Clone,
    {
        let mut nbits = 0;
        let mut it_pass1 = it.clone();
        it_pass1.try_for_each(|c| -> Result<(), CharDNEinDict> {
            if let Some(code) = self.0.get(&c) {
                nbits += code.len();
                Ok(())
            } else {
                Err(CharDNEinDict)
            }
        })?;
        let mut ret = Vec::<u8>::with_capacity(nbits);
        it.for_each(|c| {
            let v = self
                .0
                .get(&c)
                .expect("tried for existance in first loop above");
            ret.extend(v.iter());
        });
        Ok(ret)
    }
    pub fn encode(&self, data: &str) -> Result<Vec<u8>, CharDNEinDict> {
        self.encode_iterator(data.chars())
    }
    pub fn decode_iterator<I>(&self, it: I) -> String
    where
        I: Iterator<Item = u8>,
    {
        let mut rmap: Vec<(&[u8], char)> = self.0.iter().map(|(k, v)| (v.as_slice(), k)).collect();
        rmap.sort_unstable_by_key(|(k, _)| *k);
        #[inline(always)]
        fn binfind(map: &[(&[u8], char)], key: &[u8]) -> Option<char> {
            match map.binary_search_by_key(&key, |(k, _)| k) {
                Ok(index) => Some(unsafe { map.get_unchecked(index).1 }),
                Err(_) => None,
            }
        }

        let mut temp = Vec::<u8>::new();
        let mut ret: String = if let (start, Some(end)) = it.size_hint() {
            if let Some(size) = end.checked_sub(start) {
                String::with_capacity(size)
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        ret.extend(it.filter_map(|b| {
            temp.push(b);
            if let Some(c) = binfind(rmap.as_slice(), &temp) {
                temp.clear();
                Some(c)
            } else {
                None
            }
        }));
        ret
    }
    /* this function should take a &[u8] */
    pub fn decode(&self, data: Vec<u8>) -> String {
        self.decode_iterator(data.iter().copied())
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

#[derive(Debug)]
pub struct CharDNEinDict;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frequency_works() {
        let a = "aaaabbbcccddddabababa";

        let res_fn = frequency(a);
        let mut res: BTreeMap<char, i32> = BTreeMap::new();
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
        let codec = Codec::new(a);
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
    #[test]
    fn iter_works() {
        let dict = "123456789";
        let data =
            "123456789123456789123456789123456789123456789123456789123456789123456789123456789";
        let enc = Codec::new(dict);
        let encoded = enc.encode_iterator(data.chars()).unwrap();
        let decoded = enc.decode_iterator(encoded.iter().copied());
        assert_eq!(data, decoded.as_str())
    }
}
