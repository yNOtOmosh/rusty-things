fn main() {
    let scores: [i32; 5] = [90, 74, 85, 70, 100];

    println!("first score: {}", scores[0]);
    println!("All scores:");

    for score in scores.iter() {
        println!("{}", score);
    }
}
