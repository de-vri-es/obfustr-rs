use assert2::assert;

#[test]
fn obfuscate_str() {
	assert!("hello!" == obfustr::obfuscate!("hello!"));
}

#[test]
fn obfuscate_byte_str() {
	assert!(b"hello!" == obfustr::obfuscate!(b"hello!"));
}

#[test]
fn obfuscate_cstr() {
	assert!(c"hello!" == obfustr::obfuscate!(c"hello!"));
}
