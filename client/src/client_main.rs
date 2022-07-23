use alluxio_common::settings::Settings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new();
    println!("{:?}", settings);
    Ok(())
}