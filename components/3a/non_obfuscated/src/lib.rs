pub fn parse_input() -> i32 {
    std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(5)
}
