#!/bin/bash

# Downloads Sarge from GitHub, builds it with Cargo and adds auto-start script to /etc/profile.d/
# Requires sudo for the last step

URL=https://github.com/RedDocMD/Sarge # Git repo URL
DEF_LOC=${HOME}/sarge # Default install location

# Check for git
if ! command -v git &> /dev/null; then
	echo  "Please install git and run this script again"
	exit 1
fi

# Check for cargo
if ! command -v cargo &> /dev/null; then
	echo "Please install Cargo and run this script again"
	exit 1
fi

# Get install location
echo -n "Enter location to download Sarge [${DEF_LOC}]: "
read LOC
if [[ $LOC == "" ]]; then
	LOC=${DEF_LOC}
fi

# Clone repo
echo ""
echo "Cloning into repo ..."
git clone "$URL" ${LOC}

# Now to build Sarge
echo ""
echo "Building Sarge ...."
cd $LOC
cargo build --release

# Go to /tmp and make the auto-start file
echo ""
echo "Creating auto-start script in /tmp"
cd /tmp
if ! command -v mktemp &> /dev/null; then
	TMP_NAME="sarge.sh"
else
	TMP_NAME=$(mktemp "sarge-XXXX.sh")
fi
echo "$LOC/target/release/sarge &" >> ${TMP_NAME}
chmod +x $TMP_NAME

# Copying auto-start script to /etc/profile.d (requires sudo)
echo "Copying start script to /etc/profile.d"
sudo cp -pn $TMP_NAME /etc/profile.d/$TMP_NAME

exit 0
