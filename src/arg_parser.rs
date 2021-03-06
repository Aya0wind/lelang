use std::ops::RangeInclusive;

use clap::ArgEnum;
use clap::Parser;

/// lelang programming language compiler, based on LLVM compiler infrastructure
#[derive(Parser, Debug)]
pub struct Args {
    /// Set compiler optimize level
    #[clap(short = 'O', default_value_t = 0, value_name = "OPTIMIZE_LEVEL", parse(try_from_str = port_in_range))]
    pub optimization: usize,

    /// Set compiler output format
    #[clap(short = 'S', default_value_t = OutputFormatEnum::OBJ, arg_enum)]
    pub output_format: OutputFormatEnum,

    /// Set compiler output path
    #[clap(short = 'o', default_value = "./a.out", parse(from_os_str), value_name = "OUTPUT_FILE_PATH", value_hint = clap::ValueHint::DirPath)]
    pub output_path: std::path::PathBuf,

    /// Set compiler source file path
    #[clap(short = 'i', parse(from_os_str), value_name = "SOURCE_FILE_PATH", value_hint = clap::ValueHint::DirPath,)]
    pub input_path: std::path::PathBuf,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
pub enum OutputFormatEnum {
    TOKENS,
    AST,
    IR,
    ASM,
    OBJ,
    EXE,
}


const OPTIMIZE_LEVEL_RANGE: RangeInclusive<usize> = 0..=3;

fn port_in_range(s: &str) -> Result<usize, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{}` isn't a valid optimize level number", s))?;
    if OPTIMIZE_LEVEL_RANGE.contains(&port) {
        Ok(port)
    } else {
        Err(format!(
            "Optimize level can only in range {}-{}",
            OPTIMIZE_LEVEL_RANGE.start(),
            OPTIMIZE_LEVEL_RANGE.end()
        ))
    }
}
