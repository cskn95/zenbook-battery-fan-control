use std::{fs, io, process::exit};
use clap::Parser;

#[derive(Parser)]
#[command(name = "zenbook-cli", version = "0.1.0", author = "Emirhan Coşkun", about = "A battery and fan manager for ASUS Zenbook laptops")]
struct Cli {
    #[arg(short, long)]
    fan: Option<String>,

    #[arg(short, long)]
    battery: Option<String>,
}

fn write_value(file_path: &str, value: &str) -> Result<(), std::io::Error> {
    fs::write(file_path, value)
}

fn main() {
    let cli = Cli::parse();

    const PWM1_ENABLE_PATH: &str = "/sys/devices/platform/asus-nb-wmi/hwmon/hwmon7/pwm1_enable";
    const BATTERY_PATH: &str = "/sys/class/power_supply/BAT0/charge_control_end_threshold";
    const UDEV_RULE_PATH: &str = "/etc/udev/rules.d/99-zenbook-battery.rules";

    if let Some(fan_value) = cli.fan {

        let value_to_write = match fan_value.as_str() {
            "auto" => "2",
            "full" => "0",
            _ => {
                eprintln!("Invalid fan mode. Use 'auto' or 'full'.");
                exit(31)
            }
        };

        match write_value(PWM1_ENABLE_PATH, value_to_write) {
            Ok(_) => {
                println!("Successfully wrote '{}' value to {}.", value_to_write, PWM1_ENABLE_PATH);
            }
            Err(e) => {
                eprintln!("Error! Failed to write to {}!", PWM1_ENABLE_PATH);
                if e.kind() == io::ErrorKind::PermissionDenied {
                    eprintln!("Access denied, please use with root priviliges.");
                } else if e.kind() == io::ErrorKind::NotFound {
                     eprintln!("File not found: {}", PWM1_ENABLE_PATH);
                }
                exit(1); 
            }
        };
    }

    
    if let Some(battery_value) = cli.battery {
        let value_to_write = match battery_value.as_str() {
            "optimal" => "80",
            "full" => "100",
            "max-blife" => "60",
            "one-time" => "100",
            _ => {
                eprintln!("Invalid battery mode. Use 'max-blife', 'optimal' or 'full'.");
                exit(31)
            }
        };

        match write_value(BATTERY_PATH, value_to_write) {
            Ok(_) => {
                println!("Successfully wrote '{}' value to {}.", value_to_write, BATTERY_PATH);
            }
            Err(e) => {
                eprintln!("Error! Failed to write to {}!", BATTERY_PATH);
                if e.kind() == io::ErrorKind::PermissionDenied {
                    eprintln!("Access denied, please use with root priviliges.");
                } else if e.kind() == io::ErrorKind::NotFound {
                     eprintln!("File not found: {}", BATTERY_PATH);
                }
                exit(1); 
            }
        };

        if battery_value != "one-time" {
            let rule_content = format!(r#"SUBSYSTEM=="power_supply", KERNEL=="BAT0", ATTR{{charge_control_end_threshold}}="{}""#, value_to_write);

            match fs::write(UDEV_RULE_PATH, rule_content) {
            Ok(_) => println!("udev rule successfully written"),
            Err(e) => println!("failed to write to udev rule: {:?}{:?}", Some(e), Some(UDEV_RULE_PATH))
        }}
    }
}