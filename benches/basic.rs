#![feature(test)] 
extern crate test;
use test::{Bencher};
use std::sync::Mutex;
use std::fs;
use huffman_codec::Codec;

/* Copied from criterion-rs */
fn black_box<T>(dummy: T) -> T{
    unsafe{
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}

/* Should probably change this to have a static cache */
fn read_testfile() -> std::io::Result<String>{
    fs::read_to_string("./benches/testfile.txt")
}

#[bench]
fn encode(b: &mut Bencher){
    let data = read_testfile().unwrap();
    let encoder = Codec::new(&data);
    b.iter(|| {
        black_box(encoder.encode(black_box(&data)))
    })
}

#[bench]
fn decode(b: &mut Bencher){
    let data = read_testfile().unwrap();
    let decoder = Codec::new(&data);
    let data_to_decode: Vec<u8> = data.into();
    b.iter(||{
        black_box(decoder.decode(black_box(data_to_decode.clone())));
    })
}
