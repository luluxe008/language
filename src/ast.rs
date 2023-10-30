#![allow(dead_code)]
use core::slice::Iter;
use std::{iter::Peekable, collections::VecDeque};

use crate::{token::Token, errors::{Error, CompilerResult, Location, PartialLocation, self}};


/// all the operator.
/// Mul and Div won't be implement due to their complexity.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Operator{
    Plus,
    Minus,
    Mul,
    Div
}


/// A Node is value. A Node can be composed of a lot of other Node.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Expr{
    IntLitteral(u64),
    StringLitteral(String),
    Identifier(String),
    BinaryExpr{
        opr: Operator,
        l: Box<Expr>,
        r: Box<Expr>
    },
    /// A block is a suite of instruction.
    Block{
        code: Vec<Statement>
    },
    Error // used to indicate error
}


/// A statement is a line of code that is not evaluable such as:
/// ```
/// var x = (25+25)
/// ```
/// This previous line can't be evalued, but it is composed of multiple node.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Statement{
    VarDeclaration{
        identifier: String,
        value: Expr
    },
    Print{
        value: Expr
    },
    VarEdit{
        identifier: String,
        value: Expr
    },
    FuncCall{
        identifier: String,
        args: Vec<Expr>,
    },
    NoneOrError // indicate either the statement is none or an error
}

use Statement as ST;

/// contruct an Abstract Syntax Tree (AST) from a list of vectors
pub struct AbstractSyntaxTree<'a>{
    statement: Statement,
    tokens: Peekable<Iter<'a, Token>>,
    err: Vec<Error>,
    curr: Option<&'a Token>,
    pl: PartialLocation,
    line: String
}


impl<'a> AbstractSyntaxTree<'a>{

    pub fn new(tokens: &'a Vec<Token>, pl: PartialLocation, line: &str) -> Self{
        Self { statement: ST::NoneOrError, tokens: tokens.iter().peekable(), err: Vec::new(), curr: None, pl, line: line.into()}
    }

    fn advance(&mut self){
        self.curr = self.tokens.next();
    }
    /// make a node from a list of token
    /// This might call itself recursively
    fn make_expr(&mut self) -> Expr{

        match self.curr {
            Some(tk) => {
                match tk {
                    Token::Int(val) => {

                        self.advance();
                        match self.curr{ // check next token

                            Some(possible_operator) => {
                                // we check if it's an operator or a closing paren
                                println!("a token was found");

                                match possible_operator {
                                    Token::Plus => {
                                        self.advance();
                                        Expr::BinaryExpr { l: Box::new(Expr::IntLitteral(*val)), r: Box::new(self.make_expr()), opr: Operator::Plus }
                                    },

                                    Token::ClosingParen => {// if it's a paren just return the value
                                        Expr::IntLitteral(*val)
                                    }
                                    Token::Minus | Token::Div | Token::Mul => {
                                        todo!("implement other operator in make_expr")
                                    }

                                    _ => {
                                        panic!("Excepted operator, found {:?}", possible_operator);
                                    }

                                }
                            },
                            None => { // no more token
                                
                                Expr::IntLitteral(*val)
                            }
                        }
                    },

                    Token::String(val) => {
                        self.advance();
                        match self.curr{ // check next token

                            Some(possible_operator) => {
                                // we check if it's an operator or a closing paren
                                println!("a token was found");

                                match possible_operator {
                                    Token::Plus => {
                                        self.advance();
                                        Expr::BinaryExpr { l: Box::new(Expr::StringLitteral(val.clone())), r: Box::new(self.make_expr()), opr: Operator::Plus }
                                    },

                                    Token::ClosingParen => {// if it's a paren just return the value
                                        Expr::StringLitteral(val.clone())
                                    }
                                    Token::Minus | Token::Div | Token::Mul => {
                                        todo!("implement other operator in make_expr")
                                    }

                                    _ => {
                                        panic!("Excepted operator, found {:?}", possible_operator);
                                    }

                                }
                            },
                            None => { // no more token
                                
                                Expr::StringLitteral(val.clone())
                            }
                        }
                    },

                    Token::OpeningParen => {
                        self.advance(); // skip the opening paren
                        if let Some(token) = self.curr{
                            if let Token::ClosingParen = token{
                                panic!("unit-type are not allowed")

                            }
                        }


                        let tmp = self.make_expr();
                        match self.curr{
                            Some(tk) => {
                                match tk {
                                    Token::ClosingParen => {
                                        self.advance();
                                        tmp
                                    },
                                    _ => {
                                        panic!("a token was found but not an closing paren");
                                    }
                                }
                            },
                            None => {
                                panic!("missing closing paren")
                            }
                        }

                    },
                    _ => {
                        panic!("unexcepted token {:?}", tk)
                    }
                }
            }
            None => Expr::Error
        }

    }

    fn make_var_statement(&mut self) -> Statement{
        // first token must be Keyword("var")
        // now we check that the second is an identifier
        
        //TODO redo

        let mut identifier = String::new();

        self.advance();

        match self.curr {
            Some(Token::Identifier(id)) => {
                identifier = id.clone();
            },
            _ => {
                self.err.push(Error::excepted_token(
                    Location::from(self.pl.clone()), 
                    self.line.clone(), "Identifier".into())
                )
            }
            
        };

        self.advance();

        match self.curr {
            Some(Token::Assign) => {
                // nice
            },
            _ => {
                self.err.push(Error::excepted_token(
                    Location::from(self.pl.clone()), 
                    self.line.clone(), "Assign".into())
                )
            }
        };

        self.advance();

        Statement::VarDeclaration { identifier: identifier, value: self.make_expr() }

    }

    pub fn build_tree(&mut self){
        self.advance();

        if let Some(tk) = self.curr{
            if tk == &Token::Keyword("var".into()){
                self.statement = self.make_var_statement();
                return;
            }
        }

    }

    pub fn result(self) -> CompilerResult<Statement>{
        if self.err.is_empty(){
            Ok(self.statement)
        }
        else {
            Err(self.err)
        }
    }

}

#[cfg(test)]
mod test{

    use crate::{token::Tokenizer, errors::PartialLocation};

    use super::*;

    #[test]
    fn var_declaration(){
        let line = String::from("var baba = \"lol\"");
        let mut token = Tokenizer::new(&line, PartialLocation::testing(0));
        token.tokenize();
        let result = token.result().unwrap();

        let mut parser = AbstractSyntaxTree::new(&result, PartialLocation::testing(0), &line);
        parser.build_tree();
        let r = parser.result().unwrap();
        dbg!(r);
    }

    #[test]
    fn expr(){
        let line = String::from("var hello = (25)");
        let mut token = Tokenizer::new(&line, PartialLocation::testing(0));
        token.tokenize();
        let result = token.result();

        assert!(result.is_ok());
        let tokens = result.unwrap();

        let mut parser = AbstractSyntaxTree::new(&tokens, PartialLocation::testing(0), &line);
        dbg!(parser.build_tree()); // idk why this is not private but thats cool
        dbg!(parser.result());


    }

    #[test]
    fn string_expr(){
        let line = String::from("var hello = \"hello wolrd\" + \"no\" + 25");
        let mut token = Tokenizer::new(&line, PartialLocation::testing(0));
        token.tokenize();
        let result = token.result();

        assert!(result.is_ok());
        let tokens = result.unwrap();

        let mut parser = AbstractSyntaxTree::new(&tokens, PartialLocation::testing(0), &line);
        dbg!(parser.build_tree()); // idk why this is not private but thats cool
        dbg!(parser.result());
    }

    #[test]
    #[should_panic]
    fn expr_unimplemented(){
        let line = String::from("var hello = \"hello wolrd\" + \"no\" + 25 * 5");
        let mut token = Tokenizer::new(&line, PartialLocation::testing(0));
        token.tokenize();
        let result = token.result();

        assert!(result.is_ok());
        let tokens = result.unwrap();

        let mut parser = AbstractSyntaxTree::new(&tokens, PartialLocation::testing(0), &line);
        dbg!(parser.build_tree()); // idk why this is not private but thats cool
        dbg!(parser.result());
    }

}