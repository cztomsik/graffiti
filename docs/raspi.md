# Raspberry Pi

## raspi (common)
- stock raspbian buster
- `sudo apt update && sudo apt upgrade`
- `sudo apt install cmake clang xorg-dev`
- disable swap, have enough free ram (`sudo service lightdm stop`) and have a lot of patience (can take 10s of minutes)
  - or you can build on raspi4 & copy to other raspis
- sometimes you might need to:
  - `cd /usr/lib/arm-linux-gnueabihf`
  - `sudo ln -s libGL.so.1 libGL.so`

## raspi 3
- Full KMS
- verify `glxgears -info` to show `Gallium` (`apt install mesa-utils`)

## raspi 4
- Fake KMS
