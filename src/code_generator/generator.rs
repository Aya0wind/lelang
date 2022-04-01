use anyhow::Result;
use inkwell::AddressSpace;
use inkwell::basic_block::BasicBlock;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::FunctionValue;

use crate::ast::{Ast, BinaryOpExpression, CodeBlock, Expr, ExternalFunction, ForLoop, FunctionCall, FunctionDefinition, Identifier, IfStatement, NumberLiteral, Statement, Variable, WhileLoop};
use crate::code_generator::builder::{CompareOperator, LEBuilder, LEVariable, NumericTypeEnum, NumericValueEnum};
use crate::code_generator::compile_context::CompilerContext;
use crate::code_generator::symbol_table::Symbol;
use crate::error::CompileError;
use crate::lexer::{Number, Operator};

use super::builder::{LETypeEnum, LEValueEnum};

pub struct CodeGenerator<'s> {
    pub builder: LEBuilder<'s>,
    compiler_context: CompilerContext<'s>,
}


impl<'s> CodeGenerator<'s> {
    fn create_global_variable(&mut self, name: &str, ty: LETypeEnum<'s>, module: &Module<'s>, initial_value: LEValueEnum<'s>) -> Result<LEVariable<'s>> {
        let value_type = initial_value.get_type();
        if ty != value_type {
            return Err(CompileError::type_mismatched(ty.to_string(), value_type.to_string(), self.compiler_context.current_pos).into());
        }
        let variable = match ty {
            LETypeEnum::NumericType(number) => {
                match number {
                    NumericTypeEnum::FloatType(float) => { module.add_global(float, Some(AddressSpace::Global), name) }
                    NumericTypeEnum::IntegerType(int) => { module.add_global(int.value, Some(AddressSpace::Global), name) }
                }
            }
            _ => { unimplemented!() }
        };
        let le_variable = LEVariable { ty, pointer: variable.as_pointer_value() };
        self.compiler_context.symbols.insert_global_variable(name.into(), le_variable, self.compiler_context.current_pos)?;
        Ok(le_variable)
    }

    fn create_local_variable(&mut self, name: &str, target_type: LETypeEnum<'s>, initial_value: LEValueEnum<'s>) -> Result<LEVariable<'s>> {
        let current_insert_block = self.builder.llvm_builder.get_insert_block().unwrap();
        let parent_function = self.compiler_context.current_function.unwrap();
        let entry_block = parent_function.get_first_basic_block().unwrap();
        if let Some(first_instruction) = entry_block.get_first_instruction() {
            self.builder.llvm_builder.position_at(entry_block, &first_instruction);
        } else {
            self.builder.llvm_builder.position_at_end(entry_block);
        }
        let value_type = initial_value.get_type();
        let le_variable = self.builder.build_alloca(target_type)?;
        self.builder.llvm_builder.position_at_end(current_insert_block);
        let cast_value = self.builder.build_cast(initial_value, target_type)
            .map_err(|_| CompileError::type_mismatched(target_type.to_string(), value_type.to_string(), self.compiler_context.current_pos))?;
        self.builder.build_store(le_variable, cast_value, self.compiler_context.current_pos)?;
        self.compiler_context.symbols.insert_local_variable(name.into(), le_variable, self.compiler_context.current_pos)?;
        Ok(le_variable)
    }


    fn create_function(&mut self, name: &str, function: FunctionValue<'s>) -> Result<FunctionValue> {
        self.compiler_context.symbols.insert_global_function(name.into(), function, self.compiler_context.current_pos)?;
        Ok(function)
    }

    fn get_variable(&self, identifier: &str) -> Result<LEVariable<'s>> {
        self.compiler_context.symbols.get_variable(identifier, self.compiler_context.current_pos)
    }

    fn get_function(&self, identifier: &str) -> Result<FunctionValue<'s>> {
        self.compiler_context.symbols.get_function(identifier, self.compiler_context.current_pos)
    }

    fn get_type(&self, identifier: &str) -> Result<LETypeEnum<'s>> {
        self.compiler_context.symbols.get_type(identifier, self.compiler_context.current_pos)
    }

    fn get_symbol(&self, identifier: &str) -> Option<Symbol<'s>> {
        self.compiler_context.symbols.get_symbol(identifier, self.compiler_context.current_pos)
    }

    fn build_expression(&self, value: &Expr) -> Result<LEValueEnum<'s>> {
        match value {
            Expr::BinaryOperator(n) => { self.build_binary_operator_expression(n) }
            Expr::NumberLiteral(n) => { self.build_number_literal_expression(n) }
            Expr::CallExpression(n) => { self.build_call_expression(n) }
            Expr::Identifier(n) => { self.build_identifier_expression(n) }
        }
    }

    fn build_binary_operator_expression(&self, value: &BinaryOpExpression) -> Result<LEValueEnum<'s>> {
        let lhs = &value.left;
        let rhs = &value.right;
        match value.op {
            Operator::Plus => {
                let left = self.build_expression(lhs)?;
                let right = self.build_expression(rhs)?;
                self.builder.build_add(left, right)
            }
            Operator::Sub => {
                let left = self.build_expression(lhs)?;
                let right = self.build_expression(rhs)?;
                self.builder.build_sub(left, right)
            }
            Operator::Mul => {
                let left = self.build_expression(lhs)?;
                let right = self.build_expression(rhs)?;
                self.builder.build_mul(left, right)
            }
            Operator::Div => {
                let left = self.build_expression(lhs)?;
                let right = self.build_expression(rhs)?;
                self.builder.build_div(left, right)
            }
            Operator::Assign => {
                if let Expr::Identifier(identifier) = lhs.as_ref() {
                    let variable = self.compiler_context.symbols.get_variable(&identifier.name, self.compiler_context.current_pos)?;
                    let value = self.build_expression(rhs.as_ref())?;
                    self.builder.build_store(variable, value, self.compiler_context.current_pos)
                } else {
                    Err(CompileError::can_only_assign_variable(format!("{:?}", lhs), self.compiler_context.current_pos).into())
                }
            }
            Operator::Equal => {
                let left = self.build_expression(lhs)?;
                let right = self.build_expression(rhs)?;
                let integer_value = self.builder.build_compare(left, right, CompareOperator::Equal)?;
                Ok(integer_value.value.into())
            }
            Operator::GreaterThan => {
                let left = self.build_expression(lhs)?;
                let right = self.build_expression(rhs)?;
                let integer_value = self.builder.build_compare(left, right, CompareOperator::GreaterThan)?;
                Ok(integer_value.value.into())
            }
            Operator::LessThan => {
                let left = self.build_expression(lhs)?;
                let right = self.build_expression(rhs)?;
                let integer_value = self.builder.build_compare(left, right, CompareOperator::LessThan)?;
                Ok(integer_value.value.into())
            }
            Operator::GreaterOrEqualThan => {
                let left = self.build_expression(lhs)?;
                let right = self.build_expression(rhs)?;
                let integer_value = self.builder.build_compare(left, right, CompareOperator::GreaterOrEqualThan)?;
                Ok(integer_value.value.into())
            }
            Operator::LessOrEqualThan => {
                let left = self.build_expression(lhs)?;
                let right = self.build_expression(rhs)?;
                let integer_value = self.builder.build_compare(left, right, CompareOperator::LessOrEqualThan)?;
                Ok(integer_value.value.into())
            }
        }
    }

    fn build_identifier_expression(&self, value: &Identifier) -> Result<LEValueEnum<'s>> {
        let name = &value.name;
        let variable = self.compiler_context.symbols.get_variable(name, self.compiler_context.current_pos)?;
        self.builder.build_load_variable(variable)
    }

    fn build_number_literal_expression(&self, value: &NumberLiteral) -> Result<LEValueEnum<'s>> {
        match value.number {
            Number::Integer(i, signed) => {
                Ok(self.builder.llvm_context.i64_type().const_int(i, signed).into())
            }
            Number::Float(f) => {
                Ok(self.builder.llvm_context.f64_type().const_float(f).into())
            }
        }
    }

    fn build_call_expression(&self, value: &FunctionCall) -> Result<LEValueEnum<'s>> {
        let function = self.get_function(&value.function_name)?;
        let mut params = vec![];
        for param in value.params.iter() {
            params.push(self.build_expression(param)?);
        }
        self.builder.build_call(function, &params)
    }

    fn build_local_variable_definition(&mut self, value: &Variable) -> Result<LEVariable> {
        let variable = self.create_local_variable(
            &value.prototype.name,
            self.get_type(&value.prototype.type_name)?,
            self.build_expression(value.value.as_ref())?,
        )?;

        Ok(variable)
    }

    fn build_code_block(&mut self, code_block: &CodeBlock) -> Result<bool> {
        for statement in code_block.statements.iter() {
            match statement {
                Statement::Expressions(expr) => {
                    self.build_expression(expr)?;
                }
                Statement::Return(expr) => {
                    let value = self.build_expression(expr)?;
                    self.build_return(value)?;
                    return Ok(true);
                }
                Statement::If(if_expr) => {
                    self.build_if_statement(if_expr)?;
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
            }
        }
        Ok(false)
    }

    fn build_return(&self, value: LEValueEnum) -> Result<()> {
        let return_variable = self.compiler_context.return_variable;
        let return_block = self.compiler_context.return_block.unwrap();
        if let Some(return_variable) = return_variable {
            self.builder.build_store(return_variable, value, self.compiler_context.current_pos)?;
        }
        self.builder.llvm_builder.build_unconditional_branch(return_block);
        Ok(())
    }

    fn build_for_loop(&mut self, for_loop: &ForLoop) -> Result<()> {
        let loop_variable = for_loop.init_statement.as_ref();
        if let Statement::VariableDefinition(v) = loop_variable {
            self.build_local_variable_definition(v)?;
        }
        if let Statement::Expressions(cond_expr) = for_loop.condition.as_ref() {
            let cond_block = self.builder.llvm_context.insert_basic_block_after(self.builder.llvm_builder.get_insert_block().unwrap(), "");
            let body_block = self.builder.llvm_context.insert_basic_block_after(cond_block, "");
            let after_block = self.builder.llvm_context.insert_basic_block_after(body_block, "");
            self.builder.llvm_builder.build_unconditional_branch(cond_block);
            self.builder.llvm_builder.position_at_end(cond_block);
            if let LEValueEnum::NumericValue(NumericValueEnum::Integer(int)) = self.build_expression(cond_expr.as_ref())? {
                let cond_boolean = self.builder.llvm_builder.build_int_cast(int.value, self.builder.llvm_context.bool_type(), "");
                self.builder.llvm_builder.build_conditional_branch(cond_boolean, body_block, after_block);
            } else {
                return Err(CompileError::type_mismatched("".into(), "".into(), self.compiler_context.current_pos).into());
            }
            self.builder.llvm_builder.position_at_end(body_block);
            self.build_code_block(&for_loop.code_block)?;
            if let Statement::Expressions(step_expr) = for_loop.iterate.as_ref() {
                self.build_expression(step_expr.as_ref())?;
            }
            self.builder.llvm_builder.build_unconditional_branch(cond_block);
            self.builder.llvm_builder.position_at_end(after_block);
        }
        Ok(())
    }

    fn build_while_loop(&mut self, while_loop: &WhileLoop) -> Result<()> {
        let cond_block = self.builder.llvm_context.insert_basic_block_after(self.builder.llvm_builder.get_insert_block().unwrap(), "");
        let body_block = self.builder.llvm_context.insert_basic_block_after(cond_block, "");
        let after_block = self.builder.llvm_context.insert_basic_block_after(body_block, "");
        self.builder.llvm_builder.build_unconditional_branch(cond_block);
        self.builder.llvm_builder.position_at_end(cond_block);
        if let Some(cond_expr) = &while_loop.condition {
            if let LEValueEnum::NumericValue(NumericValueEnum::Integer(int)) = self.build_expression(cond_expr.as_ref())? {
                let cond_boolean = self.builder.llvm_builder.build_int_cast(int.value, self.builder.llvm_context.bool_type(), "");
                self.builder.llvm_builder.build_conditional_branch(cond_boolean, body_block, after_block);
            } else {
                return Err(CompileError::type_mismatched("".into(), "".into(), self.compiler_context.current_pos).into());
            }
        }else{
            self.builder.llvm_builder.build_unconditional_branch(body_block);
        }
        self.builder.llvm_builder.position_at_end(body_block);
        self.build_code_block(&while_loop.code_block)?;
        self.builder.llvm_builder.build_unconditional_branch(cond_block);
        self.builder.llvm_builder.position_at_end(after_block);
        Ok(())
    }

    fn build_if_statement(&mut self, statement: &IfStatement) -> Result<()> {
        let cond_value = self.build_expression(statement.cond.as_ref())?;
        let then_block = self.builder.llvm_context.insert_basic_block_after(self.builder.llvm_builder.get_insert_block().unwrap(), "");
        let else_block = self.builder.llvm_context.insert_basic_block_after(then_block, "");
        let merge_block = self.builder.llvm_context.insert_basic_block_after(else_block, "");
        if let LEValueEnum::NumericValue(NumericValueEnum::Integer(int)) = cond_value {
            let cond_boolean = self.builder.llvm_builder.build_int_cast(int.value, self.builder.llvm_context.bool_type(), "");
            self.builder.llvm_builder.build_conditional_branch(cond_boolean, then_block, else_block);
        } else {
            return Err(CompileError::type_mismatched("".into(), "".into(), self.compiler_context.current_pos).into());
        }
        self.builder.llvm_builder.position_at_end(then_block);
        let is_then_return_block = self.build_code_block(&statement.then_block)?;
        if !is_then_return_block {
            self.builder.llvm_builder.build_unconditional_branch(merge_block);
        }
        self.builder.llvm_builder.position_at_end(else_block);
        if let Some(el) = &statement.else_block {
            let is_else_return_block = self.build_code_block(el)?;
            if !is_else_return_block {
                self.builder.llvm_builder.build_unconditional_branch(merge_block);
            }
        } else {
            self.builder.llvm_builder.build_unconditional_branch(merge_block);
        }
        self.builder.llvm_builder.position_at_end(merge_block);
        Ok(())
    }

    fn build_function_prototype(&mut self, module: &Module<'s>, prototype: &ExternalFunction) -> Result<FunctionValue<'s>> {
        let mut param_types = vec![];
        for param_type in prototype.param_types.iter() {
            let ty = self.get_type(param_type)?;
            match ty {
                LETypeEnum::NumericType(number) => {
                    match number {
                        NumericTypeEnum::FloatType(f) => { param_types.push(BasicMetadataTypeEnum::from(f)); }
                        NumericTypeEnum::IntegerType(i) => { param_types.push(BasicMetadataTypeEnum::from(i.value)); }
                    }
                }
                _ => { unimplemented!() }
            }
        }
        let external_function = match &prototype.return_type {
            None => {
                self.builder.llvm_context.void_type().fn_type(&param_types, false)
            }
            Some(type_name) => {
                let ty = self.get_type(type_name)?;
                match ty {
                    LETypeEnum::NumericType(number) => {
                        match number {
                            NumericTypeEnum::FloatType(f) => { f.fn_type(&param_types, false) }
                            NumericTypeEnum::IntegerType(i) => { i.value.fn_type(&param_types, false) }
                        }
                    }
                    _ => { unimplemented!() }
                }
            }
        };
        let external_function_value = module.add_function(&prototype.name, external_function, Some(Linkage::External));
        Ok(external_function_value)
    }

    fn build_return_block(&self, return_block: BasicBlock, return_variable: LEVariable) -> Result<()> {
        self.builder.llvm_builder.position_at_end(return_block);
        let value = self.builder.build_load(return_variable)?;
        match value {
            LEValueEnum::NumericValue(number) => {
                let basic_value = number.to_basic_value_enum();
                self.builder.llvm_builder.build_return(Some(&basic_value));
                Ok(())
            }
            _ => { unimplemented!() }
        }
    }

    fn build_function(&mut self, module: &Module<'s>, function_node: &FunctionDefinition) -> Result<FunctionValue<'s>> {
        let function_value = self.build_function_prototype(module, &function_node.prototype)?;
        let entry = self.builder.llvm_context.append_basic_block(function_value, "");
        self.builder.llvm_builder.position_at_end(entry);
        let return_variable = self.builder.build_alloca(function_value.get_type().get_return_type().unwrap().into())?;
        let return_block = self.builder.llvm_context.append_basic_block(function_value, "");
        self.compiler_context.set_current_context(function_value, return_variable, return_block);
        self.build_return_block(return_block, return_variable)?;
        self.builder.llvm_builder.position_at_end(entry);
        self.compiler_context.push_block_table();
        let function = &function_value;
        let names = &function_node.param_names;
        for (param, name) in function.get_param_iter().zip(names) {
            self.create_local_variable(name, param.get_type().into(), param.into())?;
        }
        let is_return_block = self.build_code_block(&function_node.code_block)?;
        if !is_return_block {
            self.builder.llvm_builder.build_unconditional_branch(return_block);
        }
        self.compiler_context.pop_block_table();
        Ok(function_value)
    }

    fn generate_all_functions(&mut self, module: &Module<'s>, ast: &Ast) -> Result<()> {
        for function_prototype in ast.extern_functions.iter() {
            let name = function_prototype.name.clone();
            let function_value = self.build_function_prototype(module, function_prototype)?;
            self.create_function(&name, function_value)?;
        }
        for function_node in ast.function_definitions.iter() {
            let name = function_node.prototype.name.clone();
            let function_value = self.build_function(module, function_node)?;
            self.create_function(&name, function_value)?;
        }
        //self.optimize_all_functions(module,level)?;
        Ok(())
    }

    fn generate_all_global_variables(&mut self, module: &Module<'s>, ast: &Ast) -> Result<()> {
        for variable in ast.globals.iter() {
            let ty = self.get_type(&variable.prototype.type_name)?;
            self.create_global_variable(
                &variable.prototype.name,
                ty,
                module,
                self.build_expression(variable.value.as_ref())?,
            )?;
        }
        Ok(())
    }

    pub fn compile(&mut self, module: &Module<'s>, ast: &Ast) -> Result<()> {
        self.generate_all_global_variables(module, &ast)?;
        self.generate_all_functions(module, &ast)?;
        Ok(())
    }

    pub fn create(context: &'s Context) -> Self {
        let llvm_builder = context.create_builder();
        Self {
            builder: LEBuilder::new(context, llvm_builder),
            compiler_context: CompilerContext::new(context),
        }
    }
}


