use std::vec;
use std::collections::HashMap;
use std::ops;
use std::rc::Rc;
use std::cell::RefCell;

use crate::lexer::TokenType;

// ------------------ Nodes -----------------------/

#[derive(Debug)]
enum Node {
    Assign{ 
        lhs: Box<Node>, 
        rhs: Box<Node>,    
    },
    OpAdd { 
        lhs: Box<Node>, 
        rhs: Box<Node>,    
    },
    OpSub{ 
        lhs: Box<Node>, 
        rhs: Box<Node>,    
    },
    OpMul{ 
        lhs: Box<Node>, 
        rhs: Box<Node>,    
    },
    OpDiv{ 
        lhs: Box<Node>, 
        rhs: Box<Node>,    
    },
    Print(usize),
    Value(Variable),
    Var(usize),
    ProcCall(Box<Routine>),
    FuncCall,
}

impl Node {
    fn eval(self, stack: &mut Stack) -> Variable {
        let var = match self {
            Node::Assign { lhs, rhs }=> { 
                let var_rhs = rhs.eval(stack);
                lhs.assign(stack, var_rhs)
            },
            Node::OpAdd { lhs, rhs } => ( lhs.eval(stack) + rhs.eval(stack)),
            Node::OpSub { lhs, rhs } => ( lhs.eval(stack) - rhs.eval(stack)),
            Node::OpMul { lhs, rhs } => ( lhs.eval(stack) * rhs.eval(stack)),
            Node::OpDiv { lhs, rhs } => ( lhs.eval(stack) / rhs.eval(stack)),
            Node::Value(var) => {
                var.clone()
            },
            Node::Var(idx) => {
                if let Some(var) = stack.variables.get(stack.offset + idx) {
                    var.clone()
                } else {
                    panic!("");
                }
            },
            Node::Print(idx) => {
                if let Some(var) = stack.variables.get(stack.offset + idx) {
                    println!("[Out] {:?}", var);
                    Variable::Void
                } else {
                    panic!("");
                }
            }
            _ => panic!(""),
        };
        var
    }

    fn assign(self, stack: &mut Stack, other: Variable) -> Variable {
        match self {
            Node::Var(idx) => {
                if let Some(var) = stack.variables.get_mut(stack.offset + idx) {
                    
                    var.set(other);
                }
            },
            _ => panic!("Can only assign to variable"),
        };
        Variable::Void
    }
}


// ------------------ Variables -----------------------/

#[derive(Debug,Clone)]
enum Variable {
    Void,
    Bool(bool),
    Num(f64),
    Str(String),
    //Proc(Box<Node>),
}

impl Variable {
    fn set(&mut self, other: Variable) {
        match (self, other) {
            (Variable::Bool(ref mut value), Variable::Bool(value2)) => *value = value2,
            (Variable::Num(ref mut value), Variable::Num(value2)) => *value = value2,
            (Variable::Str(ref mut value), Variable::Str(ref value2)) => *value = value2.clone(),
            _ => panic!("")
        }
    }

    fn from(data_type: &TokenType) -> Result<Variable,String> {
        let var = match data_type {
            TokenType::NumType => Variable::Num(0.0),
            TokenType::BoolType => Variable::Bool(false),
            TokenType::StringType => Variable::Str(String::default()),
            _ => return Err(String::from("Unknown data type")),
        };
        
        Ok(var)
    }

    fn from_value(data_type: &TokenType, value: &TokenType) -> Result<Variable,String> {
        let var = match (data_type, value) {
            (TokenType::BoolType, TokenType::True) => Variable::Bool(true),
            (TokenType::BoolType, TokenType::False) => Variable::Bool(false),
            (TokenType::NumType, TokenType::NumValue(val)) => Variable::Num(val.parse().unwrap()),
            (TokenType::StringType, TokenType::StringValue(val)) => Variable::Str(val.clone()),
            _ => return Err(String::from("Unknown data type")),
        };
        
        Ok(var)
    }
}

impl ops::Add for Variable {
    type Output = Variable;

    fn add(self, other: Variable) -> Variable {
        match (self, other) {
            (Variable::Bool(b1), Variable::Bool(b2)) => Variable::Bool(b1 || b2),
            (Variable::Num(n1), Variable::Num(n2)) => Variable::Num(n1 + n2),
            (Variable::Str(s1), Variable::Str(s2)) => Variable::Str(s1.clone() + &s2.clone()),
            _ => panic!("Unknown combo")
        }
    }
}

impl ops::Sub for Variable {
    type Output = Variable;

    fn sub(self, other: Variable) -> Variable {
        match (self, other) {
            (Variable::Num(n1), Variable::Num(n2)) => Variable::Num(n1 - n2),
            _ => panic!("Unknown combo")
        }
    }
}

impl ops::Mul for Variable {
    type Output = Variable;

    fn mul(self, other: Variable) -> Variable {
        match (self, other) {
            (Variable::Num(n1), Variable::Num(n2)) => Variable::Num(n1 * n2),
            _ => panic!("Unknown combo")
        }
    }
}

impl ops::Div for Variable {
    type Output = Variable;

    fn div(self, other: Variable) -> Variable {
        match (self, other) {
            (Variable::Num(n1), Variable::Num(n2)) => Variable::Num(n1 / n2),
            _ => panic!("Unknown combo")
        }
    }
}

struct RapidIter<I> {
    iter: I
}

impl<I> Iterator for RapidIter<I> where I: Iterator<Item = TokenType>
{
    type Item = Node;

    fn next(&mut self) -> Option<Node> {
        None
    }
}

pub struct Program {
    modules: Vec<Module>,
    variables: Vec<Variable>,
}

impl Program {
    fn new() -> Program {
        Program {
            modules: Vec::new(),
            variables: Vec::new(),
        }
    }
}

pub struct Module {
    name: String,
    routines: Vec<Routine>,
    variables: Vec<Variable>,
}

impl Module {
    fn new(name: String) -> Module {
        Module {
            name: name, 
            routines: Vec::new(),
            variables: Vec::new(),
        }
    }
}

pub struct Scope {
    v: Vec<Variable>,
    indices: HashMap<String, i32>,
    variables: HashMap<String, (i32, Variable)>,
}

#[derive(Debug)]
pub struct Routine {
    name: String,
    arguments: Vec<Variable>,
    variables: HashMap<String,(usize, Variable)>,
    nodes: Vec<Node>,
}

impl Routine {
    fn new(name: String) -> Routine {
        Routine {
            name: name, 
            arguments: Vec::new(),
            variables: HashMap::new(),
            nodes: Vec::new(),
        }
    }
}

pub struct Stack {
    offset: usize,
    variables: Vec<Variable>,    
}


pub fn parse_tokens(tokens: Vec<TokenType>) -> Result<Program, String> {

    let mut iter = tokens.iter();

    let mut program = Program::new();

    let var1 = Variable::Str(String::from("test"));
    let var2 = Variable::Str(String::from("the banana"));
    let var3 = var1 + var2;

    println!("var3 = {:?}", var3);

    while let Some(token) = iter.next() {

        match token {
            // Valid tokens
            TokenType::Mod => { program.modules.push(read_mod(&mut iter)?); },
            // Invalid tokens
            _ => return Err(format!("Invalid token for program: {:?}", token)),
        };
    }

    Ok(program)
}

fn read_mod<'a,I>(iter: &mut I) -> Result<Module, String> where I: Iterator<Item = &'a TokenType> {
    // Create new scope that inherits parent scope
    // add routines and global variables to scope
    // exit at END_MOD

    let name = match iter.next() {
        Some(TokenType::Id(name)) => name,
        _ => return Err(String::from("Expected module name")),
    };

    let mut module = Module::new(name.clone());
        
    while let Some(token) = iter.next() {
        match token {
            // Valid tokens
            TokenType::Proc => { 
                let routine = read_proc(iter)?; 
                println!("Routine: {:?}", routine);
                test_proc(routine);
            },
            TokenType::Func => (),
            TokenType::Var => (),
            TokenType::Pers => (),
            TokenType::Local => (),
            // Closing token
            TokenType::EndMod => return Ok(module),
            // Invalid tokens
            _ => return Err(format!("Invalid token for module: {:?}", token)),
        };
    }

    return Err(String::from("Unexpected end of module"));
}

fn test_proc(routine : Routine) {
    let mut stack = Stack {
        offset: 0,
        variables: Vec::new(),
    };

    for var in routine.variables {
        let pair = var.1;
        stack.variables.push(pair.1.clone());
    }

    for node in routine.nodes {
        let var = node.eval(&mut stack);
    }
}

fn read_proc<'a,I>(iter: &mut I) -> Result<Routine, String> where I: Iterator<Item = &'a TokenType> {
    // Create new local scope that inherits parent scope
    // Add variables to scope
    // exit at END_PROC

    // Routine name
    let name = match iter.next() {
        Some(TokenType::Id(name)) => name,
        _ => return Err(String::from("Expected routine name")),
    };

    match iter.next() {
        Some(TokenType::LeftPar) => (),
        _ => return Err(String::from("Expected '('")),
    };

    let mut routine = Routine::new(name.clone());   

    let mut var_idx = 0;
    let mut variables : HashMap<String,(usize, Variable)> = HashMap::new();

    // Parse arguments
    while let Some(token) = iter.next() {
        let arg = match token {
            // Valid tokens
            TokenType::NumType => parse_arg(iter, token)?,
            TokenType::StringType => parse_arg(iter, token)?,
            TokenType::BoolType => parse_arg(iter, token)?,
            // Closing token
            TokenType::RightPar => break,
            // Invalid tokens
            _ => return Err(format!("Expected ')' {:?}", token)),
        };

        variables.insert(arg.0, (var_idx, arg.1));
        var_idx += 1;
    }

    // Parse variable declarations


    // Parse body
    while let Some(token) = iter.next() {
        match token {
            // Valid tokens
            TokenType::Var => {
                    let var = parse_var(iter)?;
                    variables.insert(var.0, (var_idx, var.1));
                    var_idx += 1;
                },
            TokenType::Id(name) => { 
                    if let Some((idx, var)) = variables.get(name) {
                        let node = Node::Var(*idx);
                        routine.nodes.push(parse_statement(iter, &variables, node)?); 
                    }
                },
            // Future
            TokenType::If => (),
            TokenType::While => (),
            TokenType::For => (),
            TokenType::Return => (),
            TokenType::TpWrite => {
                if let Some(TokenType::Id(name)) = iter.next() {
                    if let Some((idx, var)) = variables.get(name) {
                        let node = Node::Print(*idx);
                        routine.nodes.push(node); 
                    }                    
                }
                iter.next();
            },
            // Closing tokene
            TokenType::EndProc => {
                routine.variables = variables;
                return Ok(routine);
            }
            // Invalid tokens
            _ => return Err(format!("Invalid token for routine: {:?}", token)),
        };
    }

    return Err(String::from("Unexpected end of routine"));
}

fn parse_statement<'a,I>(iter: &mut I, vars_map: &HashMap<String,(usize, Variable)>, lhs_node: Node) -> Result<Node, String> where I: Iterator<Item = &'a TokenType> {

    let op = iter.next();

    // Var name
    match op {
        Some(TokenType::Assign) => (),
        _ => return Err(String::from("Expected variable assignment")),
    };
   
    while let Some(token) = iter.next() {
        let rhs_node = match token {
            TokenType::NumValue(val) => Node::Value(Variable::Num(val.parse().unwrap())),
            TokenType::StringValue(val) => Node::Value(Variable::Str(val.clone())),
            TokenType::True=> Node::Value(Variable::Bool(true)),
            TokenType::False => Node::Value(Variable::Bool(false)),
            TokenType::Id(name) => {
                if let Some(var_idx) = vars_map.get(name) {
                    Node::Var(var_idx.0)
                } else {
                    return Err(String::from("Unknown id"));
                }
            }
            // Invalid tokens
            _ => return Err(format!("Invalid token for statement: {:?}", token)),
        };

        let rhs_node = parse_sub(iter, vars_map, rhs_node)?;
        
        return Ok(Node::Assign {
            lhs: Box::from(lhs_node),
            rhs: Box::from(rhs_node),  
        });
    }

    Err(String::from("Unexpected token"))
}

fn parse_sub<'a,I>(iter: &mut I, vars_map: &HashMap<String,(usize, Variable)>, lhs_node: Node) -> Result<Node, String> where I: Iterator<Item = &'a TokenType> {

    let op = iter.next();

    let operator = match op {
        Some(TokenType::Semicolon) => return Ok(lhs_node),
        None => return Err(String::from("Expected operator")),
        Some(o) => o,
    };

    if let Some(rhs_var) = iter.next() {
        let rhs_node = match rhs_var {
            TokenType::NumValue(val) => Node::Value(Variable::Num(val.parse().unwrap())),
            TokenType::StringValue(val) => Node::Value(Variable::Str(val.clone())),
            TokenType::True=> Node::Value(Variable::Bool(true)),
            TokenType::False => Node::Value(Variable::Bool(false)),
            TokenType::Id(name) => {
                if let Some(var_idx) = vars_map.get(name) {
                    Node::Var(var_idx.0)
                } else {
                    return Err(String::from("Unknown id"));
                }
            }
            // Invalid tokens
            _ => return Err(format!("Invalid token for statement: {:?}", rhs_var)),
        };

        let node = match operator {
            TokenType::Add => {
                let rhs_node = parse_sub(iter, vars_map, rhs_node)?;
                Node::OpAdd {
                    lhs: Box::from(lhs_node),
                    rhs: Box::from(rhs_node)
                }
            },
            TokenType::Minus => { 
                let rhs_node = parse_sub(iter, vars_map, rhs_node)?;
                Node::OpSub {
                    lhs: Box::from(lhs_node),
                    rhs: Box::from(rhs_node)
                }
            },
            TokenType::Multiply => {
                let node = Node::OpMul {
                    lhs: Box::from(lhs_node),
                    rhs: Box::from(rhs_node)
                };                
                parse_sub(iter, vars_map, node)?
            },
            TokenType::Divide => {
                let node = Node::OpDiv {
                    lhs: Box::from(lhs_node),
                    rhs: Box::from(rhs_node)
                };                 
                parse_sub(iter, vars_map, node)?
            } ,
            _ => return Err(format!("Invalid token for statement: {:?}", operator))
        };

        return Ok(node);
    }

    Err(String::from("Unexpected token"))
}

fn parse_arg<'a,I>(iter: &mut I, data_type: &TokenType) -> Result<(String, Variable), String> where I: Iterator<Item = &'a TokenType> {

    // Var name
    let name = match iter.next() {
        Some(TokenType::Id(name)) => name,
        _ => return Err(String::from("Expected var name")),
    };
  
    Ok((name.clone(), Variable::from(data_type)?))
}

fn parse_var<'a,I>(iter: &mut I) -> Result<(String, Variable), String> where I: Iterator<Item = &'a TokenType> {

    let data_type = match iter.next() {
        Some(token) => token,
        None => return Err(String::from("Expected data type")),
    };

    // Var name
    let name = match iter.next() {
        Some(TokenType::Id(name)) => name,
        _ => return Err(String::from("Expected var name")),
    };

    match iter.next() {
        Some(TokenType::Assign) => (),
        Some(TokenType::Semicolon) => return Ok((name.clone(), Variable::from(data_type)?)),
        _ => return Err(String::from("Expected assign or semicolon")),
    };

    let value = match iter.next() {
        Some(token) => token,
        None => return Err(String::from("Expected value")),
    };
    
    match iter.next() {
        Some(TokenType::Semicolon) => return Ok((name.clone(), Variable::from_value(data_type, value)?)),
        _ => return Err(String::from("Expected value")),
    };
}