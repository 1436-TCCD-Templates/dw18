use std::fs;

fn main() {
    let text = fs::read_to_string("manifest.csv").expect("could not read manifest.csv");
    let cents = dw18::manifest::total_cents(&text);
    println!("Manifest total: {}", dw18::format_dollars(cents));
}
