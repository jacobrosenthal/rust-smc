# smc

>The SMC has roles in controlling thermal and power management, battery charging, video mode switching, sleep and wake, hibernation, and LED indicators. It also enables enforcement of the macOS End User License, allowing it to identify when it is running on non-Apple hardware.[3] 
https://en.wikipedia.org/wiki/System_Management_Controller

This tool is mainly implemented for reading keys and finding news keys at the moment, vs the presumably more dangerous writing new values which you do at your own risk.

# usage
```
    let smc = Smc::new().unwrap();
    let sensor = smc.get_sensor(Key::TCXC).unwrap();
    let value = sensor.read().unwrap()
```

The key enum also have Kind and Subsystem labels which can be used to return an interator matching your search. Note, sensors have nontrivial newup cost so you may wish to keep these around to read them a second time intead of creating them again.
```
    let sensors = smc.find(|key| key.kind() == Kind::Temperature);
```
or you can poll all the sensors we know about on your sytem
```
$ cargo run --example all
🌡️ TCXC 48.50 PECI CPU
🌡️ TC0P 39.00 CPU 1 Proximity
🌡️ TC0E 129.00 CPU 1
🌡️ TC0F 129.00 CPU 1
🌡️ TC1C 49.00 CPU Core 1
🌡️ TC2C 49.00 CPU Core 2
🌡️ TCSA 46.00 Platform Environment Control Interface (PECI) System Agent
🌡️ TCGC 46.00 Platform Environment Control Interface (PECI) GPU
🌡️ Ts0S 129.00 Memory Proximity
🌡️ TM0P 36.38 Memory Slot Proximity
🌡️ TPCD 43.00 Platform Controller Hub Die
🌡️ Th1H 29.56 Heatpipe 2
🌡️ Tm0P 32.12 Mainboard Proximity
🌡️ Ts0P 27.81 Palm Rest L
🌡️ TB1T 26.70 Battery 1
🌡️ TB2T 26.20 Battery 2
⚡ VC1C 0.00 CPU Core 2
⚡ VD0R 0.00 Mainboard S0 Rail
⚡ VP0R 0.00 12V Rail
🚰 IC1C 0.00 CPU VccIO
🚰 IC0R 0.00 CPU Rail
🚰 IM0C 0.00 Memory Controller
🚰 ID0R 0.00 Mainboard S0 Rail
🚰 IO0R 0.00 Misc. Rail
🚰 IPBR 0.76 Charger BMON
🔌 PC1C 0.00 CPU Core 2
🔌 PCPC 0.86 CPU Cores
🔌 PCPG 0.06 CPU GFX
🔌 PC0R 0.00 Mainboard S0 Rail
🔌 PO0R 0.00 Misc. Rail
🔌 PBLC 0.00 Battery Rail
🔌 PDTR 0.00 DC In Total
🔌 PSTR 5.24 System Total
💨 FNum 1.00 Total Fans
💨 F0Ac 1220.00 Fan 1 RPM
💨 F0Mn 1200.00 Fan 1 Min RPM
💨 F0Mx 6500.00 Fan 1 Max RPM
💨 F0Tg 1200.00 Fan 1 Target RPM
💨 FS!  0.00 Fan Mode
```

Not all keys will be known for all devices as this space is largely undocumented. As such Ive included a fuzzing example to find new undocumented keys on your machine:
```
$ cargo run --example fuzz
found CLKT 39008
found MSPC 6
found USR2 4294967300
found PM3C 0.23828125
found BRWK 150
found hSup 0
found SMBR
found DM0P 2
```

# props
This library is a direct descendant of the original reverse engineering work of devnull, currently embodied in the istats project at https://github.com/Chris911/iStats/tree/0b86af356baa680cabc5665cc0364de29f1f5958/ext/osx_stats and the fuzzing idea came from stumbling on https://github.com/theopolis/smc-fuzzer 