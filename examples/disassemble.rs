// Copyright 2017 Quentin Monnet <quentin.monnet@6wind.com>
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0> or
// the MIT license <http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.


extern crate rbpf;
use rbpf::disassembler;

fn main() {
    let prog = vec![
        0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x79, 0x12, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x79, 0x11, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xbf, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x07, 0x03, 0x00, 0x00, 0x36, 0x00, 0x00, 0x00,
        0x2d, 0x23, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x69, 0x12, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x55, 0x02, 0x10, 0x00, 0x08, 0x00, 0x00, 0x00,
        0x71, 0x12, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x55, 0x02, 0x0e, 0x00, 0x06, 0x00, 0x00, 0x00,
        0x18, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x79, 0x11, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xbf, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x57, 0x02, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00,
        0x15, 0x02, 0x08, 0x00, 0x99, 0x99, 0x00, 0x00,
        0x18, 0x02, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x5f, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xb7, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
        0x18, 0x02, 0x00, 0x00, 0x00, 0x00, 0x99, 0x99,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x1d, 0x21, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    ];
    disassembler::disassemble(&prog);
}
