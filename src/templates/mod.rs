//! Templates registry.

pub mod helpers;

use crate::{
    devices::Device,
    probe::{Log, Probe},
    utils::{ser_to_string, temp_dir},
};
use anyhow::Result;
use drone_config::Config;
use handlebars::Handlebars;
use serde_json::json;
use std::{collections::BTreeSet, error::Error, fs::File, path::Path};
use tempfile::NamedTempFile;

/// Templates registry.
pub struct Registry<'reg>(Handlebars<'reg>);

impl Registry<'_> {
    /// Creates a new templates registry.
    pub fn new() -> Result<Self> {
        let mut handlebars = Handlebars::new();
        macro_rules! template {
            ($path:expr) => {
                handlebars.register_template_string($path, include_str!(concat!($path, ".hbs")))
            };
        }

        template!("layout.ld")?;
        template!("new/src-cortexm/bin.rs")?;
        template!("new/src-cortexm/lib.rs")?;
        template!("new/src-cortexm/thr.rs")?;
        template!("new/src-cortexm/tasks/mod.rs")?;
        template!("new/src-cortexm/tasks/root.rs")?;
        template!("new/Cargo.toml")?;
        template!("new/Drone.toml")?;
        template!("new/Justfile")?;
        template!("new/rust-toolchain")?;
        template!("new/_cargo/config")?;
        template!("new/_gitignore")?;
        template!("bmp/flash.gdb")?;
        template!("bmp/gdb.gdb")?;
        template!("bmp/reset.gdb")?;
        template!("bmp/swo.gdb")?;
        template!("bmp/target.gdb")?;
        template!("bmp/target/cortexm.gdb")?;
        template!("bmp/target/stm32.gdb")?;
        template!("jlink/reset.jlink")?;
        template!("jlink/flash.jlink")?;
        template!("jlink/gdb.gdb")?;
        template!("jlink/dso.gdb")?;
        template!("openocd/flash.openocd")?;
        template!("openocd/gdb.gdb")?;
        template!("openocd/gdb.openocd")?;
        template!("openocd/reset.openocd")?;
        template!("openocd/swo.gdb")?;

        helpers::register(&mut handlebars);
        Ok(Self(handlebars))
    }

    /// Renders linker script.
    pub fn layout_ld(&self, config: &Config, prelink: bool) -> Result<NamedTempFile> {
        let data = json!({ "config": config, "prelink": prelink });
        helpers::clear_vars();
        named_temp_file(|file| self.0.render_to_write("layout.ld", &data, file))
    }

    /// Renders cortexm `src/bin.rs`.
    pub fn new_src_cortexm_bin_rs(&self, crate_name: &str) -> Result<String> {
        let data = json!({ "crate_name": crate_name });
        helpers::clear_vars();
        Ok(self.0.render("new/src-cortexm/bin.rs", &data)?)
    }

    /// Renders cortexm `src/lib.rs`.
    pub fn new_src_cortexm_lib_rs(&self, device: &Device, log: Log) -> Result<String> {
        let dso_regs = device.log_dso.as_ref().map(|x| {
            x.krate
                .used_regs()
                .iter()
                .map(|reg| format!("!{};", reg))
                .fold::<Vec<(usize, Vec<_>)>, _>(Vec::new(), |mut acc, x| {
                    if let Some((len, line)) = acc.last_mut() {
                        if *len + x.len() < 96 {
                            *len += x.len() + 1;
                            line.push(x);
                            return acc;
                        }
                    }
                    acc.push((x.len(), vec![x]));
                    acc
                })
                .iter()
                .map(|(_, line)| format!("    {}", line.join(" ")))
                .collect::<Vec<_>>()
                .join("\n")
        });
        let data = json!({
            "bindings_name": device.bindings_crate.krate.name(),
            "log_ident": ser_to_string(log),
            "dso_name": device.log_dso.as_ref().map(|x| x.krate.name()),
            "dso_regs": dso_regs,
        });
        helpers::clear_vars();
        Ok(self.0.render("new/src-cortexm/lib.rs", &data)?)
    }

    /// Renders cortexm `src/thr.rs`.
    pub fn new_src_cortexm_thr_rs(&self, device: &Device) -> Result<String> {
        let data = json!({ "bindings_name": device.bindings_crate.krate.name() });
        helpers::clear_vars();
        Ok(self.0.render("new/src-cortexm/thr.rs", &data)?)
    }

    /// Renders cortexm `src/tasks/mod.rs`.
    pub fn new_src_cortexm_tasks_mod_rs(&self) -> Result<String> {
        helpers::clear_vars();
        Ok(self.0.render("new/src-cortexm/tasks/mod.rs", &())?)
    }

    /// Renders cortexm `src/tasks/root.rs`.
    pub fn new_src_cortexm_tasks_root_rs(&self) -> Result<String> {
        helpers::clear_vars();
        Ok(self.0.render("new/src-cortexm/tasks/root.rs", &())?)
    }

    /// Renders `Cargo.toml`.
    pub fn new_cargo_toml(&self, device: &Device, crate_name: &str) -> Result<String> {
        let data = json!({
            "crate_name": crate_name,
            "platform_name": device.platform_crate.krate.name(),
            "bindings_name": device.bindings_crate.krate.name(),
            "platform_features": device.platform_crate.features,
            "bindings_features": device.bindings_crate.features,
            "dso_name": device.log_dso.as_ref().map(|x| x.krate.name()),
            "dso_features": device.log_dso.as_ref().map(|x| x.features),
        });
        helpers::clear_vars();
        Ok(self.0.render("new/Cargo.toml", &data)?)
    }

    /// Renders `Drone.toml`.
    pub fn new_drone_toml(
        &self,
        device: &Device,
        flash_size: u32,
        ram_size: u32,
        heap: &str,
        probe: Probe,
        log: Log,
    ) -> Result<String> {
        let data = json!({
            "device_flash_size": flash_size,
            "device_flash_origin": device.flash_origin,
            "device_ram_size": ram_size,
            "device_ram_origin": device.ram_origin,
            "heap": heap.trim_end(),
            "probe_ident": ser_to_string(probe),
            "probe_bmp_device": device.probe_bmp.as_ref().map(|x| x.device),
            "probe_openocd_arguments": device.probe_openocd.as_ref().map(|x| x.arguments),
            "probe_jlink_device": device.probe_jlink.as_ref().map(|x| x.device),
            "log_ident": ser_to_string(log),
            "log_swo_reset_freq": device.log_swo.as_ref().map(|x| x.reset_freq),
        });
        helpers::clear_vars();
        Ok(self.0.render("new/Drone.toml", &data)?)
    }

    /// Renders `Justfile`.
    pub fn new_justfile(&self, device: &Device) -> Result<String> {
        let data = json!({
            "device_target": device.target,
            "platform_flag_name": device.platform_crate.krate.flag_name(),
            "bindings_flag_name": device.bindings_crate.krate.flag_name(),
            "platform_flag": device.platform_crate.flag,
            "bindings_flag": device.bindings_crate.flag,
        });
        helpers::clear_vars();
        Ok(self.0.render("new/Justfile", &data)?)
    }

    /// Renders `rust-toolchain`.
    pub fn new_rust_toolchain(&self, toolchain: &str) -> Result<String> {
        let data = json!({ "toolchain": toolchain });
        helpers::clear_vars();
        Ok(self.0.render("new/rust-toolchain", &data)?)
    }

    /// Renders `.cargo/config`.
    pub fn new_cargo_config(&self) -> Result<String> {
        helpers::clear_vars();
        Ok(self.0.render("new/_cargo/config", &())?)
    }

    /// Renders `.gitignore`.
    pub fn new_gitignore(&self) -> Result<String> {
        helpers::clear_vars();
        Ok(self.0.render("new/_gitignore", &())?)
    }

    /// Renders BMP `reset` command script.
    pub fn bmp_reset(&self, config: &Config) -> Result<NamedTempFile> {
        let data = json!({ "config": config });
        helpers::clear_vars();
        named_temp_file(|file| self.0.render_to_write("bmp/reset.gdb", &data, file))
    }

    /// Renders BMP `flash` command script.
    pub fn bmp_flash(&self, config: &Config) -> Result<NamedTempFile> {
        let data = json!({ "config": config });
        helpers::clear_vars();
        named_temp_file(|file| self.0.render_to_write("bmp/flash.gdb", &data, file))
    }

    /// Renders BMP `gdb` command script.
    pub fn bmp_gdb(
        &self,
        config: &Config,
        reset: bool,
        rustc_substitute_path: &str,
    ) -> Result<NamedTempFile> {
        let data = json!({
            "config": config,
            "reset": reset,
            "rustc-substitute-path": rustc_substitute_path,
        });
        helpers::clear_vars();
        named_temp_file(|file| self.0.render_to_write("bmp/gdb.gdb", &data, file))
    }

    /// Renders BMP `swo` command script.
    pub fn bmp_swo(
        &self,
        config: &Config,
        ports: &BTreeSet<u32>,
        reset: bool,
        pipe: &Path,
    ) -> Result<NamedTempFile> {
        let data = json!({
            "config": config,
            "ports": ports,
            "reset": reset,
            "pipe": pipe,
        });
        helpers::clear_vars();
        named_temp_file(|file| self.0.render_to_write("bmp/swo.gdb", &data, file))
    }

    /// Renders J-Link `reset` command script.
    pub fn jlink_reset(&self) -> Result<NamedTempFile> {
        helpers::clear_vars();
        named_temp_file(|file| self.0.render_to_write("jlink/reset.jlink", &(), file))
    }

    /// Renders J-Link `flash` command script.
    pub fn jlink_flash(&self, firmware: &Path) -> Result<NamedTempFile> {
        let data = json!({ "firmware": firmware });
        helpers::clear_vars();
        named_temp_file(|file| self.0.render_to_write("jlink/flash.jlink", &data, file))
    }

    /// Renders J-Link `gdb` command script.
    pub fn jlink_gdb(
        &self,
        config: &Config,
        reset: bool,
        rustc_substitute_path: &str,
    ) -> Result<NamedTempFile> {
        let data = json!({
            "config": config,
            "reset": reset,
            "rustc-substitute-path": rustc_substitute_path,
        });
        helpers::clear_vars();
        named_temp_file(|file| self.0.render_to_write("jlink/gdb.gdb", &data, file))
    }

    /// Renders J-Link `dso` command script.
    pub fn jlink_dso(
        &self,
        config: &Config,
        ports: &BTreeSet<u32>,
        reset: bool,
        pipe: &Path,
    ) -> Result<NamedTempFile> {
        let data = json!({
            "config": config,
            "ports": ports,
            "reset": reset,
            "pipe": pipe,
        });
        helpers::clear_vars();
        named_temp_file(|file| self.0.render_to_write("jlink/dso.gdb", &data, file))
    }

    /// Renders OpenOCD `reset` command script.
    pub fn openocd_reset(&self) -> Result<String> {
        helpers::clear_vars();
        Ok(self.0.render("openocd/reset.openocd", &())?)
    }

    /// Renders OpenOCD `flash` command script.
    pub fn openocd_flash(&self, firmware: &Path) -> Result<String> {
        let data = json!({ "firmware": firmware });
        helpers::clear_vars();
        Ok(self.0.render("openocd/flash.openocd", &data)?)
    }

    /// Renders OpenOCD `gdb` command GDB script.
    pub fn openocd_gdb_gdb(
        &self,
        config: &Config,
        reset: bool,
        rustc_substitute_path: &str,
    ) -> Result<NamedTempFile> {
        let data = json!({
            "config": config,
            "reset": reset,
            "rustc-substitute-path": rustc_substitute_path,
        });
        helpers::clear_vars();
        named_temp_file(|file| self.0.render_to_write("openocd/gdb.gdb", &data, file))
    }

    /// Renders OpenOCD `gdb` command OpenOCD script.
    pub fn openocd_gdb_openocd(&self, config: &Config) -> Result<String> {
        let data = json!({ "config": config });
        helpers::clear_vars();
        Ok(self.0.render("openocd/gdb.openocd", &data)?)
    }

    /// Renders OpenOCD `swo` command script.
    pub fn openocd_swo(
        &self,
        config: &Config,
        ports: &BTreeSet<u32>,
        reset: bool,
        pipe: &Path,
        output: Option<&Path>,
    ) -> Result<NamedTempFile> {
        let data = json!({
            "config": config,
            "ports": ports,
            "reset": reset,
            "pipe": pipe,
            "output": output,
        });
        helpers::clear_vars();
        named_temp_file(|file| self.0.render_to_write("openocd/swo.gdb", &data, file))
    }
}

fn named_temp_file<F, E>(f: F) -> Result<NamedTempFile>
where
    F: FnOnce(&File) -> Result<(), E>,
    E: Error + Send + Sync + 'static,
{
    let temp_file = NamedTempFile::new_in(temp_dir())?;
    f(temp_file.as_file())?;
    Ok(temp_file)
}
