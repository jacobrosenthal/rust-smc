use crate::general::{translate, Kind, Subsystem};
use std::str::FromStr;
use strum::EnumProperty;
use strum_macros::{EnumIter, EnumProperty, EnumString};

//cant use display as then Custom's name cant be matched
#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(EnumIter, Debug, EnumProperty, EnumString)]
pub enum Key {
    //cant &str here sadly as we need a lifetime and EnumIter can't handle that case
    Custom(String, Kind, Subsystem),
    #[strum(props(Name = "TCXC", Kind = "Temperature", Subsystem = "Cpu", Detail = "PECI CPU"))]
    TCXC,
    #[strum(props(Name = "TC0P", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU 1 Proximity"))]
    TC0P,
    #[strum(props(Name = "TC0H", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU 1 Heatsink"))]
    TC0H,
    #[strum(props(Name = "TC0D", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU 1 Package"))]
    TC0D,
    #[strum(props(Name = "TC0E", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU 1"))]
    TC0E,
    #[strum(props(Name = "TC0F", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU 1"))]
    TC0F,
    #[strum(props(Name = "TC1C", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU Core 1"))]
    TC1C,
    #[strum(props(Name = "TC2C", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU Core 2"))]
    TC2C,
    #[strum(props(Name = "TC3C", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU Core 3"))]
    TC3C,
    #[strum(props(Name = "TC4C", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU Core 4"))]
    TC4C,
    #[strum(props(Name = "TC5C", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU Core 5"))]
    TC5C,
    #[strum(props(Name = "TC6C", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU Core 6"))]
    TC6C,
    #[strum(props(Name = "TC7C", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU Core 7"))]
    TC7C,
    #[strum(props(Name = "TC8C", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU Core 8"))]
    TC8C,
    #[strum(props(Name = "TCAH", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU 1 Heatsink Alt."))]
    TCAH,
    #[strum(props(Name = "TCAD", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU 1 Package Alt."))]
    TCAD,
    #[strum(props(Name = "TC1P", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU 2 Proximity"))]
    TC1P,
    #[strum(props(Name = "TC1H", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU 2 Heatsink"))]
    TC1H,
    #[strum(props(Name = "TC1D", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU 2 Package"))]
    TC1D,
    #[strum(props(Name = "TC1E", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU 2"))]
    TC1E,
    #[strum(props(Name = "TC1F", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU 2"))]
    TC1F,
    #[strum(props(Name = "TCBH", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU 2 Heatsink Alt."))]
    TCBH,
    #[strum(props(Name = "TCBD", Kind = "Temperature", Subsystem = "Cpu", Detail = "CPU 2 Heatsink Alt."))]
    TCBD,
    #[strum(props(Name = "TCSC", Kind = "Temperature", Subsystem = "Cpu", Detail = "Platform Environment Control Interface (PECI) System Agent" ))]
    TCSC,
    #[strum(props(Name = "TCSA", Kind = "Temperature", Subsystem = "Cpu", Detail = "Platform Environment Control Interface (PECI) System Agent" ))]
    TCSA,
    #[strum(props(Name = "TCGC", Kind = "Temperature", Subsystem = "Gpu", Detail = "Platform Environment Control Interface (PECI) GPU"))]
    TCGC,
    #[strum(props(Name = "TG0P", Kind = "Temperature", Subsystem = "Gpu", Detail = "GPU Proximity"))]
    TG0P,
    #[strum(props(Name = "TG0D", Kind = "Temperature", Subsystem = "Gpu", Detail = "GPU Die"))]
    TG0D,
    #[strum(props(Name = "TG1D", Kind = "Temperature", Subsystem = "Gpu", Detail = "GPU Die"))]
    TG1D,
    #[strum(props(Name = "TG0H", Kind = "Temperature", Subsystem = "Gpu", Detail = "GPU Heatsink"))]
    TG0H,
    #[strum(props(Name = "TG1H", Kind = "Temperature", Subsystem = "Gpu", Detail = "GPU Heatsink"))]
    TG1H,
    #[strum(props(Name = "Ts0S", Kind = "Temperature", Subsystem = "Memory", Detail = "Memory Proximity"))]
    Ts0S,
    #[strum(props(Name = "TM0P", Kind = "Temperature", Subsystem = "Memory", Detail = "Memory Slot Proximity"))]
    TM0P,
    #[strum(props(Name = "TM1P", Kind = "Temperature", Subsystem = "Memory", Detail = "Mem Bank A2"))]
    TM1P,
    #[strum(props(Name = "TM8P", Kind = "Temperature", Subsystem = "Memory", Detail = "Mem Bank B1"))]
    TM8P,
    #[strum(props(Name = "TM9P", Kind = "Temperature", Subsystem = "Memory", Detail = "Mem Bank B2"))]
    TM9P,
    #[strum(props(Name = "TM0S", Kind = "Temperature", Subsystem = "Memory", Detail = "Mem Module A1"))]
    TM0S,
    #[strum(props(Name = "TM1S", Kind = "Temperature", Subsystem = "Memory", Detail = "Mem Module A2"))]
    TM1S,
    #[strum(props(Name = "TM8S", Kind = "Temperature", Subsystem = "Memory", Detail = "Mem Module B1"))]
    TM8S,
    #[strum(props(Name = "TM9S", Kind = "Temperature", Subsystem = "Memory", Detail = "Mem Module B2"))]
    TM9S,
    #[strum(props(Name = "TN0D", Kind = "Temperature", Subsystem = "Memory", Detail = "Northbridge Die"))]
    TN0D,
    #[strum(props(Name = "TN0P", Kind = "Temperature", Subsystem = "Memory", Detail = "Northbridge Proximity 1"))]
    TN0P,
    #[strum(props(Name = "TN1P", Kind = "Temperature", Subsystem = "Memory", Detail = "Northbridge Proximity 2"))]
    TN1P,
    #[strum(props(Name = "TN0C", Kind = "Temperature", Subsystem = "Memory", Detail = "MCH Die"))]
    TN0C,
    #[strum(props(Name = "TN0H", Kind = "Temperature", Subsystem = "Memory", Detail = "MCH Heatsink"))]
    TN0H,
    #[strum(props(Name = "TP0D", Kind = "Temperature", Subsystem = "Memory", Detail = "Platform Controller Hub Die"))]
    TP0D,
    #[strum(props(Name = "TPCD", Kind = "Temperature", Subsystem = "Memory", Detail = "Platform Controller Hub Die"))]
    TPCD,
    #[strum(props(Name = "Tp0P", Kind = "Temperature", Subsystem = "Memory", Detail = "Platform Controller Hub Proximity"))]
    Tp0P,
    #[strum(props(Name = "TA0P", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Airflow 1"))]
    TA0P,
    #[strum(props(Name = "TA1P", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Airflow 2"))]
    TA1P,
    #[strum(props(Name = "Th0H", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Heatpipe 1"))]
    Th0H,
    #[strum(props(Name = "Th1H", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Heatpipe 2"))]
    Th1H,
    #[strum(props(Name = "Th2H", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Heatpipe 3"))]
    Th2H,
    #[strum(props(Name = "Tm0P", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Mainboard Proximity"))]
    Tm0P,
    #[strum(props(Name = "Ts0P", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Palm Rest L"))]
    Ts0P,
    #[strum(props(Name = "Tb0P", Kind = "Temperature", Subsystem = "Mainboard", Detail = "BLC Proximity"))]
    Tb0P,
    #[strum(props(Name = "TL0P", Kind = "Temperature", Subsystem = "Mainboard", Detail = "LCD Proximity"))]
    TL0P,
    #[strum(props(Name = "TL0P", Kind = "Temperature", Subsystem = "Mainboard", Detail = "LCD Proximity"))]
    TW0P,
    #[strum(props(Name = "TH0P", Kind = "Temperature", Subsystem = "Mainboard", Detail = "HDD Bay 1"))]
    TH0P,
    #[strum(props(Name = "TH1P", Kind = "Temperature", Subsystem = "Mainboard", Detail = "HDD Bay 2"))]
    TH1P,
    #[strum(props(Name = "TH2P", Kind = "Temperature", Subsystem = "Mainboard", Detail = "HDD Bay 3"))]
    TH2P,
    #[strum(props(Name = "TH3P", Kind = "Temperature", Subsystem = "Mainboard", Detail = "HDD Bay 4"))]
    TH3P,
    #[strum(props(Name = "TO0P", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Optical Drive"))]
    TO0P,
    #[strum(props(Name = "TO0P", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Battery TS_MAX"))]
    TB0T,
    #[strum(props(Name = "TB1T", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Battery 1"))]
    TB1T,
    #[strum(props(Name = "TB2T", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Battery 2"))]
    TB2T,
    #[strum(props(Name = "TB3T", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Battery"))]
    TB3T,
    #[strum(props(Name = "Tp0C", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Power Supply 1 Alt."))]
    Tp0C,
    #[strum(props(Name = "Tp1P", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Power Supply 2"))]
    Tp1P,
    #[strum(props(Name = "Tp1C", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Power Supply 2 Alt."))]
    Tp1C,
    #[strum(props(Name = "Tp2P", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Power Supply 3"))]
    Tp2P,
    #[strum(props(Name = "Tp3P", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Power Supply 4"))]
    Tp3P,
    #[strum(props(Name = "Tp4P", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Power Supply 5"))]
    Tp4P,
    #[strum(props(Name = "Tp5P", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Power Supply 6"))]
    Tp5P,
    #[strum(props(Name = "TS0C", Kind = "Temperature", Subsystem = "Mainboard", Detail = "Expansion Slots"))]
    TS0C,
    #[strum(props(Name = "TA0S", Kind = "Temperature", Subsystem = "Mainboard", Detail = "PCI Slot 1 Pos 1"))]
    TA0S,
    #[strum(props(Name = "TA1S", Kind = "Temperature", Subsystem = "Mainboard", Detail = "PCI Slot 1 Pos 2"))]
    TA1S,
    #[strum(props(Name = "TA2S", Kind = "Temperature", Subsystem = "Mainboard", Detail = "PCI Slot 2 Pos 1"))]
    TA2S,
    #[strum(props(Name = "TA3S", Kind = "Temperature", Subsystem = "Mainboard", Detail = "PCI Slot 2 Pos 2"))]
    TA3S,
    #[strum(props(Name = "VC0C", Kind = "Voltage", Subsystem = "Cpu", Detail = "CPU Core 1"))]
    VC0C,
    #[strum(props(Name = "VC1C", Kind = "Voltage", Subsystem = "Cpu", Detail = "CPU Core 2"))]
    VC1C,
    #[strum(props(Name = "VC2C", Kind = "Voltage", Subsystem = "Cpu", Detail = "CPU Core 3"))]
    VC2C,
    #[strum(props(Name = "VC3C", Kind = "Voltage", Subsystem = "Cpu", Detail = "CPU Core 4"))]
    VC3C,
    #[strum(props(Name = "VC4C", Kind = "Voltage", Subsystem = "Cpu", Detail = "CPU Core 5"))]
    VC4C,
    #[strum(props(Name = "VC5C", Kind = "Voltage", Subsystem = "Cpu", Detail = "CPU Core 6"))]
    VC5C,
    #[strum(props(Name = "VC6C", Kind = "Voltage", Subsystem = "Cpu", Detail = "CPU Core 7"))]
    VC6C,
    #[strum(props(Name = "VC7C", Kind = "Voltage", Subsystem = "Cpu", Detail = "CPU Core 8"))]
    VC7C,
    #[strum(props(Name = "VV1R", Kind = "Voltage", Subsystem = "Cpu", Detail = "CPU VTT"))]
    VV1R,
    #[strum(props(Name = "VG0C", Kind = "Voltage", Subsystem = "Gpu", Detail = "GPU Core"))]
    VG0C,
    #[strum(props(Name = "VM0R", Kind = "Voltage", Subsystem = "Memory", Detail = "Memory"))]
    VM0R,
    #[strum(props(Name = "VN1R", Kind = "Voltage", Subsystem = "Memory", Detail = "Platform Controller Hub"))]
    VN1R,
    #[strum(props(Name = "VN0C", Kind = "Voltage", Subsystem = "Memory", Detail = "MCH"))]
    VN0C,
    #[strum(props(Name = "VD0R", Kind = "Voltage", Subsystem = "Mainboard", Detail = "Mainboard S0 Rail"))]
    VD0R,
    #[strum(props(Name = "VD5R", Kind = "Voltage", Subsystem = "Memory", Detail = "Mainboard S5 Rail"))]
    VD5R,
    #[strum(props(Name = "VP0R", Kind = "Voltage", Subsystem = "Memory", Detail = "12V Rail"))]
    VP0R,
    #[strum(props(Name = "Vp0C", Kind = "Voltage", Subsystem = "Memory", Detail = "12V Vcc"))]
    Vp0C,
    #[strum(props(Name = "VV2S", Kind = "Voltage", Subsystem = "Memory", Detail = "Main 3V"))]
    VV2S,
    #[strum(props(Name = "VR3R", Kind = "Voltage", Subsystem = "Memory", Detail = "Main 3.3V"))]
    VR3R,
    #[strum(props(Name = "VV1S", Kind = "Voltage", Subsystem = "Memory", Detail = "Main 5V"))]
    VV1S,
    #[strum(props(Name = "VH05", Kind = "Voltage", Subsystem = "Memory", Detail = "Main 5V"))]
    VH05,
    #[strum(props(Name = "VV9S", Kind = "Voltage", Subsystem = "Memory", Detail = "Main 12V"))]
    VV9S,
    #[strum(props(Name = "VD2R", Kind = "Voltage", Subsystem = "Memory", Detail = "Main 12V"))]
    VD2R,
    #[strum(props(Name = "VV7S", Kind = "Voltage", Subsystem = "Memory", Detail = "Auxiliary 3V"))]
    VV7S,
    #[strum(props(Name = "VV3S", Kind = "Voltage", Subsystem = "Memory", Detail = "Standby 3V"))]
    VV3S,
    #[strum(props(Name = "VV8S", Kind = "Voltage", Subsystem = "Memory", Detail = "Standby 5V"))]
    VV8S,
    #[strum(props(Name = "VeES", Kind = "Voltage", Subsystem = "Memory", Detail = "PCIe 12V"))]
    VeES,
    #[strum(props(Name = "VBAT", Kind = "Voltage", Subsystem = "Memory", Detail = "Battery"))]
    VBAT,
    #[strum(props(Name = "Vb0R", Kind = "Voltage", Subsystem = "Memory", Detail = "CMOS Battery"))]
    Vb0R,
    #[strum(props(Name = "IC0C", Kind = "Voltage", Subsystem = "Cpu", Detail = "CPU Core"))]
    IC0C,
    #[strum(props(Name = "IC1C", Kind = "Current", Subsystem = "Cpu", Detail = "CPU VccIO"))]
    IC1C,
    #[strum(props(Name = "IC2C", Kind = "Current", Subsystem = "Cpu", Detail = "CPU VccSA"))]
    IC2C,
    #[strum(props(Name = "IC0R", Kind = "Current", Subsystem = "Cpu", Detail = "CPU Rail"))]
    IC0R,
    #[strum(props(Name = "IC5R", Kind = "Current", Subsystem = "Cpu", Detail = "CPU DRAM"))]
    IC5R,
    #[strum(props(Name = "IC8R", Kind = "Current", Subsystem = "Cpu", Detail = "CPU PLL"))]
    IC8R,
    #[strum(props(Name = "IC0G", Kind = "Current", Subsystem = "Cpu", Detail = "CPU GFX"))]
    IC0G,
    #[strum(props(Name = "IC0M", Kind = "Current", Subsystem = "Cpu", Detail = "CPU Memory"))]
    IC0M,
    #[strum(props(Name = "IG0C", Kind = "Current", Subsystem = "Gpu", Detail = "GPU Rail"))]
    IG0C,
    #[strum(props(Name = "IM0C", Kind = "Current", Subsystem = "Memory", Detail = "Memory Controller"))]
    IM0C,
    #[strum(props(Name = "IM0R", Kind = "Current", Subsystem = "Memory", Detail = "Memory Rail"))]
    IM0R,
    #[strum(props(Name = "IN0C", Kind = "Current", Subsystem = "Memory", Detail = "MCH"))]
    IN0C,
    #[strum(props(Name = "ID0R", Kind = "Current", Subsystem = "Memory", Detail = "Mainboard S0 Rail"))]
    ID0R,
    #[strum(props(Name = "ID5R", Kind = "Current", Subsystem = "Memory", Detail = "Mainboard S5 Rail"))]
    ID5R,
    #[strum(props(Name = "IO0R", Kind = "Current", Subsystem = "Memory", Detail = "Misc. Rail"))]
    IO0R,
    #[strum(props(Name = "IB0R", Kind = "Current", Subsystem = "Memory", Detail = "Battery Rail"))]
    IB0R,
    #[strum(props(Name = "IPBR", Kind = "Current", Subsystem = "Memory", Detail = "Charger BMON"))]
    IPBR,
    #[strum(props(Name = "PC0C", Kind = "Power", Subsystem = "Cpu", Detail = "CPU Core 1"))]
    PC0C,
    #[strum(props(Name = "PC1C", Kind = "Power", Subsystem = "Cpu", Detail = "CPU Core 2"))]
    PC1C,
    #[strum(props(Name = "PC2C", Kind = "Power", Subsystem = "Cpu", Detail = "CPU Core 3"))]
    PC2C,
    #[strum(props(Name = "PC3C", Kind = "Power", Subsystem = "Cpu", Detail = "CPU Core 4"))]
    PC3C,
    #[strum(props(Name = "PC4C", Kind = "Power", Subsystem = "Cpu", Detail = "CPU Core 5"))]
    PC4C,
    #[strum(props(Name = "PC5C", Kind = "Power", Subsystem = "Cpu", Detail = "CPU Core 6"))]
    PC5C,
    #[strum(props(Name = "PC6C", Kind = "Power", Subsystem = "Cpu", Detail = "CPU Core 7"))]
    PC6C,
    #[strum(props(Name = "PC7C", Kind = "Power", Subsystem = "Cpu", Detail = "CPU Core 8"))]
    PC7C,
    #[strum(props(Name = "PCPC", Kind = "Power", Subsystem = "Cpu", Detail = "CPU Cores"))]
    PCPC,
    #[strum(props(Name = "PCPG", Kind = "Power", Subsystem = "Cpu", Detail = "CPU GFX"))]
    PCPG,
    #[strum(props(Name = "PCPD", Kind = "Power", Subsystem = "Cpu", Detail = "CPU DRAM"))]
    PCPD,
    #[strum(props(Name = "PCTR", Kind = "Power", Subsystem = "Cpu", Detail = "CPU Total"))]
    PCTR,
    #[strum(props(Name = "PCPL", Kind = "Power", Subsystem = "Cpu", Detail = "CPU Total"))]
    PCPL,
    #[strum(props(Name = "PC1R", Kind = "Power", Subsystem = "Cpu", Detail = "CPU Rail"))]
    PC1R,
    #[strum(props(Name = "PC5R", Kind = "Power", Subsystem = "Cpu", Detail = "CPU S0 Rail"))]
    PC5R,
    #[strum(props(Name = "PGTR", Kind = "Power", Subsystem = "Gpu", Detail = "GPU Total"))]
    PGTR,
    #[strum(props(Name = "PG0R", Kind = "Power", Subsystem = "Gpu", Detail = "GPU Rail"))]
    PG0R,
    #[strum(props(Name = "PM0R", Kind = "Power", Subsystem = "Memory", Detail = "Memory Rail"))]
    PM0R,
    #[strum(props(Name = "PN0C", Kind = "Power", Subsystem = "Memory", Detail = "MCH"))]
    PN0C,
    #[strum(props(Name = "PN1R", Kind = "Power", Subsystem = "Memory", Detail = "Platform Controller Hub Rail"))]
    PN1R,
    #[strum(props(Name = "PC0R", Kind = "Power", Subsystem = "Mainboard", Detail = "Mainboard S0 Rail"))]
    PC0R,
    #[strum(props(Name = "PD0R", Kind = "Power", Subsystem = "Mainboard", Detail = "Mainboard S0 Rail"))]
    PD0R,
    #[strum(props(Name = "PD5R", Kind = "Power", Subsystem = "Mainboard", Detail = "Mainboard S5 Rail"))]
    PD5R,
    #[strum(props(Name = "PH02", Kind = "Power", Subsystem = "Mainboard", Detail = "Main 3.3V Rail"))]
    PH02,
    #[strum(props(Name = "PH05", Kind = "Power", Subsystem = "Mainboard", Detail = "Main 5V Rail"))]
    PH05,
    #[strum(props(Name = "Pp0R", Kind = "Power", Subsystem = "Mainboard", Detail = "12V Rail"))]
    Pp0R,
    #[strum(props(Name = "PD2R", Kind = "Power", Subsystem = "Mainboard", Detail = "Main 12V Rail"))]
    PD2R,
    #[strum(props(Name = "PO0R", Kind = "Power", Subsystem = "Mainboard", Detail = "Misc. Rail"))]
    PO0R,
    #[strum(props(Name = "PBLC", Kind = "Power", Subsystem = "Mainboard", Detail = "Battery Rail"))]
    PBLC,
    #[strum(props(Name = "PB0R", Kind = "Power", Subsystem = "Mainboard", Detail = "Battery Rail"))]
    PB0R,
    #[strum(props(Name = "PDTR", Kind = "Power", Subsystem = "Mainboard", Detail = "DC In Total"))]
    PDTR,
    #[strum(props(Name = "PSTR", Kind = "Power", Subsystem = "Mainboard", Detail = "System Total"))]
    PSTR,
    #[strum(props(Name = "FNum", Kind = "Fan", Subsystem = "Unknown", Detail = "Total Fans"))]
    FNum,
    #[strum(props(Name = "F0Ac", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 1 RPM"))]
    F0Ac,
    #[strum(props(Name = "F1Ac", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 2 RPM"))]
    F1Ac,
    #[strum(props(Name = "F2Ac", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 3 RPM"))]
    F2Ac,
    #[strum(props(Name = "F3Ac", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 4 RPM"))]
    F3Ac,
    #[strum(props(Name = "F4Ac", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 5 RPM"))]
    F4Ac,
    #[strum(props(Name = "F5Ac", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 6 RPM"))]
    F5Ac,
    #[strum(props(Name = "F0Mn", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 1 Min RPM"))]
    F0Mn,
    #[strum(props(Name = "F1Mn", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 2 Min RPM"))]
    F1Mn,
    #[strum(props(Name = "F2Mn", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 3 Min RPM"))]
    F2Mn,
    #[strum(props(Name = "F3Mn", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 4 Min RPM"))]
    F3Mn,
    #[strum(props(Name = "F4Mn", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 5 Min RPM"))]
    F4Mn,
    #[strum(props(Name = "F5Mn", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 6 Min RPM"))]
    F5Mn,
    #[strum(props(Name = "F0Mx", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 1 Max RPM"))]
    F0Mx,
    #[strum(props(Name = "F1Mx", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 2 Max RPM"))]
    F1Mx,
    #[strum(props(Name = "F2Mx", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 3 Max RPM"))]
    F2Mx,
    #[strum(props(Name = "F3Mx", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 4 Max RPM"))]
    F3Mx,
    #[strum(props(Name = "F4Mx", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 5 Max RPM"))]
    F4Mx,
    #[strum(props(Name = "F5Mx", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 6 Max RPM"))]
    F5Mx,
    #[strum(props(Name = "F0Sf", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 1 Safe RPM"))]
    F0Sf,
    #[strum(props(Name = "F1Sf", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 2 Safe RPM"))]
    F1Sf,
    #[strum(props(Name = "F2Sf", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 3 Safe RPM"))]
    F2Sf,
    #[strum(props(Name = "F3Sf", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 4 Safe RPM"))]
    F3Sf,
    #[strum(props(Name = "F4Sf", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 5 Safe RPM"))]
    F4Sf,
    #[strum(props(Name = "F5Sf", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 6 Safe RPM"))]
    F5Sf,
    #[strum(props(Name = "F0Tg", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 1 Target RPM"))]
    F0Tg,
    #[strum(props(Name = "F1Tg", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 2 Target RPM"))]
    F1Tg,
    #[strum(props(Name = "F2Tg", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 3 Target RPM"))]
    F2Tg,
    #[strum(props(Name = "F3Tg", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 4 Target RPM"))]
    F3Tg,
    #[strum(props(Name = "F4Tg", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 5 Target RPM"))]
    F4Tg,
    #[strum(props(Name = "F5Tg", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 6 Target RPM"))]
    F5Tg,
    #[strum(props(Name = "FS! ", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan Mode"))]
    FS,
    #[strum(props(Name = "F0Md", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 1 Mode"))]
    F0Md,
    #[strum(props(Name = "F1Md", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 2 Mode"))]
    F1Md,
    #[strum(props(Name = "F2Md", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 3 Mode"))]
    F2Md,
    #[strum(props(Name = "F3Md", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 4 Mode"))]
    F3Md,
    #[strum(props(Name = "F4Md", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 5 Mode"))]
    F4Md,
    #[strum(props(Name = "F5Md", Kind = "Fan", Subsystem = "Unknown", Detail = "Fan 6 Mode"))]
    F5Md,
}

impl Key {
    pub fn name(&self) -> &str {
        match self {
            Key::Custom(name, _kind, _subsystem) => {
                // assert_eq!(4, name.len());
                name
            }
            //explicit fail if someone fat fingers strum strings
            _ => self.get_str("Name").unwrap(),
        }
    }

    pub fn detail(&self) -> &str {
        match self {
            //allow no Detail column
            _ => self.get_str("Detail").unwrap_or_else(|| ""),
        }
    }

    pub fn value(&self) -> u32 {
        //Custom's default name becomes ""
        match self.name().len() {
            4 => translate(&self.name()),
            _ => 0,
        }
    }

    pub fn kind(&self) -> Kind {
        match self {
            Key::Custom(_name, kind, _subsystem) => kind.clone(),
            _ => {
                //explicit fail if someone fat fingers strum strings
                let kind = self.get_str("Kind").unwrap();
                Kind::from_str(kind).unwrap()
            }
        }
    }

    pub fn subsystem(&self) -> Subsystem {
        match self {
            Key::Custom(_name, _kind, subsystem) => subsystem.clone(),
            _ => {
                //explicit fail if someone fat fingers strum strings
                let subsystem = self.get_str("Subsystem").unwrap();
                Subsystem::from_str(subsystem).unwrap()
            }
        }
    }
}
