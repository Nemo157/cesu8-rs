#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    if let Ok(s) = cesu8::from_cesu8(data) {
        assert!(std::str::from_utf8(s.as_bytes()).is_ok());
    }
});
