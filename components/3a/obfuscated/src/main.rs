use non_obfuscated::parse_input;

fn secret_message() -> &'static str {
    "AMICE_RUST_STRING_TEST_20260822"
}

fn classify(n: i32) -> &'static str {
    if n < 0 { "negative" }
    else if n == 0 { "zero" }
    else if n < 10 { "small" }
    else { "large" }
}

fn main() {
    println!("{}", secret_message());
    let input = parse_input();
    println!("{}", classify(std::hint::black_box(input)));
}
