#![feature(test)]

extern crate test;
use armdecode;
use test::{black_box, Bencher};

// See testsuite/arm_decode_test.S; trying to hit all cases
const TEST_ARR: [u32; 65] = [
    0xe2a54004, 0xe0a54006, 0xe2854004, 0xe0854006,
    0xe2054004, 0xe0054006, 0xeaffffff, 0xebffffff,
    0xe12caf7e, 0xe3c54004, 0xe1c54006, 0xe12fff30,
    0xe12fff10, 0xeef00d00, 0xe16f4f15, 0xe3740004,
    0xe1740005, 0xe3540004, 0xe1540005, 0xe2254004,
    0xe0254006, 0xe92ddff8, 0xe89d6ff0, 0xe59f30a0,
    0xe5df309c, 0xe1df39b8, 0xe1df39d4, 0xe1df39f0,
    0xee070f15, 0xee153f10, 0xe0236594, 0xe3a030ff,
    0xe1a03004, 0xe10f3000, 0xe121f000, 0xe321f0d0,
    0xe0030594, 0xe3e030ff, 0xe1e03004, 0xe3854004,
    0xe1854006, 0xe1053054, 0xe1453054, 0xe1253054,
    0xe1653054, 0xe2654004, 0xe0654006, 0xe2e54004,
    0xe0e54006, 0xe2c54004, 0xe0c54006, 0xe50f30d4,
    0xe54f30d8, 0xe14f3dbc, 0xe2454004, 0xe0454006,
    0xe1053094, 0xe1453094, 0xef00cafe, 0xe33300ff,
    0xe1330004, 0xe31300ff, 0xe1130004, 0xe0a43695,
    0xe0843695,
];

#[bench]
fn arm_inst_space(b: &mut Bencher) { 
    b.iter(|| 
        for val in TEST_ARR.iter() { 
            let inst = armdecode::decode(*val);
        }
    );
}


