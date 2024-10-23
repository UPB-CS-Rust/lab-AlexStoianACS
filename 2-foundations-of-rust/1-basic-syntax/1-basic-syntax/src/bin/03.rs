fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let mut max = 0;
    let mut min = 9999;

    for i in 0..8 {
        if input[i] > max {
            max = input[i];
        }
        if input[i] < min {
            min = input[i];
        }
    }

    println!("{} is largest and {} is smallest", max, min);
}
