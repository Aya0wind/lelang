
use std::collections::HashMap;



use anyhow::Result;
use inkwell::AddressSpace;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::{PassManagerBuilder};
use inkwell::types::{AnyType, AnyTypeEnum, BasicMetadataTypeEnum, BasicTypeEnum};
use inkwell::values::{AnyValue, AnyValueEnum, BasicValueEnum, FunctionValue, GlobalValue, InstructionValue};

use crate::ast::{Ast, BinaryOperatorNode, CallExpressionNode, NumberLiteralNode, UnaryOperatorNode, BExpr, Expr, VariableNode, IdentifierNode, FunctionNode, Statement};
use crate::error::CompileError;
use crate::lexer::{LEToken, LELexer, Number, Operator};

enum Symbol<'s>{
    Variable(AnyValueEnum<'s>),
    Type(BasicTypeEnum<'s>),
}

struct SymbolMap<'s>{
    map:HashMap<String,Symbol<'s>>,
}


struct SymbolTable<'s>{
    global:SymbolMap<'s>,
    function_table:SymbolMap<'s>,
}



pub struct ModuleCodeGenerator<'s> {
    pub context: &'s Context,
    pub builder: Builder<'s>,
    pub optimizer: PassManagerBuilder,
    symbol_table: SymbolTable<'s>,
}




impl<'s> ModuleCodeGenerator<'s> {
    pub fn compile_module(&mut self, module: &Module<'s>, mut tokens: LELexer) -> Result<()> {
        let ast = Ast::from_tokens(tokens)?;
        eprintln!("{:#?}",ast);
        self.generate(module, &ast)?;
        module.print_to_stderr();
        Ok(())
        // let i64_type = self.context.i64_type();
        // let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
        // let function = self.module.add_function("sum", fn_type, None);
        // let basic_block = self.context.append_basic_block(function, "entry");
        //
        // self.builder.position_at_end(basic_block);
        //
        // let x = function.get_nth_param(0)?.into_int_value();
        // let y = function.get_nth_param(1)?.into_int_value();
        // let z = function.get_nth_param(2)?.into_int_value();
        //
        // let sum = self.builder.build_int_add(x, y, "sum");
        // let sum = self.builder.build_int_add(sum, z, "sum");
        // self.builder.build_return(Some(&sum));
        // let i32_type = self.context.i32_type();
        // let main_fn_type = i32_type.fn_type(&[], false);
        // let main_fn = self.module.add_function("main", main_fn_type, None);
        // let basic_block = self.context.append_basic_block(main_fn, "entry");
        // self.builder.position_at_end(basic_block);
        // self.builder.build_return(Some(&i32_type.const_zero()));
    }


    fn generate_value(&self, value: &Expr) ->Result<AnyValueEnum>{

        let res = match value{
            Expr::BinaryOperator(n) => {self.generate_binary_operator_node(n)}
            Expr::NumberLiteral(n) => {self.generate_number_literal_node(n)}
            Expr::UnaryOperator(n) => {self.generate_unary_operator_node(n)}
            Expr::CallExpression(n) => {self.generate_call_expression_node(n)}
            Expr::Identifier(n)=>{self.generate_identifier_expression_node(n)}
        }?;
        eprintln!("{:?}",res.print_to_string());
        Ok(res)
    }

    fn generate_binary_operator_node(&self, value:&BinaryOperatorNode) ->Result<AnyValueEnum>{

        match value.op{
            Operator::Plus => {
                Ok(self.builder.build_int_add(self.generate_value(&*value.left)?.into_int_value(), self.generate_value(&*value.right)?.into_int_value(), "addtemp").as_any_value_enum())
            }
            Operator::Sub => {Ok(self.builder.build_int_sub(self.generate_value(&*value.left)?.into_int_value(), self.generate_value(&*value.right)?.into_int_value(), "subtemp").as_any_value_enum())}
            Operator::Mul => {Ok(self.builder.build_int_mul(self.generate_value(&*value.left)?.into_int_value(), self.generate_value(&*value.right)?.into_int_value(), "multemp").as_any_value_enum())}
            Operator::Div => {Ok(self.builder.build_int_signed_div(self.generate_value(&*value.left)?.into_int_value(), self.generate_value(&*value.right)?.into_int_value(), "divtemp").as_any_value_enum())}
            // Operator::Assign => {}
            // Operator::Equal => {}
            _=>{unimplemented!()}
        }
    }
    fn generate_identifier_expression_node(&self, value:&IdentifierNode)->Result<AnyValueEnum>{
        let name = &value.name;
        if let Some(symbol) = self.symbol_table.function_table.map.get(name){
            match symbol {
                Symbol::Variable(v) => { return Ok(v.as_any_value_enum())}
                Symbol::Type(_) => {unreachable!()}
            }
        }
        unreachable!()
    }

    fn generate_number_literal_node(&self, value:&NumberLiteralNode) ->Result<AnyValueEnum>{
        match value.number{
            Number::I8(n) => { Ok(self.context.i8_type().const_int(n as u64, true).as_any_value_enum())}
            Number::I16(n) => {Ok(self.context.i16_type().const_int(n as u64, true).as_any_value_enum())}
            Number::I32(n) => {Ok(self.context.i32_type().const_int(n as u64, true).as_any_value_enum())}
            Number::I64(n) => {Ok(self.context.i64_type().const_int(n as u64, true).as_any_value_enum())}
            Number::U8(n) => {Ok(self.context.i8_type().const_int(n as u64, false).as_any_value_enum())}
            Number::U16(n) => {Ok(self.context.i8_type().const_int(n as u64, false).as_any_value_enum())}
            Number::U32(n) => {Ok(self.context.i8_type().const_int(n as u64, false).as_any_value_enum())}
            Number::U64(n) => {Ok(self.context.i8_type().const_int(n as u64, false).as_any_value_enum())}
            Number::F32(n) => {Ok(self.context.f32_type().const_float(n as f64).as_any_value_enum())}
            Number::F64(n) => {Ok(self.context.f64_type().const_float(n as f64).as_any_value_enum())}
        }
    }

    fn generate_unary_operator_node(&self, value:&UnaryOperatorNode)->Result<AnyValueEnum>{
        unimplemented!()
    }

    fn generate_variable_node(&self, value:&VariableNode) ->Result<AnyValueEnum>{
       self.generate_value(&value.value)
    }

    fn generate_call_expression_node(&self, value:&CallExpressionNode) ->Result<AnyValueEnum>{
        unimplemented!()
    }


    fn get_type(&self,name:String)->Result<AnyTypeEnum>{
        match name.as_str() {
            "i8"=>{ Ok(self.context.i32_type().as_any_type_enum())}
            "i16"=>{ Ok(self.context.i32_type().as_any_type_enum())}
            "i32"=>{ Ok(self.context.i32_type().as_any_type_enum())}
            _=>{unreachable!()}
        }
    }

    fn generate_functions(&mut self,module:&Module<'s>,ast:&Ast)->Result<()>{
        for (name,function_node) in ast.functions.iter(){
            let param_types = function_node.params.iter().map(|x|BasicMetadataTypeEnum::IntType(self.context.i32_type())).collect::<Vec<_>>();
            let return_type = self.context.i32_type();
            let fn_type = return_type.fn_type(&param_types,false);
            let function = module.add_function(name,fn_type,None);
            for (param,name) in function.get_params().iter().zip(function_node.params.iter()){
                self.symbol_table.function_table.map.insert(name.identifier.clone(),Symbol::Variable(param.as_any_value_enum()));
            }
            self.build_function_block(function,function_node)?;
        }
        Ok(())
    }

    fn build_function_block(&self,function_value:FunctionValue,function:&FunctionNode)->Result<()>{
        let entry_block = self.context.append_basic_block(function_value, "entry");
        self.builder.position_at_end(entry_block);
        match &function.code_block.statements[0]{
            Statement::Expressions(expr) => {
                let value = self.generate_value(expr)?.into_int_value();

                self.builder.build_return(Some(&value));
            }
            Statement::VariableDeclare(_) => {}
            Statement::Return(expr) => {
                let value = self.generate_value(expr)?.into_int_value();
                value.print_to_stderr();
                self.builder.build_return(Some(&value));
            }
            Statement::If(_) => {}
        }
        Ok(())
    }

    fn generate(&mut self,module:&Module<'s>,ast:&Ast)->Result<()>{
        self.generate_global_variables(module,ast)?;
        self.generate_functions(module,ast)?;
        Ok(())
    }

    fn generate_global_variables(&self, module: &Module<'s>, ast: &Ast) -> Result<()> {
        for (name, value) in ast.globals.iter() {
            let global = self.add_generic_global(name, value.type_name.as_str(), module);
            let value = self.generate_variable_node(value)?;
            global.set_initializer(&value.into_int_value());
        }
        Ok(())
    }

    fn add_generic_global(&self, name: &str, type_name: &str, module: &Module<'s>) -> GlobalValue {
        match type_name {
            "i32" | "u32" => { module.add_global(self.context.i32_type(), Some(AddressSpace::Global), name) }
            "f32" => { module.add_global(self.context.f32_type(), Some(AddressSpace::Global), name) }
            "i8" | "u8" => { module.add_global(self.context.i8_type(), Some(AddressSpace::Global), name) }
            "i16" | "u16" => { module.add_global(self.context.i16_type(), Some(AddressSpace::Global), name) }
            "i64" | "u64" => { module.add_global(self.context.i64_type(), Some(AddressSpace::Global), name) }
            "f64" => { module.add_global(self.context.f64_type(), Some(AddressSpace::Global), name) }
            _ => { unreachable!() }
        }
    }

    pub fn create(context: &'s Context) -> Self {
        Self { context, builder: context.create_builder(), optimizer: PassManagerBuilder::create(), symbol_table: SymbolTable{ global: SymbolMap { map: Default::default()}, function_table: SymbolMap {  map: Default::default()} } }
    }
}


