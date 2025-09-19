use rendering::run;

fn main() {
    run().unwrap_or_else(|_| {println!("Application encountered an error and had to close.")});
}