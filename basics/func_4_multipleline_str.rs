
fn main() {
	io::println(~"This is:
		a
		multiple
		line 
		string
    ");
	// JS style using backslash https://github.com/mozilla/rust/issues/448
	// No whitespace
	io::println(~"This is: \
		a \
		multiple \
		line \
		string without unexpected white spaces \
    ");
}