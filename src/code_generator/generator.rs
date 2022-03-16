use anyhow::Result;
use inkwell::{AddressSpace, IntPredicate, OptimizationLevel};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::object_file::Symbol;
use inkwell::passes::{PassManager, PassManagerBuilder};
use inkwell::types::{AnyTypeEnum, BasicMetadataTypeEnum, BasicType, BasicTypeEnum, IntType};
use inkwell::values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, FunctionValue, GlobalValue, IntValue, PointerValue};

use crate::ast::{Ast, BinaryOperatorNode, CodeBlock, Expr, FunctionCallNode, IdentifierNode, IfStatement, NumberLiteralNode, Statement, VariableNode};
use crate::code_generator::SymbolTable;
use crate::error::CompileError;
use crate::lexer::{LELexer, Number, Operator};

pub struct ModuleCodeGenerator<'s> {
    pub context: &'s Context,
    pub builder: Builder<'s>,
    pub optimizer: PassManagerBuilder,
    global_symbols: SymbolTable<'s>,
    local_symbols: SymbolTable<'s>,
}


impl<'s> ModuleCodeGenerator<'s> {
    pub fn compile_module(&mut self, module: &Module<'s>, mut tokens: LELexer) -> Result<()> {
        let ast = Ast::from_tokens(tokens)?;
        eprintln!("{:?}",ast);
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
        // self.builder.build_return(Some(&i32_type.const_zero());
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
                Ok(self.builder.build_int_add::<IntValue>(left, right, "").as_basic_value_enum())
            }
            Operator::Sub => {
                let left = self.generate_value(lhs)?.into_int_value();
                let right = self.generate_value(rhs)?.into_int_value();
                Ok(self.builder.build_int_sub::<IntValue>(left, right, "").as_basic_value_enum())
            }
            Operator::Mul => {
                let left = self.generate_value(lhs)?.into_int_value();
                let right = self.generate_value(rhs)?.into_int_value();
                Ok(self.builder.build_int_mul::<IntValue>(left, right, "").as_basic_value_enum())
            }
            Operator::Div => {
                let left = self.generate_value(lhs)?.into_int_value();
                let right = self.generate_value(rhs)?.into_int_value();
                Ok(self.builder.build_int_signed_div::<IntValue>(left, right, "").as_basic_value_enum())
            }
            Operator::Assign => {
                if let Expr::Identifier(identifier) = lhs.as_ref(){
                    let left_value = self.get_variable(&identifier.name)?;
                    let value = self.generate_value(lhs.as_ref())?;
                    self.builder.build_store(left_value,value);
                    Ok(value)
                }else{
                    Err(CompileError::can_only_assign_variable(format!("{:?}",lhs)).into())
                }
            }
            Operator::Equal => {
                let left = self.generate_value(lhs)?.into_int_value();
                let right = self.generate_value(rhs)?.into_int_value();
                Ok(self.builder.build_int_compare(IntPredicate::EQ, left, right, "").as_basic_value_enum())
            }
            _ => { unimplemented!() }
        }
    }

    fn generate_identifier_expression_node(&self, value: &IdentifierNode) -> Result<BasicValueEnum> {
        let name = &value.name;
        let pointer_value = self.get_variable(name)?;
        Ok(self.builder.build_load(pointer_value,""))
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
        match self
            .local_symbols
            .types
            .get(type_name) {
            Some(s) => { Ok(*s) }
            None => {
                match self.global_symbols.types.get(type_name) {
                    None => { Err(CompileError::identifier_is_not_type(type_name.into()).into()) }
                    Some(s) => { Ok(*s) }
                }
            }
        }
    }

    fn get_variable(&self, variable_name: &str) -> Result<PointerValue<'s>> {
        match self
            .local_symbols
            .variables
            .get(variable_name) {
            Some(s) => { Ok(*s) }
            None => {
                match self.global_symbols.variables.get(variable_name) {
                    None => { Err(CompileError::identifier_is_not_variable(variable_name.into()).into()) }
                    Some(s) => { Ok(*s) }
                }
            }
        }
    }

    fn get_function(&self, function_name: &str) -> Result<FunctionValue<'s>> {
        match self
            .local_symbols
            .functions
            .get(function_name) {
            Some(s) => { Ok(*s) }
            None => {
                match self.global_symbols.functions.get(function_name) {
                    None => { Err(CompileError::identifier_is_not_variable(function_name.into()).into()) }
                    Some(s) => { Ok(*s) }
                }
            }
        }
    }


    fn generate_functions(&mut self, module: &Module<'s>, ast: &Ast) -> Result<()> {
        let pass_manager = PassManager::create(module);
        self.optimizer.populate_function_pass_manager(&pass_manager);
        for (name, function_node) in ast.functions.iter() {
            self.local_symbols = SymbolTable::default();
            let param_types = function_node.params.iter().map(|a| {
                BasicMetadataTypeEnum::from(self.get_type(&a.type_name).unwrap().into_int_type())
            }).collect::<Vec<_>>();
            let return_type = self.get_type(&function_node.return_type)?;
            let fn_type = return_type.fn_type(&param_types, false);
            let function_value = module.add_function(name, fn_type, None);
            let entry = self.context.append_basic_block(function_value,"");
            self.builder.position_at_end(entry);
            for (index, param) in function_value.get_param_iter().enumerate() {
                let pointer_value = self.builder.build_alloca(param.get_type(), "");
                self.builder.build_store(pointer_value,param);
                self.local_symbols.variables.insert(function_node.params[index].identifier.clone(), pointer_value);
            }
            self.generate_local_variables(&function_node.code_block.variables)?;
            self.generate_code_block_without_variables(&function_node.code_block)?;
            pass_manager.run_on(&function_value);
        }
        Ok(())
    }

    fn generate_local_variables(&mut self, variables: &[VariableNode]) -> Result<()> {
        for variable in variables {
            let pointer_value = self.builder.build_alloca(self.get_type(&variable.type_name)?, "");
            self.builder.build_store(pointer_value,self.generate_value(variable.value.as_ref())?);
            self.local_symbols.variables.insert(variable.name.clone(), pointer_value);
        }
        Ok(())
    }


    fn generate_code_block_without_variables(&self,code_block:&CodeBlock)->Result<bool>{
        let mut is_return_block = false;
        for statement in code_block.expression.iter() {
            match statement {
                Statement::Expressions(expr) => {
                    self.generate_value(expr)?;
                }
                Statement::Return(expr) => {
                    let value = self.generate_value(expr)?;
                    self.builder.build_return(Some(&value));
                    is_return_block=true;
                }
                Statement::If(if_expr) => {
                    self.generate_if_statement(if_expr)?;
                    is_return_block=false;
                }
                _ => { unimplemented!()}
            }
        }
        Ok(is_return_block)
    }



    fn generate_if_statement(&self, statement: &IfStatement) -> Result<()> {
        let cond_value = self.generate_value(statement.cond.as_ref())?;
        let then_block = self.context.insert_basic_block_after(self.builder.get_insert_block().unwrap(),"");
        let else_block = self.context.insert_basic_block_after(then_block,"");
        let merge_block = self.context.insert_basic_block_after(else_block,"");
        let cond_boolean = self.builder.build_int_cast(cond_value.into_int_value(), self.context.bool_type(), "");
        self.builder.build_conditional_branch(cond_boolean,then_block,else_block);
        self.builder.position_at_end(then_block);
        let mut is_return_block = self.generate_code_block_without_variables(&statement.then_block)?;
        if !is_return_block{
            self.builder.build_unconditional_branch(merge_block);
        }
        is_return_block=false;
        self.builder.position_at_end(else_block);
        if let Some(el) = &statement.else_block{
            is_return_block = self.generate_code_block_without_variables(el)?;
        }
        if !is_return_block{
            self.builder.build_unconditional_branch(merge_block);
        }
        self.builder.position_at_end(merge_block);
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
            global_symbols: SymbolTable::default(),
            local_symbols: SymbolTable::default(),
        };
        s.optimizer.set_optimization_level(OptimizationLevel::None);
        s.global_symbols.types.insert("i8".into(), context.i8_type().as_basic_type_enum());
        s.global_symbols.types.insert("i16".into(), context.i16_type().as_basic_type_enum());
        s.global_symbols.types.insert("i32".into(), context.i32_type().as_basic_type_enum());
        s.global_symbols.types.insert("i64".into(), context.i64_type().as_basic_type_enum());
        s.global_symbols.types.insert("u8".into(), context.i8_type().as_basic_type_enum());
        s.global_symbols.types.insert("u16".into(), context.i16_type().as_basic_type_enum());
        s.global_symbols.types.insert("u32".into(), context.i32_type().as_basic_type_enum());
        s.global_symbols.types.insert("u64".into(), context.i64_type().as_basic_type_enum());
        s.global_symbols.types.insert("f32".into(), context.f32_type().as_basic_type_enum());
        s.global_symbols.types.insert("f64".into(), context.f64_type().as_basic_type_enum());
        s
    }
}


