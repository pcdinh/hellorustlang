use io::println;
use vec::each;

fn main() {
	for each([2, 4, 8, 5, 16]) |n| {
		if *n % 2 != 0 {
			println(~"found odd number!");
			break;
		}
	}
}