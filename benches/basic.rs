#![feature(test)]
#![feature(const_in_array_repeat_expressions)]
extern crate test;

use huffman_codec::Codec;
use std::fs;
use test::Bencher;

static SMALLFILE: &str = "benches/testfile.txt";
static MEDIUMFILE: &str = "benches/huffman_wiki.txt";

/* Copied from criterion-rs */
fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}

/* Should probably change this to have a static cache */
fn read_testfile<P: AsRef<std::path::Path>>(pathname: P) -> std::io::Result<String> {
    fs::read_to_string(&pathname)
}

#[bench]
fn small_encode(b: &mut Bencher) {
    let data = read_testfile(SMALLFILE).unwrap();
    let encoder = Codec::new(&data);
    b.iter(|| black_box(encoder.encode(black_box(&data))))
}

#[bench]
fn small_decode(b: &mut Bencher) {
    let data = read_testfile(SMALLFILE).unwrap();
    let decoder = Codec::new(&data);
    let data_to_decode: Vec<u8> = data.into();
    b.iter(|| {
        black_box(decoder.decode(black_box(data_to_decode.clone())));
    })
}

#[bench]
fn medium_encode(b: &mut Bencher) {
    let data = read_testfile(MEDIUMFILE).unwrap();
    let encoder = Codec::new(&data);
    b.iter(|| black_box(encoder.encode(black_box(&data))))
}

#[bench]
fn medium_decode(b: &mut Bencher) {
    let data = read_testfile(MEDIUMFILE).unwrap();
    let decoder = Codec::new(&data);
    let data_to_decode: Vec<u8> = data.into();
    b.iter(|| {
        black_box(decoder.decode(black_box(data_to_decode.clone())));
    })
}

#[test]
fn medium_encode_decode_test() {
    let data = read_testfile(MEDIUMFILE).unwrap();
    let encoder = Codec::new(&data);
    let encoded = encoder.encode(&data).unwrap();
    let decoded = encoder.decode(encoded);
    assert_eq!(data, decoded)
}
