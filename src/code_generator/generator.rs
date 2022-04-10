use inkwell::AddressSpace;
use inkwell::basic_block::BasicBlock;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::{BasicMetadataTypeEnum, BasicTypeEnum};
use inkwell::values::FunctionValue;

use builder::Result;

use crate::ast::nodes::{Ast, BinaryOpExpression, CodeBlock, Expr, ExternalFunction, ForLoop, FunctionCall, FunctionDefinition, Identifier, IfStatement, NumberLiteral, Position, Statement, UnaryOpExpression, Variable, WhileLoop};
use crate::code_generator;
use crate::code_generator::builder;
use crate::code_generator::builder::LEBuilder;
use crate::code_generator::builder::binary_operator_builder::CompareOperator;
use crate::code_generator::builder::llvm_type_wrapper::{LEBasicTypeGenericRef, LEBasicValue, LEBasicValueEnum, LEIntegerValue, LEPointerValue};
use crate::code_generator::compile_context::CompilerContext;
use crate::code_generator::symbol_table::Symbol;
use crate::error::CompileError;
use crate::lexer::{BinaryOperator, Number};

pub struct CodeGenerator<'ctx> {
    pub builder: LEBuilder<'ctx>,
    pub current_pos: Position,
}


impl<'ctx> CodeGenerator<'ctx> {
    fn build_expression<'a>(&mut self, value: &Expr, compiler_context: &mut CompilerContext<'ctx>) -> Result<LEBasicValueEnum<'ctx, 'a>> {
        match value {
            Expr::UnaryOperator(n) => { self.build_unary_operator_expression(n, compiler_context) }
            Expr::BinaryOperator(n) => { self.build_binary_operator_expression(n, compiler_context) }
            Expr::NumberLiteral(n) => { self.build_number_literal_expression(n) }
            Expr::CallExpression(n) => { self.build_call_expression(n, compiler_context) }
            Expr::Identifier(n) => { self.build_identifier_expression(n, compiler_context) }
        }
    }

    fn build_unary_operator_expression<'a>(&mut self, value: &UnaryOpExpression, compiler_context: &mut CompilerContext<'ctx>) -> Result<LEBasicValueEnum<'ctx, 'a>> {
        unimplemented!()
    }


    fn build_binary_operator_expression<'a>(&mut self, value: &BinaryOpExpression, compiler_context: &mut CompilerContext<'ctx>) -> Result<LEBasicValueEnum<'ctx, 'a>> {
        let lhs = &value.left;
        let rhs = &value.right;
        match value.op {
            BinaryOperator::Plus => {
                let left = self.build_expression(lhs, compiler_context)?;
                let right = self.build_expression(rhs, compiler_context)?;
                self.builder.build_add(left, right)
            }
            BinaryOperator::Sub => {
                let left = self.build_expression(lhs, compiler_context)?;
                let right = self.build_expression(rhs, compiler_context)?;
                self.builder.build_sub(left, right)
            }
            BinaryOperator::Mul => {
                let left = self.build_expression(lhs, compiler_context)?;
                let right = self.build_expression(rhs, compiler_context)?;
                self.builder.build_mul(left, right)
            }
            BinaryOperator::Div => {
                let left = self.build_expression(lhs, compiler_context)?;
                let right = self.build_expression(rhs, compiler_context)?;
                self.builder.build_div(left, right)
            }
            BinaryOperator::Assign => {
                if let Expr::Identifier(identifier) = lhs.as_ref() {
                    let variable = compiler_context.symbols.get_variable(&identifier.name)?;
                    let value = self.build_expression(rhs.as_ref(), compiler_context)?;
                    self.builder.build_variable_store(variable, value, self.current_pos)
                } else {
                    Err(CompileError::can_only_assign_variable(format!("{:?}", lhs)).into())
                }
            }
            BinaryOperator::Equal => {
                let left = self.build_expression(lhs, compiler_context)?;
                let right = self.build_expression(rhs, compiler_context)?;
                let integer_value = self.builder.build_compare(left, right, CompareOperator::Equal)?;
                Ok(integer_value.as_le_value_enum())
            }
            BinaryOperator::GreaterThan => {
                let left = self.build_expression(lhs, compiler_context)?;
                let right = self.build_expression(rhs, compiler_context)?;
                let integer_value = self.builder.build_compare(left, right, CompareOperator::GreaterThan)?;
                Ok(integer_value.as_le_value_enum())
            }
            BinaryOperator::LessThan => {
                let left = self.build_expression(lhs, compiler_context)?;
                let right = self.build_expression(rhs, compiler_context)?;
                let integer_value = self.builder.build_compare(left, right, CompareOperator::LessThan)?;
                Ok(integer_value.as_le_value_enum())
            }
            BinaryOperator::GreaterOrEqualThan => {
                let left = self.build_expression(lhs, compiler_context)?;
                let right = self.build_expression(rhs, compiler_context)?;
                let integer_value = self.builder.build_compare(left, right, CompareOperator::GreaterOrEqualThan)?;
                Ok(integer_value.as_le_value_enum())
            }
            BinaryOperator::LessOrEqualThan => {
                let left = self.build_expression(lhs, compiler_context)?;
                let right = self.build_expression(rhs, compiler_context)?;
                let integer_value = self.builder.build_compare(left, right, CompareOperator::LessOrEqualThan)?;
                Ok(integer_value.as_le_value_enum())
            }
        }
    }

    fn build_identifier_expression<'a>(&mut self, value: &Identifier, compiler_context: &mut CompilerContext<'ctx>) -> Result<LEBasicValueEnum<'ctx, 'a>> {
        let name = &value.name;
        let variable = compiler_context.symbols.get_variable(name)?;
        self.builder.build_variable_load(variable)
    }

    fn build_number_literal_expression<'a>(&mut self, value: &NumberLiteral) -> Result<LEBasicValueEnum<'ctx, 'a>> {
        match value.number {
            Number::Integer(i) => {
                Ok(self.builder.llvm_context.i64_type().const_int(i, true).into())
            }
            Number::Float(f) => {
                Ok(self.builder.llvm_context.f64_type().const_float(f).into())
            }
        }
    }

    fn build_call_expression<'a>(&mut self, value: &FunctionCall, compiler_context: &mut CompilerContext<'ctx>) -> Result<LEBasicValueEnum<'ctx, 'a>> {
        let function = self.get_function(&value.function_name)?;
        let mut params = vec![];
        for param in value.params.iter() {
            params.push(self.build_expression(param, compiler_context)?);
        }
        self.builder.build_call(function, &params)
    }

    fn build_local_variable_definition<'a>(&mut self, value: &Variable, compiler_context: &mut CompilerContext<'ctx>) -> Result<LEPointerValue> {
        self.create_local_variable(
            &value.prototype.name,
            self.get_type(&value.prototype.type_name)?,
            self.build_expression(value.value.as_ref(), compiler_context)?,
            compiler_context,
        )
    }

    fn build_code_block<'a>(&mut self, code_block: &CodeBlock, compiler_context: &mut CompilerContext<'ctx>) -> Result<bool> {
        for statement in code_block.statements.iter() {
            match statement {
                Statement::Expressions(expr) => {
                    self.build_expression(expr, compiler_context)?;
                }
                Statement::Return(expr) => {
                    let value = self.build_expression(expr, compiler_context)?;
                    self.build_return(value, compiler_context)?;
                    return Ok(true);
                }
                Statement::If(if_expr) => {
                    self.build_if_statement(if_expr, compiler_context)?;
                }
                Statement::ForLoop(for_loop) => {
                    self.build_for_loop(for_loop, compiler_context)?;
                }
                Statement::VariableDefinition(variable_definition) => {
                    self.build_local_variable_definition(variable_definition, compiler_context)?;
                }
                Statement::Void => {}
                Statement::WhileLoop(while_loop) => {
                    self.build_while_loop(while_loop, compiler_context)?;
                }
            }
        }
        Ok(false)
    }

    fn build_return<'a>(&mut self, value: LEBasicValueEnum, compiler_context: &mut CompilerContext<'ctx>) -> Result<()> {
        let return_variable = compiler_context.return_variable;
        let return_block = compiler_context.return_block.unwrap();
        if let Some(return_variable) = return_variable {
            self.builder.build_store(return_variable, value, self.current_pos)?;
        }
        self.builder.llvm_builder.build_unconditional_branch(return_block);
        Ok(())
    }

    fn build_for_loop<'a>(&mut self, for_loop: &ForLoop, compiler_context: &mut CompilerContext<'ctx>) -> Result<()> {
        let loop_variable = for_loop.init_statement.as_ref();
        if let Statement::VariableDefinition(v) = loop_variable {
            self.build_local_variable_definition(v, compiler_context)?;
        }
        if let Statement::Expressions(cond_expr) = for_loop.condition.as_ref() {
            let cond_block = self.builder.llvm_context.insert_basic_block_after(self.builder.llvm_builder.get_insert_block().unwrap(), "");
            let body_block = self.builder.llvm_context.insert_basic_block_after(cond_block, "");
            let after_block = self.builder.llvm_context.insert_basic_block_after(body_block, "");
            self.builder.llvm_builder.build_unconditional_branch(cond_block);
            self.builder.llvm_builder.position_at_end(cond_block);
            if let LEBasicValueEnum::IntegerValue(int) = self.build_expression(cond_expr.as_ref(), compiler_context)? {
                let cond_boolean = self.builder.llvm_builder.build_int_cast(int.llvm_value, self.builder.llvm_context.bool_type(), "");
                self.builder.llvm_builder.build_conditional_branch(cond_boolean, body_block, after_block);
            } else {
                return Err(CompileError::type_mismatched("".into(), "".into()).into());
            }
            self.builder.llvm_builder.position_at_end(body_block);
            self.build_code_block(&for_loop.code_block, compiler_context)?;
            if let Statement::Expressions(step_expr) = for_loop.iterate.as_ref() {
                self.build_expression(step_expr.as_ref(), compiler_context)?;
            }
            self.builder.llvm_builder.build_unconditional_branch(cond_block);
            self.builder.llvm_builder.position_at_end(after_block);
        }
        Ok(())
    }

    fn build_while_loop<'a>(&mut self, while_loop: &WhileLoop, compiler_context: &mut CompilerContext<'ctx>) -> Result<()> {
        let cond_block = self.builder.llvm_context.insert_basic_block_after(self.builder.llvm_builder.get_insert_block().unwrap(), "");
        let body_block = self.builder.llvm_context.insert_basic_block_after(cond_block, "");
        let after_block = self.builder.llvm_context.insert_basic_block_after(body_block, "");
        self.builder.llvm_builder.build_unconditional_branch(cond_block);
        self.builder.llvm_builder.position_at_end(cond_block);
        if let Some(cond_expr) = &while_loop.condition {
            if let LEBasicValueEnum::IntegerValue(int) = self.build_expression(cond_expr.as_ref(), compiler_context)? {
                let cond_boolean = self.builder.llvm_builder.build_int_cast(int.llvm_value, self.builder.llvm_context.bool_type(), "");
                self.builder.llvm_builder.build_conditional_branch(cond_boolean, body_block, after_block);
            } else {
                return Err(CompileError::type_mismatched("".into(), "".into()).into());
            }
        } else {
            self.builder.llvm_builder.build_unconditional_branch(body_block);
        }
        self.builder.llvm_builder.position_at_end(body_block);
        self.build_code_block(&while_loop.code_block, compiler_context)?;
        self.builder.llvm_builder.build_unconditional_branch(cond_block);
        self.builder.llvm_builder.position_at_end(after_block);
        Ok(())
    }

    fn build_if_statement<'a>(&mut self, statement: &IfStatement, compiler_context: &mut CompilerContext<'ctx>) -> Result<()> {
        let cond_value = self.build_expression(statement.cond.as_ref(), compiler_context)?;
        let then_block = self.builder.llvm_context.insert_basic_block_after(self.builder.llvm_builder.get_insert_block().unwrap(), "");
        let else_block = self.builder.llvm_context.insert_basic_block_after(then_block, "");
        let merge_block = self.builder.llvm_context.insert_basic_block_after(else_block, "");
        if let LEBasicValueEnum::IntegerValue(int) = cond_value {
            let cond_boolean = self.builder.llvm_builder.build_int_cast(int.llvm_value, self.builder.llvm_context.bool_type(), "");
            self.builder.llvm_builder.build_conditional_branch(cond_boolean, then_block, else_block);
        } else {
            return Err(CompileError::type_mismatched("".into(), "".into()).into());
        }
        self.builder.llvm_builder.position_at_end(then_block);
        let is_then_return_block = self.build_code_block(&statement.then_block, compiler_context)?;
        if !is_then_return_block {
            self.builder.llvm_builder.build_unconditional_branch(merge_block);
        }
        self.builder.llvm_builder.position_at_end(else_block);
        if let Some(el) = &statement.else_block {
            let is_else_return_block = self.build_code_block(el, compiler_context)?;
            if !is_else_return_block {
                self.builder.llvm_builder.build_unconditional_branch(merge_block);
            }
        } else {
            self.builder.llvm_builder.build_unconditional_branch(merge_block);
        }
        self.builder.llvm_builder.position_at_end(merge_block);
        Ok(())
    }

    fn build_function_prototype<'a>(&mut self, module: &Module<'ctx>, prototype: &ExternalFunction, compiler_context: &mut CompilerContext<'ctx>) -> Result<FunctionValue<'ctx>> {
        let mut param_types = vec![];
        for param_type in prototype.param_types.iter() {
            let ty = compiler_context.get_type(param_type)?;
            param_types.push(BasicMetadataTypeEnum::from(ty.to))

        }
        let external_function = match &prototype.return_type {
            None => {
                self.builder.llvm_context.void_type().fn_type(&param_types, false)
            }
            Some(type_name) => {
                let ty = compiler_context.get_type(type_name)?;
                match ty {
                    LEBasicTypeGenericRef::IntegerType(i) => {i.llvm_type.fn_type(&param_types, false)}
                    LEBasicTypeGenericRef::FloatType(i) => {i.llvm_type.fn_type(&param_types, false)}
                    LEBasicTypeGenericRef::PointerType(i) => {i.llvm_type.fn_type(&param_types, false)}
                    LEBasicTypeGenericRef::ArrayType(i) => {i.llvm_type.fn_type(&param_types, false)}
                    LEBasicTypeGenericRef::StructType(i) => {i.llvm_type.fn_type(&param_types, false)}
                    LEBasicTypeGenericRef::VectorType(i) => {i.llvm_type.fn_type(&param_types, false)}
                    LEBasicTypeGenericRef::UnitType => {self.builder.llvm_context.void_type().fn_type(&param_types, false)}
                }
            }
        };
        let external_function_value = module.add_function(&prototype.name, external_function, Some(Linkage::External));
        compiler_context.symbols.insert_global_function(prototype.name.clone(), external_function_value)?;
        Ok(external_function_value)
    }

    fn build_return_block<'a>(&mut self, return_block: BasicBlock, return_variable: Option<LEPointerValue>, compiler_context: &mut CompilerContext<'ctx>) -> Result<()> {
        self.builder.llvm_builder.position_at_end(return_block);
        if let Some(value) = return_variable {
            let value = self.builder.build_load(value)?;
            match value {
                LEBasicValueEnum::NumericValue(number) => {
                    let basic_value = number.to_basic_value_enum();
                    self.builder.llvm_builder.build_return(Some(&basic_value));
                    Ok(())
                }
                _ => { unimplemented!() }
            }
        } else {
            self.builder.llvm_builder.build_return(None);
            Ok(())
        }
    }

    fn build_function<'a>(&mut self, module: &Module<'ctx>, function_node: &FunctionDefinition, compiler_context: &mut CompilerContext<'ctx>) -> Result<FunctionValue<'ctx>> {
        let function_value = self.build_function_prototype(module, &function_node.prototype, compiler_context)?;
        let entry = self.builder.llvm_context.append_basic_block(function_value, "");
        let return_block = self.builder.llvm_context.append_basic_block(function_value, "");
        let return_type = function_value.get_type().get_return_type();
        if let Some(none_void_type) = return_type {
            self.builder.llvm_builder.position_at_end(entry);
            let return_variable = self.builder.build_alloca(none_void_type.into())?;
            compiler_context.set_current_context(function_value, Some(return_variable), return_block);
            self.builder.llvm_builder.position_at_end(return_block);
            self.build_return_block(return_block, Some(return_variable), compiler_context)?;
        } else {
            compiler_context.set_current_context(function_value, None, return_block);
            self.build_return_block(return_block, None, compiler_context)?;
        }
        self.builder.llvm_builder.position_at_end(entry);
        compiler_context.push_block_table();
        let function = &function_value;
        let names = &function_node.param_names;
        for (param, name) in function.get_param_iter().zip(names) {
            self.create_local_variable(name, param.get_type().into(), param.into(), compiler_context)?;
        }

        let is_return_block = self.build_code_block(&function_node.code_block, compiler_context)?;
        if !is_return_block {
            self.builder.llvm_builder.build_unconditional_branch(return_block);
        }
        compiler_context.pop_block_table();
        Ok(function_value)
    }

    pub fn build_global_variable<'a>(&mut self, name: &str, ty: &'a LEBasicTypeGenericRef<'ctx>, module: &Module<'ctx>, initial_value: LEBasicValueEnum<'ctx, 'a>) -> Result<LEPointerValue<'ctx>> {
        let value_type = initial_value.get_type();
        if ty != value_type {
            return Err(CompileError::type_mismatched(ty.to_string(), value_type.to_string()).into());
        }
        let variable = match ty {
            LEBasicTypeGenericRef::IntegerType(t) => {module.add_global(t.llvm_type, Some(AddressSpace::Global), "")}
            LEBasicTypeGenericRef::FloatType(t) => {module.add_global(t.llvm_type, Some(AddressSpace::Global), "")}
            LEBasicTypeGenericRef::PointerType(t) => {module.add_global(t.llvm_type, Some(AddressSpace::Global), "")}
            LEBasicTypeGenericRef::ArrayType(t) => {module.add_global(t.llvm_type, Some(AddressSpace::Global), "")}
            LEBasicTypeGenericRef::StructType(t) => {module.add_global(t.llvm_type, Some(AddressSpace::Global), "")}
            LEBasicTypeGenericRef::VectorType(t) => {module.add_global(t.llvm_type, Some(AddressSpace::Global), "")}
            _=>unimplemented!()
        };
        let le_variable = LEPointerValue { ty:*ty, llvm_value: variable.as_pointer_value() };
        self.insert_global_variable(name.into(), le_variable, self.current_pos)?;
        Ok(le_variable)
    }

    pub fn create_local_variable<'a>(&mut self, name: &str, target_type: LEBasicTypeGenericRef<'ctx>, initial_value: LEBasicValueEnum<'ctx, 'a>, compiler_context: &mut CompilerContext<'ctx>) -> Result<LEPointerValue<'ctx>> {
        let current_insert_block = self.builder.llvm_builder.get_insert_block().unwrap();
        let parent_function = self.current_function.unwrap();
        let entry_block = parent_function.get_first_basic_block().unwrap();
        if let Some(first_instruction) = entry_block.get_first_instruction() {
            self.builder.llvm_builder.position_at(entry_block, &first_instruction);
        } else {
            self.builder.llvm_builder.position_at_end(entry_block);
        }
        let value_type = initial_value.get_type();
        let le_variable = self.builder.build_alloca(&target_type)?;
        self.builder.llvm_builder.position_at_end(current_insert_block);
        let cast_value = self.builder.build_cast(initial_value, &target_type)
            .map_err(|_| CompileError::type_mismatched(target_type.to_string(), value_type.to_string()))?;
        self.builder.build_store(le_variable, cast_value, self.current_pos)?;
        compiler_context.insert_local_variable(name.into(), le_variable)?;
        Ok(le_variable)
    }

    fn generate_all_functions<'a>(&mut self, module: &Module<'ctx>, ast: &Ast, compiler_context: &mut CompilerContext<'ctx>) -> Result<()> {
        for function_prototype in ast.extern_functions.iter() {
            let name = function_prototype.name.clone();
            self.build_function_prototype(module, function_prototype, compiler_context)?;
        }
        for function_node in ast.function_definitions.iter() {
            let name = function_node.prototype.name.clone();
            self.build_function(module, function_node, compiler_context)?;
        }
        Ok(())
    }

    fn generate_all_global_variables<'a>(&mut self, module: &Module<'ctx>, ast: &Ast, compiler_context: &mut CompilerContext<'ctx>) -> Result<()> {
        for variable in ast.globals.iter() {
            let ty = self.get_type(&variable.prototype.type_name)?;
            self.build_global_variable(
                &variable.prototype.name,
                ty,
                module,
                self.build_expression(variable.value.as_ref(), compiler_context)?,
            )?;
        }
        Ok(())
    }

    pub fn compile(&mut self, module: &Module<'ctx>, ast: &Ast) -> Result<()> {
        let mut compiler_context = CompilerContext::new(self.builder.llvm_context);
        self.generate_all_global_variables(module, ast, &mut compiler_context)?;
        self.generate_all_functions(module, ast, &mut compiler_context)?;
        Ok(())
    }

    pub fn create(context: &'ctx Context) -> Self {
        let llvm_builder = context.create_builder();
        Self {
            builder: LEBuilder::new(context, llvm_builder),
            current_pos: Position { line: 0 },
        }
    }
}


