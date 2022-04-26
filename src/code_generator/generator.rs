use inkwell::AddressSpace;
use inkwell::basic_block::BasicBlock;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::BasicValueEnum;

use crate::ast::nodes::*;
use crate::code_generator;
use crate::code_generator::builder::*;
use crate::code_generator::builder::binary_operator_builder::{CompareBinaryOperator, LogicBinaryOperator};
use crate::code_generator::builder::expression::Expression;
use crate::code_generator::context::LEContext;
use crate::error::{CompileError, LEError, Result};
use crate::lexer::{Number, Operator, Position};

macro_rules! le_error {
    ($expr:expr,$pos:expr) => {
        $expr.map_err(|e|e.to_leerror($pos))
    };
}


pub struct CodeGenerator<'ctx> {
    pub context: LEContext<'ctx>,
    pub builder: LEBuilder<'ctx>,
}

impl<'ctx> CodeGenerator<'ctx> {
    fn build_expression(&mut self, value: &Expr) -> Result<Expression<'ctx>> {
        match value {
            Expr::UnaryOperator(n) => { self.build_unary_operator_expression(n) }
            Expr::BinaryOperator(n) => { self.build_binary_operator_expression(n) }
            Expr::NumberLiteral(n) => { self.build_number_literal_expression(n) }
            Expr::CallExpression(n) => { self.build_call_expression(n) }
            Expr::Identifier(n) => { self.build_identifier_expression(n) }
            Expr::ArrayInitializer(n) => { self.build_array_initializer(n) }
            Expr::StructureInitializer(n) => { self.build_structure_initializer(n) }
            _ => { unimplemented!() }
            // Expr::StringLiteral(n) => { Ok(Some(self.build_string_literal(n)?)) }
        }
    }


    fn build_structure_initializer(&mut self, expr: &StructureInitializer) -> Result<Expression<'ctx>> {
        let struct_type = self.context.get_generic_type(&TypeDeclarator::TypeIdentifier(expr.structure_name.clone()))
            .map_err(|e| LEError::new_compile_error(e, expr.structure_name.pos.clone()))?;
        if let LEBasicTypeEnum::Struct(struct_type) = struct_type {
            let initializer_member_num = expr.member_initial_values.len();
            if struct_type.get_llvm_type().get_field_types().len() != initializer_member_num {
                return Err(CompileError::TypeMismatched { expect: struct_type.to_string(), found: expr.structure_name.name.clone() }.to_leerror(expr.pos()));
            }
            let mut value_array = vec![];
            for (name, initial_value) in expr.member_initial_values.iter() {
                let value = self.build_expression(initial_value.as_ref())?;
                value_array.push((struct_type.get_member_offset(name).unwrap(), value));
            }
            value_array.sort_unstable_by(|x, y| x.0.cmp(&y.0));
            let struct_llvm_value = &value_array
                .into_iter()
                .map(|x| self.builder.read_expression(&self.context, x.1).unwrap().to_llvm_basic_value_enum()).collect::<Vec<_>>();
            let struct_value = struct_type.get_llvm_type().const_named_struct(struct_llvm_value);
            Ok(Expression::Right(LEStructValue { ty: struct_type, llvm_value: struct_value }.to_le_value_enum()))
        } else {
            Err(LEError::new_compile_error(CompileError::TypeMismatched { expect: "Struct".into(), found: struct_type.name().into() }, expr.pos.clone()))
        }
    }

    fn build_unary_operator_expression(&mut self, expr: &UnaryOpExpression) -> Result<Expression<'ctx>> {
        let value = self.build_expression(expr.expr.as_ref())?;
        match expr.op {
            Operator::Plus => {
                Ok(value)
            }
            Operator::Sub => {
                Ok(Expression::Right(self.builder.build_neg(&self.context, value).map_err(|e| e.to_leerror(expr.pos.clone()))?))
            }
            // Operator::Not => {
            //
            // }
            // Operator::Rev => {}
            _ => { unimplemented!() }
        }
    }


    fn build_array_initializer(&mut self, value: &ArrayInitializer) -> Result<Expression<'ctx>> {
        if value.elements.is_empty() {
            Err(CompileError::NotAllowZeroLengthArray.to_leerror(value.pos.clone()))
        } else {
            let mut array_values = vec![];
            for v in value.elements.iter() {
                let expr = self.build_expression(v)?;
                array_values.push(self.builder.read_expression(&self.context, expr).map_err(|e| e.to_leerror(v.pos()))?);
            }
            let first_value = array_values.first().unwrap();
            let element_type = LEBasicValue::get_le_type(first_value);
            let array_type = LEBasicType::get_array_type(&element_type, value.elements.len() as u32);
            for (index, others) in array_values.iter().enumerate() {
                if others.get_le_type() != element_type {
                    return Err(CompileError::TypeMismatched {
                        expect: first_value.get_le_type().to_string(),
                        found: others.get_le_type().to_string(),
                    }.to_leerror(value.elements[index].pos()));
                }
            }

            match element_type {
                LEBasicTypeEnum::Integer(t) => {
                    let array_initial_values = array_values.into_iter().map(|v| v.try_into().unwrap()).collect::<Vec<LEIntegerValue>>();
                    Ok(Expression::Right(t.const_array(&array_initial_values).to_le_value_enum()))
                }
                LEBasicTypeEnum::Float(t) => {
                    let array_initial_values = array_values.into_iter().map(|v| v.try_into().unwrap()).collect::<Vec<LEFloatValue>>();
                    Ok(Expression::Right(t.const_array(&array_initial_values).to_le_value_enum()))
                }
                LEBasicTypeEnum::Bool(t) => {
                    let array_initial_values = array_values.into_iter().map(|v| v.try_into().unwrap()).collect::<Vec<LEBoolValue>>();
                    Ok(Expression::Right(t.const_array(&array_initial_values).to_le_value_enum()))
                }
                LEBasicTypeEnum::Pointer(t) => {
                    let array_initial_values = array_values.into_iter().map(|v| v.try_into().unwrap()).collect::<Vec<LEPointerValue>>();
                    Ok(Expression::Right(t.const_array(&array_initial_values).to_le_value_enum()))
                }
                LEBasicTypeEnum::Array(t) => {
                    let array_initial_values = array_values.into_iter().map(|v| v.try_into().unwrap()).collect::<Vec<LEArrayValue>>();
                    Ok(Expression::Right(t.const_array(&array_initial_values).to_le_value_enum()))
                }
                LEBasicTypeEnum::Struct(t) => {
                    let array_initial_values = array_values.into_iter().map(|v| v.try_into().unwrap()).collect::<Vec<LEStructValue>>();
                    Ok(Expression::Right(t.const_array(&array_initial_values).to_le_value_enum()))
                }
                LEBasicTypeEnum::Vector(t) => {
                    let array_initial_values = array_values.into_iter().map(|v| v.try_into().unwrap()).collect::<Vec<LEVectorValue>>();
                    Ok(Expression::Right(t.const_array(&array_initial_values).to_le_value_enum()))
                }
            }
        }
    }

    fn build_binary_operator_expression(&mut self, value: &BinaryOpExpression) -> Result<Expression<'ctx>> {
        match value.op {
            Operator::Plus => {
                let left = self.build_expression(value.left.as_ref())?;
                let right = self.build_expression(value.right.as_ref())?;
                Ok(Expression::Right(le_error!(self.builder.build_add(&self.context,left, right),value.pos())?))
            }
            Operator::Sub => {
                let left = self.build_expression(value.left.as_ref())?;
                let right = self.build_expression(value.right.as_ref())?;
                Ok(Expression::Right(le_error!(self.builder.build_sub(&self.context,left, right),value.pos())?))
            }
            Operator::Mul => {
                let left = self.build_expression(value.left.as_ref())?;
                let right = self.build_expression(value.right.as_ref())?;
                Ok(Expression::Right(le_error!(self.builder.build_mul(&self.context,left, right),value.pos())?))
            }
            Operator::Div => {
                let left = self.build_expression(value.left.as_ref())?;
                let right = self.build_expression(value.right.as_ref())?;
                Ok(Expression::Right(le_error!(self.builder.build_div(&self.context,left, right),value.pos())?))
            }
            Operator::Assign => {
                let left = self.build_expression(value.left.as_ref())?;
                let right = self.build_expression(value.right.as_ref())?;
                Ok(Expression::Left(le_error!(self.builder.build_assign(&self.context,left, right),value.pos())?))
            }
            Operator::Equal => {
                let left = self.build_expression(value.left.as_ref())?;
                let right = self.build_expression(value.right.as_ref())?;
                Ok(Expression::Right(le_error!(self.builder.build_compare(&self.context,left, right, CompareBinaryOperator::Equal),value.pos())?.to_le_value_enum()))
            }
            Operator::NotEqual => {
                let left = self.build_expression(value.left.as_ref())?;
                let right = self.build_expression(value.right.as_ref())?;
                Ok(Expression::Right(le_error!(self.builder.build_compare(&self.context,left, right, CompareBinaryOperator::NotEqual), value.pos())?.to_le_value_enum()))
            }
            Operator::GreaterThan => {
                let left = self.build_expression(value.left.as_ref())?;
                let right = self.build_expression(value.right.as_ref())?;
                Ok(Expression::Right(le_error!(self.builder.build_compare(&self.context,left, right, CompareBinaryOperator::GreaterThan), value.pos())?.to_le_value_enum()))
            }
            Operator::LessThan => {
                let left = self.build_expression(value.left.as_ref())?;
                let right = self.build_expression(value.right.as_ref())?;
                Ok(Expression::Right(le_error!(self.builder.build_compare(&self.context,left, right, CompareBinaryOperator::LessThan), value.pos())?.to_le_value_enum()))
            }
            Operator::GreaterOrEqualThan => {
                let left = self.build_expression(value.left.as_ref())?;
                let right = self.build_expression(value.right.as_ref())?;
                Ok(Expression::Right(le_error!(self.builder.build_compare(&self.context,left, right, CompareBinaryOperator::GreaterOrEqualThan),value.pos())?.to_le_value_enum()))
            }
            Operator::LessOrEqualThan => {
                let left = self.build_expression(value.left.as_ref())?;
                let right = self.build_expression(value.right.as_ref())?;
                Ok(Expression::Right(le_error!(self.builder.build_compare(&self.context,left, right, CompareBinaryOperator::LessOrEqualThan),value.pos())?.to_le_value_enum()))
            }
            Operator::Dot => {
                let left = self.build_expression(value.left.as_ref())?;
                if let Expr::Identifier(identifier) = value.right.as_ref() {
                    Ok(Expression::Left(le_error!(self.builder.build_dot(&self.context,left, &identifier.name),value.pos())?))
                } else {
                    Err(CompileError::NoSuitableBinaryOperator {
                        op: Operator::Dot,
                        left_type: "".to_string(),
                        right_type: "".to_string(),
                    }.to_leerror(value.pos.clone()))
                }
            }
            Operator::And => {
                let left = self.build_expression(value.left.as_ref())?;
                let right = self.build_expression(value.right.as_ref())?;
                Ok(Expression::Right(le_error!(self.builder.build_binary_logic(&self.context,left, right, LogicBinaryOperator::And),value.pos())?.to_le_value_enum()))
            }
            Operator::Or => {
                let left = self.build_expression(value.left.as_ref())?;
                let right = self.build_expression(value.right.as_ref())?;
                Ok(Expression::Right(le_error!(self.builder.build_binary_logic(&self.context,left, right, LogicBinaryOperator::Or),value.pos())?.to_le_value_enum()))
            }
            Operator::Xor => {
                let left = self.build_expression(value.left.as_ref())?;
                let right = self.build_expression(value.right.as_ref())?;
                Ok(Expression::Right(le_error!(self.builder.build_binary_logic(&self.context,left, right, LogicBinaryOperator::Xor),value.pos())?.to_le_value_enum()))
            }

            Operator::Mod => {
                let left = self.build_expression(value.left.as_ref())?;
                let right = self.build_expression(value.right.as_ref())?;
                Ok(Expression::Right(le_error!(self.builder.build_mod(&self.context,left, right),value.pos())?.to_le_value_enum()))
            }
            Operator::Cast => {
                let left = self.build_expression(value.left.as_ref())?;
                if let Expr::Identifier(type_identifier) = value.right.as_ref() {
                    let ty = le_error!(self.context.get_generic_type(&TypeDeclarator::TypeIdentifier(type_identifier.clone())),type_identifier.pos())?;
                    Ok(Expression::Right(le_error!(self.builder.build_cast(&self.context,left,ty),value.right.pos())?))
                } else {
                    Err(CompileError::ExpressionIsNotType { pos: value.right.pos() }.to_leerror(value.right.pos()))
                }
            }
            _ => { unimplemented!() }
        }
    }

    fn build_identifier_expression(&mut self, value: &Identifier) -> Result<Expression<'ctx>> {
        match value.name.as_str() {
            "true" => { Ok(Expression::Right(self.context.bool_type().const_true_value().to_le_value_enum())) }
            "false" => { Ok(Expression::Right(self.context.bool_type().const_false_value().to_le_value_enum())) }
            _ => { Ok(Expression::Left(self.context.get_variable(&value.name).map_err(|e| e.to_leerror(value.pos.clone()))?)) }
        }
    }

    fn build_number_literal_expression(&mut self, value: &NumberLiteral) -> Result<Expression<'ctx>> {
        match value.number {
            Number::Integer(i) => {
                let ty = self.context.i32_type();
                let value = ty.get_llvm_type().const_int(i, true);
                Ok(Expression::Right(LEIntegerValue { ty, llvm_value: value }.to_le_value_enum()))
            }
            Number::Float(f) => {
                let ty = self.context.double_type();
                let value = ty.get_llvm_type().const_float(f);
                Ok(Expression::Right(LEFloatValue { ty, llvm_value: value }.to_le_value_enum()))
            }
        }
    }

    fn build_call_expression(&mut self, value: &FunctionCall) -> Result<Expression<'ctx>> {
        let function = le_error!(self.context.compiler_context.get_function(&value.function_name.name),value.function_name.pos())?;
        let mut params = vec![];
        for param in value.params.iter() {
            params.push(self.build_expression(param)?)
        }
        self.builder.build_call(&self.context, function, &params).map_err(|e| e.to_leerror(value.pos.clone()))
    }

    fn build_local_variable_definition(&mut self, variable: &Variable) -> Result<Expression<'ctx>> {
        let initial_value_expr = self.build_expression(variable.value.as_ref())?;
        let initial_value = le_error!(self.builder.read_expression(&self.context, initial_value_expr),variable.value.pos())?;
        let initial_type = LEBasicValue::get_le_type(&initial_value);

        let current_insert_block = self.builder.llvm_builder.get_insert_block().unwrap();
        let parent_function = self.context.compiler_context.current_function.unwrap();
        let entry_block = parent_function.get_first_basic_block().unwrap();
        if let Some(first_instruction) = entry_block.get_first_instruction() {
            self.builder.llvm_builder.position_at(entry_block, &first_instruction);
        } else {
            self.builder.llvm_builder.position_at_end(entry_block);
        }

        let pointer = if let Some(variable_type) = &variable.prototype.type_declarator {
            let target_type = le_error!(self.context.get_generic_type(variable_type),variable_type.pos())?;
            if target_type == initial_type {
                self.builder.build_alloca(&self.context, initial_type)
            } else {
                return Err(CompileError::TypeMismatched { expect: target_type.to_string(), found: initial_type.to_string() }.to_leerror(variable.pos()));
            }
        } else {
            self.builder.build_alloca(&self.context, initial_type)
        };
        self.builder.llvm_builder.position_at_end(current_insert_block);
        le_error!(self.builder.build_store(&self.context, pointer.clone(),initial_value),variable.pos())?;
        le_error!(self.context.insert_local_variable(
            variable.prototype.identifier.name.clone(),
            pointer,variable.prototype.identifier.pos()),
            variable.prototype.identifier.pos()
        )?;
        Ok(Expression::Unit)
    }

    fn build_code_block(&mut self, code_block: &CodeBlock) -> Result<bool> {
        for statement in code_block.statements.iter() {
            match statement {
                Statement::Expressions(expr) => {
                    self.build_expression(expr)?;
                }
                Statement::Return(expr) => {
                    let value = self.build_expression(expr)?;
                    self.build_return(value, expr.pos())?;
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
                Statement::Void(_) => {}
                Statement::WhileLoop(while_loop) => {
                    self.build_while_loop(while_loop)?;
                }
            }
        }
        Ok(false)
    }

    fn build_return(&mut self, expr: Expression, position: Position) -> Result<()> {
        let return_variable = &self.context.compiler_context.return_variable;
        let return_block = self.context.compiler_context.return_block.unwrap();
        let return_value = le_error!(self.builder.read_expression(&self.context, expr),position.clone())?;
        if let Some(return_variable) = return_variable {
            le_error!(self.builder.build_store(&self.context, return_variable.clone(), return_value),position)?;
        }
        self.builder.llvm_builder.build_unconditional_branch(return_block);
        Ok(())
    }

    fn build_for_loop(&mut self, for_loop: &ForLoop) -> Result<()> {
        let loop_variable = for_loop.init_statement.as_ref();

        if let Statement::Expressions(cond_expr) = for_loop.condition.as_ref() {
            let cond_block = self.context.llvm_context.insert_basic_block_after(self.builder.llvm_builder.get_insert_block().unwrap(), "");
            let body_block = self.context.llvm_context.insert_basic_block_after(cond_block, "");
            let after_block = self.context.llvm_context.insert_basic_block_after(body_block, "");
            self.context.compiler_context.push_block_table();
            if let Statement::VariableDefinition(v) = loop_variable {
                self.build_local_variable_definition(v)?;
            }
            self.builder.llvm_builder.build_unconditional_branch(cond_block);
            self.builder.llvm_builder.position_at_end(cond_block);
            let cond = self.build_expression(cond_expr.as_ref())?;
            let cond_value = le_error!(self.builder.read_expression(&self.context, cond),cond_expr.pos())?;
            if let LEBasicValueEnum::Bool(bool_cond) = cond_value {
                self.builder.llvm_builder.build_conditional_branch(bool_cond.get_llvm_value(), body_block, after_block);
            } else {
                return Err(CompileError::TypeMismatched {
                    expect: "bool".into(),
                    found: LEBasicValue::get_le_type(&cond_value).to_string(),
                }.to_leerror(cond_expr.pos()));
            }
            self.builder.llvm_builder.position_at_end(body_block);
            self.build_code_block(&for_loop.code_block)?;

            if let Statement::Expressions(step_expr) = for_loop.iterate.as_ref() {
                self.build_expression(step_expr.as_ref())?;
            }
            self.builder.llvm_builder.build_unconditional_branch(cond_block);
            self.builder.llvm_builder.position_at_end(after_block);
            self.context.compiler_context.pop_block_table();
        }
        Ok(())
    }

    fn build_while_loop(&mut self, while_loop: &WhileLoop) -> Result<()> {
        let cond_block = self.context.llvm_context.insert_basic_block_after(self.builder.llvm_builder.get_insert_block().unwrap(), "");
        let body_block = self.context.llvm_context.insert_basic_block_after(cond_block, "");
        let after_block = self.context.llvm_context.insert_basic_block_after(body_block, "");
        self.builder.llvm_builder.build_unconditional_branch(cond_block);
        self.builder.llvm_builder.position_at_end(cond_block);
        self.context.compiler_context.push_block_table();
        let cond = self.build_expression(while_loop.condition.as_ref())?;
        let cond_value = le_error!(self.builder.read_expression(&self.context, cond),while_loop.condition.pos())?;
        if let LEBasicValueEnum::Bool(bool_cond) = cond_value {
            self.builder.llvm_builder.build_conditional_branch(bool_cond.get_llvm_value(), body_block, after_block);
        } else {
            return Err(CompileError::TypeMismatched {
                expect: "bool".into(),
                found: LEBasicValue::get_le_type(&cond_value).to_string(),
            }.to_leerror(while_loop.condition.pos()));
        }
        self.builder.llvm_builder.position_at_end(body_block);
        self.build_code_block(&while_loop.code_block)?;
        self.builder.llvm_builder.build_unconditional_branch(cond_block);
        self.builder.llvm_builder.position_at_end(after_block);
        self.context.compiler_context.pop_block_table();
        Ok(())
    }

    fn build_if_statement(&mut self, statement: &IfStatement) -> Result<()> {
        let then_block = self.context.llvm_context.insert_basic_block_after(self.builder.llvm_builder.get_insert_block().unwrap(), "");
        let else_block = self.context.llvm_context.insert_basic_block_after(then_block, "");
        let merge_block = self.context.llvm_context.insert_basic_block_after(else_block, "");
        let cond = self.build_expression(statement.cond.as_ref())?;
        let cond_value = le_error!(self.builder.read_expression(&self.context, cond),statement.cond.pos())?;
        if let LEBasicValueEnum::Bool(bool_cond) = cond_value {
            self.builder.llvm_builder.build_conditional_branch(bool_cond.get_llvm_value(), then_block, else_block);
        } else {
            return Err(CompileError::TypeMismatched {
                expect: "bool".into(),
                found: LEBasicValue::get_le_type(&cond_value).to_string(),
            }.to_leerror(statement.cond.pos()));
        }
        self.builder.llvm_builder.position_at_end(then_block);
        self.context.compiler_context.push_block_table();
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
        self.context.compiler_context.pop_block_table();
        Ok(())
    }

    fn build_function_prototype(&mut self, module: &Module<'ctx>, prototype: &FunctionPrototype) -> Result<LEFunctionValue<'ctx>> {
        let mut param_llvm_metadata_types = vec![];
        let mut param_types = vec![];
        for param_type in prototype.param_types.iter() {
            let ty = self.context.get_generic_type(param_type).map_err(|e| e.to_leerror(param_type.pos()))?;
            param_types.push(ty.clone());
            param_llvm_metadata_types.push(BasicMetadataTypeEnum::from(ty.get_llvm_basic_type()))
        }
        let return_type;
        let external_function = match &prototype.return_type {
            None => {
                return_type = None;
                self.context.llvm_context.void_type().fn_type(&param_llvm_metadata_types, false)
            }
            Some(type_declarator) => {
                let ty = self.context.get_generic_type(type_declarator).map_err(|e| e.to_leerror(type_declarator.pos()))?;
                return_type = Some(ty.clone());
                match ty {
                    LEBasicTypeEnum::Integer(i) => { i.get_llvm_type().fn_type(&param_llvm_metadata_types, false) }
                    LEBasicTypeEnum::Bool(i) => { i.get_llvm_type().fn_type(&param_llvm_metadata_types, false) }
                    LEBasicTypeEnum::Float(i) => { i.get_llvm_type().fn_type(&param_llvm_metadata_types, false) }
                    LEBasicTypeEnum::Pointer(i) => { i.get_llvm_type().fn_type(&param_llvm_metadata_types, false) }
                    LEBasicTypeEnum::Array(i) => { i.get_llvm_type().fn_type(&param_llvm_metadata_types, false) }
                    LEBasicTypeEnum::Struct(i) => { i.get_llvm_type().fn_type(&param_llvm_metadata_types, false) }
                    LEBasicTypeEnum::Vector(i) => { i.get_llvm_type().fn_type(&param_llvm_metadata_types, false) }
                }
            }
        };
        let external_function_value = module.add_function(&prototype.identifier.name, external_function, Some(Linkage::External));
        let function_type = LEFunctionType::new(external_function, return_type, param_types);
        let le_function = LEFunctionValue { ty: function_type, llvm_value: external_function_value };
        le_error!(self.context.insert_global_function(
            prototype.identifier.name.clone(),
            le_function.clone(),
            prototype.identifier.pos()),prototype.identifier.pos())?;
        Ok(le_function)
    }

    fn build_return_block(&mut self, return_block: BasicBlock, return_variable: Option<LEPointerValue>) -> Result<()> {
        self.builder.llvm_builder.position_at_end(return_block);
        if let Some(value) = return_variable {
            let value = self.builder.build_load(&self.context, value);
            self.builder.llvm_builder.build_return(Some(&value.to_llvm_basic_value_enum()));
            Ok(())
        } else {
            self.builder.llvm_builder.build_return(None);
            Ok(())
        }
    }

    fn build_function(&mut self, module: &Module<'ctx>, function_node: &FunctionDefinition) -> Result<LEFunctionValue<'ctx>> {
        let function_value = self.build_function_prototype(module, &function_node.prototype)?;
        let entry = self.context.llvm_context.append_basic_block(function_value.llvm_value, "");
        let return_block = self.context.llvm_context.append_basic_block(function_value.llvm_value, "");
        let return_type = function_value.ty.return_type();
        if let Some(none_void_type) = return_type {
            self.builder.llvm_builder.position_at_end(entry);
            let return_variable = self.builder.build_alloca(&self.context, none_void_type);
            self.context.compiler_context.set_current_context(function_value.llvm_value, Some(return_variable.clone()), return_block);
            self.builder.llvm_builder.position_at_end(return_block);
            self.build_return_block(return_block, Some(return_variable.clone()))?;
        } else {
            self.context.compiler_context.set_current_context(function_value.llvm_value, None, return_block);
            self.build_return_block(return_block, None)?;
        }
        self.builder.llvm_builder.position_at_end(entry);
        self.context.compiler_context.push_block_table();
        let function = &function_value;
        let names = &function_node.param_names;
        let param_value_iter = function.llvm_value.get_param_iter();
        let param_type_iter = function.ty.param_types();
        for (index, ((param, name), param_type)) in param_value_iter.zip(names).zip(param_type_iter).enumerate() {
            let param_pos = function_node.prototype.param_types[index].pos();
            let param_value = le_error!(LEBasicValueEnum::from_type_and_llvm_value(param_type.clone(), param),param_pos.clone())?;
            let param_pointer = self.builder.build_alloca_with_initial_value(&self.context, param_value);
            le_error!(self.context.insert_local_variable(name.clone(),param_pointer,param_pos.clone()),param_pos)?;
        }

        let is_return_block = self.build_code_block(&function_node.code_block)?;
        if !is_return_block {
            self.builder.llvm_builder.build_unconditional_branch(return_block);
        }
        self.context.compiler_context.pop_block_table();
        Ok(function_value)
    }


    pub fn create_global_function(&mut self, name: String, function: LEFunctionValue<'ctx>, position: Position) -> Result<LEFunctionValue<'ctx>> {
        le_error!(self.context.insert_global_function(name,function,position.clone()),position)
    }


    fn generate_all_functions(&mut self, module: &Module<'ctx>, ast: &Ast) -> Result<()> {
        for function_prototype in ast.extern_functions.iter() {
            let name = function_prototype.identifier.clone();
            self.build_function_prototype(module, function_prototype)?;
        }
        for function_node in ast.function_definitions.iter() {
            let name = function_node.prototype.identifier.clone();
            self.build_function(module, function_node)?;
        }
        Ok(())
    }

    fn generate_all_global_variables(&mut self, module: &Module<'ctx>, ast: &Ast) -> Result<()> {
        for variable in ast.globals_variables.iter() {
            let expr_value = self.build_expression(variable.value.as_ref())?;
            let initial = le_error!(self.builder.read_expression(&self.context,expr_value),variable.value.pos())?;
            let initial_type = LEBasicValue::get_le_type(&initial);
            if let Some(exact_type) = &variable.prototype.type_declarator {
                let target_type = le_error!(self.context.get_generic_type(exact_type),exact_type.pos())?;
                if target_type == initial_type {
                    self.create_global_variable(
                        variable.prototype.identifier.name.clone(),
                        initial,
                        module,
                        variable.prototype.identifier.pos(),
                    )?;
                } else {
                    return Err(CompileError::TypeMismatched { expect: target_type.to_string(), found: initial_type.to_string() }.to_leerror(variable.pos()));
                }
            } else {
                self.create_global_variable(
                    variable.prototype.identifier.name.clone(),
                    initial,
                    module,
                    variable.prototype.identifier.pos(),
                )?;
            }
        }
        Ok(())
    }

    pub fn create_global_variable(&mut self, name: String, initial_value: LEBasicValueEnum<'ctx>, module: &Module<'ctx>, position: Position) -> Result<LEPointerValue<'ctx>> {
        let pointer = self.builder.build_global_alloca_with_initial_value(initial_value, module, Some(AddressSpace::Global));
        le_error!(self.context.insert_global_variable(name, pointer.clone(), position.clone()),position)?;
        Ok(pointer)
    }

    fn generate_all_global_structures(&mut self, module: &Module, ast: &Ast) -> Result<()> {
        for structure in ast.globals_structures.iter() {
            let mut names = vec![];
            let mut types = vec![];
            for (name, ty) in structure.members.iter() {
                names.push(name.as_str());
                types.push(le_error!(self.context.get_generic_type(ty),ty.pos())?);
            }
            let structure_type = LEStructType::from_llvm_type(&self.context, &names, &types);
            le_error!(self.context.insert_global_type(
                structure.identifier.name.clone(),
                structure_type.to_le_type_enum(),
                structure.identifier.pos(),
            ),structure.identifier.pos())?;
        }
        Ok(())
    }


    pub fn compile(&mut self, module: &Module<'ctx>, ast: &Ast) -> Result<()> {
        self.generate_all_global_variables(module, ast)?;
        self.generate_all_global_structures(module, ast)?;
        self.generate_all_functions(module, ast)?;
        Ok(())
    }

    pub fn create(context: &'ctx Context) -> Self {
        let llvm_builder = context.create_builder();
        Self {
            builder: LEBuilder::new(llvm_builder),
            context: LEContext::new(context),
        }
    }
}


