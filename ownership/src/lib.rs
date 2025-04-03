pub fn first_subword(mut s: String) -> String {
    let mut chars = s.chars();
    let mut subword = String::new();

    if let Some(first) = chars.next() {
        subword.push(first);
    }

    for c in chars {
        if c.is_uppercase() || c == '_' {
            break;
        }
        subword.push(c);
    }

    subword
}
