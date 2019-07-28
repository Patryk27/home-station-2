#!/bin/bash

set -e

cross build --target arm-unknown-linux-gnueabihf --release
rsync -az target/arm-unknown-linux-gnueabihf/release/home-station-2 pi.home:/home/pi/home-station --info=progress2