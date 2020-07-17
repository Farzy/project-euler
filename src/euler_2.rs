const MAX: u32 = 4_000_000;

pub fn main() {
    let mut sum = 0;
    // Let's start at i = 2 because it's the first non-null even number to sum
    let (mut i, mut j) = (2u32, 3u32);
    let mut temp;
    while i <= MAX {
        sum += i;
        // Every 1 out of 3 fibonacci numbers is even
        for _ in 1..=3 {
            // Maybe one day Rust will support destructuring assignment like Python…
            temp = i;
            i = j;
            j = temp + j;
            println!("i: {}, sum: {}", temp, sum);
        }
    }
}
