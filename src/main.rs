    use std::{fs, io, path::Path, process::exit};
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
    
        let pwm1_enable_path: String;
        const HWMON_DIR_PATH: &str = "/sys/devices/platform/asus-nb-wmi/hwmon";
        const BATTERY_DIR_PATH: &str = "/sys/class/power_supply/BAT0";
        const BATTERY_PATH: &str = "/sys/class/power_supply/BAT0/charge_control_end_threshold";
        const UDEV_RULE_PATH: &str = "/etc/udev/rules.d/99-zenbook-battery.rules";
    
        if let Some(fan_value) = cli.fan {
    
            if !Path::new(HWMON_DIR_PATH).is_dir() {
                eprintln!("Error: {} directory not found.", HWMON_DIR_PATH);
                eprintln!("This might mean your laptop model is not supported or the necessary kernel modules are not loaded.");
                eprintln!("laptop not supported");
                exit(69);
            }

            match fs::read_dir(HWMON_DIR_PATH) {
                Err(why) => {
                    println!("! {:?}", why.kind());
                    exit(404);
                },
                Ok(paths) => {
                    let paths_vec: Vec<_> = paths.collect::<Result<Vec<_>, _>>().unwrap_or_else(|e| {
                        println!("! Dosyaları okurken hata: {:?}", e);
                        exit(1);
                    });

                    if paths_vec.len() == 1 {
                        let path = paths_vec[0].path();
                        pwm1_enable_path = path.to_string_lossy().to_string();
                    } else {
                        println!("Multiple hardware watchers detected, please report your laptop model for support");
                        exit(16);
                    }
                }
            }
    
            let value_to_write = match fan_value.as_str() {
                "auto" => "2",
                "full" => "0",
                _ => {
                    eprintln!("Invalid fan mode. Use 'auto' or 'full'.");
                    exit(31)
                }
            };
    
            match write_value(&pwm1_enable_path, value_to_write) {
                Ok(_) => {
                    println!("Successfully wrote '{}' value to {}.", value_to_write, pwm1_enable_path);
                }
                Err(e) => {
                    eprintln!("Error! Failed to write to {}!", pwm1_enable_path);
                    if e.kind() == io::ErrorKind::PermissionDenied {
                        eprintln!("Access denied, please use with root priviliges.");
                    } else if e.kind() == io::ErrorKind::NotFound {
                         eprintln!("File not found: {}", pwm1_enable_path);
                    }
                    exit(1); 
                }
            };
        }
        
        if let Some(battery_value) = cli.battery {
    
            if !Path::new(BATTERY_DIR_PATH).is_dir() {
                eprintln!("Error: {} directory not found.", BATTERY_PATH);
                eprintln!("This might mean your laptop model is not supported or the necessary kernel modules are not loaded.");
                eprintln!("laptop not supported");
                exit(69); // Uygun bir hata kodu ile çıkış yapalım (örn: 1)
            }
            
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
                let rule_content = format!(
                    r#"ACTION=="change", SUBSYSTEM=="power_supply", KERNEL=="AC0", ENV{{POWER_SUPPLY_ONLINE}}=="1", RUN+="/bin/sh -c 'echo {} > /sys/class/power_supply/BAT0/charge_control_end_threshold'""#,
                    value_to_write
                );
    
                match fs::write(UDEV_RULE_PATH, rule_content) {
                Ok(_) => println!("udev rule successfully written"),
                Err(e) => println!("failed to write to udev rule: {:?}{:?}", Some(e), Some(UDEV_RULE_PATH))
            }}
        }
    }