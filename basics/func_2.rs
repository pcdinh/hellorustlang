// use io::println;

fn a(x: ~str) -> ~str {
    // https://github.com/mozilla/rust/blob/master/src/libcore/io.rs 
	// https://mail.mozilla.org/pipermail/rust-dev/2012-August/002236.html
	// https://mail.mozilla.org/pipermail/rust-dev/2012-August/002239.html Polymorphism & default parameters in rust
    fmt("First function with %s", x)
}

fn a(x: ~str, y: ~str) -> ~str {
    fmt("Second function with %s and %s", x, y)
}

fn main() {
    info("Result: ");
}