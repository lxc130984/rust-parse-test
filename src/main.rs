use chumsky::{ prelude::*};
use rust_parse_test::{Env, eval, parse_error_handle, parser, value_handle};
use rustyline::{DefaultEditor, error::ReadlineError};

fn main(){
    let nth = std::env::args().nth(1);
    match nth{
	Some(str)=>{
	    readfile_eval(str)
	},
	None=>{readline_eval()}
    }
    
}

fn readfile_eval(str:String){
    
    match std::fs::read_to_string(str){
	Ok(str)=>{
	let mut env = Env::new();
    output(&str,&mut env)
	},
	Err(e)=>{println!("{:?}",e)}
    }
    
    
    
}

fn readline_eval(){
    let mut rl = DefaultEditor::new().unwrap();
    let mut env = Env::new();
    loop{
	let readline = rl.readline(">> ");
	match readline{
	    Ok(line)=>{
		rl.add_history_entry(&line).unwrap();
		output(&line,&mut env)
	    },
	    Err(ReadlineError::Eof)|Err(ReadlineError::Interrupted)=>{break},
	    Err(e)=>{println!("{:?}",e);break}
	}
    }
}

fn output(str:&str,env:&mut Env){
 
    match parser().parse(str).into_result(){
	Ok(expr)=>{
	    //println!("{:#?}",expr);
	    let eval_result=eval(&expr,env);
	    match eval_result{
		Ok(v)=>{value_handle(v)}
		Err(str)=>{println!("{}",str);}
	    }
	},
	Err(e)=>{parse_error_handle(e,str)},
    }
}


