use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

#[derive(Clone, Copy)]
pub enum SearchMode {
    Sequential,
    Concurrent,
    ConcurrentChunk,
}

pub fn search_word(pattern: &str, paths_files: Vec<String>, mode: SearchMode) -> (Result<Vec<String>>, f64) {
    let mut results: Result<Vec<String>>;

    let now = Instant::now();
    match mode {
        SearchMode::Sequential => results = search_word_sequential(pattern, paths_files),
        SearchMode::Concurrent => results = search_pattern_several_files_concurrent(pattern, paths_files),
        SearchMode::ConcurrentChunk => results = search_word_concurrent_chunk(pattern, paths_files),
    }

    let total_duration: f64 = now.elapsed().as_secs_f64();

    (results, total_duration)
}

fn search_word_sequential(pattern: &str, paths_files: Vec<String>) -> Result<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();

    for path_file in paths_files {
        let file_lines: Vec<String> = read_file(path_file.as_str())?;
        let filtered_lines: Vec<String> = file_lines.filter(|line| line.contains(pattern)).collect::<Vec<String>>();
        result.push(filtered_lines);
    }

    let flatted_result = result.iter().flatten().collect::<Vec<String>>();
    Ok(flatted_result)
}

fn search_pattern_several_files_concurrent(pattern: &str, paths_files: Vec<String>) -> Result<Vec<String>> {
    thread::scope(|s| {
        let threads_results: Vec<Vec<String>> = paths_files.iter().map(|path| {
            s.spawn(move || {
                let file_lines = read_file(path.as_str()).unwrap();
                file_lines.into_iter().filter(|line| line.contains(pattern)).collect::<Vec<String>>()
            })
        })
            .map(|thread_result| {
                thread_result.join().unwrap_or_else(|_| Vec::new()) })
            .collect();

        Ok(threads_results.into_iter().flatten().collect::<Vec<String>>())
    })
}

fn search_word_concurrent_chunk(pattern: &str, paths_files: Vec<String>) -> Result<Vec<String>> {
    let result = Arc::new(Mutex::new(Vec::new()));

    thread::scope(|s| {
        for path_file in paths_files {
            let result = Arc::clone(&result);
            s.spawn(move || {
                if let Ok(file_lines) = read_file(path_file.as_str()) {
                    let chunks = file_lines.chunks(4);
                    for chunk in chunks {
                        let result = Arc::clone(&result);
                        let chunk: Vec<String> = chunk.to_vec();
                        s.spawn(move || {
                            let mut result = result.lock().unwrap();
                            push_result(pattern, &mut result, chunk);
                        });
                    }
                }
            });
        }
    });

    let result = Arc::try_unwrap(result).unwrap().into_inner().unwrap();
    Ok(result)
}

fn push_result(pattern: &str, result: &mut Vec<String>, file_lines: Vec<String>) {
    for line in file_lines {
        if line.contains(pattern) {
            result.push(line);
        }
    }
}

pub fn read_file(file_path: &str) -> Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.trim().is_empty())
        .collect();
    Ok(lines)
}