# smc

>The SMC has roles in controlling thermal and power management, battery charging, video mode switching, sleep and wake, hibernation, and LED indicators. It also enables enforcement of the macOS End User License, allowing it to identify when it is running on non-Apple hardware.[3] 
https://en.wikipedia.org/wiki/System_Management_Controller

This tool is mainly implemented for reading values vs the presumably more dangerous writing values which you do at your own risk.

## [Documentation](https://docs.rs/smc)

# usage
```
    let smc = Smc::new().unwrap();
    let sensor = smc.get_sensor_by_name("TCXC").unwrap();
    let value = sensor.read().unwrap()
```

You can also request an iterator of all sensors on the system which you can filter by Kind. Note, sensors have nontrivial newup cost so you may wish to keep these around to read them a second time intead of creating them again.
```
    let sensors = smc
        .iter()
        .filter(|sensor| sensor.kind() == Kind::Temperature);
```

There may be undocumented sensors on your device, for which the fuzz example can be fun:
```
$ cargo run --example fuzz
found KPPW
found KPST 0
found OSK1 0
found CRDP 0
```

# props
This library is a direct descendant of the original reverse engineering work of devnull, currently embodied in the istats project at https://github.com/Chris911/iStats/tree/0b86af356baa680cabc5665cc0364de29f1f5958/ext/osx_stats and the fuzzing idea came from stumbling on https://github.com/theopolis/smc-fuzzer
