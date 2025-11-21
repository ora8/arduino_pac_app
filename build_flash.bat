cargo build --release
rem arm-none-eabi-objcopy -O binary target\thumbv7em-none-eabihf\release\arduino_pac_app -j .text -j .data image.bin
rem cargo-objcopy --bin arduino_pac_app --target thumbv7em-none-eabihf --release -- --output-target=binary -j .text -j .data image.bin
rem arduino-cli upload -b arduino:renesas_uno:unor4wifi -i image.bin -p COM10
rem mode com10:1200,n,8,1
rem "C:\Users\ora8\AppData\Local\Arduino15\packages\arduino\tools\bossac\1.9.1-arduino5/bossac" -d --port=COM10 -U -e -w image.bin -R



