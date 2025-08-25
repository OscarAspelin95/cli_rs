use crate::utils::{Args, FileMeta, Modified, Size};
use std::time::SystemTime;
use tabled::{
    settings::{object::Columns, Color, Style},
    Table,
};

fn modified_since(modified_seconds: u64) -> Modified {
    if modified_seconds < 60 {
        return Modified::Seconds(modified_seconds);
    } else if modified_seconds < 3600 {
        return Modified::Minutes(modified_seconds / 60);
    } else if modified_seconds < 86400 {
        return Modified::Hours(modified_seconds / 3600);
    } else {
        return Modified::Days(modified_seconds / 86400);
    }
}

fn get_size(size: u64) -> Size {
    let max_b = 2u64.pow(10);
    let max_kb = max_b.pow(2);
    let max_mb = max_b.pow(3);

    // Size is in bytes.
    if size < max_b {
        return Size::Byte(size);
    }

    // Size is in kilobytes.
    if size < max_kb {
        return Size::KiloByte(size / max_b);
    }

    // Size is in megabytes.
    if size < max_mb {
        return Size::MegaByte(size / max_kb);
    }

    // Size is in gigabytes.
    return Size::GigaByte(size / max_mb);
}

pub fn cli(args: &Args) {
    let systime = SystemTime::now();
    let mut results: Vec<FileMeta> = Vec::new();

    for content in std::fs::read_dir(&args.dir).unwrap() {
        match content {
            Ok(p) => {
                if p.path().is_file() {
                    // Extract file information.
                    let meta_data = p.metadata().unwrap();

                    let modified_seconds = systime
                        .duration_since(meta_data.modified().unwrap())
                        .unwrap()
                        .as_secs();

                    // Struct instance for file meta data.
                    let file_meta = FileMeta {
                        name: p.file_name().into_string().unwrap(),
                        size: get_size(meta_data.len()),
                        // Minutes since file was modified.
                        modified: modified_since(modified_seconds),
                    };
                    results.push(file_meta);
                }
            }
            Err(_) => {}
        }
    }

    match (args.by_size, args.by_modified) {
        (true, true) => {
            results.sort_by(|a, b| b.size.cmp(&a.size).then(a.modified.cmp(&b.modified)));
        }
        (true, _) => {
            results.sort_by_key(|result| std::cmp::Reverse(result.size));
        }
        (_, true) => {
            results.sort_by_key(|result| std::cmp::Reverse(result.modified));
        }
        _ => {}
    }

    let mut table = Table::new(results);

    table
        .with(Style::rounded())
        .modify(Columns::one(0), Color::FG_BRIGHT_CYAN)
        .modify(Columns::one(1), Color::FG_BRIGHT_WHITE)
        .modify(Columns::one(2), Color::FG_RED);

    println!("{}", table);
}
