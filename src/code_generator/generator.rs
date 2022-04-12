use std::process::id;
use std::rc::Rc;

use inkwell::AddressSpace;
use inkwell::basic_block::BasicBlock;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::{BasicMetadataTypeEnum, BasicTypeEnum};
use inkwell::values::FunctionValue;
use nom::combinator::value;

use builder::Result;

use crate::ast::nodes::{Ast, BinaryOpExpression, CodeBlock, Expr, ExternalFunction, ForLoop, FunctionCall, FunctionDefinition, Identifier, IfStatement, NumberLiteral, Position, Statement, UnaryOpExpression, Variable, WhileLoop};
use crate::code_generator;
use crate::code_generator::builder;
use crate::code_generator::builder::binary_operator_builder::CompareOperator;
use crate::code_generator::builder::compile_context::CompilerContext;
use crate::code_generator::builder::le_type::{LEBasicType, LEBasicTypeEnum, LEBasicValue, LEBasicValueEnum, LEFloatValue, LEFunctionType, LEFunctionValue, LEIntegerValue, LEPointerValue};
use crate::code_generator::builder::LEGenerator;
use crate::error::CompileError;
use crate::lexer::{BinaryOperator, Number, UnaryOperator};

pub struct CodeGenerator<'ctx> {
    pub generator: LEGenerator<'ctx>,
    pub current_pos: Position,
}


impl<'ctx> CodeGenerator<'ctx> {
    fn build_expression(&mut self, value: &Expr) -> Result<Option<LEBasicValueEnum<'ctx>>> {
        match value {
            Expr::UnaryOperator(n) => { self.build_unary_operator_expression(n) }
            Expr::BinaryOperator(n) => { self.build_binary_operator_expression(n) }
            Expr::NumberLiteral(n) => { Ok(Some(self.build_number_literal_expression(n)?)) }
            Expr::CallExpression(n) => { self.build_call_expression(n) }
            Expr::Identifier(n) => { Ok(Some(self.build_identifier_expression(n)?)) }
        }
    }

    fn build_unary_operator_expression(&mut self, expr: &UnaryOpExpression) -> Result<Option<LEBasicValueEnum<'ctx>>> {
        // match expr.op {
        //     UnaryOperator::Plus => {}
        //     UnaryOperator::Sub => {
        //         let expr_value =
        //     }
        //     UnaryOperator::Dot => {}
        // }
        unimplemented!()
    }


    fn build_binary_operator_expression(&mut self, value: &BinaryOpExpression) -> Result<Option<LEBasicValueEnum<'ctx>>> {
        let lhs = &value.left;
        let rhs = &value.right;
        match value.op {
            BinaryOperator::Plus => {
                let left = self.build_expression(lhs)?
                    .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::Plus, "unit type".into()))?;
                let right = self.build_expression(rhs)?
                    .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::Plus, "unit type".into()))?;
                Ok(Some(self.generator.build_add(left, right)?))
            }
            BinaryOperator::Sub => {
                let left = self.build_expression(lhs)?
                    .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::Plus, "unit type".into()))?;
                let right = self.build_expression(rhs)?
                    .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::Plus, "unit type".into()))?;
                Ok(Some(self.generator.build_sub(left, right)?))
            }
            BinaryOperator::Mul => {
                let left = self.build_expression(lhs)?
                    .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::Plus, "unit type".into()))?;
                let right = self.build_expression(rhs)?
                    .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::Plus, "unit type".into()))?;
                Ok(Some(self.generator.build_mul(left, right)?))
            }
            BinaryOperator::Div => {
                let left = self.build_expression(lhs)?
                    .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::Plus, "unit type".into()))?;
                let right = self.build_expression(rhs)?
                    .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::Plus, "unit type".into()))?;
                Ok(Some(self.generator.build_div(left, right)?))
            }
            BinaryOperator::Assign => {
                if let Expr::Identifier(identifier) = lhs.as_ref() {
                    let value = self.build_expression(rhs.as_ref())?
                        .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::Plus, "unit type".into()))?;
                    ;
                    Ok(Some(self.generator.build_store_variable(&identifier.name, value)?))
                } else {
                    Err(CompileError::can_only_assign_variable(format!("{:?}", lhs)))
                }
            }
            BinaryOperator::Equal => {
                let left = self.build_expression(lhs)?
                    .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::Equal, "unit type".into()))?;
                let right = self.build_expression(rhs)?
                    .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::Equal, "unit type".into()))?;
                Ok(Some(self.generator.build_compare(left, right, CompareOperator::Equal)?.as_le_basic_value_enum()))
            }
            BinaryOperator::GreaterThan => {
                let left = self.build_expression(lhs)?
                    .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::GreaterThan, "unit type".into()))?;
                let right = self.build_expression(rhs)?
                    .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::GreaterThan, "unit type".into()))?;
                Ok(Some(self.generator.build_compare(left, right, CompareOperator::GreaterThan)?.as_le_basic_value_enum()))
            }
            BinaryOperator::LessThan => {
                let left = self.build_expression(lhs)?
                    .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::LessThan, "unit type".into()))?;
                let right = self.build_expression(rhs)?
                    .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::LessThan, "unit type".into()))?;
                Ok(Some(self.generator.build_compare(left, right, CompareOperator::LessThan)?.as_le_basic_value_enum()))
            }
            BinaryOperator::GreaterOrEqualThan => {
                let left = self.build_expression(lhs)?
                    .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::GreaterOrEqualThan, "unit type".into()))?;
                let right = self.build_expression(rhs)?
                    .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::GreaterOrEqualThan, "unit type".into()))?;
                Ok(Some(self.generator.build_compare(left, right, CompareOperator::GreaterOrEqualThan)?.as_le_basic_value_enum()))
            }
            BinaryOperator::LessOrEqualThan => {
                let left = self.build_expression(lhs)?
                    .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::LessOrEqualThan, "unit type".into()))?;
                let right = self.build_expression(rhs)?
                    .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::LessOrEqualThan, "unit type".into()))?;
                Ok(Some(self.generator.build_compare(left, right, CompareOperator::LessOrEqualThan)?.as_le_basic_value_enum()))
            }
        }
    }

    fn build_identifier_expression(&mut self, value: &Identifier) -> Result<LEBasicValueEnum<'ctx>> {
        let name = &value.name;
        self.generator.build_load_variable(name)
    }

    fn build_number_literal_expression(&mut self, value: &NumberLiteral) -> Result<LEBasicValueEnum<'ctx>> {
        match value.number {
            Number::Integer(i) => {
                let ty = self.generator.context.i64_type();
                let value = ty.get_llvm_type().const_int(i, true);

                Ok(LEIntegerValue { ty, llvm_value: value }.as_le_basic_value_enum())
            }
            Number::Float(f) => {
                let ty = self.generator.context.double_type();
                let value = ty.get_llvm_type().const_float(f);
                Ok(LEFloatValue { ty, llvm_value: value }.as_le_basic_value_enum())
            }
        }
    }

    fn build_call_expression(&mut self, value: &FunctionCall) -> Result<Option<LEBasicValueEnum<'ctx>>> {
        let function = self.generator.context.compiler_context.get_function(&value.function_name)?;
        let mut params = vec![];
        for param in value.params.iter() {
            params.push(self.build_expression(param)?
                .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::GreaterOrEqualThan, "unit type".into()))?);
        }
        self.generator.build_call(function, &params)
    }

    fn build_local_variable_definition(&mut self, value: &Variable) -> Result<LEPointerValue> {
        let name = value.prototype.name.clone();
        let initial_value = self.build_expression(value.value.as_ref())?
            .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::GreaterOrEqualThan, "unit type".into()))?;
        self.generator.create_local_variable(value.prototype.name.clone(), initial_value)
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

    fn build_return(&mut self, value: Option<LEBasicValueEnum>) -> Result<()> {
        let return_variable = &self.generator.context.compiler_context.return_variable;
        let return_block = self.generator.context.compiler_context.return_block.unwrap();
        if let Some(return_variable) = return_variable {
            if let Some(return_value) = value {
                self.generator.build_store(return_variable.clone(), return_value)?;
            } else {
                return Err(CompileError::type_mismatched("".into(), "".into()));
            }
        }
        self.generator.context.llvm_builder.build_unconditional_branch(return_block);
        Ok(())
    }

    fn build_for_loop(&mut self, for_loop: &ForLoop) -> Result<()> {
        let loop_variable = for_loop.init_statement.as_ref();

        if let Statement::Expressions(cond_expr) = for_loop.condition.as_ref() {
            let cond_block = self.generator.context.llvm_context.insert_basic_block_after(self.generator.context.llvm_builder.get_insert_block().unwrap(), "");
            let body_block = self.generator.context.llvm_context.insert_basic_block_after(cond_block, "");
            let after_block = self.generator.context.llvm_context.insert_basic_block_after(body_block, "");
            self.generator.context.compiler_context.push_block_table();
            if let Statement::VariableDefinition(v) = loop_variable {
                self.build_local_variable_definition(v)?;
            }
            self.generator.context.llvm_builder.build_unconditional_branch(cond_block);
            self.generator.context.llvm_builder.position_at_end(cond_block);
            if let LEBasicValueEnum::IntegerValue(int) = self.build_expression(cond_expr.as_ref())?
                .ok_or_else(|| CompileError::no_suitable_binary_operator(BinaryOperator::GreaterOrEqualThan, "unit type".into()))? {
                let cond_boolean = self.generator.context.llvm_builder.build_int_cast(int.llvm_value, self.generator.context.llvm_context.bool_type(), "");
                self.generator.context.llvm_builder.build_conditional_branch(cond_boolean, body_block, after_block);
            } else {
                return Err(CompileError::type_mismatched("".into(), "".into()));
            }
            self.generator.context.llvm_builder.position_at_end(body_block);
            self.build_code_block(&for_loop.code_block)?;

            if let Statement::Expressions(step_expr) = for_loop.iterate.as_ref() {
                self.build_expression(step_expr.as_ref())?;
            }
            self.generator.context.llvm_builder.build_unconditional_branch(cond_block);
            self.generator.context.llvm_builder.position_at_end(after_block);
            self.generator.context.compiler_context.pop_block_table();
        }
        Ok(())
    }

    fn build_while_loop(&mut self, while_loop: &WhileLoop) -> Result<()> {
        let cond_block = self.generator.context.llvm_context.insert_basic_block_after(self.generator.context.llvm_builder.get_insert_block().unwrap(), "");
        let body_block = self.generator.context.llvm_context.insert_basic_block_after(cond_block, "");
        let after_block = self.generator.context.llvm_context.insert_basic_block_after(body_block, "");
        self.generator.context.llvm_builder.build_unconditional_branch(cond_block);
        self.generator.context.llvm_builder.position_at_end(cond_block);
        self.generator.context.compiler_context.push_block_table();
        if let Some(cond_expr) = &while_loop.condition {
            if let LEBasicValueEnum::IntegerValue(int) = self.build_expression(cond_expr.as_ref())?
                .ok_or(CompileError::no_suitable_binary_operator(BinaryOperator::GreaterOrEqualThan, "unit type".into()))? {
                let cond_boolean = self.generator.context.llvm_builder.build_int_cast(int.llvm_value, self.generator.context.llvm_context.bool_type(), "");
                self.generator.context.llvm_builder.build_conditional_branch(cond_boolean, body_block, after_block);
            } else {
                return Err(CompileError::type_mismatched("".into(), "".into()));
            }
        } else {
            self.generator.context.llvm_builder.build_unconditional_branch(body_block);
        }
        self.generator.context.llvm_builder.position_at_end(body_block);
        self.build_code_block(&while_loop.code_block)?;
        self.generator.context.llvm_builder.build_unconditional_branch(cond_block);
        self.generator.context.llvm_builder.position_at_end(after_block);
        self.generator.context.compiler_context.pop_block_table();
        Ok(())
    }

    fn build_if_statement(&mut self, statement: &IfStatement) -> Result<()> {
        let cond_value = self.build_expression(statement.cond.as_ref())?;
        let then_block = self.generator.context.llvm_context.insert_basic_block_after(self.generator.context.llvm_builder.get_insert_block().unwrap(), "");
        let else_block = self.generator.context.llvm_context.insert_basic_block_after(then_block, "");
        let merge_block = self.generator.context.llvm_context.insert_basic_block_after(else_block, "");
        if let LEBasicValueEnum::IntegerValue(int) = cond_value
            .ok_or(CompileError::no_suitable_binary_operator(BinaryOperator::GreaterOrEqualThan, "unit type".into()))? {
            let cond_boolean = self.generator.context.llvm_builder.build_int_cast(int.llvm_value, self.generator.context.llvm_context.bool_type(), "");
            self.generator.context.llvm_builder.build_conditional_branch(cond_boolean, then_block, else_block);
        } else {
            return Err(CompileError::type_mismatched("".into(), "".into()).into());
        }
        self.generator.context.llvm_builder.position_at_end(then_block);
        self.generator.context.compiler_context.push_block_table();
        let is_then_return_block = self.build_code_block(&statement.then_block)?;
        if !is_then_return_block {
            self.generator.context.llvm_builder.build_unconditional_branch(merge_block);
        }
        self.generator.context.llvm_builder.position_at_end(else_block);
        if let Some(el) = &statement.else_block {
            let is_else_return_block = self.build_code_block(el)?;
            if !is_else_return_block {
                self.generator.context.llvm_builder.build_unconditional_branch(merge_block);
            }
        } else {
            self.generator.context.llvm_builder.build_unconditional_branch(merge_block);
        }
        self.generator.context.llvm_builder.position_at_end(merge_block);
        self.generator.context.compiler_context.pop_block_table();
        Ok(())
    }

    fn build_function_prototype(&mut self, module: &Module<'ctx>, prototype: &ExternalFunction) -> Result<LEFunctionValue<'ctx>> {
        let mut param_llvm_metadata_types = vec![];
        let mut param_types = vec![];
        for param_type in prototype.param_types.iter() {
            let ty = self.generator.get_generic_type(param_type)?;
            param_types.push(ty.clone());
            param_llvm_metadata_types.push(BasicMetadataTypeEnum::from(ty.get_basic_llvm_type()))
        }
        let mut return_type;
        let external_function = match &prototype.return_type {
            None => {
                return_type = None;
                self.generator.context.llvm_context.void_type().fn_type(&param_llvm_metadata_types, false)
            }
            Some(type_name) => {
                let ty = self.generator.get_generic_type(type_name)?;
                return_type = Some(ty.clone());
                match ty {
                    LEBasicTypeEnum::IntegerType(i) => { i.get_llvm_type().fn_type(&param_llvm_metadata_types, false) }
                    LEBasicTypeEnum::FloatType(i) => { i.get_llvm_type().fn_type(&param_llvm_metadata_types, false) }
                    LEBasicTypeEnum::PointerType(i) => { i.get_llvm_type().fn_type(&param_llvm_metadata_types, false) }
                    LEBasicTypeEnum::ArrayType(i) => { i.get_llvm_type().fn_type(&param_llvm_metadata_types, false) }
                    LEBasicTypeEnum::StructType(i) => { i.get_llvm_type().fn_type(&param_llvm_metadata_types, false) }
                    LEBasicTypeEnum::VectorType(i) => { i.get_llvm_type().fn_type(&param_llvm_metadata_types, false) }
                }
            }
        };
        let external_function_value = module.add_function(&prototype.name, external_function, Some(Linkage::External));
        let function_type = LEFunctionType::new(external_function, return_type, param_types);
        let le_function = LEFunctionValue { ty: function_type, llvm_value: external_function_value };
        self.generator.insert_global_function(prototype.name.clone(), le_function.clone())?;
        Ok(le_function)
    }

    fn build_return_block(&mut self, return_block: BasicBlock, return_variable: Option<LEPointerValue>) -> Result<()> {
        self.generator.context.llvm_builder.position_at_end(return_block);
        if let Some(value) = return_variable {
            let value = self.generator.build_load(value)?;
            self.generator.context.llvm_builder.build_return(Some(&value.to_llvm_basic_value_enum()));
            Ok(())
        } else {
            self.generator.context.llvm_builder.build_return(None);
            Ok(())
        }
    }

    fn build_function(&mut self, module: &Module<'ctx>, function_node: &FunctionDefinition) -> Result<LEFunctionValue<'ctx>> {
        let function_value = self.build_function_prototype(module, &function_node.prototype)?;
        let entry = self.generator.context.llvm_context.append_basic_block(function_value.llvm_value, "");
        let return_block = self.generator.context.llvm_context.append_basic_block(function_value.llvm_value, "");
        let return_type = function_value.ty.return_type();
        if let Some(none_void_type) = return_type {
            self.generator.context.llvm_builder.position_at_end(entry);
            let return_variable = self.generator.build_alloca_without_initialize(none_void_type)?;
            self.generator.context.compiler_context.set_current_context(function_value.llvm_value, Some(return_variable.clone()), return_block);
            self.generator.context.llvm_builder.position_at_end(return_block);
            self.build_return_block(return_block, Some(return_variable.clone()))?;
        } else {
            self.generator.context.compiler_context.set_current_context(function_value.llvm_value, None, return_block);
            self.build_return_block(return_block, None)?;
        }
        self.generator.context.llvm_builder.position_at_end(entry);
        self.generator.context.compiler_context.push_block_table();
        let function = &function_value;
        let names = &function_node.param_names;
        for ((param, name), param_type) in function.llvm_value.get_param_iter().zip(names).zip(function.ty.param_types().iter()) {
            let param_value = LEBasicValueEnum::from_llvm_basic_value_enum_and_type(param, param_type.clone());
            self.create_local_variable(name, param_type.clone(), param_value)?;
        }

        let is_return_block = self.build_code_block(&function_node.code_block)?;
        if !is_return_block {
            self.generator.context.llvm_builder.build_unconditional_branch(return_block);
        }
        self.generator.context.compiler_context.pop_block_table();
        Ok(function_value)
    }


    pub fn create_local_variable(&mut self, name: &str, target_type: LEBasicTypeEnum<'ctx>, initial_value: LEBasicValueEnum<'ctx>) -> Result<LEPointerValue<'ctx>> {
        let current_insert_block = self.generator.context.llvm_builder.get_insert_block().unwrap();
        let parent_function = self.generator.context.compiler_context.current_function.unwrap();
        let entry_block = parent_function.get_first_basic_block().unwrap();
        if let Some(first_instruction) = entry_block.get_first_instruction() {
            self.generator.context.llvm_builder.position_at(entry_block, &first_instruction);
        } else {
            self.generator.context.llvm_builder.position_at_end(entry_block);
        }
        let cast_value = self.generator.build_cast(initial_value.clone(), target_type.clone())?;
        let le_variable = self.generator.create_local_variable(name.into(), cast_value)?;
        self.generator.context.llvm_builder.position_at_end(current_insert_block);
        Ok(le_variable)
    }

    fn generate_all_functions(&mut self, module: &Module<'ctx>, ast: &Ast) -> Result<()> {
        for function_prototype in ast.extern_functions.iter() {
            let name = function_prototype.name.clone();
            self.build_function_prototype(module, function_prototype)?;
        }
        for function_node in ast.function_definitions.iter() {
            let name = function_node.prototype.name.clone();
            self.build_function(module, function_node)?;
        }
        Ok(())
    }

    fn generate_all_global_variables(&mut self, module: &Module<'ctx>, ast: &Ast) -> Result<()> {
        for variable in ast.globals.iter() {
            let ty = self.generator.get_generic_type(&variable.prototype.type_name)?;
            let value = self.build_expression(variable.value.as_ref())?.unwrap();
            self.generator.create_global_variable(
                variable.prototype.name.clone(),
                value,
                module,
            )?;
        }
        Ok(())
    }

    pub fn compile(&mut self, module: &Module<'ctx>, ast: &Ast) -> Result<()> {
        self.generate_all_global_variables(module, ast)?;
        self.generate_all_functions(module, ast)?;
        Ok(())
    }

    pub fn create(context: &'ctx Context) -> Self {
        let llvm_builder = context.create_builder();
        Self {
            generator: LEGenerator::new(context, llvm_builder),
            current_pos: Position { line: 0 },
        }
    }
}


