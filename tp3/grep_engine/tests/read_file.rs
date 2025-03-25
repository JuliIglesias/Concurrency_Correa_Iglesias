#[cfg(test)]
mod tests{
    use grep_engine::read_file;
    #[test]
    fn read_file_with_one_line_test_001() {
        let file = read_file("C:/Users/Julieta/projects/concurrency/Concurrency_Correa_Iglesias/tp3/grep_engine/tests/test_001.txt").unwrap();
        let mut expected: Vec<String> = Vec::new();
        expected.push("Hello, World!".to_string());

        assert_eq!(file, expected);
    }

    #[test]
    fn read_file_with_two_lines_test_002(){
        let file = read_file("C:/Users/Julieta/projects/concurrency/Concurrency_Correa_Iglesias/tp3/grep_engine/tests/test_002.txt").unwrap();
        let mut expected: Vec<String> = Vec::new();
        expected.push("Hello, World!".to_string());
        expected.push("Goodbye, World!".to_string());

        assert_eq!(file, expected);
    }

    #[test]
    fn should_return_file_not_found_when_file_not_found_test_003(){
        let file = read_file("test_003.txt");

        assert!(file.is_err());
    }
}
