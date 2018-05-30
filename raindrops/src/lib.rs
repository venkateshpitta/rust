pub fn raindrops(n: usize) -> String {
    let mut out = String :: new();
    
    if n%3 == 0 {
        out.push_str("Pling")
    }
    if n%5 == 0 {
        out.push_str("Plang")
    }
    if n%7 == 0 {
        out.push_str("Plong")
    }
    match out.is_empty() {
        true => n.to_string(),
        false => out,
    }
}
