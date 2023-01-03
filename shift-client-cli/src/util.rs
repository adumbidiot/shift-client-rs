use std::io::Write;

/// Read a string from the console.
pub fn input() -> String {
    let mut s = String::new();
    let _ = std::io::stdout().flush();
    let _ = std::io::stdin().read_line(&mut s);
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

/// Read a (Y/N) from the console.
pub fn input_yn() -> bool {
    matches!(input().chars().next(), Some('Y') | Some('y'))
}
