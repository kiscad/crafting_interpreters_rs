use super::G_HAD_ERROR;

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, where_: &str, message: &str) {
    println!("[line {line}] Error {where_}: {message}");
    unsafe { G_HAD_ERROR = true };
}
