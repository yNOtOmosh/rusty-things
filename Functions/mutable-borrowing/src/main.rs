fn modify(s: &mut String) {
    s.push_str(" omondi");
}

fn main() {
    let mut name = String::from("tony");
    modify(&mut name);
    println!("{}", name);
}