use std::env;
use grep_engine::search_word;
use grep_engine::SearchMode;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <mode> <pattern> <file1> <file2> ...", args[0]);
        std::process::exit(1);
    }

    let mode = match args[1].as_str() {
        "seq" => SearchMode::Sequential,
        "conc" => SearchMode::Concurrent,
        "c-chunk" => SearchMode::ConcurrentChunk,
        _ => {
            eprintln!("Invalid mode: {}", args[1]);
            std::process::exit(1);
        }
    };

    let pattern = &args[2];
    let files = &args[3..];

    for file in files {
        let result = search_word(pattern, vec![file.to_string()], mode).0.unwrap();
        for line in result {
            println!("{}", line);
        }
    }
}
