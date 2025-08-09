#[cfg(not(feature = "build_bindings"))]
fn main() {
    println!("cargo:rerun-if-changed=build.rs"); // never rerun
}

#[cfg(feature = "build_bindings")]
fn main() {
    println!("cargo:rerun-if-changed=build.rs"); // avoids double-build when we output into src
    generate::generate().unwrap();
}

#[cfg(feature = "build_bindings")]
mod generate {
    use std::error::Error;
    use std::io::Write;
    use std::process::{Command, Stdio};

    use regex::Regex;
    use std::env;
    use std::fs::File;
    use std::path::{Path, PathBuf};

    const CONST_PREFIX: &str = "DRM_FOURCC_";

    pub fn get_header_include_paths() -> Vec<PathBuf> {
        let library = pkg_config::Config::new()
            .probe("libdrm")
            .expect("Failed to find libdrm");

        library.include_paths
    }

    pub fn generate() -> Result<(), Box<dyn Error + Sync + Send>> {
        let extra_includes = get_header_include_paths();
        let out_dir = env::var("OUT_DIR").unwrap();
        let wrapper_path = Path::new(&out_dir).join("wrapper.h");

        // First get all the macros in drm_fourcc.h
        let mut cmd = Command::new("clang");
        cmd.arg("-E") // run pre-processor only
            .arg("-dM"); // output all macros defined

        for include_path in &extra_includes {
            cmd.arg("-I").arg(include_path);
        }

        let mut cmd = cmd
            .arg("-") // take input from stdin
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        {
            let stdin = cmd.stdin.as_mut().expect("failed to open stdin");
            stdin.write_all(b"#include <drm_fourcc.h>\n")?;
        }

        let result = cmd.wait_with_output()?;
        let stdout = String::from_utf8(result.stdout)?;
        if !result.status.success() {
            panic!("Clang failed with output: {}", stdout)
        }

        // Then get the names of the format macros

        let fmt_re = Regex::new(r"^\s*#define (?P<full>DRM_FORMAT_(?P<short>\w+)) ")?;
        let format_names: Vec<(&str, &str)> = stdout
            .lines()
            .filter_map(|line| {
                if line.contains("DRM_FORMAT_RESERVED")
                    || line.contains("INVALID")
                    || line.contains("_MOD_")
                {
                    return None;
                }

                fmt_re.captures(line).map(|caps| {
                    let full = caps.name("full").unwrap().as_str();
                    let short = caps.name("short").unwrap().as_str();

                    (full, short)
                })
            })
            .collect();

        let vendor_re = Regex::new(r"^\s*#define (?P<full>DRM_FORMAT_MOD_VENDOR_(?P<short>\w+)) ")?;
        let vendor_names: Vec<(&str, &str)> = stdout
            .lines()
            .filter_map(|line| {
                if line.contains("DRM_FORMAT_MOD_VENDOR_NONE") {
                    return None;
                }

                vendor_re.captures(line).map(|caps| {
                    let full = caps.name("full").unwrap().as_str();
                    let short = caps.name("short").unwrap().as_str();

                    (full, short)
                })
            })
            .collect();

        let mod_re = Regex::new(
            r"^\s*#define (?P<full>(DRM|I915)_FORMAT_MOD_(?P<short>\w+)) (DRM_FORMAT_MOD_(?P<alias>\w+)$)?",
        )?;
        let modifier_names_with_alias: Vec<(&str, String, Option<&str>)> = stdout
            .lines()
            .filter_map(|line| {
                if line.contains("DRM_FORMAT_MOD_NONE")
                    || line.contains("DRM_FORMAT_MOD_RESERVED")
                    || line.contains("VENDOR")
                    // grrr..
                    || line.contains("ARM_TYPE")
                {
                    return None;
                }

                mod_re.captures(line).map(|caps| {
                    let full = caps.name("full").unwrap().as_str();
                    let short = caps.name("short").unwrap().as_str();
                    let alias = caps.name("alias").map(|m| m.as_str());

                    (
                        full,
                        if full.contains("I915") {
                            format!("I915_{}", short)
                        } else {
                            String::from(short)
                        },
                        alias,
                    )
                })
            })
            .collect();

        // Then create a file with a variable defined for every format macro

        let mut wrapper = File::create(&wrapper_path)?;

        wrapper.write_all(b"#include <stdint.h>\n")?;
        wrapper.write_all(b"#include <drm_fourcc.h>\n")?;

        for (full, short) in &format_names {
            writeln!(wrapper, "uint32_t {}{} = {};\n", CONST_PREFIX, short, full)?;
        }
        for (full, short) in &vendor_names {
            writeln!(wrapper, "uint8_t {}{} = {};\n", CONST_PREFIX, short, full)?;
        }
        for (full, short, _alias) in &modifier_names_with_alias {
            writeln!(wrapper, "uint64_t {}{} = {};\n", CONST_PREFIX, short, full)?;
        }

        wrapper.flush()?;

        let mut builder = bindgen::builder()
            .ctypes_prefix("crate::_fake_ctypes")
            .header(wrapper_path.to_str().unwrap())
            .whitelist_var("DRM_FOURCC_.*");

        // Add additional include paths
        for include_path in &extra_includes {
            let path = include_path.to_str().expect("path is not valid utf8");
            builder = builder.clang_args(["-I", path]);
        }

        // Then generate bindings from that file
        builder.generate().unwrap().write_to_file("src/consts.rs")?;

        // Then generate our enums
        fn write_enum<'a>(
            as_enum: &mut File,
            name: &str,
            repr: &str,
            names: impl Iterator<Item = (&'a str, &'a str)>,
        ) -> Result<(), std::io::Error> {
            as_enum.write_all(b"#[derive(Copy, Clone)]")?;
            as_enum.write_all(
                b"#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]",
            )?;
            // writeln!(as_enum, "#[repr({})]", repr)?;
            writeln!(as_enum, "pub enum {} {{", name)?;

            let members: Vec<(String, String)> = names
                .into_iter()
                .map(|(_, short)| {
                    (
                        enum_member_case(short),
                        format!("consts::{}{}", CONST_PREFIX, short),
                    )
                })
                .collect();

            for (member, _value) in &members {
                writeln!(as_enum, "{},", member)?;
            }
            writeln!(as_enum, "Unrecognized({repr})")?;

            as_enum.write_all(b"}\n")?;

            writeln!(as_enum, "impl {} {{", name)?;
            writeln!(
                as_enum,
                "pub(crate) fn from_{}(n: {}) -> Self {{\n",
                repr, repr
            )?;
            as_enum.write_all(b"match n {\n")?;

            for (member, value) in &members {
                writeln!(as_enum, "{} => Self::{},", value, member)?;
            }

            as_enum.write_all(b"x => Self::Unrecognized(x),\n")?;
            as_enum.write_all(b"}}\n")?;

            writeln!(
                as_enum,
                "pub(crate) fn into_{}(self) -> {} {{\n",
                repr, repr
            )?;
            as_enum.write_all(b"match self {\n")?;

            for (member, value) in &members {
                writeln!(as_enum, "Self::{} => {},", member, value)?;
            }

            as_enum.write_all(b"Self::Unrecognized(x) => x,\n")?;
            as_enum.write_all(b"}}}\n")?;

            Ok(())
        }

        let as_enum_path = "src/as_enum.rs";
        {
            let mut as_enum = File::create(as_enum_path)?;

            as_enum.write_all(b"// Automatically generated by build.rs\n")?;
            as_enum.write_all(b"use crate::consts;")?;

            write_enum(&mut as_enum, "DrmFourcc", "u32", format_names.into_iter())?;

            as_enum.write_all(b"#[derive(Debug)]")?;
            write_enum(&mut as_enum, "DrmVendor", "u8", vendor_names.into_iter())?;
            let modifier_names = modifier_names_with_alias
                .iter()
                .filter(|(_, _, alias)| alias.is_none())
                .map(|(name, value, _)| (*name, value.as_str()));
            as_enum.write_all(b"#[derive(Debug)]")?;
            write_enum(&mut as_enum, "DrmModifier", "u64", modifier_names)?;
            as_enum.write_all(b"/// Aliases to duplicate variants\n")?;
            as_enum.write_all(b"#[allow(non_upper_case_globals)]\n")?;
            as_enum.write_all(b"impl DrmModifier {\n")?;
            for (_, short, alias) in &modifier_names_with_alias {
                if let Some(alias) = alias {
                    writeln!(
                        as_enum,
                        "pub const {}: Self = Self::{};\n",
                        enum_member_case(short),
                        enum_member_case(alias),
                    )?;
                }
            }
            as_enum.write_all(b"}\n")?;
        }

        Command::new("rustfmt").arg(as_enum_path).spawn()?.wait()?;

        Ok(())
    }

    fn enum_member_case(s: &str) -> String {
        let (first, rest) = s.split_at(1);
        format!("{}{}", first, rest.to_ascii_lowercase())
    }
}
