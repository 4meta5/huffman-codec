# Huffman Codes

Lossless data compression via [Huffman Codes](https://en.wikipedia.org/wiki/Huffman_coding)

## Rust

![Build](https://github.com/4meta5/huffman-codec/workflows/Build/badge.svg)
[![](https://meritbadge.herokuapp.com/huffman-codec)](https://crates.io/crates/huffman-codec)

~300 loc with no dependencies, `no_std`

### Benchmarks

The implementation stores [Ascii](https://www.asciitable.com/) representations in [`Vec`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html), and non-ascii codes in a [`BTreeMap`](https://doc.rust-lang.org/alloc/collections/btree_map/struct.BTreeMap.html). Benchmarks on `2.3 GHz 8-Core Intel Core i9` processor measure vector access as ~5x faster than `BTreeMap` for encode and decode.

```
test medium_decode ... bench:   1,047,881 ns/iter (+/- 82,172)
test medium_encode ... bench:     218,537 ns/iter (+/- 22,107)
test small_decode  ... bench:     202,586 ns/iter (+/- 14,843)
test small_encode  ... bench:      44,425 ns/iter (+/- 9,182)
```

## Prolog

*All example queries are made from the [swish](https://www.swi-prolog.org/Download.html) REPL*

```prolog
(main)⚡ % swipl huffman.pl
Welcome to SWI-Prolog (threaded, 64 bits, version 8.2.1)
SWI-Prolog comes with ABSOLUTELY NO WARRANTY. This is free software.
Please run ?- license. for legal details.
For online help and background, visit https://www.swi-prolog.org
For built-in help, use ?- help(Topic). or ?- apropos(Word).
?- 
```

Example queries with output:
```prolog
?- make_code('How are you doing?',C),ncode('How are',C,R),write(R).
[0,0,0,1,1,1,1,1,0,1,0,1,1,0,0,0,1,0,1,0,0,0,0,1,0,0]
C = [[1, ?, [0, 0, 0, 0]], [1, 'H', [0, 0, 0, 1]], [1, a, [0, 0, 1|...]], [1, d, [0, 0|...]], [1, e, [0|...]], [1, g, [...|...]], [1, i|...], [1|...], [...|...]|...],
R = [0, 0, 0, 1, 1, 1, 1, 1, 0|...] .

?- make_code('How are you doing?',C),dcode([0,0,0,1,1,1,1,1,0,1,0,1,1,0,0,0,1,0,1,0,0,0,0,1,0,0],C,R).
C = [[1, ?, [0, 0, 0, 0]], [1, 'H', [0, 0, 0, 1]], [1, a, [0, 0, 1|...]], [1, d, [0, 0|...]], [1, e, [0|...]], [1, g, [...|...]], [1, i|...], [1|...], [...|...]|...],
R = "How are" .

?- test.
Character            Frequency          Code
! :                  1                  00010
c :                  1                  00011
g :                  1                  00100
l :                  1                  00101
p :                  1                  00110
r :                  1                  00111
u :                  1                  01000
x :                  1                  01001
d :                  2                  11110
f :                  2                  11111
h :                  2                  0000
i :                  3                  0101
m :                  3                  0110
o :                  3                  0111
s :                  3                  1010
a :                  4                  1011
e :                  4                  1100
n :                  4                  1101
t :                  4                  1110
  :                  7                  100
true .
```

### License

Copyright 2020 4meta5

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.