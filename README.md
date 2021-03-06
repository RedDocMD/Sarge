# Sarge
A laptop battery-status notification daemon written in Rust.

## How it works
*Currently only supports Linux*

Sarge reads the files in the `/sys/class/power_supply` directory, which contains information regarding the battery and power adapter. In its config file are defined a set of *triggers* which dictate when a notification will be shown. A trigger maybe defined for when a certain battery percentage is reached or for when the ac power is connected or disconnected.

## Dependencies 
Sarge uses D-Bus notifications to send its notifications. Desktop environments typically have this out of the box. If you use a discrete window-manager, then you might need to install a notification serer. I personally use [Dunst](https://github.com/dunst-project/dunst). For further information, please refer to [this](https://wiki.archlinux.org/index.php/Desktop_notifications) Arch Wiki page.

Also, to build and install (or to use the script for that), you must have `git` and `cargo` installed and in your `$PATH`. For installing `cargo`, please look at the Rust [install](https://www.rust-lang.org/tools/install) documentation.

## Installation

### Build and install Script
Sarge can be built and installed by the following command:
```
bash <(wget -qO- https://raw.githubusercontent.com/RedDocMD/Sarge/master/build.sh)
```

Alternatively, you can download the script from [here](https://raw.githubusercontent.com/RedDocMD/Sarge/master/build.sh) and run it.

### Manual build and install
Suppose you are in `$HOME` when you perform the install.
```
git clone https://github.com/RedDocMD/Sarge
cd Sarge
cargo build --release
```
The binary will be formed in `target/release/sarge`.

To automatically start this on login, make a file called `sarge.sh` in `/etc/profile.d` and make it executable. Add the line `$HOME/Sarge/target/release/sarge`, save and re-login.
```
cd /tmp
touch sarge.sh
echo "$HOME/Sarge/target/release/sarge &" > sarge.sh
sudo bash
cp sarge.sh /etc/profile.d/
chmod +x /etc/profle.d/sarge.sh
exit
```

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

## License
Sarge is released under the [MIT](https://github.com/RedDocMD/Sarge/blob/master/LICENSE) license.
