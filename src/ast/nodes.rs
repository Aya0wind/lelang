use std::{clone, io};
use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt::{Debug, Display, format, Formatter};
use std::io::Write;

use ptree::{Style, TreeBuilder, TreeItem};

use crate::ast::parser::{parse_extern_function_prototype, parse_function, parse_structure, parse_variable_declaration};
use crate::error::{LEError, Result, SyntaxError, TokenType};
use crate::lexer::{KeyWord, LELexer, LEToken, Position};
use crate::lexer::{Number, Operator};

pub trait ASTNode {
    fn pos(&self) -> Position;
    fn build_tree_format(&self, builder: &mut TreeBuilder);
}

#[derive(Debug, Clone)]
pub struct AnonymousFunction {
    pub prototype: FunctionPrototype,
    pub param_names: Vec<String>,
    pub code_block: CodeBlock,
    pub pos: Position,
}


#[derive(Debug, Clone)]
pub struct BinaryOpExpression {
    pub op: Operator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub function_name: Identifier,
    pub params: Vec<Expr>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct ForLoop {
    pub init_statement: Box<Statement>,
    pub condition: Box<Statement>,
    pub iterate: Box<Statement>,
    pub code_block: CodeBlock,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct WhileLoop {
    pub condition: Box<Expr>,
    pub code_block: CodeBlock,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct FunctionPrototype {
    pub identifier: Identifier,
    pub param_types: Vec<TypeDeclarator>,
    pub return_type: Option<TypeDeclarator>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub prototype: FunctionPrototype,
    pub param_names: Vec<String>,
    pub code_block: CodeBlock,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub statements: Vec<Statement>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub cond: Box<Expr>,
    pub then_block: CodeBlock,
    pub else_block: Option<CodeBlock>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct NumberLiteral {
    pub number: Number,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct UnaryOpExpression {
    pub op: Operator,
    pub expr: Box<Expr>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub prototype: VariablePrototype,
    pub value: Box<Expr>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct VariablePrototype {
    pub type_declarator: Option<TypeDeclarator>,
    pub identifier: Identifier,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct ArrayInitializer {
    pub elements: Vec<Expr>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct ArrayDeclarator {
    pub element_type: TypeDeclarator,
    pub len: u32,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct Structure {
    pub identifier: Identifier,
    pub members: Vec<(String, TypeDeclarator)>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct StructureInitializer {
    pub structure_name: Identifier,
    pub member_initial_values: Vec<(String, Box<Expr>)>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub content: String,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub enum TypeDeclarator {
    TypeIdentifier(Identifier),
    Array(Box<ArrayDeclarator>),
    Reference(Box<TypeDeclarator>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    BinaryOperator(BinaryOpExpression),
    UnaryOperator(UnaryOpExpression),
    NumberLiteral(NumberLiteral),
    ArrayInitializer(ArrayInitializer),
    StructureInitializer(StructureInitializer),
    StringLiteral(StringLiteral),
    Identifier(Identifier),
    CallExpression(FunctionCall),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expressions(Box<Expr>),
    VariableDefinition(Variable),
    Return(Box<Expr>),
    If(IfStatement),
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
    Void(Position),
}

impl ASTNode for StringLiteral {
    fn pos(&self) -> Position {
        self.pos.clone()
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        builder.add_empty_child(format!("`{}`", self.content));
    }
}

impl ASTNode for ArrayInitializer {
    fn pos(&self) -> Position {
        self.pos.clone()
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        builder.begin_child("elements".to_string());
        for e in &self.elements {
            e.build_tree_format(builder);
        }
        builder.end_child();
    }
}

impl ASTNode for NumberLiteral {
    fn pos(&self) -> Position {
        self.pos.clone()
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        builder.add_empty_child(
            match self.number {
                Number::Integer(v) => { format!("`{}`", v) }
                Number::Float(v) => { format!("`{}`", v) }
            }
        );
    }
}

impl ASTNode for StructureInitializer {
    fn pos(&self) -> Position {
        self.pos.clone()
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        builder.begin_child("identifier".to_string());
        self.structure_name.build_tree_format(builder);
        builder.end_child();

        builder.begin_child("members".to_string());
        for (member_name, value) in &self.member_initial_values {
            builder.begin_child(member_name.to_string());
            value.build_tree_format(builder);
            builder.end_child();
        }
        builder.end_child();
    }
}

impl ASTNode for ForLoop {
    fn pos(&self) -> Position {
        self.pos.clone()
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        builder.begin_child("condition".to_string());
        self.condition.build_tree_format(builder);
        builder.end_child();

        builder.begin_child("init".to_string());
        self.init_statement.build_tree_format(builder);
        builder.end_child();

        builder.begin_child("iterate".to_string());
        self.iterate.build_tree_format(builder);
        builder.end_child();

        builder.begin_child("code_block".to_string());
        self.code_block.build_tree_format(builder);
        builder.end_child();
    }
}

impl ASTNode for Structure {
    fn pos(&self) -> Position {
        self.pos.clone()
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        builder.begin_child("identifier".to_string());
        self.identifier.build_tree_format(builder);
        builder.end_child();

        builder.begin_child("members".to_string());
        for (member_name, member_type) in &self.members {
            builder.begin_child(member_name.clone());
            member_type.build_tree_format(builder);
            builder.end_child();
        }
        builder.end_child();
    }
}

impl ASTNode for ArrayDeclarator {
    fn pos(&self) -> Position {
        self.pos.clone()
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        builder.begin_child("element_type".to_string());
        self.element_type.build_tree_format(builder);
        builder.end_child();

        builder.begin_child("length".to_string());
        builder.add_empty_child(self.len.to_string());
        builder.end_child();
    }
}

impl ASTNode for Identifier {
    fn pos(&self) -> Position {
        self.pos.clone()
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        builder.add_empty_child(format!("`{}`", self.name));
    }
}

impl ASTNode for VariablePrototype {
    fn pos(&self) -> Position {
        self.pos.clone()
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        builder.begin_child("identifier".to_string());
        self.identifier.build_tree_format(builder);
        builder.end_child();

        builder.begin_child("type".to_string());
        self.identifier.build_tree_format(builder);
        builder.end_child();
    }
}

impl ASTNode for Variable {
    fn pos(&self) -> Position {
        self.pos.clone()
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        builder.begin_child("prototype".to_string());
        self.prototype.build_tree_format(builder);
        builder.end_child();

        builder.begin_child("value".to_string());
        self.value.build_tree_format(builder);
        builder.end_child();
    }
}

impl ASTNode for UnaryOpExpression {
    fn pos(&self) -> Position {
        self.pos.clone()
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        builder.begin_child("operator".to_string());
        builder.add_empty_child(self.op.to_string());
        builder.end_child();

        builder.begin_child("expression".to_string());
        self.expr.build_tree_format(builder);
        builder.end_child();
    }
}

impl ASTNode for IfStatement {
    fn pos(&self) -> Position {
        self.pos.clone()
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        builder.begin_child("condition".to_string());
        self.cond.build_tree_format(builder);
        builder.end_child();

        builder.begin_child("then_block".to_string());
        self.then_block.build_tree_format(builder);
        builder.end_child();

        if let Some(else_block) = &self.else_block {
            builder.begin_child("else_block".to_string());
            else_block.build_tree_format(builder);
            builder.end_child();
        }
    }
}

impl ASTNode for CodeBlock {
    fn pos(&self) -> Position {
        self.pos.clone()
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        builder.begin_child("statements".to_string());
        for statement in &self.statements {
            statement.build_tree_format(builder);
        }
        builder.end_child();
    }
}

impl ASTNode for FunctionDefinition {
    fn pos(&self) -> Position {
        self.pos.clone()
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        builder.begin_child("prototype".to_string());
        self.prototype.build_tree_format(builder);
        builder.end_child();

        builder.begin_child("param_names".to_string());
        for name in &self.param_names {
            builder.add_empty_child(format!("`{}`", name));
        }
        builder.end_child();

        builder.begin_child("body".to_string());
        self.code_block.build_tree_format(builder);
        builder.end_child();
    }
}

impl ASTNode for FunctionPrototype {
    fn pos(&self) -> Position {
        self.pos.clone()
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        builder.begin_child("identifier".to_string());
        self.identifier.build_tree_format(builder);
        builder.end_child();

        builder.begin_child("param_types".to_string());
        for ty in &self.param_types {
            ty.build_tree_format(builder);
        }
        builder.end_child();

        builder.begin_child("return_type".to_string());
        if let Some(ret) = &self.return_type {
            ret.build_tree_format(builder);
        } else {
            builder.add_empty_child("void".to_string());
        }
        builder.end_child();
    }
}

impl ASTNode for BinaryOpExpression {
    fn pos(&self) -> Position {
        self.pos.clone()
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        builder.begin_child("left".to_string());
        self.left.build_tree_format(builder);
        builder.end_child();

        builder.begin_child("operator".to_string());
        builder.add_empty_child(format!("` {} `", self.op));
        builder.end_child();

        builder.begin_child("right".to_string());
        self.right.build_tree_format(builder);
        builder.end_child();
    }
}

impl ASTNode for FunctionCall {
    fn pos(&self) -> Position {
        self.pos.clone()
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        builder.begin_child("function_name".to_string());
        self.function_name.build_tree_format(builder);
        builder.end_child();

        builder.begin_child("right".to_string());
        for param in &self.params {
            param.build_tree_format(builder);
        }
        builder.end_child();
    }
}

impl ASTNode for WhileLoop {
    fn pos(&self) -> Position {
        self.pos.clone()
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        builder.begin_child("condition".to_string());
        self.condition.build_tree_format(builder);
        builder.end_child();

        builder.begin_child("body".to_string());
        self.code_block.build_tree_format(builder);
        builder.end_child();
    }
}

impl ASTNode for TypeDeclarator {
    fn pos(&self) -> Position {
        match self {
            TypeDeclarator::TypeIdentifier(e) => { e.pos() }
            TypeDeclarator::Array(e) => { e.pos() }
            TypeDeclarator::Reference(e) => { e.pos() }
        }
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        match self {
            TypeDeclarator::TypeIdentifier(t) => {
                builder.begin_child("type_identifier".to_string());
                t.build_tree_format(builder);
                builder.end_child();
            }
            TypeDeclarator::Array(t) => {
                builder.begin_child("array_type".to_string());
                t.build_tree_format(builder);
                builder.end_child();
            }
            TypeDeclarator::Reference(t) => {
                builder.begin_child("reference_type".to_string());
                t.build_tree_format(builder);
                builder.end_child();
            }
        };
    }
}

impl ASTNode for Expr {
    fn pos(&self) -> Position {
        match self {
            Expr::BinaryOperator(e) => { e.pos() }
            Expr::UnaryOperator(e) => { e.pos() }
            Expr::NumberLiteral(e) => { e.pos() }
            Expr::ArrayInitializer(e) => { e.pos() }
            Expr::StructureInitializer(e) => { e.pos() }
            Expr::StringLiteral(e) => { e.pos() }
            Expr::Identifier(e) => { e.pos() }
            Expr::CallExpression(e) => { e.pos() }
        }
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        match self {
            Expr::BinaryOperator(e) => {
                builder.begin_child("binary_op_expr".to_string());
                e.build_tree_format(builder);
                builder.end_child();
            }
            Expr::UnaryOperator(e) => {
                builder.begin_child("unary_op_expr".to_string());
                e.build_tree_format(builder);
                builder.end_child();
            }
            Expr::NumberLiteral(e) => {
                builder.begin_child("number_literal".to_string());
                e.build_tree_format(builder);
                builder.end_child();
            }
            Expr::ArrayInitializer(e) => {
                builder.begin_child("array_initializer".to_string());
                e.build_tree_format(builder);
                builder.end_child();
            }
            Expr::StructureInitializer(e) => {
                builder.begin_child("struct_initializer".to_string());
                e.build_tree_format(builder);
                builder.end_child();
            }
            Expr::StringLiteral(e) => {
                builder.begin_child("string_literal".to_string());
                e.build_tree_format(builder);
                builder.end_child();
            }
            Expr::Identifier(e) => {
                builder.begin_child("identifier".to_string());
                e.build_tree_format(builder);
                builder.end_child();
            }
            Expr::CallExpression(e) => {
                builder.begin_child("call_expr".to_string());
                e.build_tree_format(builder);
                builder.end_child();
            }
        }
    }
}

impl ASTNode for Statement {
    fn pos(&self) -> Position {
        match self {
            Statement::Expressions(e) => { e.pos() }
            Statement::VariableDefinition(e) => { e.pos() }
            Statement::Return(e) => { e.pos() }
            Statement::If(e) => { e.pos() }
            Statement::ForLoop(e) => { e.pos() }
            Statement::WhileLoop(e) => { e.pos() }
            Statement::Void(p) => { p.clone() }
        }
    }

    fn build_tree_format(&self, builder: &mut TreeBuilder) {
        match self {
            Statement::Expressions(s) => {
                builder.begin_child("expr".to_string());
                s.build_tree_format(builder);
                builder.end_child();
            }
            Statement::VariableDefinition(s) => {
                builder.begin_child("variable_definition".to_string());
                s.build_tree_format(builder);
                builder.end_child();
            }
            Statement::Return(s) => {
                builder.begin_child("return_expr".to_string());
                s.build_tree_format(builder);
                builder.end_child();
            }
            Statement::If(s) => {
                builder.begin_child("if_statement".to_string());
                s.build_tree_format(builder);
                builder.end_child();
            }
            Statement::ForLoop(s) => {
                builder.begin_child("for_loop".to_string());
                s.build_tree_format(builder);
                builder.end_child();
            }
            Statement::WhileLoop(s) => {
                builder.begin_child("while_loop".to_string());
                s.build_tree_format(builder);
                builder.end_child();
            }
            Statement::Void(_) => {
                builder.add_empty_child("void statement".to_string());
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct Ast {
    pub globals_variables: Vec<Variable>,
    pub globals_structures: Vec<Structure>,
    pub function_definitions: Vec<FunctionDefinition>,
    pub extern_functions: Vec<FunctionPrototype>,
}


impl Ast {
    pub fn from_lexer(tokens: LELexer) -> Result<Self> {
        let mut ast = Self { globals_variables: vec![], globals_structures: vec![], function_definitions: vec![], extern_functions: vec![] };
        ast.parse(tokens)?;
        Ok(ast)
    }

    fn parse(&mut self, mut lexer: LELexer) -> Result<()> {
        loop {
            let next_token = lexer.current();
            match next_token {
                None => { break; }
                Some(token) => {
                    if let LEToken::KeyWord(keyword) = token {
                        match keyword {
                            KeyWord::Declare => {
                                lexer.consume_keyword()?;
                                let function_prototype = parse_extern_function_prototype(&mut lexer)?;
                                lexer.consume_semicolon()?;
                                self.extern_functions.push(function_prototype);
                            }
                            KeyWord::FunctionDefine => {
                                let function = parse_function(&mut lexer)?;
                                self.function_definitions.push(function);
                            }
                            KeyWord::VariableDeclare => {
                                let variable = parse_variable_declaration(&mut lexer)?;
                                self.globals_variables.push(variable);
                            }
                            KeyWord::StructureDeclare => {
                                let structure = parse_structure(&mut lexer)?;
                                self.globals_structures.push(structure);
                            }
                            _ => {
                                return Err(LEError::new_syntax_error(
                                    SyntaxError::unexpect_token(vec![TokenType::FunctionDefine, TokenType::FunctionDeclare], LEToken::KeyWord(keyword)),
                                    lexer.pos()));
                            }
                        }
                    } else {
                        return Err(SyntaxError::unexpect_token(vec![TokenType::FunctionDefine, TokenType::FunctionDeclare], token.clone()).to_leerror(lexer.pos()));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn print_to_with_root_name<W: io::Write>(&self, w: W, root: String) -> std::io::Result<()> {
        let mut builder = TreeBuilder::new(root);
        let builder_ref = &mut builder;

        builder_ref.begin_child("external functions".to_string());
        for (index, f) in self.extern_functions.iter().enumerate() {
            builder_ref.begin_child(index.to_string());
            f.build_tree_format(builder_ref);
            builder_ref.end_child();
        }
        builder_ref.end_child();


        builder_ref.begin_child("function_definitions".to_string());
        for (index, f) in self.function_definitions.iter().enumerate() {
            builder_ref.begin_child(index.to_string());
            f.build_tree_format(builder_ref);
            builder_ref.end_child();
        }
        builder_ref.end_child();


        builder_ref.begin_child("globals_structures".to_string());
        for (index, f) in self.globals_structures.iter().enumerate() {
            builder_ref.begin_child(index.to_string());
            f.build_tree_format(builder_ref);
            builder_ref.end_child();
        }
        builder_ref.end_child();


        builder_ref.begin_child("globals_variables".to_string());
        for (index, f) in self.globals_variables.iter().enumerate() {
            builder_ref.begin_child(index.to_string());
            f.build_tree_format(builder_ref);
            builder_ref.end_child();
        }
        builder_ref.end_child();

        let tree = builder_ref.build();

        let mut tree_string = String::new();

        ptree::write_tree(&tree, w)
    }
}
