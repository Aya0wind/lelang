use inkwell::targets::InitializationConfig;
use anyhow::Result;
pub fn initialize_target_config() ->Result<()>{
    inkwell::targets::Target::initialize_native(&InitializationConfig{
        asm_parser: true,
        asm_printer: true,
        base: false,
        disassembler: false,
        info: false,
        machine_code: false
    }).unwrap();
    Ok(())

}