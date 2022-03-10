use anyhow::Result;
use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManagerBuilder;
use inkwell::types::{AnyType, AnyTypeEnum, BasicMetadataTypeEnum};
use inkwell::values::{AnyValue, AnyValueEnum, FunctionValue, GlobalValue};

use crate::ast::{Ast, BinaryOperatorNode, CallExpressionNode, CodeBlock, Expr, IdentifierNode, NumberLiteralNode, Statement, UnaryOperatorNode, VariableNode};
use crate::code_generator::{GlobalSymbolTable, VariableTable};
use crate::error::CompileError;
use crate::lexer::{LELexer, Number, Operator};

#[derive(Default)]
struct CompilerContext<'s> {
    local_variables: VariableTable<'s>,
}


pub struct ModuleCodeGenerator<'s> {
    pub context: &'s Context,
    pub builder: Builder<'s>,
    pub optimizer: PassManagerBuilder,
    symbol_table: GlobalSymbolTable<'s>,
    compiler_context: CompilerContext<'s>,
}


impl<'s> ModuleCodeGenerator<'s> {
    pub fn compile_module(&mut self, module: &Module<'s>, mut tokens: LELexer) -> Result<()> {
        let ast = Ast::from_tokens(tokens)?;
        //self.generate(module, &ast)?;
        // eprintln!("{:#?}", ast);
        //module.print_to_file("out.ll").unwrap();
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


    fn generate_value(&self, value: &Expr) -> Result<AnyValueEnum> {
        let res = match value {
            Expr::BinaryOperator(n) => { self.generate_binary_operator_node(n) }
            Expr::NumberLiteral(n) => { self.generate_number_literal_node(n) }
            Expr::UnaryOperator(n) => { self.generate_unary_operator_node(n) }
            Expr::CallExpression(n) => { self.generate_call_expression_node(n) }
            Expr::Identifier(n) => { self.generate_identifier_expression_node(n) }
        }?;
        Ok(res)
    }

    fn generate_binary_operator_node(&self, value: &BinaryOperatorNode) -> Result<AnyValueEnum> {
        match value.op {
            Operator::Plus => {
                Ok(self.builder.build_int_add(self.generate_value(&*value.left)?.into_int_value(), self.generate_value(&*value.right)?.into_int_value(), "addtemp").as_any_value_enum())
            }
            Operator::Sub => { Ok(self.builder.build_int_sub(self.generate_value(&*value.left)?.into_int_value(), self.generate_value(&*value.right)?.into_int_value(), "subtemp").as_any_value_enum()) }
            Operator::Mul => { Ok(self.builder.build_int_mul(self.generate_value(&*value.left)?.into_int_value(), self.generate_value(&*value.right)?.into_int_value(), "multemp").as_any_value_enum()) }
            Operator::Div => { Ok(self.builder.build_int_signed_div(self.generate_value(&*value.left)?.into_int_value(), self.generate_value(&*value.right)?.into_int_value(), "divtemp").as_any_value_enum()) }
            // Operator::Assign => {}
            // Operator::Equal => {}
            _ => { unimplemented!() }
        }
    }
    fn generate_identifier_expression_node(&self, value: &IdentifierNode) -> Result<AnyValueEnum> {
        let name = &value.name;
        if let Some(&value) = self.compiler_context.local_variables.get(name) {
            Ok(value)
        } else if let Some(&value) = self.symbol_table.variables.get(name) {
            Ok(value)
        } else {
            unreachable!()
        }
    }

    fn generate_number_literal_node(&self, value: &NumberLiteralNode) -> Result<AnyValueEnum> {
        match value.number {
            Number::Integer(i, signed) => {
                Ok(self.context.i64_type().const_int(i, signed).as_any_value_enum())
            }
            Number::Float(f, _) => {
                Ok(self.context.f64_type().const_float(f).as_any_value_enum())
            }
        }
    }

    fn generate_unary_operator_node(&self, value: &UnaryOperatorNode) -> Result<AnyValueEnum> {
        unimplemented!()
    }

    fn generate_variable_node(&self, value: &VariableNode) -> Result<AnyValueEnum> {
        self.generate_value(&value.value)
    }

    fn generate_call_expression_node(&self, value: &CallExpressionNode) -> Result<AnyValueEnum> {
        unimplemented!()
    }


    fn get_type(&self, type_name: &str) -> Result<AnyTypeEnum<'s>> {
        Ok(*self
            .symbol_table
            .types
            .get(type_name)
            .ok_or_else(|| CompileError::UnknownType(type_name.into()))?)
    }

    fn generate_functions(&mut self, module: &Module<'s>, ast: &Ast) -> Result<()> {
        for (name, function_node) in ast.functions.iter() {
            let variable_table = VariableTable::new();
            let param_types = function_node.params.iter().map(|a| {
                BasicMetadataTypeEnum::from(self.get_type(&a.type_name).unwrap().into_int_type())
            }).collect::<Vec<_>>();
            let return_type = self.get_type(&function_node.return_type)?.into_int_type();
            let fn_type = return_type.fn_type(&param_types, false);
            let function_value = module.add_function(name, fn_type, None);
            self.compiler_context.local_variables = variable_table;
            self.build_function_block(function_value, &function_node.code_block)?;
        }
        Ok(())
    }

    fn build_function_block(&self, function_value: FunctionValue, code_block: &CodeBlock) -> Result<()> {
        let entry_block = self.context.append_basic_block(function_value, "entry");
        self.builder.position_at_end(entry_block);
        let int_32 = self.get_type("i32")?.into_int_type();
        self.builder.build_int_add(int_32.const_int(100, false), int_32.const_int(200, false), "");
        match &code_block.statements[0] {
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
            symbol_table: GlobalSymbolTable::default(),
            compiler_context: CompilerContext::default(),
        };
        s.symbol_table.types.insert("i8".into(), context.i8_type().as_any_type_enum());
        s.symbol_table.types.insert("i16".into(), context.i16_type().as_any_type_enum());
        s.symbol_table.types.insert("i32".into(), context.i32_type().as_any_type_enum());
        s.symbol_table.types.insert("i64".into(), context.i64_type().as_any_type_enum());
        s.symbol_table.types.insert("u8".into(), context.i8_type().as_any_type_enum());
        s.symbol_table.types.insert("u16".into(), context.i16_type().as_any_type_enum());
        s.symbol_table.types.insert("u32".into(), context.i32_type().as_any_type_enum());
        s.symbol_table.types.insert("u64".into(), context.i64_type().as_any_type_enum());
        s.symbol_table.types.insert("f32".into(), context.f32_type().as_any_type_enum());
        s.symbol_table.types.insert("f64".into(), context.f64_type().as_any_type_enum());
        s
    }
}


