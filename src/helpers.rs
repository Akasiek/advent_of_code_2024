pub fn get_input_from_file(day: i32) -> String {
    let path = format!("src/inputs/day_{}", day);
    
    std::fs::read_to_string(path).unwrap()
}