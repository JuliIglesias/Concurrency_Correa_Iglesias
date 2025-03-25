use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;
use std::{thread};
use std::slice::Chunks;

pub enum SearchMode {
    Sequential,
    Concurrent,
    ConcurrentChunk,
}

pub fn search_word(pattern: &str, paths_files: Vec<String>, mode: SearchMode ) -> Result<Vec<String>> {
    match mode {
        SearchMode::Sequential => search_word_sequential(pattern, paths_files),
        SearchMode::Concurrent => search_pattern_several_files_concurrent(pattern, paths_files),
        SearchMode::ConcurrentChunk => search_word_concurrent_chunk(pattern, paths_files),
    }
}



fn search_word_sequential(pattern: &str, paths_files: Vec<String>) -> Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();

    for path_file in paths_files {
        let file_lines = read_file(path_file.as_str())?; // traspasa a un vector de strings
        push_result(pattern, &mut result, file_lines);
    }

    Ok(result)
}

fn search_pattern_several_files_concurrent(pattern: &str, paths_files: Vec<String>) -> Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();

    thread::scope(|s| {
        for path_file in paths_files{
            s.spawn(move |_| {
                let file_lines = read_file(path_file.as_str()).unwrap(); // traspasa a un vector de strings
                push_result(pattern, &mut result, file_lines);
            });
        }
    });

    Ok(result)
}

fn search_word_concurrent_chunk(pattern: &str, paths_files: Vec<String>) -> Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();

    thread::scope(|s| {
        for path_file in paths_files{
            s.spawn(move |_| {
                let file_chunks:Chunks<String> = read_file(path_file.as_str()).unwrap().chunks(4); // traspasa a un vector de strings
                for chunk in file_chunks {
                    let vector:Vec<String> = chunk.iter().collect();
                    s.spawn(|_| {
                        push_result(pattern, &mut result, vector);

                    });
                }
            });
        }
    });

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
        .map(|line| line.unwrap())
        .filter(|line| !line.trim().is_empty())
        .collect();
    Ok(lines)
}
