# Sarge
A laptop battery-status notification daemon written in Rust.

## How it works
*Currently only supports Linux*

Sarge reads the files in the `/sys/class/power_supply` directory, which contains information regarding the battery and power adapter. In its config file are defined a set of *triggers* which dictate when a notification will be shown. A trigger maybe defined for when a certain battery percentage is reached or for when the ac power is connected or disconnected.

## Configuration

### File location
Sarge looks in the following files, in order, for its config:

1. `$XDG_CONFIG_HOME/sarge/sarge.yml`
2. `$XDG_CONFIG_HOME/sarge.yml`
3. `$HOME/.config/sarge/sarge.yml`
4. `$HOME/.sarge.yml`

If a config file is not found, Sarge uses its own default config. The default config is provided [here](https://github.com/RedDocMD/Sarge/blob/master/sarge.yml).

### File format
The config file is to be written in YAML. The keys are as follows:

1. `update_interval`: The time-interval, in milisecond, at which Sarge will update the battery info.
2. `triggers`: Array for the triggers, which is a set of the following keys:
	- `percentage`: The battery percentage at which the trigger takes place. 
	- `when`: It is one of the following alternatives (**case matters**):
		* `Equal` - when the battery percentage becomes equal to `percentage`
		* `Above` - when the battery percentage goes above the `percentage`
		* `Below` - when the battery percentage goes below the `percentage`
		* `Charging` - when the AC adapter is plugged in. `percentage` key is ignored
		* `Discharging` - when the AC adapter is unplugged. `percentage` key is ignored

Sarge supports *hot-reloading* of the config file, which means that it watches for changes in its config file.

## Requirements
Sarge uses D-Bus notifications to send its notifications. Desktop environments typically have this out of the box. If you use a discrete window-manager, then you might need to install a notification serer. I personally use [Dunst](https://github.com/dunst-project/dunst). For further information, please refer to [this](https://wiki.archlinux.org/index.php/Desktop_notifications) Arch Wiki page.

## Building
You must have `cargo` installed.
```
git clone https://github.com/RedDocMD/Sarge
cd Sarge
cargo build --release
```
The binary will be formed in `target/release/sarge`.

## License
Sarge is released under the [MIT](https://github.com/RedDocMD/Sarge/blob/master/LICENSE) license.
