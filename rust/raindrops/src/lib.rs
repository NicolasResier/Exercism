pub fn raindrops(n: u32) -> String {
    let mut a = String::new();
    let b = String::from(format!("{}", n));
    if n%3 == 0 {a.push_str("Pling");};
    if n%5 == 0 {a.push_str("Plang");};
    if n%7 == 0 {a.push_str("Plong");};

    if a.is_empty() {
        b
    } else {
        a
    }

}
