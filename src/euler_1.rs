pub fn main() {
    // Let's analyze the series of multiples of 3 and 5 in order to find a cycle in the increment
    // Series:
    //   3  5  6  9  10  12  15  18  20  21  24  25  27  30  33  35  36  39  40  42  45  48  50  51  54  55
    // Increment between each element, starting from 0:
    //   3  2  1  3   1   2   3   3   2   1   3   1   2   3   3   2   1   3   1   2   3   3   2   1   3   1
    // Smallest repeated increment sequence is: 3, 2, 1, 3, 1, 2, 3
    let mut inc_3_5 = [3, 2, 1, 3, 1, 2, 3].iter().cycle();
    let mut sum = 0;
    let mut n = 0;
    loop {
        n += *inc_3_5.next().unwrap();
        if n >= 1000 {
            break;
        }
        sum += n;
    }
    println!("{}", sum);
}
