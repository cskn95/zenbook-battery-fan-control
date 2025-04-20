# Zenbook Battery & Fan Control

This project is a **command-line tool** designed to control the **battery charge limit** and **fan speed** of **ASUS Zenbook** laptops.

## Features
- **Battery management**: Set the charge limit to optimize battery lifespan.
- **Fan control**: Adjust fan speed manually or set it to automatic mode.
- **User-friendly commands**: Quickly manage settings via terminal commands.

## Installation
<details>
<summary>This project is developed using <B>Rust</B> To use it, you must install <b>Rust</b> first; </summary>
 <code>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
</code>
</details>

If rust is installed on your system, continue

```bash
git clone https://github.com/cskn95/zenbook-battery-fan-control.git
```
```bash
cd zenbook-battery-fan-control
```
```bash
cargo build --release
```

----------------------------------------------------------------------------------

### **Usage**  

#### **Battery Charge Management**  
- **Optimize Battery Life**  
  Sets the battery charge limit to **60%** to reduce long-term battery degradation and extend lifespan. Ideal for users who frequently keep their laptop plugged in.  
  Usage: `--battery max-blife`  

- **Set Battery to Optimal Mode**  
  Configures the battery charge limit to **80%**, balancing battery health and usability. Recommended for users who need slightly longer unplugged runtime.  
  Usage: `--battery optimal`  

- **Disable Battery Charge Limit**  
  Allows the battery to charge up to **100%**, useful for scenarios where maximum battery capacity is required.  
  Usage: `--battery full`  

#### **Fan Speed Management**  
- **Enable Automatic Fan Mode**  
  The system automatically adjusts fan speed based on temperature, ensuring optimal cooling efficiency with minimal noise.  
  Usage: `--fan auto`  

- **Set Manual Fan Speed**  
  Users can manually set fan speed by specifying a percentage. Example: **70% speed** for increased cooling during high-performance tasks.  
  Usage: `--fan [percentage]` (e.g., `--fan 70`)  

- **Enable Maximum Fan Speed**  
  If extreme cooling is required, this mode sets the fan to **maximum speed** to prevent overheating.  
  Usage: `--fan max`  

#### **Check System Status**  
- **Display Current Battery Settings**  
  Shows the active battery charge limit and related settings. Useful for verifying configurations.  
  Usage: `--battery status`  

- **Check Fan Speed Settings**  
  Displays the current fan mode (manual or automatic) and speed percentage. Helps monitor cooling efficiency.  
  Usage: `--fan status`  
