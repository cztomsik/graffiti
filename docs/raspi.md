## raspi (common)
- `sudo apt update && sudo apt upgrade`
- `sudo apt install cmake clang xorg-dev`

## raspi 3
- Full KMS
- verify `glxgears -info` to show `Gallium` (`apt install mesa-utils`)
- build with X down (`sudo service lightdm stop`) & limit threads `npm run build -- -j 1`

## raspi 4 (buster)
- Fake KMS
- `cd /usr/lib/arm-linux-gnueabihf`
- `ln -s libGL.so.1 libGL.so`
