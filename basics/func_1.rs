fn test_branching(value: ~str) -> ~str {
    // https://github.com/mozilla/rust/issues/3463
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