const MAX: u32 = 4_000_000;

pub fn main() -> String {
    let mut sum = 0;
    let (mut a, mut b) = (1u32, 1u32);
    let mut c = a + b;
    while c <= MAX {
        sum += c;
        a = b + c;
        b = a + c;
        c = a + b;
    }
    format!("{}", sum)
}
