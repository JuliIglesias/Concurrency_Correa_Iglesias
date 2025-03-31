use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;
use std::thread;
use std::time::Instant;

#[derive(Clone, Copy)]
pub enum SearchMode {
    Sequential,
    Concurrent,
    ConcurrentChunk,
}

pub fn search_word(pattern: &str, paths_files: Vec<String>, mode: SearchMode) -> (Result<Vec<String>>, f64) {
    let results: Result<Vec<String>>;

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
        let filtered_lines: Vec<String> = file_lines.into_iter().filter(|line| line.contains(pattern)).collect::<Vec<String>>();
        result.push(filtered_lines);
    }

    let flatted_result = result.into_iter().flatten().collect::<Vec<String>>();
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
    thread::scope(|s| {
        let threads_results: Vec<Vec<String>> = paths_files.iter().map(|path| {
            s.spawn(move || {
                let file_lines = read_file(path.as_str()).unwrap();
                let chunk_lines = file_lines.chunks(8);

                thread::scope(|chunk_scope| {
                    let chunks_threads_results: Vec<Vec<String>> = chunk_lines.map(|chunk_line| {
                        chunk_scope.spawn(move || {
                            chunk_line.to_vec().into_iter().filter(|line| line.contains(pattern)).collect::<Vec<String>>()
                        })
                    })
                        .map(|chunk_thread_result| {
                            chunk_thread_result.join().unwrap_or_else(|_| Vec::new())
                        })
                        .collect();

                    chunks_threads_results.into_iter().flatten().collect::<Vec<String>>()
                })
            })
        })
            .map(|thread_result| {
                thread_result.join().unwrap_or_else(|_| Vec::new())
            })
            .collect();

        Ok(threads_results.into_iter().flatten().collect::<Vec<String>>())
    })
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
