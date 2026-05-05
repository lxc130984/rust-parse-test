use chumsky::{ prelude::*};
use rust_parse_test::{parser,eval,value_handle,parse_error_handle};
use rustyline::{DefaultEditor, error::ReadlineError};

fn main(){
    let mut rl = DefaultEditor::new().unwrap();
    loop{
	let readline = rl.readline(">> ");
	match readline{
	    Ok(line)=>{
		rl.add_history_entry(&line).unwrap();
		output(&line)
	    },
	    Err(ReadlineError::Eof)|Err(ReadlineError::Interrupted)=>{break},
	    Err(e)=>{println!("{:?}",e);break}
	}
    }
}



fn output(str:&str){
    match parser().parse(str).into_result(){
	Ok(expr)=>{
	    println!("{:#?}",expr);
	    let eval_result=eval(&expr);
	    match eval_result{
		Ok(v)=>{value_handle(v)}
		Err(str)=>{println!("{}",str);}
	    }
	},
	Err(e)=>{parse_error_handle(e,str)},
    }
}


