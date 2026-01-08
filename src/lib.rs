use chrono::{Local, Months, NaiveDate};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn resolve_path(input: &str) -> PathBuf {
    let p = Path::new(input);

    if p.is_absolute() {
        p.to_path_buf()
    } else {
        env::current_dir()
            .expect("failed to get current directory")
            .join(p)
    }
}

pub fn get_date() -> String {
    let mut date = String::new();
    eprint!("Enter valid date with YYYY-MM-DD format: ");
    io::stdin()
        .read_line(&mut date)
        .expect("Failed to read line date");
    date.trim().to_string()
}

pub fn get_path() -> PathBuf {
    let mut path = String::new();
    eprint!("Enter your path: ");
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line path");
    resolve_path(&path.trim())
}

pub fn last_month_date() -> NaiveDate {
    let today = Local::now().date_naive();
    today
        .checked_sub_months(Months::new(1))
        .expect("date out of range")
} // YYYY-MM-DD

pub fn clean(path: &Path, cutoff: NaiveDate) {
    if !path.is_dir() {
        return;
    }

    for entry in fs::read_dir(path).expect("failed to read directory") {
        let entry = entry.expect("failed to read directory entry");
        let entry_path = entry.path();

        if entry_path.is_dir() {
            let name = entry_path.file_name().and_then(|s| s.to_str());
            if let Some(name) = name {
                if let Ok(folder_date) = NaiveDate::parse_from_str(name, "%Y-%m-%d") {
                    if folder_date < cutoff {
                        println!("Deleting {:?}", entry_path);
                        // let _ = fs::remove_dir_all(&entry_path).expect("failed to delete directory");
                        continue;
                    }
                }
                clean(&entry_path, cutoff);
            }
        }
    }
}

pub fn clean_recursive(path: &Path, cutoff: NaiveDate) {
    for entry in fs::read_dir(path).expect("failed to read directory") {
        let entry = entry.expect("failed to read directory entry");
        let entry_path = entry.path();

        if entry_path.is_dir() {
            clean_recursive(&entry_path, cutoff);
        }

        let name = entry_path.file_name().and_then(|s| s.to_str());
        if let Some(name) = name {
            if let Ok(folder_date) = NaiveDate::parse_from_str(name, "%Y-%m-%d") {
                if folder_date < cutoff {
                    println!("Deleting {:?}", entry_path);
                    let _ = fs::remove_dir_all(&entry_path).expect("failed to delete directory");
                }
            }
        }
    }
}

fn parse_date_from_filename(name: &str) -> Option<NaiveDate> {
    // expects: YYYY-MM-DD.log
    let date_part = name.strip_suffix(".log")?;
    NaiveDate::parse_from_str(date_part, "%Y-%m-%d").ok()
}

pub fn clean_recursive_file(
    root: &Path,
    cutoff: NaiveDate,
    max_valid_logs: usize,
) {
    let mut stack = vec![root.to_path_buf()];

    while let Some(path) = stack.pop() {
        let mut files: Vec<(NaiveDate, PathBuf)> = Vec::new();

        let entries = match fs::read_dir(&path) {
            Ok(e) => e,
            Err(_) => continue,
        };

        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let p = entry.path();

            if p.is_dir() {
                stack.push(p);
                continue;
            }

            if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
                if let Some(date) = parse_date_from_filename(name) {
                    files.push((date, p));
                }
            }
        }

        // sort newest first
        files.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        for (idx, (date, file)) in files.into_iter().enumerate() {
            if idx >= max_valid_logs && date < cutoff {
                println!("Deleting {:?}", file);
                let _ = fs::remove_file(file);
            }
        }
    }
}

