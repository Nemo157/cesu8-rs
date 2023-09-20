#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        assert_eq!(cesu8::from_java_cesu8(&cesu8::to_java_cesu8(s)).unwrap(), s);
    }
});
