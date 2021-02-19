fn fizz_buzz(n: i32) -> String {
    match (n % 3 == 0, n % 5 == 0) {
        (true, false) => "fizz".to_string(),
        (false, true) => "buzz".to_string(),
        (true, true) => "fizz buzz".to_string(),
        _ => n.to_string()
    }
}

fn main() {
    for i in 1..20 {
        println!("{}", fizz_buzz(i));
    }
}
