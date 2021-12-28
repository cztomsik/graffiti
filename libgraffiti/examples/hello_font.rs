use graffiti::swash::{FontDataRef};
use graffiti::fontdb;

fn main() {
    let mut db = fontdb::Database::new();
    let now = std::time::Instant::now();
    db.load_system_fonts();
    db.set_serif_family("Times New Roman");
    db.set_sans_serif_family("Arial");
    db.set_cursive_family("Comic Sans MS");
    db.set_fantasy_family("Impact");
    db.set_monospace_family("Courier New");
    println!("Loaded {} font faces in {}ms.", db.len(), now.elapsed().as_millis());

    let now = std::time::Instant::now();
    let DIRS = [
        "/Library/Fonts",
        "/System/Library/Fonts",
        "/usr/share/fonts",
        "/usr/local/share/fonts",
        "C:\\Windows\\Fonts",
    ];

    let mut len = 0;
    for dir in DIRS {
        len += load_dir(dir);
    }
    println!("Loaded {} font faces in {}ms.", len, now.elapsed().as_millis());

    fn load_dir(dir: &str) -> usize {
        let mut len = 0;

        if let Ok(dir) = std::fs::read_dir(dir) {
            for entry in dir.filter_map(Result::ok) {
                let path = entry.path();

                if path.is_file() {
                    if let Ok(meta) = std::fs::metadata(&path) {
                        if meta.len() > 10_000_000 {
                            continue;
                        }

                        if let Some(path) = path.to_str() {
                            len += load_file(path);
                        }
                    }
                } else if path.is_dir() {
                    if let Some(path) = path.to_str() {
                        len += load_dir(path);
                    }
                }
            }
        }

        len
    }

    fn load_file(file: &str) -> usize {
        if let Ok(data) = std::fs::read(file) {
            if let Some(font_data) = FontDataRef::new(&data) {
                let len = font_data.len();

                for i in 0..len {
                    //println!("loading {:?}", (file, i));
                }

                return len
            }
        }

        0
    }
}