# Rust 8080 

[![Build Status](https://travis-ci.org/Tom-Goring/Rust-8080.svg?branch=master)](https://travis-ci.org/Tom-Goring/Rust-8080) 
[![codecov](https://codecov.io/gh/Tom-Goring/Rust-8080/branch/master/graph/badge.svg)](https://codecov.io/gh/Tom-Goring/Rust-8080)


An Intel 8080 CPU emulator implemented using Rust. 

## Getting Started

Clone the repository, and then from the repo directory run ```cargo run```

To run the tests, run ```cargo test```

## TODO: 
- Locate & fix errors preventing space invaders from working:
    - PC: 0906 appears to be broken
    - Documentation suggests this should be ADD vs DAD?
    - This entire area is presumably corrupted for some reason, needs further investigation


## Current State: 
- Space Invaders displays title & attempts to start game before getting stuck
- Single alien displayed in incorrect location
- No controls seem to work

## Resources:

I'm using a few resources for this:
- [Emulator 101's 8080 OpCode reference](http://www.emulator101.com/reference/8080-by-opcode.html)
- [This summary of 8080 instructions](http://textfiles.com/programming/8080.op)
- [The 8080 Programmers Manual](https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf)