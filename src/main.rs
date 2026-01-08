use rust_cleanup::{clean_recursive_file, get_path, last_month_date, resolve_path};
use std::path::Path;
use std::env;

fn main() {
    let last_month = last_month_date();

    let root = match env::args().nth(1) {
        Some(arg) => resolve_path(&arg),
        None => get_path(),
    };

    clean_recursive_file(Path::new(&root), last_month, 10);
}
