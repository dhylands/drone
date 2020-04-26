use super::*;

/// Array of all supported devices.
pub const REGISTRY: &[Device] = &[
    Device {
        name: "stm32f100",
        target: "thumbv7m-none-eabi",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm3_r1p1",
            features: &["bit-band"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32f100",
            features: &["dma", "gpio", "spi", "tim"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32f100" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32f1x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 8_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32f101",
        target: "thumbv7m-none-eabi",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm3_r1p1",
            features: &["bit-band"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32f101",
            features: &["dma", "gpio", "spi", "tim"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32f101" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32f1x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 8_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32f102",
        target: "thumbv7m-none-eabi",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm3_r1p1",
            features: &["bit-band"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32f102",
            features: &["dma", "gpio", "spi", "tim"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32f102" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32f1x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 8_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32f103",
        target: "thumbv7m-none-eabi",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm3_r1p1",
            features: &["bit-band"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32f103",
            features: &["dma", "gpio", "spi", "tim"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32f103" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32f1x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 8_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32f107",
        target: "thumbv7m-none-eabi",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm3_r1p1",
            features: &["bit-band"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32f107",
            features: &["dma", "gpio", "spi", "tim"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32f107" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32f1x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 8_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32f401",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32f401",
            features: &["adc", "dma", "exti", "gpio", "tim"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32f401" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32f4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 16_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32f405",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32f405",
            features: &["adc", "dma", "exti", "gpio", "tim"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32f405" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32f4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 16_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32f407",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32f407",
            features: &["adc", "dma", "exti", "gpio", "tim"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32f407" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32f4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 16_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32f410",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32f410",
            features: &["adc", "dma", "exti", "gpio", "tim"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32f410" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32f4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 16_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32f411",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32f411",
            features: &["adc", "dma", "exti", "gpio", "tim"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32f411" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32f4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 16_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32f412",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32f412",
            features: &["adc", "dma", "exti", "gpio", "tim"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32f412" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32f4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 16_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32f413",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32f413",
            features: &["adc", "dma", "exti", "gpio", "tim"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32f413" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32f4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 16_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32f427",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32f427",
            features: &["adc", "dma", "exti", "gpio", "tim"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32f427" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32f4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 16_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32f429",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32f429",
            features: &["adc", "dma", "exti", "gpio", "tim"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32f429" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32f4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 16_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32f446",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32f446",
            features: &["adc", "dma", "exti", "gpio", "tim"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32f446" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32f4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 16_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32f469",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32f469",
            features: &["adc", "dma", "exti", "gpio", "tim"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32f469" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32f4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 16_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32l4x1",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32l4x1",
            features: &["dma", "exti", "gpio", "i2c", "rtc", "spi", "tim", "uart"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32l4x1" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32l4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 4_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32l4x2",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32l4x2",
            features: &["dma", "exti", "gpio", "i2c", "rtc", "spi", "tim", "uart"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32l4x2" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32l4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 4_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32l4x3",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32l4x3",
            features: &["dma", "exti", "gpio", "i2c", "rtc", "spi", "tim", "uart"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32l4x3" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32l4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 4_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32l4x5",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32l4x5",
            features: &["dma", "exti", "gpio", "i2c", "rtc", "spi", "tim", "uart"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32l4x5" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32l4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 4_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32l4x6",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32l4x6",
            features: &["dma", "exti", "gpio", "i2c", "rtc", "spi", "tim", "uart"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32l4x6" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32l4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 4_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32l4r5",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32l4r5",
            features: &["adc", "dma", "exti", "gpio", "i2c", "rtc", "spi", "tim", "uart"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32l4r5" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32l4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 4_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32l4s5",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32l4s5",
            features: &["adc", "dma", "exti", "gpio", "i2c", "rtc", "spi", "tim", "uart"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32l4s5" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32l4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 4_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32l4r7",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32l4r7",
            features: &["adc", "dma", "exti", "gpio", "i2c", "rtc", "spi", "tim", "uart"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32l4r7" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32l4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 4_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32l4s7",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32l4s7",
            features: &["adc", "dma", "exti", "gpio", "i2c", "rtc", "spi", "tim", "uart"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32l4s7" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32l4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 4_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32l4r9",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32l4r9",
            features: &["adc", "dma", "exti", "gpio", "i2c", "rtc", "spi", "tim", "uart"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32l4r9" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32l4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 4_000_000 }),
        log_dso: None,
    },
    Device {
        name: "stm32l4s9",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0800_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Stm32,
            flag: "stm32l4s9",
            features: &["adc", "dma", "exti", "gpio", "i2c", "rtc", "spi", "tim", "uart"],
        },
        probe_bmp: Some(ProbeBmp { device: "stm32l4s9" }),
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/stm32l4x.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 4_000_000 }),
        log_dso: None,
    },
    Device {
        name: "nrf52810",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0000_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Nrf,
            flag: "nrf52810",
            features: &[],
        },
        probe_bmp: None,
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/nrf52.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 32_000_000 }),
        log_dso: None,
    },
    Device {
        name: "nrf52811",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0000_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Nrf,
            flag: "nrf52811",
            features: &[],
        },
        probe_bmp: None,
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/nrf52.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 32_000_000 }),
        log_dso: None,
    },
    Device {
        name: "nrf52832",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0000_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Nrf,
            flag: "nrf52832",
            features: &[],
        },
        probe_bmp: None,
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/nrf52.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 32_000_000 }),
        log_dso: None,
    },
    Device {
        name: "nrf52840",
        target: "thumbv7em-none-eabihf",
        flash_origin: 0x0000_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm4f_r0p1",
            features: &["bit-band", "floating-point-unit", "memory-protection-unit"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Nrf,
            flag: "nrf52840",
            features: &[],
        },
        probe_bmp: None,
        probe_openocd: Some(ProbeOpenocd {
            arguments: &["-f", "interface/stlink.cfg", "-f", "target/nrf52.cfg"],
        }),
        probe_jlink: None,
        log_swo: Some(LogSwo { reset_freq: 32_000_000 }),
        log_dso: None,
    },
    Device {
        name: "nrf9160",
        target: "thumbv8m.main-none-eabihf",
        flash_origin: 0x0000_0000,
        ram_origin: 0x2000_0000,
        platform_crate: PlatformCrate {
            krate: crates::Platform::Cortexm,
            flag: "cortexm33f_r0p2",
            features: &["floating-point-unit", "memory-protection-unit", "security-extension"],
        },
        bindings_crate: BindingsCrate {
            krate: crates::Bindings::Nrf,
            flag: "nrf9160",
            features: &["uarte"],
        },
        probe_bmp: None,
        probe_openocd: None,
        probe_jlink: Some(ProbeJlink { device: "NRF9160" }),
        log_swo: None,
        log_dso: Some(LogDso { krate: crates::Dso::Nrf91, features: &[] }),
    },
];
