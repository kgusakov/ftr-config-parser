use std::fs;
use std::process;

fn main() {
    let dir = match fs::read_dir("data/ftr-site-config") {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error: cannot open data/ftr-site-config: {e}");
            eprintln!("hint: run `just setup` to fetch the corpus submodule");
            process::exit(2);
        }
    };

    let mut ok = 0u32;
    let mut failed: Vec<(String, String)> = Vec::new();

    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("txt") {
            continue;
        }
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));

        match ftr_config_parser::parse_config(&content) {
            Ok(_) => ok += 1,
            Err(e) => failed.push((name, e.to_string())),
        }
    }

    for (name, err) in &failed {
        println!("FAIL {name}: {err}");
    }

    println!("\n{ok} ok, {} failed", failed.len());

    if !failed.is_empty() {
        process::exit(1);
    }
}
