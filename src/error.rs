

pub fn error(line:usize, message:&str) {
    report(line, "", message)
}

pub fn report(line:usize, r#where:&str, message:&str) {
    panic!("[line {}] Error {}: {}", line, r#where, message);
}