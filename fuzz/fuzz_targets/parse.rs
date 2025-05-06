#![no_main]
use libfuzzer_sys::fuzz_target;

use titleformat_rs::program::Program;

fuzz_target!(|data: &str| {
    let mut program = Program::new();
    let Ok(_) = program.parse(data) else {
        return;
    };
    let Ok(_result) = program.run() else {
        return;
    };
});
