extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("RF Utility")
        .version("1.0")
        .author("Zackary Troop <zack.tro@gmail.com>")
        .about("Provides various RF utilities")
        .subcommand(
            SubCommand::with_name("power_conversion")
                .about("Transmitter Power mW <-> dBm Conversion")
                .arg(
                    Arg::with_name("value")
                        .help("The value to convert, followed by its unit (mW or dBm)")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("path_loss")
                .about("Free Space Path Loss Calculation")
                .arg(
                    Arg::with_name("frequency")
                        .help("Frequency in MHz")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("distance")
                        .help("Distance in meters")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("link_range")
                .about("RF Link Range Calculation")
                .arg(
                    Arg::with_name("transmitter_power")
                        .help("Transmitter power in dBm")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("receiver_sensitivity")
                        .help("Receiver sensitivity in dBm")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("frequency")
                        .help("Frequency in MHz")
                        .required(true)
                        .index(3),
                ),
        )
        .subcommand(
            SubCommand::with_name("times_further")
                .about("Times Further Calculation")
                .arg(
                    Arg::with_name("current_distance")
                        .help("Current distance in meters")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("new_distance")
                        .help("New distance in meters")
                        .required(true)
                        .index(2),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("power_conversion", Some(sub_m)) => {
            let input = sub_m.value_of("power").unwrap();
            let result = transmitter_power_conversion(input);
            match result {
                Ok(v) => println!("{}", v),
                Err(e) => println!("{}", e),
            }
        }
        ("path_loss", Some(sub_m)) => {
            let frequency: f64 = sub_m.value_of("frequency").unwrap().parse().unwrap();
            let distance: f64 = sub_m.value_of("distance").unwrap().parse().unwrap();
            let fspl = free_space_path_loss(frequency, distance);
            println!("{}", fspl);
        }
        ("link_range", Some(sub_m)) => {
            let pt: f64 = sub_m
                .value_of("transmitter_power")
                .unwrap()
                .parse()
                .unwrap();
            let pr: f64 = sub_m
                .value_of("receiver_sensitivity")
                .unwrap()
                .parse()
                .unwrap();
            let frequency: f64 = sub_m.value_of("frequency").unwrap().parse().unwrap();
            let lr = rf_link_range(pt, pr, frequency);
            println!("{}", lr);
        }
        ("times_further", Some(sub_m)) => {
            let d1: f64 = sub_m.value_of("current_distance").unwrap().parse().unwrap();
            let d2: f64 = sub_m.value_of("new_distance").unwrap().parse().unwrap();
            let tf = times_further(d1, d2);
            println!("{}", tf);
        }
        _ => println!(
            "Invalid subcommand or no subcommand provided. Use --help for more information."
        ),
    }
}

/// Converts between mW and dBm.
///
/// # Arguments
///
/// * `input` - The power value with unit (either "mW" or "dBm").
///
/// # Returns
///
/// * `Result<f64, &'static str>` - A result containing the converted value or an error string.
fn transmitter_power_conversion(input: &str) -> Result<f64, &'static str> {
    let input = input.to_lowercase();
    if input.ends_with("mw") {
        let value: f64 = input.replace("mw", "").trim().parse().unwrap();
        Ok(10.0 * value.log(10.0))
    } else if input.ends_with("dbm") {
        let value: f64 = input.replace("dbm", "").trim().parse().unwrap();
        Ok(10.0_f64.powf(value / 10.0))
    } else {
        Err("Invalid input. Please enter a valid value followed by its unit (mW or dBm).")
    }
}

/// Calculates the Free Space Path Loss (FSPL) given a frequency and distance.
///
/// # Arguments
///
/// * `frequency` - The frequency in MHz.
/// * `distance` - The distance in meters.
///
/// # Returns
///
/// * `f64` - The FSPL value.
fn free_space_path_loss(frequency: f64, distance: f64) -> f64 {
    let c = 299_792_458.0; // Speed of light in meters/second
    20.0 * distance.log10()
        + 20.0 * (frequency * 1_000_000.0).log10()
        + 20.0 * (c / (4.0 * std::f64::consts::PI)).log10()
}

/// Calculates the RF Link Range using the Friis transmission formula.
///
/// # Arguments
///
/// * `pt` - The transmitter power in dBm.
/// * `pr` - The receiver sensitivity in dBm.
/// * `frequency` - The frequency in MHz.
///
/// # Returns
///
/// * `f64` - The RF link range value.
fn rf_link_range(pt: f64, pr: f64, frequency: f64) -> f64 {
    let c = 299_792_458.0;
    let lambda = c / (frequency * 1_000_000.0);
    (lambda / (4.0 * std::f64::consts::PI)) * 10.0_f64.powf((pr - pt) / 20.0)
}

/// Calculates how many times further one distance is from another.
///
/// # Arguments
///
/// * `d1` - The current distance in meters.
/// * `d2` - The new distance in meters.
///
/// # Returns
///
/// * `f64` - The "times further" value.
fn times_further(d1: f64, d2: f64) -> f64 {
    d2 / d1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transmitter_power_conversion_mw_to_dbm() {
        let result = transmitter_power_conversion("10mW").unwrap();
        assert!((result - 10.0).abs() < 0.01);

        let result = transmitter_power_conversion("1mW").unwrap();
        assert!((result - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_transmitter_power_conversion_dbm_to_mw() {
        let result = transmitter_power_conversion("10dBm").unwrap();
        assert!((result - 10.0).abs() < 0.01);

        let result = transmitter_power_conversion("0dBm").unwrap();
        assert!((result - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_free_space_path_loss() {
        let fspl = free_space_path_loss(500.0, 1000.0);
        println!("FSPL Calculated: {}", fspl);
        assert!((fspl - 381.53).abs() < 0.01);
    }

    #[test]
    fn test_rf_link_range() {
        let range = rf_link_range(-30.0, -60.0, 500.0);
        let rounded_range = (range * 1_000_000.0).round() / 1_000_000.0;
        println!("Range Calculated: {}", rounded_range);
        assert!((rounded_range - 0.001509).abs() < 0.000001);
    }

    #[test]
    fn test_times_further() {
        let times = times_further(100.0, 200.0);
        assert_eq!(times, 2.0);
    }
}
