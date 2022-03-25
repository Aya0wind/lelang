use anyhow::Result;
use inkwell::{AddressSpace, IntPredicate, OptimizationLevel};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType, IntType};
use inkwell::values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, CallSiteValue, FunctionValue, GlobalValue, InstructionValue, IntValue, PointerValue};

use crate::ast::{Ast, BinaryOpExpression, CodeBlock, Expr, ForLoop, FunctionCall, FunctionDefinition, ExternFunction, IdentifierNode, IfStatement, NumberLiteralNode, Statement, VariableNode, WhileLoop};
use crate::code_gen::optimizer::Optimizer;
use crate::code_gen::symbol_table::CompilerContext;
use crate::error::CompileError;
use crate::lexer::{LELexer, Number, Operator};

pub struct ModuleCodeGenerator<'s> {
    pub context: &'s Context,
    pub builder: Builder<'s>,
    compiler_context: CompilerContext<'s>,
}


impl<'s> ModuleCodeGenerator<'s> {
    pub fn compile_module(&mut self, module: &Module<'s>, tokens: LELexer, level: OptimizationLevel) -> Result<()> {
        let ast = Ast::from_tokens(tokens)?;
        //eprintln!("{:#?}", ast);
        self.build_from_ast(module, &ast, level)?;
        Ok(())
    }


    fn build_expression(&self, value: &Expr) -> Result<BasicValueEnum> {
        match value {
            Expr::BinaryOperator(n) => { self.build_binary_operator_expression(n) }
            Expr::NumberLiteral(n) => { self.build_number_literal_expression(n) }
            Expr::CallExpression(n) => { self.build_call_expression(n) }
            Expr::Identifier(n) => { self.build_identifier_expression(n) }
        }
    }


    fn build_binary_operator_expression(&self, value: &BinaryOpExpression) -> Result<BasicValueEnum> {
        let lhs = &value.left;
        let rhs = &value.right;

        match value.op {
            Operator::Plus => {
                let left = self.build_expression(lhs)?.into_int_value();
                let right = self.build_expression(rhs)?.into_int_value();
                Ok(self.builder.build_int_add::<IntValue>(left, right, "").as_basic_value_enum())
            }
            Operator::Sub => {
                let left = self.build_expression(lhs)?.into_int_value();
                let right = self.build_expression(rhs)?.into_int_value();
                Ok(self.builder.build_int_sub::<IntValue>(left, right, "").as_basic_value_enum())
            }
            Operator::Mul => {
                let left = self.build_expression(lhs)?.into_int_value();
                let right = self.build_expression(rhs)?.into_int_value();
                Ok(self.builder.build_int_mul::<IntValue>(left, right, "").as_basic_value_enum())
            }
            Operator::Div => {
                let left = self.build_expression(lhs)?.into_int_value();
                let right = self.build_expression(rhs)?.into_int_value();
                Ok(self.builder.build_int_signed_div::<IntValue>(left, right, "").as_basic_value_enum())
            }
            Operator::Assign => {
                if let Expr::Identifier(identifier) = lhs.as_ref() {
                    let left_value = self.compiler_context.get_variable(&identifier.name)?;
                    let value = self.build_expression(rhs.as_ref())?;
                    self.builder.build_store(left_value, value);
                    Ok(value)
                } else {
                    Err(CompileError::can_only_assign_variable(format!("{:?}", lhs)).into())
                }
            }
            Operator::Equal => {
                let left = self.build_expression(lhs)?.into_int_value();
                let right = self.build_expression(rhs)?.into_int_value();
                Ok(self.builder.build_int_compare(IntPredicate::EQ, left, right, "").as_basic_value_enum())
            }
            Operator::GreaterThan => {
                let left = self.build_expression(lhs)?.into_int_value();
                let right = self.build_expression(rhs)?.into_int_value();
                Ok(self.builder.build_int_compare(IntPredicate::SGT, left, right, "").as_basic_value_enum())
            }
            Operator::LessThan => {
                let left = self.build_expression(lhs)?.into_int_value();
                let right = self.build_expression(rhs)?.into_int_value();
                Ok(self.builder.build_int_compare(IntPredicate::SLT, left, right, "").as_basic_value_enum())
            }
            Operator::GreaterOrEqualThan => {
                let left = self.build_expression(lhs)?.into_int_value();
                let right = self.build_expression(rhs)?.into_int_value();
                Ok(self.builder.build_int_compare(IntPredicate::SGE, left, right, "").as_basic_value_enum())
            }
            Operator::LessOrEqualThan => {
                let left = self.build_expression(lhs)?.into_int_value();
                let right = self.build_expression(rhs)?.into_int_value();
                Ok(self.builder.build_int_compare(IntPredicate::SLE, left, right, "").as_basic_value_enum())
            }
        }
    }

    fn build_identifier_expression(&self, value: &IdentifierNode) -> Result<BasicValueEnum> {
        let name = &value.name;
        let pointer_value = self.compiler_context.get_variable(name)?;
        Ok(self.builder.build_load(pointer_value, ""))
    }

    fn build_number_literal_expression(&self, value: &NumberLiteralNode) -> Result<BasicValueEnum> {
        match value.number {
            Number::Integer(i, signed) => {
                Ok(BasicValueEnum::from(self.context.i32_type().const_int(i, signed)))
            }
            Number::Float(f, _) => {
                Ok(BasicValueEnum::from(self.context.f64_type().const_float(f)))
            }
        }
    }

    fn build_local_variable_definition(&mut self, value: &VariableNode) -> Result<PointerValue> {
        let current_insert_block = self.builder.get_insert_block().unwrap();
        let parent_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        let entry_block = parent_function.get_first_basic_block().unwrap();
        if let Some(first_instruction) = entry_block.get_first_instruction() {
            self.builder.position_at(entry_block, &first_instruction);
        } else {
            self.builder.position_at_end(entry_block);
        }
        let pointer_value = self.builder.build_alloca(self.compiler_context.get_type(&value.prototype.type_name)?, "");
        self.builder.build_store(pointer_value, self.build_expression(value.value.as_ref())?);
        self.compiler_context.insert_local_variable(value.prototype.name.clone(), pointer_value)?;
        self.builder.position_at_end(current_insert_block);
        Ok(pointer_value)
    }

    fn build_call_expression(&self, value: &FunctionCall) -> Result<BasicValueEnum> {
        let function = self.compiler_context.get_function(&value.function_name)?;
        let mut params = vec![];
        for param in value.params.iter() {
            params.push(BasicMetadataValueEnum::from(self.build_expression(param)?));
        }
        let either = self.builder.build_call(function, &params, "call").try_as_basic_value();
        if let Some(value) = either.left() {
            Ok(value)
        } else {
            Ok(BasicValueEnum::from(self.context.i32_type().const_int(0, true)))
        }
    }


    fn get_function_type(&mut self, prototype: &ExternFunction) -> Result<FunctionType<'s>> {
        let param_types = prototype.param_types.iter().map(|a| {
            BasicMetadataTypeEnum::from(self.compiler_context.get_type(a).unwrap().into_int_type())
        }).collect::<Vec<_>>();
        match &prototype.return_type {
            None => { Ok(self.context.void_type().fn_type(&param_types, false)) }
            Some(type_name) => { Ok(self.compiler_context.get_type(type_name)?.fn_type(&param_types, false)) }
        }
    }

    fn build_function_prototype(&mut self, module: &Module<'s>, prototype: &ExternFunction) -> Result<FunctionValue<'s>> {
        let external_function = self.get_function_type(prototype)?;
        let external_function_value = module.add_function(&prototype.name, external_function, Some(Linkage::External));
        Ok(external_function_value)
    }

    fn optimize_all_functions(&mut self, module: &Module<'s>, level: OptimizationLevel) -> Result<()> {
        let optimizer = Optimizer::new(module, level);
        for (_, function) in self.compiler_context.global_symbols.functions.iter() {
            optimizer.run(function);
        }
        Ok(())
    }

    fn build_global_functions(&mut self, module: &Module<'s>, ast: &Ast, level: OptimizationLevel) -> Result<()> {
        for function_prototype in ast.extern_functions.iter() {
            let function_value = self.build_function_prototype(module, function_prototype)?;
            self.compiler_context.insert_global_function(function_prototype.name.clone(), function_value)?;
        }
        for function_node in ast.function_definitions.iter() {
            let function_value = self.build_function(module, function_node)?;
            self.compiler_context.insert_global_function(function_node.prototype.name.clone(), function_value)?;
        }
        //self.optimize_all_functions(module,level)?;
        Ok(())
    }

    fn build_function(&mut self, module: &Module<'s>, function_node: &FunctionDefinition) -> Result<FunctionValue<'s>> {
        let function_value = self.build_function_prototype(module, &function_node.prototype)?;
        let entry = self.context.append_basic_block(function_value, "");
        self.builder.position_at_end(entry);
        self.build_function_params(&function_value, &function_node.param_names)?;
        self.build_code_block(&function_node.code_block)?;
        self.compiler_context.clear_local_symbols();
        Ok(function_value)
    }


    fn build_function_params(&mut self, function: &FunctionValue<'s>, names: &[String]) -> Result<()> {
        for (param, name) in function.get_param_iter().zip(names) {
            let pointer_value = self.builder.build_alloca(param.get_type(), "");
            self.builder.build_store(pointer_value, param);
            self.compiler_context.insert_local_variable(name.clone(), pointer_value)?;
        }
        Ok(())
    }

    fn build_code_block(&mut self, code_block: &CodeBlock) -> Result<bool> {
        self.compiler_context.push_block_table();
        let mut is_return_block = false;
        for statement in code_block.statements.iter() {
            match statement {
                Statement::Expressions(expr) => {
                    self.build_expression(expr)?;
                }
                Statement::Return(expr) => {
                    let value = self.build_expression(expr)?;
                    self.builder.build_return(Some(&value));
                    is_return_block = true;
                }
                Statement::If(if_expr) => {
                    self.build_if_statement(if_expr)?;
                    is_return_block = false;
                }
                Statement::ForLoop(for_loop) => {
                    self.build_for_loop(for_loop)?;
                }
                Statement::VariableDefinition(variable_definition) => {
                    self.build_local_variable_definition(variable_definition)?;
                }
                Statement::Void => {}
                Statement::WhileLoop(while_loop) => {
                    self.build_while_loop(while_loop)?;
                }
                _ => { unimplemented!() }
            }
        }
        self.compiler_context.pop_block_table();
        Ok(is_return_block)
    }


    fn build_for_loop(&mut self, for_loop: &ForLoop) -> Result<()> {
        let loop_variable = for_loop.init_statement.as_ref();
        if let Statement::VariableDefinition(v) = loop_variable {
            self.build_local_variable_definition(v)?;
        }
        if let Statement::Expressions(cond_expr) = for_loop.condition.as_ref() {
            let cond_block = self.context.insert_basic_block_after(self.builder.get_insert_block().unwrap(), "");
            let body_block = self.context.insert_basic_block_after(cond_block, "");
            let after_block = self.context.insert_basic_block_after(body_block, "");
            self.builder.build_unconditional_branch(cond_block);
            self.builder.position_at_end(cond_block);
            let loop_condition = self.build_expression(cond_expr.as_ref())?;
            let bool_cond = self.builder.build_int_cast(loop_condition.into_int_value(), self.context.bool_type(), "");
            self.builder.build_conditional_branch(bool_cond, body_block, after_block);
            self.builder.position_at_end(body_block);
            let is_return_block = self.build_code_block(&for_loop.code_block)?;
            if let Statement::Expressions(step_expr) = for_loop.iterate.as_ref() {
                self.build_expression(step_expr.as_ref())?;
            }
            self.builder.build_unconditional_branch(cond_block);
            self.builder.position_at_end(after_block);
        }
        Ok(())
    }


    fn build_while_loop(&mut self, while_loop: &WhileLoop) -> Result<()> {
        if let Some(cond_expr) = &while_loop.condition {
            let cond_block = self.context.insert_basic_block_after(self.builder.get_insert_block().unwrap(), "");
            let body_block = self.context.insert_basic_block_after(cond_block, "");
            let after_block = self.context.insert_basic_block_after(body_block, "");
            self.builder.build_unconditional_branch(cond_block);
            self.builder.position_at_end(cond_block);
            let loop_condition = self.build_expression(cond_expr.as_ref())?;
            let bool_cond = self.builder.build_int_cast(loop_condition.into_int_value(), self.context.bool_type(), "");
            self.builder.build_conditional_branch(bool_cond, body_block, after_block);
            self.builder.position_at_end(body_block);
            let is_return_block = self.build_code_block(&while_loop.code_block)?;
            self.builder.build_unconditional_branch(cond_block);
            self.builder.position_at_end(after_block);
        }
        Ok(())
    }


    fn build_if_statement(&mut self, statement: &IfStatement) -> Result<()> {
        let cond_value = self.build_expression(statement.cond.as_ref())?;
        let then_block = self.context.insert_basic_block_after(self.builder.get_insert_block().unwrap(), "");
        let else_block = self.context.insert_basic_block_after(then_block, "");
        let merge_block = self.context.insert_basic_block_after(else_block, "");
        let cond_boolean = self.builder.build_int_cast(cond_value.into_int_value(), self.context.bool_type(), "");
        self.builder.build_conditional_branch(cond_boolean, then_block, else_block);
        self.builder.position_at_end(then_block);
        let mut is_return_block = self.build_code_block(&statement.then_block)?;
        if !is_return_block {
            self.builder.build_unconditional_branch(merge_block);
        }
        is_return_block = false;
        self.builder.position_at_end(else_block);
        if let Some(el) = &statement.else_block {
            is_return_block = self.build_code_block(el)?;
        }
        if !is_return_block {
            self.builder.build_unconditional_branch(merge_block);
        }
        self.builder.position_at_end(merge_block);
        Ok(())
    }

    fn build_from_ast(&mut self, module: &Module<'s>, ast: &Ast, level: OptimizationLevel) -> Result<()> {
        self.build_global_variables(module, ast)?;
        self.build_global_functions(module, ast, level)?;
        Ok(())
    }

    fn build_global_variables(&self, module: &Module<'s>, ast: &Ast) -> Result<()> {
        for variable in ast.globals.iter() {
            let global = self.add_global_variable(&variable.prototype.name, &variable.prototype.type_name, module)?;
            let value = self.build_expression(variable.value.as_ref())?;
            global.set_initializer(&value.into_int_value());
        }
        Ok(())
    }

    fn add_global_variable(&self, name: &str, type_name: &str, module: &Module<'s>) -> Result<GlobalValue> {
        Ok(module.add_global(self.compiler_context.get_type(type_name)?.into_int_type(), Some(AddressSpace::Global), name))
    }


    pub fn create(context: &'s Context) -> Self {
        let mut s = Self {
            context,
            builder: context.create_builder(),
            compiler_context: CompilerContext::default(),
        };
        s.compiler_context.insert_global_type("i8".into(), context.i8_type().as_basic_type_enum()).unwrap();
        s.compiler_context.insert_global_type("i16".into(), context.i16_type().as_basic_type_enum()).unwrap();
        s.compiler_context.insert_global_type("i32".into(), context.i32_type().as_basic_type_enum()).unwrap();
        s.compiler_context.insert_global_type("i64".into(), context.i64_type().as_basic_type_enum()).unwrap();
        s.compiler_context.insert_global_type("u8".into(), context.i8_type().as_basic_type_enum()).unwrap();
        s.compiler_context.insert_global_type("u16".into(), context.i16_type().as_basic_type_enum()).unwrap();
        s.compiler_context.insert_global_type("u32".into(), context.i32_type().as_basic_type_enum()).unwrap();
        s.compiler_context.insert_global_type("u64".into(), context.i64_type().as_basic_type_enum()).unwrap();
        s.compiler_context.insert_global_type("f32".into(), context.f32_type().as_basic_type_enum()).unwrap();
        s.compiler_context.insert_global_type("f64".into(), context.f64_type().as_basic_type_enum()).unwrap();
        s
    }
}


