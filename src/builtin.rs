use crate::error::{runtime_error as error, Result};
use crate::token::Location;
use crate::value::Value;

pub fn print(_loc: &Location, args: Vec<Value>) -> Result<Value> {
    for (i, arg) in args.iter().enumerate() {
        if i != 0 {
            print!(" ");
        }
        match arg {
            Value::Integer(num) => print!("{}", num),
            Value::Float(num) => print!("{}", num),
            Value::String(string) => print!("{}", string),
            Value::Boolean(boolean) => print!("{}", boolean),
            Value::Nothing => print!("nothing"),
            Value::Iterator(_) => print!("<iterator>"),
            Value::Range(start, end) => print!("{}..{}", start, end),
            _ => print!("{:?}", arg),
        }
    }
    println!();
    Ok(Value::Nothing)
}

pub fn len(loc: &Location, args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        error!(loc, "len() takes exactly one argument");
    }

    Ok(match &args[0] {
        Value::String(string) => Value::Integer(string.len() as i64),
        other => error!(loc, "len() does not support {:?}", other),
    })
}

pub fn exit(loc: &Location, args: Vec<Value>) -> Result<Value> {
    let code = match args.get(0) {
        Some(Value::Integer(i)) => *i,
        Some(_) => error!(loc, "exit() may only take an integer as argument"),
        None => 0,
    };

    match code.try_into() {
        Ok(code) => std::process::exit(code),
        Err(_) => error!(
            loc,
            "argument to exit() is too large, {code} does not fit into an i32"
        ),
    }
}

pub fn input(loc: &Location, args: Vec<Value>) -> Result<Value> {
    // could be temporary, just really nice for testing.
    if args.len() == 1 {
        if let Value::String(string) = &args[0] {
            print!("{}", string);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }else{
            error!(loc, "input() takes either no arguments or a string");
        }
    } else if args.len() > 1 {
        error!(loc, "input() takes either no arguments or a string");
    }
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.ends_with('\n') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }
    Ok(Value::String(input))
}
