use text_io::read;
use std::time::Duration;
use std::thread::sleep;
use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    println!("Please enter the feature description:");
    let _feature_description: String = read!();

    println!("How long do you expect it to take? ");

    let _estimate: String = read!();

    println!("Using TWP model to create a more accurate estimate... Please wait...");
    sleep(Duration::from_millis(rng.random_range(0..10000)));

    println!("Model Estimate: Two Weeks");
}
