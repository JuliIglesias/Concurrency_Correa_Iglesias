pub struct Stats {
    pub total_exceptions: usize,
    pub files_processed: usize,
    pub exceptions_per_file: Vec<(String, usize)>,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            total_exceptions: 0,
            files_processed: 0,
            exceptions_per_file: Vec::new(),
        }
    }

    pub fn format_stats(&self) -> String {
        let per_file_formatted: Vec<String> = self.exceptions_per_file
            .iter()
            .map(|(file, count)| format!("\"{}\": {}", file, count))
            .collect();

        format!(
            "Total exceptions: {}\r\n\
            Files processed: {}\r\n\
            Per file: {{{}}}\r\n",
            self.total_exceptions,
            self.files_processed,
            per_file_formatted.join(", ")
        )
    }
}