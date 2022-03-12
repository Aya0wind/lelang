use anyhow::Result;
use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManagerBuilder;
use inkwell::types::{AnyTypeEnum, BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, FunctionValue, GlobalValue, IntValue};


use crate::ast::{Ast, BinaryOperatorNode, CodeBlock, Expr, FunctionCallNode, IdentifierNode, NumberLiteralNode, Statement, VariableNode};
use crate::code_generator::{Symbol, SymbolTable};
use crate::error::CompileError;
use crate::lexer::{LELexer, Number, Operator};

#[derive(Default)]
struct CompilerContext<'s> {
    symbol_table: SymbolTable<'s>,
}


pub struct ModuleCodeGenerator<'s> {
    pub context: &'s Context,
    pub builder: Builder<'s>,
    pub optimizer: PassManagerBuilder,
    global_symbol_table: SymbolTable<'s>,
    compiler_context: CompilerContext<'s>,
}


impl<'s> ModuleCodeGenerator<'s> {
    pub fn compile_module(&mut self, module: &Module<'s>, mut tokens: LELexer) -> Result<()> {
        let ast = Ast::from_tokens(tokens)?;
        self.generate(module, &ast)?;
        module.print_to_file("out.ll").unwrap();
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


    fn generate_value(&self, value: &Expr) -> Result<BasicValueEnum> {
        match value {
            Expr::BinaryOperator(n) => { self.generate_binary_operator_node(n) }
            Expr::NumberLiteral(n) => { self.generate_number_literal_node(n) }
            Expr::UnaryOperator(n) => { unimplemented!() }
            Expr::CallExpression(n) => { self.generate_call_expression_node(n) }
            Expr::Identifier(n) => { self.generate_identifier_expression_node(n) }
        }
    }

    // fn upcast_number_value(&self,lhs:&BasicValueEnum,rhs:&BasicValueEnum)->BasicTypeEnum{}




    fn generate_binary_operator_node(&self, value: &BinaryOperatorNode) -> Result<BasicValueEnum> {
        let lhs = &value.left;
        let rhs = &value.right;

        match value.op {
            Operator::Plus => {
                let left = self.generate_value(lhs)?.into_int_value();
                let right = self.generate_value(rhs)?.into_int_value();
                Ok(self.builder.build_int_add::<IntValue>(left, right, "addtemp").as_basic_value_enum())
            }
            Operator::Sub => {
                let left = self.generate_value(lhs)?.into_int_value();
                let right = self.generate_value(rhs)?.into_int_value();
                Ok(self.builder.build_int_sub::<IntValue>(left, right, "addtemp").as_basic_value_enum())
            }
            Operator::Mul => {
                let left = self.generate_value(lhs)?.into_int_value();
                let right = self.generate_value(rhs)?.into_int_value();
                Ok(self.builder.build_int_mul::<IntValue>(left, right, "addtemp").as_basic_value_enum())
            }
            Operator::Div => {
                let left = self.generate_value(lhs)?.into_int_value();
                let right = self.generate_value(rhs)?.into_int_value();
                Ok(self.builder.build_int_signed_div::<IntValue>(left, right, "addtemp").as_basic_value_enum())
            }
            // Operator::Assign => {}
            Operator::Equal => {
                let left = self.generate_value(lhs)?.into_int_value();
                let right = self.generate_value(rhs)?.into_int_value();
                Ok(self.builder.build_int_add::<IntValue>(left, right, "addtemp").as_basic_value_enum())
            }
            _ => { unimplemented!() }
        }
    }
    fn generate_identifier_expression_node(&self, value: &IdentifierNode) -> Result<BasicValueEnum> {
        let name = &value.name;
        self.get_variable(name)
    }

    fn generate_number_literal_node(&self, value: &NumberLiteralNode) -> Result<BasicValueEnum> {
        match value.number {
            Number::Integer(i, signed) => {
                Ok(BasicValueEnum::from(self.context.i32_type().const_int(i, signed)))
            }
            Number::Float(f, _) => {
                Ok(BasicValueEnum::from(self.context.f64_type().const_float(f)))
            }
        }
    }

    fn generate_variable_node(&self, value: &VariableNode) -> Result<BasicValueEnum> {
        self.generate_value(&value.value)
    }

    fn generate_call_expression_node(&self, value: &FunctionCallNode) -> Result<BasicValueEnum> {
        let function = self.get_function(&value.function_name)?;
        let mut params = vec![];
        for param in value.params.iter() {
            params.push(BasicMetadataValueEnum::from(self.generate_value(param)?));
        }
        let call_site_value = self.builder.build_call(function, &params, "call");
        Ok(call_site_value.try_as_basic_value().unwrap_left())
    }


    fn get_type(&self, type_name: &str) -> Result<BasicTypeEnum<'s>> {
        let function_symbol = match self
            .compiler_context
            .symbol_table
            .get(type_name) {
            Some(s) => { s }
            None => {
                match self.global_symbol_table.get(type_name) {
                    None => { return Err(CompileError::identifier_is_not_type(type_name.into()).into()); }
                    Some(s) => { s }
                }
            }
        };
        if let Symbol::Type(t) = function_symbol {
            Ok(*t)
        } else {
            Err(CompileError::identifier_is_not_type(type_name.into()).into())
        }
    }

    fn get_variable(&self, variable_name: &str) -> Result<BasicValueEnum<'s>> {
        let variable = match self
            .compiler_context
            .symbol_table
            .get(variable_name) {
            Some(s) => { s }
            None => {
                match self.global_symbol_table.get(variable_name) {
                    None => { return Err(CompileError::identifier_is_not_variable(variable_name.into()).into()); }
                    Some(s) => { s }
                }
            }
        };
        if let Symbol::Variable(v) = variable {
            Ok(*v)
        } else {
            Err(CompileError::IdentifierIsNotType { identifier: variable_name.into() }.into())
        }
    }

    fn get_function(&self, function_name: &str) -> Result<FunctionValue<'s>> {
        let variable = match self
            .global_symbol_table
            .get(function_name) {
            Some(s) => { s }
            None => {
                return Err(CompileError::unknown_identifier(function_name.into()).into());
            }
        };
        if let Symbol::Function(v) = variable {
            Ok(*v)
        } else {
            Err(CompileError::identifier_is_not_function(function_name.into()).into())
        }
    }


    fn generate_functions(&mut self, module: &Module<'s>, ast: &Ast) -> Result<()> {
        for (name, function_node) in ast.functions.iter() {
            let mut function_symbol_table = SymbolTable::new();
            let param_types = function_node.params.iter().map(|a| {
                BasicMetadataTypeEnum::from(self.get_type(&a.type_name).unwrap().into_int_type())
            }).collect::<Vec<_>>();
            let return_type = self.get_type(&function_node.return_type)?;
            let fn_type = return_type.fn_type(&param_types, false);
            let function_value = module.add_function(name, fn_type, None);
            for (index,param) in function_value.get_param_iter().enumerate(){
                function_symbol_table.insert(function_node.params[index].identifier.clone(),Symbol::Variable(param));
            }
            self.compiler_context.symbol_table = function_symbol_table;
            self.build_function_block(function_value, &function_node.code_block)?;
        }
        Ok(())
    }

    fn build_function_block(&self, function_value: FunctionValue, code_block: &CodeBlock) -> Result<()> {
        let entry_block = self.context.append_basic_block(function_value, "entry");
        self.builder.position_at_end(entry_block);
        match &code_block.statements[0] {
            Statement::Expressions(expr) => {
                let value = self.generate_value(expr)?.as_instruction_value().unwrap();
                self.builder.insert_instruction(&value,None);
            }
            Statement::VariableDeclare(_) => {}
            Statement::Return(expr) => {
                let value = self.generate_value(expr)?;
                self.builder.build_return(Some(&value));
            }
            Statement::If(_) => {}
            _ => { unimplemented!() }
        }
        Ok(())
    }

    fn generate(&mut self, module: &Module<'s>, ast: &Ast) -> Result<()> {
        self.generate_global_variables(module, ast)?;
        self.generate_functions(module, ast)?;
        Ok(())
    }

    fn generate_global_variables(&self, module: &Module<'s>, ast: &Ast) -> Result<()> {
        for (name, value) in ast.globals.iter() {
            let global = self.add_generic_global(name, value.type_name.as_str(), module)?;
            let value = self.generate_variable_node(value)?;
            global.set_initializer(&value.into_int_value());
        }
        Ok(())
    }

    fn add_generic_global(&self, name: &str, type_name: &str, module: &Module<'s>) -> Result<GlobalValue> {
        Ok(module.add_global(self.get_type(type_name)?.into_int_type(), Some(AddressSpace::Global), name))
    }

    pub fn create(context: &'s Context) -> Self {
        let mut s = Self {
            context,
            builder: context.create_builder(),
            optimizer: PassManagerBuilder::create(),
            global_symbol_table: SymbolTable::default(),
            compiler_context: CompilerContext::default(),
        };
        s.global_symbol_table.insert("i8".into(), Symbol::Type(context.i8_type().as_basic_type_enum()));
        s.global_symbol_table.insert("i16".into(), Symbol::Type(context.i16_type().as_basic_type_enum()));
        s.global_symbol_table.insert("i32".into(), Symbol::Type(context.i32_type().as_basic_type_enum()));
        s.global_symbol_table.insert("i64".into(), Symbol::Type(context.i64_type().as_basic_type_enum()));
        s.global_symbol_table.insert("u8".into(), Symbol::Type(context.i8_type().as_basic_type_enum()));
        s.global_symbol_table.insert("u16".into(), Symbol::Type(context.i16_type().as_basic_type_enum()));
        s.global_symbol_table.insert("u32".into(), Symbol::Type(context.i32_type().as_basic_type_enum()));
        s.global_symbol_table.insert("u64".into(), Symbol::Type(context.i64_type().as_basic_type_enum()));
        s.global_symbol_table.insert("f32".into(), Symbol::Type(context.f32_type().as_basic_type_enum()));
        s.global_symbol_table.insert("f64".into(), Symbol::Type(context.f64_type().as_basic_type_enum()));
        s
    }
}


