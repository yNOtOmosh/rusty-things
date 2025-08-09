fn main() {
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            if i == 2 && j == 2 {
                break 'outer; // exits outer loop
            }
            println!("i = {}, j = {}", i, j);
        }
    }
}
