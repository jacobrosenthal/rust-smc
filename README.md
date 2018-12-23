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
```
$ cargo run --example temperature
🌡️ TCXC 61.97
🌡️ TC0P 52.25
🌡️ TM0P 45.62
🌡️ TC0E 61.53
🌡️ TC0F 64.47
🌡️ TC1C 58.00
🌡️ TC2C 60.00
🌡️ TCSA 60.00
🌡️ TCGC 60.00
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