pub fn is_empty(v: &str) -> bool {
    v.len() == 0
}

pub fn is_ascii(v: &str) -> bool {
    v.bytes().all(|b| b <= 127)
}

pub fn contains(v: &str, pat: &str) -> bool {
    if pat.is_empty() {
        return true;
    }

    if pat.len() > v.len() {
        return false;
    }

    let v_bytes = v.as_bytes();
    let pat_bytes = pat.as_bytes();
    
    for i in 0..=v.len() - pat.len() {
        let mut found = true;
        
        for j in 0..pat.len() {
            if v_bytes[i + j] != pat_bytes[j] {
                found = false;
                break;
            }
        }
        
        if found {
            return true;
        }
    }
    
    false
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    let v_bytes = v.as_bytes();
    
    for (i, ch) in v.char_indices() {
        if ch == pat {
            return i;
        }
    }
    
    v.len()
}