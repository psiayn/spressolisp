pub mod ast;
pub mod env;
pub mod errors;
pub mod eval;
mod utils;

use core::fmt;
use std::collections::{BTreeMap, VecDeque};
use std::iter::Peekable;
use std::ops::RangeInclusive;
use std::rc::Rc;
use std::str::Chars;

use colored::Colorize;
use eval::execute_single;
use itertools::Itertools;

use crate::ast::{Atom, Expr, ExprKind, Number};
use crate::env::Env;
use crate::errors::{SpressoError, SyntaxError};
use crate::utils::range_stack::RangeStack;

pub fn evaluate_expression(
    name: String,
    input: String,
    env: &mut Env,
) -> Result<Expr, SpressoError> {
    // we store individual lines of the program because we need to print lines during error
    let program_lines = input.lines().map(|s| s.to_string()).collect();
    let program = Rc::new(Program {
        name,
        text: input,
        lines: program_lines,
    });

    let mut tokenized_input: VecDeque<Token> = tokenize(program);
    let mut exprs = Vec::new();

    while !tokenized_input.is_empty() {
        exprs.push(parse(&mut tokenized_input)?);
    }

    // TODO: use a unit type
    let mut res = ExprKind::Atom(Atom::Bool(true)).into();

    for expr in exprs {
        res = execute_single(expr, env)?;
    }

    Ok(res)
}

#[derive(Debug)]
pub struct Program {
    /// Name of the program
    ///
    /// This will be `input[num]` when executing from the REPL,
    /// where `num` is the input number.
    name: String,
    /// Entire text of the program
    text: String,
    /// Text split by lines.
    ///
    /// Used to quickly get a particular line given the line number.
    lines: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Token {
    text: String,
    // TODO: some tokens like string could go across multiple lines
    // store both line_num_start and line_num_end
    line_num: usize,
    col_num_start: usize,
    col_num_end: usize,
    program: Rc<Program>,
    type_: TokenType,
}

fn display_and_mark(f: &mut fmt::Formatter<'_>, tokens: &Vec<Token>) -> fmt::Result {
    type Ranges = Vec<RangeInclusive<usize>>;
    // we store a mapping of
    // program_ptr => (program,
    //                 line_num => (ranges to highlight)
    //                )
    //
    // The program_ptr is stored as raw pointer to the underlying Program stored in the Rc.
    // A raw pointer because I didn't want to do the effort of making Program impl Eq, Hash, etc.
    // required to make it a valid key. A pointer works just fine and does not need unsafe either
    // (because we never dereference it).
    let mut program_line_map =
        BTreeMap::<*const Program, (Rc<Program>, BTreeMap<usize, Ranges>)>::new();

    tokens.iter().for_each(|token| {
        let program_key = Rc::as_ptr(&token.program);

        let (_, line_map) = program_line_map
            .entry(program_key)
            .or_insert((Rc::clone(&token.program), BTreeMap::<usize, Ranges>::new()));

        let ranges = line_map.entry(token.line_num).or_insert(Vec::new());
        ranges.push(token.col_num_start..=token.col_num_end);
    });

    for (_, (program, line_map)) in program_line_map.iter() {
        write!(f, "In {}:\n", program.name.green(),)?;

        for (line_num, ranges) in line_map.iter() {
            // print line with line number
            write!(
                f,
                "{}| {}\n",
                format!("{:<width$}", line_num, width = 4).blue(),
                program.lines[*line_num - 1],
            )?;

            let ranges: RangeStack = ranges.clone().into_iter().collect();
            let first_start = *ranges.ranges.first().unwrap_or(&(0..=0)).start();
            write!(f, "{}", " ".repeat(4 + 2 - 1 + first_start))?;
            let mut last_marked = first_start;
            for range in ranges.ranges {
                write!(
                    f,
                    "{space}{marker}",
                    marker = "^".repeat(range.end() - range.start()).yellow(),
                    space = " ".repeat(range.start() - last_marked)
                )?;
                last_marked = *range.end();
            }
            write!(f, "\n")?;
        }
    }

    Ok(())
}

#[derive(PartialEq, Clone, Debug)]
enum TokenType {
    OpenParen,
    CloseParen,
    Number,
    String,
    Symbol,
    Unit,
}

fn tokenize(program: Rc<Program>) -> VecDeque<Token> {
    let mut tokens = VecDeque::new();

    // we start from 1 here
    let mut line_num = 1;
    let mut col_num = 1;

    let char_processor = |c: char,
                          chars: &mut Peekable<Chars>,
                          line_num: &mut usize,
                          col_num: &mut usize|
     -> Option<(String, TokenType)> {
        let mut new_token = String::from(c);
        match c {
            '(' => {
                // a () is a unit type
                if let Some(')') = chars.peek() {
                    new_token.push(chars.next().unwrap());
                    Some((new_token, TokenType::Unit))
                } else {
                    Some((new_token, TokenType::OpenParen))
                }
            },
            ')' => Some((new_token, TokenType::CloseParen)),
            '0'..='9' | '.' => {
                // takes as long as numbers are found
                let new_chars = chars.peeking_take_while(|c| match c {
                    '0'..='9' | '.' => true,
                    // TODO: stop only at whitespace
                    // otherwise, show error
                    _ => false,
                });
                new_token.extend(new_chars);
                Some((new_token, TokenType::Number))
            }
            ' ' => {
                *col_num += 1;
                None
            }
            '\n' => {
                *line_num += 1;
                *col_num = 1;
                None
            }
            '"' => {
                // takes everything before closing '"'
                let new_chars = chars.peeking_take_while(|c| match c {
                    '"' => false,
                    _ => true,
                });
                new_token.extend(new_chars);

                // check if string is closed
                if let Some('"') = chars.peek() {
                    new_token.push(chars.next().unwrap());
                }
                // TODO: show error if string not closed

                Some((new_token, TokenType::String))
            }
            _ => {
                // take everything until some other token is found
                // TODO: move this set of chars somewhere else
                let new_chars = chars.peeking_take_while(|c| match c {
                    ' ' | '\n' | '(' | ')' => false,
                    _ => true,
                });
                new_token.extend(new_chars);

                Some((new_token, TokenType::Symbol))
            }
        }
    };

    // we will be processing each char one by one using this single iterator
    let mut chars = program.text.chars().peekable();

    // loop until chars are present
    while let Some(c) = chars.next() {
        // record starting col number
        let col_num_start = col_num;

        if let Some((new_token, type_)) = char_processor(c, &mut chars, &mut line_num, &mut col_num)
        {
            // new col number is old + size of current token
            // when there isn't any token, char_processor handles
            // incrementing col_num
            col_num += new_token.len();

            tokens.push_back(Token {
                text: new_token,
                line_num,
                col_num_start,
                col_num_end: col_num,
                program: Rc::clone(&program),
                type_,
            })
        }
    }

    tokens
}

fn parse(tokens: &mut VecDeque<Token>) -> Result<Expr, SpressoError> {
    let token = match tokens.pop_front() {
        Some(token) => token,
        // no tokens (vec was empty)
        None => return Err(SyntaxError::from("Unexpected EOF".to_string()).into()),
    };

    match token.type_ {
        TokenType::OpenParen => {
            // collect everything before ")"
            let mut ast: Vec<Expr> = Vec::new();
            while !tokens.is_empty() && tokens[0].type_ != TokenType::CloseParen {
                // recursively parse each of them
                let inner_ast = parse(tokens)?;
                ast.push(inner_ast);
            }

            // there should be a closing ")" after parsing everything inside
            if let None = tokens.pop_front() {
                return Err(
                    SpressoError::from(SyntaxError::from("'(' not closed")).with_token(token)
                );
            }

            return Ok(ExprKind::List(ast).into());
        }
        TokenType::CloseParen => {
            return Err(SpressoError::from(SyntaxError::from("Unexpected ')'")).with_token(token))
        }
        _ => Ok(Expr::from(ExprKind::Atom(parse_atom(token.clone())?)).with_token(token)),
    }
}

fn parse_atom(token: Token) -> Result<Atom, SpressoError> {
    match token.type_ {
        TokenType::Number => {
            let text = token.text.clone();

            if let Ok(num) = text.parse::<i64>() {
                return Ok(Atom::Number(Number::Int(num)));
            }

            if let Ok(num) = text.parse::<f64>() {
                return Ok(Atom::Number(Number::Float(num)));
            }

            Err(SpressoError::from(SyntaxError::from("Could not parse number")).with_token(token))
        }
        TokenType::Unit => Ok(Atom::Unit),
        // remove quotes from string token and store
        TokenType::String => Ok(Atom::String(
            token.text[1..token.text.len() - 1].to_string(),
        )),
        TokenType::Symbol => Ok(Atom::Symbol(token.text)),
        TokenType::OpenParen | TokenType::CloseParen => Err(SpressoError::from(SyntaxError::from(
            "Cannot extract atom from these lol",
        ))
        .with_token(token)),
    }
}

trait TokenHoarder {
    fn with_token(self, token: Token) -> Self;

    fn with_tokens(mut self, tokens: Vec<Token>) -> Self
    where
        Self: Sized,
    {
        for token in tokens {
            self = self.with_token(token);
        }

        self
    }

    fn maybe_with_tokens(self, tokens: Option<Vec<Token>>) -> Self
    where
        Self: Sized,
    {
        if let Some(tokens) = tokens {
            self.with_tokens(tokens)
        } else {
            self
        }
    }

    fn maybe_with_token(self, token: Option<Token>) -> Self
    where
        Self: Sized,
    {
        if let Some(token) = token {
            self.with_token(token)
        } else {
            self
        }
    }
}

// with_token should work when both value and error are hoarders
impl<T, E> TokenHoarder for Result<T, E>
where
    T: TokenHoarder,
    E: TokenHoarder,
{
    fn with_token(self, token: Token) -> Self {
        match self {
            Ok(val) => Ok(val.with_token(token)),
            Err(err) => Err(err.with_token(token)),
        }
    }
}

trait TokenGiver {
    fn get_tokens(&self) -> Option<Vec<Token>>;
}

// get_tokens should work when both value and error are givers
impl<T, E> TokenGiver for Result<T, E>
where
    T: TokenGiver,
    E: TokenGiver,
{
    fn get_tokens(&self) -> Option<Vec<Token>> {
        match self {
            Ok(val) => val.get_tokens(),
            Err(err) => err.get_tokens(),
        }
    }
}
