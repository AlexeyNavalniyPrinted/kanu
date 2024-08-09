pub(crate) fn validate_target_name(target_name: &str) {
    if target_name.is_empty() {
        panic!("Target name cannot be empty")
    }
    if !target_name.is_ascii() {
        panic!("Target name should contain only letters (Non ascii characters not allowed)")
    }

    for char in target_name.chars() {
        if char.is_ascii_digit() {
            panic!("Target name should contain only letters (No numbers are allowed)")
        }
        match char {
            '!' | '\"' | '#' | '$' | '%' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | '-' | '.' | '/'
            | ':' | ';' | '<' | '=' | '>' | '?' | '@' | '[' | '\\' | ']' | '^' | '_' | '`' | '{' | '|'
            | '}' | '~' | '\t' | '\n' | '\r' | '\x00'..='\x1F' | '\x7F' | ' ' => {
                panic!("Target name should contain only letters (No special characters are allowed)")
            }
            _ => {}
        }
    }
}