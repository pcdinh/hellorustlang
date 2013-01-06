fn test_branching(value: ~str) -> ~str {
    // https://github.com/mozilla/rust/issues/3463
    // https://github.com/mozilla/rust/issues/3294
	// https://mail.mozilla.org/pipermail/rust-dev/2012-August/002231.html Ignore trailing semicolons
	// https://github.com/mozilla/rust/issues/1712 Expression syntax and semicolon
	// https://github.com/mozilla/rust/blob/master/src/test/run-pass/utf8.rs
    return if value == ~"hello_world" {
               ~"Hello World"
           } else if value == ~"" {
               ~"Nothing"
           } else {
               ~"Anything"
           }
}

fn main() {
   io::println(test_branching(~"Test2"))
}