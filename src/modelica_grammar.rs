//! This module provides implementations for converting the automatic Parol AST
//! (Abstract Syntax Tree) into the internal AST representation (`crate::ir::ast`).
//!
//! The module includes various `TryFrom` implementations for converting between
//! the `modelica_grammar_trait` types and the internal `ir::ast` types. These
//! conversions handle different Modelica constructs such as stored definitions,
//! class definitions, equations, statements, expressions, and more.
//!
//! # Key Features
//!
//! - **StoredDefinition Conversion**: Converts a `StoredDefinition` from the
//!   `modelica_grammar_trait` to the internal `ir::ast::StoredDefinition`.
//! - **Token Conversion**: Converts tokens from the Parol runtime to the internal
//!   `ir::ast::Token` representation.
//! - **ClassDefinition Conversion**: Handles the conversion of Modelica class
//!   definitions, including long and short class specifiers.
//! - **Composition and ElementList**: Converts Modelica compositions and element
//!   lists into their internal representations.
//! - **Equation and Algorithm Sections**: Converts Modelica equation and algorithm
//!   sections, including initial and non-initial variants.
//! - **Expressions and Statements**: Provides detailed conversions for Modelica
//!   expressions and statements, including binary, unary, and function call expressions.
//! - **Component References**: Converts Modelica component references and their parts
//!   into the internal representation.
//!
//! # Notes
//!
//! - Some features, such as `extends`, `der`, `enum class specifier`, and others,
//!   are marked as `todo!` and are not yet implemented.
//! - The module uses `anyhow::Error` for error handling during conversions.
//! - Default values are provided for certain constructs, such as default start values
//!   for components based on their type.
//!
//! # Example Usage
//!
//! This module is primarily used internally by the `ModelicaGrammar` struct, which
//! implements the `modelica_grammar_trait::ModelicaGrammarTrait` trait. The `stored_definition`
//! method is used to parse and store the converted Modelica AST.
use crate::ir;
use crate::modelica_grammar_trait;
use indexmap::IndexMap;
use parol_runtime::{Result, Token};
use std::fmt::{Debug, Display, Error, Formatter};

//-----------------------------------------------------------------------------
impl TryFrom<&modelica_grammar_trait::StoredDefinition> for ir::ast::StoredDefinition {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::StoredDefinition,
    ) -> std::result::Result<Self, Self::Error> {
        let mut def = ir::ast::StoredDefinition {
            classes: IndexMap::new(),
            ..Default::default()
        };
        for class in &ast.stored_definition_list {
            def.classes.insert(
                class.class_definition.name.text.clone(),
                class.class_definition.clone(),
            );
        }
        def.within = match &ast.stored_definition_opt {
            Some(within) => match &within.stored_definition_opt1 {
                Some(within) => Some(within.name.clone()),
                None => None,
            },
            None => None,
        };
        Ok(def)
    }
}

//-----------------------------------------------------------------------------
impl TryFrom<&Token<'_>> for ir::ast::Token {
    type Error = anyhow::Error;

    fn try_from(value: &Token<'_>) -> std::result::Result<Self, Self::Error> {
        Ok(ir::ast::Token {
            text: value.text().to_string(),
            location: ir::ast::Location {
                start_line: value.location.start_line,
                start_column: value.location.start_column,
                end_line: value.location.end_line,
                end_column: value.location.end_column,
                start: value.location.start,
                end: value.location.end,
                file_name: value
                    .location
                    .file_name
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            },
            token_number: value.token_number,
            token_type: value.token_type,
        })
    }
}

//-----------------------------------------------------------------------------
impl TryFrom<&modelica_grammar_trait::ClassDefinition> for ir::ast::ClassDefinition {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::ClassDefinition,
    ) -> std::result::Result<Self, Self::Error> {
        match &ast.class_specifier {
            modelica_grammar_trait::ClassSpecifier::LongClassSpecifier(long) => {
                match &long.long_class_specifier {
                    modelica_grammar_trait::LongClassSpecifier::StandardClassSpecifier(
                        class_specifier,
                    ) => {
                        let spec = &class_specifier.standard_class_specifier;
                        Ok(ir::ast::ClassDefinition {
                            name: spec.name.clone(),
                            extends: spec.composition.extends.clone(),
                            classes: spec.composition.classes.clone(),
                            imports: spec.composition.imports.clone(),
                            equations: spec.composition.equations.clone(),
                            algorithms: spec.composition.algorithms.clone(),
                            initial_equations: spec.composition.initial_equations.clone(),
                            initial_algorithms: spec.composition.initial_algorithms.clone(),
                            components: spec.composition.components.clone(),
                            encapsulated: ast.class_definition_opt.is_some(),
                        })
                    }
                    modelica_grammar_trait::LongClassSpecifier::ExtendsClassSpecifier(..) => {
                        todo!("extends")
                    }
                }
            }
            modelica_grammar_trait::ClassSpecifier::DerClassSpecifier(_spec) => todo!("der"),
            modelica_grammar_trait::ClassSpecifier::ShortClassSpecifier(short) => {
                match &short.short_class_specifier {
                    modelica_grammar_trait::ShortClassSpecifier::EnumClassSpecifier(_spec) => {
                        todo!("enum class specifier")
                    }
                    modelica_grammar_trait::ShortClassSpecifier::TypeClassSpecifier(_spec) => {
                        //spec.type_class_specifier.base_prefix.
                        todo!("type class specifier");
                    }
                }
            }
        }
    }
}

//-----------------------------------------------------------------------------
#[derive(Debug, Default, Clone)]
#[allow(unused)]
pub struct Composition {
    pub extends: Vec<ir::ast::Extend>,
    pub imports: Vec<String>,
    pub classes: IndexMap<String, ir::ast::ClassDefinition>,
    pub components: IndexMap<String, ir::ast::Component>,
    pub equations: Vec<ir::ast::Equation>,
    pub initial_equations: Vec<ir::ast::Equation>,
    pub algorithms: Vec<Vec<ir::ast::Statement>>,
    pub initial_algorithms: Vec<Vec<ir::ast::Statement>>,
}

impl TryFrom<&modelica_grammar_trait::Composition> for Composition {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::Composition,
    ) -> std::result::Result<Self, Self::Error> {
        let mut comp = Composition {
            ..Default::default()
        };

        comp.components = ast.element_list.components.clone();
        comp.classes = ast.element_list.classes.clone();
        comp.imports = ast.element_list.imports.clone();
        comp.extends = ast.element_list.extends.clone();

        for comp_list in &ast.composition_list {
            match &comp_list.composition_list_group {
                modelica_grammar_trait::CompositionListGroup::PublicElementList(_elem_list) => {
                    todo!("public element list")
                }
                modelica_grammar_trait::CompositionListGroup::ProtectedElementList(_elem_list) => {
                    todo!("protected element list")
                }
                modelica_grammar_trait::CompositionListGroup::EquationSection(eq_sec) => {
                    let sec = &eq_sec.equation_section;
                    for eq in &sec.equations {
                        if sec.initial {
                            comp.initial_equations.push(eq.clone());
                        } else {
                            comp.equations.push(eq.clone());
                        }
                    }
                }
                modelica_grammar_trait::CompositionListGroup::AlgorithmSection(alg_sec) => {
                    let sec = &alg_sec.algorithm_section;
                    let mut algo = vec![];
                    for stmt in &sec.statements {
                        algo.push(stmt.clone());
                    }
                    if sec.initial {
                        comp.initial_algorithms.push(algo);
                    } else {
                        comp.algorithms.push(algo);
                    }
                }
            }
        }
        Ok(comp)
    }
}

//-----------------------------------------------------------------------------
#[derive(Debug, Default, Clone)]
#[allow(unused)]
pub struct ElementList {
    pub classes: IndexMap<String, ir::ast::ClassDefinition>,
    pub components: IndexMap<String, ir::ast::Component>,
    pub imports: Vec<String>,
    pub extends: Vec<ir::ast::Extend>,
}

impl TryFrom<&modelica_grammar_trait::ElementList> for ElementList {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::ElementList,
    ) -> std::result::Result<Self, Self::Error> {
        let mut def = ElementList {
            components: IndexMap::new(),
            ..Default::default()
        };
        for elem_list in &ast.element_list_list {
            match &elem_list.element {
                modelica_grammar_trait::Element::ElementDefinition(edef) => {
                    match &edef.element_definition.element_definition_group {
                        modelica_grammar_trait::ElementDefinitionGroup::ClassDefinition(class) => {
                            def.classes.insert(
                                class.class_definition.name.text.clone(),
                                class.class_definition.clone(),
                            );
                        }
                        modelica_grammar_trait::ElementDefinitionGroup::ComponentClause(clause) => {
                            let connection =
                                match &clause.component_clause.type_prefix.type_prefix_opt {
                                    Some(opt) => match &opt.type_prefix_opt_group {
                                        modelica_grammar_trait::TypePrefixOptGroup::Flow(flow) => {
                                            ir::ast::Connection::Flow(flow.flow.flow.clone())
                                        }
                                        modelica_grammar_trait::TypePrefixOptGroup::Stream(
                                            stream,
                                        ) => ir::ast::Connection::Stream(
                                            stream.stream.stream.clone(),
                                        ),
                                    },
                                    None => ir::ast::Connection::Empty,
                                };

                            let variability = match &clause
                                .component_clause
                                .type_prefix
                                .type_prefix_opt0
                            {
                                Some(opt) => match &opt.type_prefix_opt0_group {
                                    modelica_grammar_trait::TypePrefixOpt0Group::Constant(c) => {
                                        ir::ast::Variability::Constant(c.constant.constant.clone())
                                    }
                                    modelica_grammar_trait::TypePrefixOpt0Group::Discrete(c) => {
                                        ir::ast::Variability::Discrete(c.discrete.discrete.clone())
                                    }
                                    modelica_grammar_trait::TypePrefixOpt0Group::Parameter(c) => {
                                        ir::ast::Variability::Parameter(
                                            c.parameter.parameter.clone(),
                                        )
                                    }
                                },
                                None => ir::ast::Variability::Empty,
                            };

                            let causality =
                                match &clause.component_clause.type_prefix.type_prefix_opt1 {
                                    Some(opt) => match &opt.type_prefix_opt1_group {
                                        modelica_grammar_trait::TypePrefixOpt1Group::Input(c) => {
                                            ir::ast::Causality::Input(c.input.input.clone())
                                        }
                                        modelica_grammar_trait::TypePrefixOpt1Group::Output(c) => {
                                            ir::ast::Causality::Output(c.output.output.clone())
                                        }
                                    },
                                    None => ir::ast::Causality::Empty,
                                };

                            for c in &clause.component_clause.component_list.components {
                                let mut value = ir::ast::Component {
                                    name: c.declaration.ident.text.clone(),
                                    type_name: clause.component_clause.type_specifier.name.clone(),
                                    variability: variability.clone(),
                                    causality: causality.clone(),
                                    connection: connection.clone(),
                                    description: c.description.description_string.tokens.clone(),
                                    start: ir::ast::Expression::Terminal {
                                        terminal_type: ir::ast::TerminalType::UnsignedReal,
                                        token: ir::ast::Token {
                                            text: "0.0".to_string(),
                                            ..Default::default()
                                        },
                                    },
                                };

                                // set default start value
                                value.start = match value.type_name.to_string().as_str() {
                                    "Real" => ir::ast::Expression::Terminal {
                                        terminal_type: ir::ast::TerminalType::UnsignedReal,
                                        token: ir::ast::Token {
                                            text: "0.0".to_string(),
                                            ..Default::default()
                                        },
                                    },
                                    "Integer" => ir::ast::Expression::Terminal {
                                        terminal_type: ir::ast::TerminalType::UnsignedInteger,
                                        token: ir::ast::Token {
                                            text: "0".to_string(),
                                            ..Default::default()
                                        },
                                    },
                                    "Bool" => ir::ast::Expression::Terminal {
                                        terminal_type: ir::ast::TerminalType::Bool,
                                        token: ir::ast::Token {
                                            text: "0".to_string(),
                                            ..Default::default()
                                        },
                                    },
                                    _ => ir::ast::Expression::Empty {},
                                };

                                // handle for component modification
                                if let Some(modif) = &c.declaration.declaration_opt0 {
                                    match &modif.modification {
                                        modelica_grammar_trait::Modification::ClassModificationModificationOpt(
                                            class_mod,
                                        ) => {
                                            let modif = &*(class_mod.class_modification);
                                            match &modif.class_modification_opt {
                                                Some(_opt) => {
                                                    //opt.argument_list.args
                                                },
                                                None => {},
                                            }
                                        }
                                        modelica_grammar_trait::Modification::EquModificationExpression(
                                            eq_mod,
                                        ) => {
                                            match &eq_mod.modification_expression {
                                                modelica_grammar_trait::ModificationExpression::Expression(expr) => {
                                                    value.start = expr.expression.clone();
                                                }
                                                &modelica_grammar_trait::ModificationExpression::Break(..) => {
                                                    todo!("break")
                                                }
                                            }
                                        }
                                    }
                                }

                                def.components
                                    .insert(c.declaration.ident.text.clone(), value);
                            }
                        }
                    }
                }
                modelica_grammar_trait::Element::ImportClause(clause) => {
                    match &clause.import_clause.import_clause_group {
                        modelica_grammar_trait::ImportClauseGroup::ImportStatement(import) => {
                            def.imports.push(import.import_statement.name.to_string());
                            // todo, handle optional group
                        }
                        modelica_grammar_trait::ImportClauseGroup::RenamingImport(..) => {
                            todo!("renaming import")
                        }
                    }
                }
                modelica_grammar_trait::Element::ExtendsClause(clause) => {
                    if let Some(_opt) = &clause.extends_clause.extends_clause_opt {
                        todo!("unhandled extends class or inheritance modification")
                    }
                    if let Some(_opt) = &clause.extends_clause.extends_clause_opt0 {
                        todo!("unhandled annotation")
                    }
                    def.extends.push(ir::ast::Extend {
                        comp: clause.extends_clause.type_specifier.name.clone(),
                    });
                }
                modelica_grammar_trait::Element::ElementReplaceableDefinition(..) => {
                    todo!("element replaceable definition")
                }
            }
        }
        Ok(def)
    }
}

//-----------------------------------------------------------------------------
impl TryFrom<&modelica_grammar_trait::String> for ir::ast::Token {
    type Error = anyhow::Error;

    fn try_from(ast: &modelica_grammar_trait::String) -> std::result::Result<Self, Self::Error> {
        let mut tok = ast.string.clone();
        // remove quotes from string text
        tok.text = tok.text[1..tok.text.len() - 1].to_string();
        Ok(tok)
    }
}

//-----------------------------------------------------------------------------
#[derive(Default, Clone, Debug, PartialEq)]
#[allow(unused)]
pub struct TokenList {
    pub tokens: Vec<ir::ast::Token>,
}

impl TryFrom<&modelica_grammar_trait::DescriptionString> for TokenList {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::DescriptionString,
    ) -> std::result::Result<Self, Self::Error> {
        let mut tokens = Vec::new();
        match &ast.description_string_opt {
            Some(opt) => {
                tokens.push(opt.string.clone());
                for string in &opt.description_string_opt_list {
                    tokens.push(string.string.clone());
                }
            }
            None => {}
        }
        Ok(TokenList { tokens })
    }
}

//-----------------------------------------------------------------------------
#[derive(Debug, Default, Clone)]
#[allow(unused)]
pub struct ComponentList {
    pub components: Vec<modelica_grammar_trait::ComponentDeclaration>,
}

impl TryFrom<&modelica_grammar_trait::ComponentList> for ComponentList {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::ComponentList,
    ) -> std::result::Result<Self, Self::Error> {
        let mut components = vec![ast.component_declaration.clone()];
        for comp in &ast.component_list_list {
            components.push(comp.component_declaration.clone());
        }
        Ok(ComponentList { components })
    }
}

//-----------------------------------------------------------------------------
#[derive(Debug, Default, Clone)]
#[allow(unused)]
pub struct EquationSection {
    pub initial: bool,
    pub equations: Vec<ir::ast::Equation>,
}

impl TryFrom<&modelica_grammar_trait::EquationSection> for EquationSection {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::EquationSection,
    ) -> std::result::Result<Self, Self::Error> {
        let mut def = EquationSection {
            initial: ast.equation_section_opt.is_some(),
            equations: vec![],
        };
        for eq in &ast.equation_section_list {
            def.equations.push(eq.some_equation.clone());
        }
        Ok(def)
    }
}

//-----------------------------------------------------------------------------
#[derive(Debug, Default, Clone)]
#[allow(unused)]
pub struct AlgorithmSection {
    pub initial: bool,
    pub statements: Vec<ir::ast::Statement>,
}

impl TryFrom<&modelica_grammar_trait::AlgorithmSection> for AlgorithmSection {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::AlgorithmSection,
    ) -> std::result::Result<Self, Self::Error> {
        let mut def = AlgorithmSection {
            initial: ast.algorithm_section_opt.is_some(),
            statements: vec![],
        };
        for alg in &ast.algorithm_section_list {
            def.statements.push(alg.statement.clone());
        }
        Ok(def)
    }
}

//-----------------------------------------------------------------------------
impl TryFrom<&modelica_grammar_trait::Ident> for ir::ast::Token {
    type Error = anyhow::Error;

    fn try_from(ast: &modelica_grammar_trait::Ident) -> std::result::Result<Self, Self::Error> {
        match ast {
            modelica_grammar_trait::Ident::BasicIdent(tok) => Ok(ir::ast::Token {
                location: tok.basic_ident.location.clone(),
                text: tok.basic_ident.text.clone(),
                token_number: tok.basic_ident.token_number,
                token_type: tok.basic_ident.token_type,
            }),
            modelica_grammar_trait::Ident::QIdent(tok) => Ok(ir::ast::Token {
                location: tok.q_ident.location.clone(),
                text: tok.q_ident.text.clone(),
                token_number: tok.q_ident.token_number,
                token_type: tok.q_ident.token_type,
            }),
        }
    }
}

//-----------------------------------------------------------------------------
impl TryFrom<&modelica_grammar_trait::UnsignedInteger> for ir::ast::Token {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::UnsignedInteger,
    ) -> std::result::Result<Self, Self::Error> {
        Ok(ir::ast::Token {
            location: ast.unsigned_integer.location.clone(),
            text: ast.unsigned_integer.text.clone(),
            token_number: ast.unsigned_integer.token_number,
            token_type: ast.unsigned_integer.token_type,
        })
    }
}

//-----------------------------------------------------------------------------
impl TryFrom<&modelica_grammar_trait::UnsignedReal> for ir::ast::Token {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::UnsignedReal,
    ) -> std::result::Result<Self, Self::Error> {
        match &ast {
            modelica_grammar_trait::UnsignedReal::Decimal(num) => Ok(num.decimal.clone()),
            modelica_grammar_trait::UnsignedReal::Scientific(num) => Ok(num.scientific.clone()),
            modelica_grammar_trait::UnsignedReal::Scientific2(num) => Ok(num.scientific2.clone()),
        }
    }
}

//-----------------------------------------------------------------------------
impl TryFrom<&modelica_grammar_trait::EquationBlock> for ir::ast::EquationBlock {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::EquationBlock,
    ) -> std::result::Result<Self, Self::Error> {
        Ok(ir::ast::EquationBlock {
            cond: ast.expression.clone(),
            eqs: ast
                .equation_block_list
                .iter()
                .map(|x| x.some_equation.clone())
                .collect(),
        })
    }
}

impl TryFrom<&modelica_grammar_trait::StatementBlock> for ir::ast::StatementBlock {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::StatementBlock,
    ) -> std::result::Result<Self, Self::Error> {
        Ok(ir::ast::StatementBlock {
            cond: ast.expression.clone(),
            stmts: ast
                .statement_block_list
                .iter()
                .map(|x| x.statement.clone())
                .collect(),
        })
    }
}

impl TryFrom<&modelica_grammar_trait::SomeEquation> for ir::ast::Equation {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::SomeEquation,
    ) -> std::result::Result<Self, Self::Error> {
        match &ast.some_equation_option {
            modelica_grammar_trait::SomeEquationOption::SimpleEquation(eq) => {
                match &eq.simple_equation.simple_equation_opt {
                    Some(rhs) => Ok(ir::ast::Equation::Simple {
                        lhs: eq.simple_equation.simple_expression.clone(),
                        rhs: rhs.expression.clone(),
                    }),
                    None => {
                        // this is a function call eq (reinit, assert, terminate, etc.)
                        // see 8.3.6-8.3.8
                        match &eq.simple_equation.simple_expression {
                            ir::ast::Expression::FunctionCall { comp, args } => {
                                Ok(ir::ast::Equation::FunctionCall {
                                    comp: comp.clone(),
                                    args: args.clone(),
                                })
                            }
                            _ => Err(anyhow::anyhow!(
                                "Modelica only allows functional call statement as equation: {:?}",
                                ast
                            )),
                        }
                    }
                }
            }
            modelica_grammar_trait::SomeEquationOption::ConnectEquation(eq) => {
                Ok(ir::ast::Equation::Connect {
                    lhs: eq.connect_equation.component_reference.clone(),
                    rhs: eq.connect_equation.component_reference0.clone(),
                })
            }
            modelica_grammar_trait::SomeEquationOption::ForEquation(..) => todo!("for"),
            modelica_grammar_trait::SomeEquationOption::IfEquation(eq) => {
                let mut blocks = vec![eq.if_equation.if0.clone()];
                for when in &eq.if_equation.if_equation_list {
                    blocks.push(when.elseif0.clone());
                }
                Ok(ir::ast::Equation::If {
                    cond_blocks: blocks,
                    else_block: match &eq.if_equation.if_equation_opt {
                        Some(opt) => Some(
                            opt.if_equation_opt_list
                                .iter()
                                .map(|x| x.some_equation.clone())
                                .collect(),
                        ),
                        None => None,
                    },
                })
            }
            modelica_grammar_trait::SomeEquationOption::WhenEquation(eq) => {
                let mut cond_blocks = vec![eq.when_equation.when0.clone()];
                for when in &eq.when_equation.when_equation_list {
                    cond_blocks.push(when.elsewhen0.clone());
                }
                Ok(ir::ast::Equation::When(cond_blocks))
            }
        }
    }
}

//-----------------------------------------------------------------------------
impl TryFrom<&modelica_grammar_trait::Statement> for ir::ast::Statement {
    type Error = anyhow::Error;

    fn try_from(ast: &modelica_grammar_trait::Statement) -> std::result::Result<Self, Self::Error> {
        match &ast.statement_option {
            modelica_grammar_trait::StatementOption::ComponentStatement(stmt) => {
                match &stmt.component_statement.component_statement_group {
                    modelica_grammar_trait::ComponentStatementGroup::ColonEquExpression(assign) => {
                        Ok(ir::ast::Statement::Assignment {
                            comp: stmt.component_statement.component_reference.clone(),
                            value: assign.expression.clone(),
                        })
                    }
                    modelica_grammar_trait::ComponentStatementGroup::FunctionCallArgs(args) => {
                        Ok(ir::ast::Statement::FunctionCall {
                            comp: stmt.component_statement.component_reference.clone(),
                            args: args.function_call_args.args.clone(),
                        })
                    }
                }
            }
            modelica_grammar_trait::StatementOption::Break(tok) => Ok(ir::ast::Statement::Break {
                token: tok.r#break.r#break.clone(),
            }),
            modelica_grammar_trait::StatementOption::Return(tok) => {
                Ok(ir::ast::Statement::Return {
                    token: tok.r#return.r#return.clone(),
                })
            }
            modelica_grammar_trait::StatementOption::ForStatement(..) => {
                Ok(ir::ast::Statement::For {
                    indices: vec![], // todo
                    equations: vec![],
                })
            }
            modelica_grammar_trait::StatementOption::IfStatement(..) => todo!("if"),
            modelica_grammar_trait::StatementOption::WhenStatement(..) => todo!("when"),
            modelica_grammar_trait::StatementOption::WhileStatement(..) => todo!("while"),
            modelica_grammar_trait::StatementOption::FunctionCallOutputStatement(..) => {
                todo!("function call")
            }
        }
    }
}

//-----------------------------------------------------------------------------
#[derive(Debug, Default, Clone)]
#[allow(unused)]
pub struct ArraySubscripts {
    pub subscripts: Vec<ir::ast::Subscript>,
}

impl TryFrom<&modelica_grammar_trait::ArraySubscripts> for ArraySubscripts {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::ArraySubscripts,
    ) -> std::result::Result<Self, Self::Error> {
        let mut subscripts = vec![ast.subscript.as_ref().clone()];
        for subscript in &ast.array_subscripts_list {
            subscripts.push(subscript.subscript.clone());
        }
        Ok(ArraySubscripts { subscripts })
    }
}

impl TryFrom<&modelica_grammar_trait::Subscript> for ir::ast::Subscript {
    type Error = anyhow::Error;

    fn try_from(ast: &modelica_grammar_trait::Subscript) -> std::result::Result<Self, Self::Error> {
        match ast {
            modelica_grammar_trait::Subscript::Colon(tok) => Ok(ir::ast::Subscript::Range {
                token: tok.colon.clone(),
            }),
            modelica_grammar_trait::Subscript::Expression(expr) => Ok(
                ir::ast::Subscript::Expression(expr.expression.as_ref().clone()),
            ),
        }
    }
}

//-----------------------------------------------------------------------------
#[derive(Debug, Default, Clone)]
#[allow(unused)]
pub struct ExpressionList {
    pub args: Vec<ir::ast::Expression>,
}

impl TryFrom<&modelica_grammar_trait::FunctionArgument> for ir::ast::Expression {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::FunctionArgument,
    ) -> std::result::Result<Self, Self::Error> {
        match &ast {
            modelica_grammar_trait::FunctionArgument::Expression(expr) => {
                Ok(expr.expression.as_ref().clone())
            }
            modelica_grammar_trait::FunctionArgument::FunctionPartialApplication(..) => {
                todo!("partial application")
            }
        }
    }
}

impl TryFrom<&modelica_grammar_trait::FunctionArguments> for ExpressionList {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::FunctionArguments,
    ) -> std::result::Result<Self, Self::Error> {
        match &ast {
            modelica_grammar_trait::FunctionArguments::ExpressionFunctionArgumentsOpt(def) => {
                let mut args = vec![*def.expression.clone()];
                match &def.function_arguments_opt {
                    Some(opt) => {
                        match &opt.function_arguments_opt_group {
                            modelica_grammar_trait::FunctionArgumentsOptGroup::CommaFunctionArgumentsNonFirst(
                                expr,
                            ) => {
                                args.append(&mut expr.function_arguments_non_first.args.clone());
                            }
                            modelica_grammar_trait::FunctionArgumentsOptGroup::ForForIndices(..) => {
                                todo!("for indices")
                            }
                        }
                    }
                    None => {}
                }
                Ok(ExpressionList { args })
            }
            modelica_grammar_trait::FunctionArguments::FunctionPartialApplicationFunctionArgumentsOpt0(..) => {
                todo!("partial application")
            }
            modelica_grammar_trait::FunctionArguments::NamedArguments(..) => {
                todo!("named arguments")
            }
        }
    }
}

impl TryFrom<&modelica_grammar_trait::FunctionArgumentsNonFirst> for ExpressionList {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::FunctionArgumentsNonFirst,
    ) -> std::result::Result<Self, Self::Error> {
        match &ast {
            modelica_grammar_trait::FunctionArgumentsNonFirst::FunctionArgumentFunctionArgumentsNonFirstOpt(expr) => {
                let mut args = vec![expr.function_argument.clone()];
                match &expr.function_arguments_non_first_opt {
                    Some(opt) => {
                        args.append(&mut opt.function_arguments_non_first.args.clone());
                    }
                    None => {}
                }
                Ok(ExpressionList { args })
            }
            modelica_grammar_trait::FunctionArgumentsNonFirst::NamedArguments(..) => {
                todo!("named arguments")
            }
        }
    }
}

//-----------------------------------------------------------------------------
impl TryFrom<&modelica_grammar_trait::ArgumentList> for ExpressionList {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::ArgumentList,
    ) -> std::result::Result<Self, Self::Error> {
        let mut args = vec![(*ast.argument).clone()];
        for arg in &ast.argument_list_list {
            args.push(arg.argument.clone())
        }
        Ok(ExpressionList { args })
    }
}

impl TryFrom<&modelica_grammar_trait::Argument> for ir::ast::Expression {
    type Error = anyhow::Error;

    fn try_from(ast: &modelica_grammar_trait::Argument) -> std::result::Result<Self, Self::Error> {
        match ast {
            modelica_grammar_trait::Argument::ElementModificationOrReplaceable(modif) => {
                match &modif.element_modification_or_replaceable.element_modification_or_replaceable_group {
                    modelica_grammar_trait::ElementModificationOrReplaceableGroup::ElementModification(elem) => {
                        match &elem.element_modification.element_modification_opt {
                            Some(opt) => {
                                match &opt.modification {
                                    modelica_grammar_trait::Modification::ClassModificationModificationOpt(_modif) => {
                                        todo!("argument class modification")
                                    }
                                    modelica_grammar_trait::Modification::EquModificationExpression(modif) => {
                                        match &modif.modification_expression {
                                            modelica_grammar_trait::ModificationExpression::Break(..) => {
                                                todo!("break expression")
                                            }
                                            modelica_grammar_trait::ModificationExpression::Expression(expr) => {
                                                Ok(expr.expression.clone())
                                            }
                                        }
                                    }
                                }
                            }
                            None => {
                                Ok(ir::ast::Expression::Empty)
                            }
                        }
                    }
                    modelica_grammar_trait::ElementModificationOrReplaceableGroup::ElementReplaceable(..) => {
                        todo!("element replaceable")
                    }
                }
            }
            modelica_grammar_trait::Argument::ElementRedeclaration(_redcl) => {
                todo!("element redeclaration")
            }
        }
    }
}

//-----------------------------------------------------------------------------
impl TryFrom<&modelica_grammar_trait::OutputExpressionList> for ExpressionList {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::OutputExpressionList,
    ) -> std::result::Result<Self, Self::Error> {
        let mut v = Vec::new();
        if let Some(opt) = &ast.output_expression_list_opt {
            v.push(opt.expression.clone());
        }
        for expr in &ast.output_expression_list_list {
            if let Some(opt) = &expr.output_expression_list_opt0 {
                v.push(opt.expression.clone());
            }
        }
        Ok(ExpressionList { args: v })
    }
}

impl TryFrom<&modelica_grammar_trait::FunctionCallArgs> for ExpressionList {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::FunctionCallArgs,
    ) -> std::result::Result<Self, Self::Error> {
        if let Some(opt) = &ast.function_call_args_opt {
            Ok(ExpressionList {
                args: opt.function_arguments.args.clone(),
            })
        } else {
            Ok(ExpressionList { args: vec![] })
        }
    }
}

impl TryFrom<&modelica_grammar_trait::Primary> for ir::ast::Expression {
    type Error = anyhow::Error;

    fn try_from(ast: &modelica_grammar_trait::Primary) -> std::result::Result<Self, Self::Error> {
        match &ast {
            modelica_grammar_trait::Primary::ComponentPrimary(comp) => {
                match &comp.component_primary.component_primary_opt {
                    Some(args) => Ok(ir::ast::Expression::FunctionCall {
                        comp: (*comp.component_primary.component_reference).clone(),
                        args: args.function_call_args.args.clone(),
                    }),
                    None => Ok(ir::ast::Expression::ComponentReference(
                        comp.component_primary.component_reference.as_ref().clone(),
                    )),
                }
            }
            modelica_grammar_trait::Primary::UnsignedNumber(unsigned_num) => {
                match &unsigned_num.unsigned_number {
                    modelica_grammar_trait::UnsignedNumber::UnsignedInteger(unsigned_int) => {
                        Ok(ir::ast::Expression::Terminal {
                            terminal_type: ir::ast::TerminalType::UnsignedInteger,
                            token: unsigned_int.unsigned_integer.clone(),
                        })
                    }
                    modelica_grammar_trait::UnsignedNumber::UnsignedReal(unsigned_real) => {
                        Ok(ir::ast::Expression::Terminal {
                            terminal_type: ir::ast::TerminalType::UnsignedReal,
                            token: unsigned_real.unsigned_real.clone(),
                        })
                    }
                }
            }
            modelica_grammar_trait::Primary::String(string) => Ok(ir::ast::Expression::Terminal {
                terminal_type: ir::ast::TerminalType::String,
                token: string.string.clone(),
            }),
            modelica_grammar_trait::Primary::True(bool) => Ok(ir::ast::Expression::Terminal {
                terminal_type: ir::ast::TerminalType::Bool,
                token: bool.r#true.r#true.clone(),
            }),
            modelica_grammar_trait::Primary::False(bool) => Ok(ir::ast::Expression::Terminal {
                terminal_type: ir::ast::TerminalType::Bool,
                token: bool.r#false.r#false.clone(),
            }),
            modelica_grammar_trait::Primary::End(end) => Ok(ir::ast::Expression::Terminal {
                terminal_type: ir::ast::TerminalType::End,
                token: end.end.end.clone(),
            }),
            modelica_grammar_trait::Primary::ArrayPrimary(..) => {
                todo!("array")
            }
            modelica_grammar_trait::Primary::RangePrimary(..) => {
                todo!("expression list")
            }
            modelica_grammar_trait::Primary::OutputPrimary(output) => {
                let primary = &output.output_primary;
                if primary.output_primary_opt.is_some() {
                    todo!("output_primary array subs/ ident");
                };
                if primary.output_expression_list.args.len() > 1 {
                    todo!("comma in output primary");
                }
                Ok(primary.output_expression_list.args[0].clone())
            }
            modelica_grammar_trait::Primary::GlobalFunctionCall(expr) => {
                let tok = match &expr.global_function_call.global_function_call_group {
                    modelica_grammar_trait::GlobalFunctionCallGroup::Der(expr) => {
                        expr.der.der.clone()
                    }
                    modelica_grammar_trait::GlobalFunctionCallGroup::Initial(expr) => {
                        expr.initial.initial.clone()
                    }
                    modelica_grammar_trait::GlobalFunctionCallGroup::Pure(expr) => {
                        expr.pure.pure.clone()
                    }
                };
                let part = ir::ast::ComponentRefPart {
                    ident: tok,
                    subs: None,
                };
                Ok(ir::ast::Expression::FunctionCall {
                    comp: ir::ast::ComponentReference {
                        local: false,
                        parts: vec![part],
                    },
                    args: expr.global_function_call.function_call_args.args.clone(),
                })
            }
        }
    }
}

impl TryFrom<&modelica_grammar_trait::Factor> for ir::ast::Expression {
    type Error = anyhow::Error;

    fn try_from(ast: &modelica_grammar_trait::Factor) -> std::result::Result<Self, Self::Error> {
        if ast.factor_list.is_empty() {
            return Ok(ast.primary.as_ref().clone());
        } else {
            Ok(ir::ast::Expression::Binary {
                op: ir::ast::OpBinary::Exp(ir::ast::Token::default()),
                lhs: Box::new(ast.primary.as_ref().clone()),
                rhs: Box::new(ast.factor_list[0].primary.clone()),
            })
        }
    }
}

impl TryFrom<&modelica_grammar_trait::Term> for ir::ast::Expression {
    type Error = anyhow::Error;

    fn try_from(ast: &modelica_grammar_trait::Term) -> std::result::Result<Self, Self::Error> {
        if ast.term_list.is_empty() {
            return Ok(ast.factor.clone());
        } else {
            let mut lhs = ast.factor.clone();
            for factor in &ast.term_list {
                lhs = ir::ast::Expression::Binary {
                    lhs: Box::new(lhs),
                    op: match &factor.mul_operator {
                        modelica_grammar_trait::MulOperator::Star(op) => {
                            ir::ast::OpBinary::Mul(op.star.clone())
                        }
                        modelica_grammar_trait::MulOperator::Slash(op) => {
                            ir::ast::OpBinary::Div(op.slash.clone())
                        }
                        modelica_grammar_trait::MulOperator::DotSlash(op) => {
                            ir::ast::OpBinary::DivElem(op.dot_slash.clone())
                        }
                        modelica_grammar_trait::MulOperator::DotStar(op) => {
                            ir::ast::OpBinary::MulElem(op.dot_star.clone())
                        }
                    },
                    rhs: Box::new(factor.factor.clone()),
                };
            }
            Ok(lhs)
        }
    }
}

impl TryFrom<&modelica_grammar_trait::ArithmeticExpression> for ir::ast::Expression {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::ArithmeticExpression,
    ) -> std::result::Result<Self, Self::Error> {
        // handle first term
        let mut lhs = match &ast.arithmetic_expression_opt {
            Some(opt) => ir::ast::Expression::Unary {
                op: match &opt.add_operator {
                    modelica_grammar_trait::AddOperator::Minus(tok) => {
                        ir::ast::OpUnary::Minus(tok.minus.clone())
                    }
                    modelica_grammar_trait::AddOperator::Plus(tok) => {
                        ir::ast::OpUnary::Plus(tok.plus.clone())
                    }
                    modelica_grammar_trait::AddOperator::DotMinus(tok) => {
                        ir::ast::OpUnary::DotMinus(tok.dot_minus.clone())
                    }
                    modelica_grammar_trait::AddOperator::DotPlus(tok) => {
                        ir::ast::OpUnary::DotPlus(tok.dot_plus.clone())
                    }
                },
                rhs: Box::new(ast.term.as_ref().clone()),
            },
            None => ast.term.as_ref().clone(),
        };

        // if has term list, process expressions
        if !ast.arithmetic_expression_list.is_empty() {
            for term in &ast.arithmetic_expression_list {
                lhs = ir::ast::Expression::Binary {
                    lhs: Box::new(lhs),
                    op: match &term.add_operator {
                        modelica_grammar_trait::AddOperator::Plus(tok) => {
                            ir::ast::OpBinary::Add(tok.plus.clone())
                        }
                        modelica_grammar_trait::AddOperator::Minus(tok) => {
                            ir::ast::OpBinary::Sub(tok.minus.clone())
                        }
                        modelica_grammar_trait::AddOperator::DotPlus(tok) => {
                            ir::ast::OpBinary::AddElem(tok.dot_plus.clone())
                        }
                        modelica_grammar_trait::AddOperator::DotMinus(tok) => {
                            ir::ast::OpBinary::SubElem(tok.dot_minus.clone())
                        }
                    },
                    rhs: Box::new(term.term.clone()),
                };
            }
        }
        Ok(lhs)
    }
}

impl TryFrom<&modelica_grammar_trait::Relation> for ir::ast::Expression {
    type Error = anyhow::Error;

    fn try_from(ast: &modelica_grammar_trait::Relation) -> std::result::Result<Self, Self::Error> {
        match &ast.relation_opt {
            Some(relation) => Ok(ir::ast::Expression::Binary {
                lhs: Box::new(ast.arithmetic_expression.as_ref().clone()),
                op: match &relation.relational_operator {
                    modelica_grammar_trait::RelationalOperator::EquEqu(tok) => {
                        ir::ast::OpBinary::Eq(tok.equ_equ.clone())
                    }
                    modelica_grammar_trait::RelationalOperator::GT(tok) => {
                        ir::ast::OpBinary::Gt(tok.g_t.clone())
                    }
                    modelica_grammar_trait::RelationalOperator::LT(tok) => {
                        ir::ast::OpBinary::Lt(tok.l_t.clone())
                    }
                    modelica_grammar_trait::RelationalOperator::GTEqu(tok) => {
                        ir::ast::OpBinary::Ge(tok.g_t_equ.clone())
                    }
                    modelica_grammar_trait::RelationalOperator::LTEqu(tok) => {
                        ir::ast::OpBinary::Le(tok.l_t_equ.clone())
                    }
                    modelica_grammar_trait::RelationalOperator::LTGT(tok) => {
                        ir::ast::OpBinary::Neq(tok.l_t_g_t.clone())
                    }
                },
                rhs: Box::new(relation.arithmetic_expression.clone()),
            }),
            None => Ok(ast.arithmetic_expression.as_ref().clone()),
        }
    }
}

impl TryFrom<&modelica_grammar_trait::LogicalFactor> for ir::ast::Expression {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::LogicalFactor,
    ) -> std::result::Result<Self, Self::Error> {
        match &ast.logical_factor_opt {
            Some(opt) => {
                let not_tok = opt.not.not.clone();
                Ok(ir::ast::Expression::Unary {
                    op: ir::ast::OpUnary::Not(not_tok),
                    rhs: Box::new(ast.relation.as_ref().clone()),
                })
            }
            None => Ok(ast.relation.as_ref().clone()),
        }
    }
}

impl TryFrom<&modelica_grammar_trait::LogicalTerm> for ir::ast::Expression {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::LogicalTerm,
    ) -> std::result::Result<Self, Self::Error> {
        if ast.logical_term_list.is_empty() {
            return Ok(ast.logical_factor.as_ref().clone());
        } else {
            let mut lhs = ast.logical_factor.as_ref().clone();
            for term in &ast.logical_term_list {
                lhs = ir::ast::Expression::Binary {
                    lhs: Box::new(lhs),
                    op: ir::ast::OpBinary::And(ir::ast::Token::default()),
                    rhs: Box::new(term.logical_factor.clone()),
                };
            }
            Ok(lhs)
        }
    }
}

impl TryFrom<&modelica_grammar_trait::LogicalExpression> for ir::ast::Expression {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::LogicalExpression,
    ) -> std::result::Result<Self, Self::Error> {
        if ast.logical_expression_list.is_empty() {
            return Ok(ast.logical_term.as_ref().clone());
        } else {
            let mut lhs = ast.logical_term.as_ref().clone();
            for term in &ast.logical_expression_list {
                lhs = ir::ast::Expression::Binary {
                    lhs: Box::new(lhs),
                    op: ir::ast::OpBinary::Or(ir::ast::Token::default()),
                    rhs: Box::new(term.logical_term.clone()),
                };
            }
            Ok(lhs)
        }
    }
}

impl TryFrom<&modelica_grammar_trait::SimpleExpression> for ir::ast::Expression {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::SimpleExpression,
    ) -> std::result::Result<Self, Self::Error> {
        match &ast.simple_expression_opt {
            Some(opt) => match &opt.simple_expression_opt0 {
                Some(opt0) => Ok(ir::ast::Expression::Range {
                    start: Box::new(ast.logical_expression.clone()),
                    step: Some(Box::new(opt.logical_expression.clone())),
                    end: Box::new(opt0.logical_expression.clone()),
                }),
                None => Ok(ir::ast::Expression::Range {
                    start: Box::new(ast.logical_expression.clone()),
                    step: None,
                    end: Box::new(opt.logical_expression.clone()),
                }),
            },
            None => Ok(ast.logical_expression.clone()),
        }
    }
}

impl TryFrom<&modelica_grammar_trait::Expression> for ir::ast::Expression {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::Expression,
    ) -> std::result::Result<Self, Self::Error> {
        match &ast {
            modelica_grammar_trait::Expression::SimpleExpression(simple_expression) => {
                Ok(simple_expression.simple_expression.as_ref().clone())
            }
            modelica_grammar_trait::Expression::IfExpression(..) => {
                todo!("if")
            }
        }
    }
}

//-----------------------------------------------------------------------------
impl TryFrom<&modelica_grammar_trait::ComponentReference> for ir::ast::ComponentReference {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::ComponentReference,
    ) -> std::result::Result<Self, Self::Error> {
        let mut parts = Vec::new();
        parts.push(ir::ast::ComponentRefPart {
            ident: ast.ident.clone(),
            subs: None,
        });
        for comp_ref in &ast.component_reference_list {
            parts.push(comp_ref.component_ref_part.clone());
        }
        Ok(ir::ast::ComponentReference {
            local: ast.component_reference_opt.is_some(),
            parts,
        })
    }
}

//-----------------------------------------------------------------------------
impl TryFrom<&modelica_grammar_trait::ComponentRefPart> for ir::ast::ComponentRefPart {
    type Error = anyhow::Error;

    fn try_from(
        ast: &modelica_grammar_trait::ComponentRefPart,
    ) -> std::result::Result<Self, Self::Error> {
        Ok(ir::ast::ComponentRefPart {
            ident: ast.ident.clone(),
            subs: match &ast.component_ref_part_opt {
                Some(subs) => Some(subs.array_subscripts.subscripts.clone()),
                None => None,
            },
        })
    }
}

//-----------------------------------------------------------------------------
impl TryFrom<&modelica_grammar_trait::Name> for ir::ast::Name {
    type Error = anyhow::Error;

    fn try_from(ast: &modelica_grammar_trait::Name) -> std::result::Result<Self, Self::Error> {
        let mut name = vec![ast.ident.clone()];
        for ident in &ast.name_list {
            name.push(ident.ident.clone());
        }
        Ok(ir::ast::Name { name })
    }
}

//-----------------------------------------------------------------------------
#[derive(Debug, Default)]
pub struct ModelicaGrammar<'t> {
    pub modelica: Option<ir::ast::StoredDefinition>,
    _phantom: std::marker::PhantomData<&'t str>,
}

impl ModelicaGrammar<'_> {
    pub fn new() -> Self {
        ModelicaGrammar::default()
    }
}

impl<'t> Display for modelica_grammar_trait::StoredDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl Display for ModelicaGrammar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match &self.modelica {
            Some(modelica) => writeln!(f, "{:#?}", modelica),
            None => write!(f, "No parse result"),
        }
    }
}

impl<'t> modelica_grammar_trait::ModelicaGrammarTrait for ModelicaGrammar<'t> {
    fn stored_definition(&mut self, arg: &modelica_grammar_trait::StoredDefinition) -> Result<()> {
        self.modelica = Some(arg.try_into()?);
        Ok(())
    }
}
