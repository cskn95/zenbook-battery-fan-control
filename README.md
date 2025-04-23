# Zenbook Battery Fan Control

Zenbook Battery Fan Control is a Rust-based application designed to give the user basic fan speed control and battery charge limit management for Asus Zenbook series laptops. This software aims to improve device performance and battery life by managing fan control and battery charge limit.

## Features
- **Fan control:** Only max fan speed and auto modes are available now.
- **Battery charge control:** Can add charging limits to increase battery life.
- **Open-source:** Developed with community support and freely available.

## Usage via pre-compiled executable (recommended)
- Download the latest release from the "releases" section.
- Rotate to the folder that contains the downloaded file from the terminal (e.g., `cd $HOME/Downloads/`).
- Use the app from the terminal with the desired commands.
- E.g.: `sudo ./zenbook-cli -h`

## Installation via auto-compile script from source
- This script checks and downloads (if necessary) the required Rust tools to compile the code and copies the compiled executable to the ".local/bin" directory.
- Use the app from the terminal with the desired commands.
- E.g.: `sudo $Home/.local/bin/zenbook-cli -h`
  
```bash
git clone https://github.com/cskn95/zenbook-battery-fan-control
```
```bash
cd zenbook-battery-fan-control
```
```bash
chmod +x start.sh
```
```bash
./start.sh
```

## Available commands
- **-b**
  - optimal: Charging limit to 80%.
  - max-blife: Charging limit to 60%.
  - full: No limit.
  - one-time: Charge to 100% only once.
- **-f**
  - full: Maximum fan speed.
  - auto: Auto fan speed.
 
E.g.: `zenbook-cli -b optimal`
