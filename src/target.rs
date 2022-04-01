use inkwell::targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple};
use anyhow::Result;
use crate::arg_parser::Args;
use crate::compiler::optimize_number_to_level;

pub fn initialize_target_machine(config:&Args) ->Result<TargetMachine>{
    inkwell::targets::Target::initialize_native(&InitializationConfig{
        asm_parser: false,
        asm_printer: true,
        base: true,
        disassembler: false,
        info: false,
        machine_code: true
    }).unwrap();
    let target_triple = inkwell::targets::TargetMachine::get_default_triple();
    let target = inkwell::targets::Target::get_first().unwrap();
    let cpu = inkwell::targets::TargetMachine::get_host_cpu_name();
    let cpu_features = inkwell::targets::TargetMachine::get_host_cpu_features();
    let cpu_features = inkwell::targets::TargetMachine::get_host_cpu_features();
    let reloc = RelocMode::Default;
    let model = CodeModel::Default;
    Ok(target.create_target_machine(
        &target_triple,
        cpu.to_str().unwrap(),
        cpu_features.to_str().unwrap(),
        optimize_number_to_level(config.optimization),
        reloc,
        model,
    ).unwrap())
}