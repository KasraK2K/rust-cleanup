use rust_cleanup::{get_path, clean_recursive_file, last_month_date};
use std::path::Path;

fn main() {
    let last_month = last_month_date();

    let root = get_path();

    clean_recursive_file(Path::new(&root), last_month, 10);
}
