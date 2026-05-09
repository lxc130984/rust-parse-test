use std::{ops::Range, rc::Rc,};
use ariadne::{ ColorGenerator, Label, Report,  Source};




#[derive(Debug,Clone)]
pub enum Value{
    Unit,
    Num(f64),
    Bool(bool),
    
}

impl Value{
    fn value_add(a_value:Value,b_value:Value,r:Range<usize>)->Result<Value,EvalError>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Num(a+b))},
	    _=>{
		Err(EvalError::ErrorMessage{
		    eval_match:"Add".to_string(),
		    range:r
		})
	    },
	}
    }
    fn value_sub(a_value:Value,b_value:Value,r:Range<usize>)->Result<Value,EvalError>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Num(a-b))},
	    _=>{
		Err(EvalError::ErrorMessage{
		    eval_match:"Sub".to_string(),
		    range:r
		})
	    },
	}
    }
    fn value_mul(a_value:Value,b_value:Value,r:Range<usize>)->Result<Value,EvalError>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Num(a*b))},
	    _=>{
		Err(EvalError::ErrorMessage{
		    eval_match:"Mul".to_string(),
		    range:r
		})
	    },
	}
    }
    fn value_div(a_value:Value,b_value:Value,r:Range<usize>)->Result<Value,EvalError>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Num(a/b))},
	    _=>{
		Err(EvalError::ErrorMessage{
		    eval_match:"Div".to_string(),
		    range:r
		})
	    },
	}
    }
    fn value_neg(a_value:Value,r:Range<usize>)->Result<Value,EvalError>{
	match a_value{
	    Value::Num(a)=>{Ok(Value::Num(-a))},
	    _=>{
		Err(EvalError::ErrorMessage{
		    eval_match:"Neg".to_string(),
		    range:r
		})
	    },
	}
    }

    fn value_eq(a_value:Value,b_value:Value,r:Range<usize>)->Result<Value,EvalError>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Bool(a==b))},
	    (Value::Bool(a),Value::Bool(b))=>{Ok(Value::Bool(a==b))},
	    _=>{
		Err(EvalError::ErrorMessage{
		    eval_match:"Eq".to_string(),
		    range:r
		})
	    },
	}
    }
    fn value_neq(a_value:Value,b_value:Value,r:Range<usize>)->Result<Value,EvalError>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Bool(a!=b))},
	    (Value::Bool(a),Value::Bool(b))=>{Ok(Value::Bool(a==b))},
	    _=>{
		Err(EvalError::ErrorMessage{
		    eval_match:"Neq".to_string(),
		    range:r
		})
	    },
	}
    }
    fn value_gt(a_value:Value,b_value:Value,r:Range<usize>)->Result<Value,EvalError>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Bool(a>b))},
	    _=>{
		Err(EvalError::ErrorMessage{
		    eval_match:"Gt".to_string(),
		    range:r
		})
	    },
	}
    }
    fn value_lt(a_value:Value,b_value:Value,r:Range<usize>)->Result<Value,EvalError>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Bool(a<b))},
	    _=>{
		Err(EvalError::ErrorMessage{
		    eval_match:"Lt".to_string(),
		    range:r
		})
	    },
	}
    }
    fn value_ge(a_value:Value,b_value:Value,r:Range<usize>)->Result<Value,EvalError>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Bool(a>=b))},
	    _=>{
		Err(EvalError::ErrorMessage{
		    eval_match:"Ge".to_string(),
		    range:r
		})
	    },
	}
    }
    fn value_le(a_value:Value,b_value:Value,r:Range<usize>)->Result<Value,EvalError>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Bool(a<=b))},
	    _=>{
		Err(EvalError::ErrorMessage{
		    eval_match:"Le".to_string(),
		    range:r
		})
	    },
	}
    }

    fn value_and(a_value:Value,b_value:Value,r:Range<usize>)->Result<Value,EvalError>{
	match (a_value,b_value){
	    (Value::Bool(a),Value::Bool(b))=>{Ok(Value::Bool(a&&b))},
	    _=>{
		Err(EvalError::ErrorMessage{
		    eval_match:"And".to_string(),
		    range:r
		})
	    },
	}
    }
    fn value_or(a_value:Value,b_value:Value,r:Range<usize>)->Result<Value,EvalError>{
	match (a_value,b_value){
	    (Value::Bool(a),Value::Bool(b))=>{Ok(Value::Bool(a||b))},
	    _=>{
		Err(EvalError::ErrorMessage{
		    eval_match:"Or".to_string(),
		    range:r
		})
	    },
	}
    }
}



#[derive(Debug)]
pub enum EvalError{
    ErrorMessage{
	eval_match:String,
	range:Range<usize>,
    },
}


#[derive(Debug,Clone)]
pub enum Expr{
    Num(f64),
    Bool(bool),
    String(String,Range<usize>),
    Unit,
    
    Neg(Box<Expr>,Range<usize>),
    Sub(Box<Expr>,Box<Expr>,Range<usize>),
    Add(Box<Expr>,Box<Expr>,Range<usize>),
    Mul(Box<Expr>,Box<Expr>,Range<usize>),
    Div(Box<Expr>,Box<Expr>,Range<usize>),

    Eq(Box<Expr>,Box<Expr>,Range<usize>),
    Neq(Box<Expr>,Box<Expr>,Range<usize>),
    Gt(Box<Expr>,Box<Expr>,Range<usize>), //>
    Lt(Box<Expr>,Box<Expr>,Range<usize>), //<
    Ge(Box<Expr>,Box<Expr>,Range<usize>), //>=
    Le(Box<Expr>,Box<Expr>,Range<usize>), //<=

    And(Box<Expr>,Box<Expr>,Range<usize>),
    Or(Box<Expr>,Box<Expr>,Range<usize>),
    
    Block(Vec<Expr>,Range<usize>),
    Var(String,Range<usize>),
    
    Let{
	name:String,
	value:Box<Expr>,
	then:Box<Expr>,
	range:Range<usize>,
    },

    If{
	cond:Box<Expr>,
	then_expr:Box<Expr>,
	else_expr:Option<Box<Expr>>,
	range:Range<usize>,
    },

    Fn{
	name:String,
	args:Vec<String>,
	body:Box<Expr>,
	then:Box<Expr>,
	range:Range<usize>,
    },

    Call{
	name:String,
	args:Vec<Expr>,
	range:Range<usize>,
    }
}

pub fn eval(
    expr:&Expr,
    env:&mut Env,
)->Result<Value,EvalError>{
    match expr {
		Expr::Var(name,r)=>{
	    match env.get_var(&name){
		Some(v)=>{Ok(v.clone())},
		None=>{
		    Err(EvalError::ErrorMessage{
			eval_match:"Var".to_string(),
			range:r.clone()
		    })
		}
	    }
	},
	Expr::Block(vec,_)=>{
	    let mut last = Value::Unit;
	    for e in vec{
		last = eval(e,env)?;
	    }
	    Ok(last)
	},
	



	//改成函数方便测试


	
	Expr::Let { name, value, then ,range:_ }=>{
	    let val = eval(value,env)?;
	    env.push_var(name,val);
	    let result = eval(then,env);
	    env.pop_var();
	    result
	},
	Expr::If { cond, then_expr, else_expr ,range}=>{
	    let cond_value = eval(cond,env)?;
	    match cond_value {
		Value::Bool(true)=>{eval(then_expr,env)},
		Value::Bool(false)=>{
		    match else_expr{
			Some(e)=>{eval(e,env)},
			None=>{Ok(Value::Unit)},
		    }
		},
		_=>{
		    Err(EvalError::ErrorMessage{
			eval_match:"If".to_string(),
			range:range.clone()
		    })
		}
	    }
	},
	Expr::Fn{name,args,body,then,range:_}=>{
	    let fuc_def = FucDef{args:args.clone(),body:body.clone()};
	    env.push_func(name,fuc_def);
	    let result = eval(then,env);
	    env.pop_func();
	    result
	},
	Expr::Call{name,args,range}=>{
	    let func_find = env
		.get_func(name)
		.ok_or_else(||{
		    EvalError::ErrorMessage{
			eval_match:"Call-func_find".to_string(),
			range:range.clone()
		    }
		})?;
	    if func_find.args.len() != args.len(){
		return Err(EvalError::ErrorMessage{
		    eval_match:"Call-func_find".to_string(),
		    range:range.clone()
		});
	    }
	    let mut arg_values = Vec::new();
	    for arg in args{
		arg_values.push(eval(arg,env)?);
	    }

	    let old_len = env.bindings.len();
	    for (param_name,arg_val) in func_find.args.iter().zip(arg_values){
		env.push_var(param_name,arg_val);
	    }

	    let result = eval(&func_find.body,env);
	    env.bindings.truncate(old_len);
	    
	    result
	},

	Expr::Num(f)=>{Ok(Value::Num(*f))},
	Expr::Neg(e,r)=>{
	    Value::value_neg(eval(e,env)?,r.clone())
	},
	Expr::Add(a,b,r)=>{
	    Value::value_add(eval(a,env)?,eval(b,env)?,r.clone())
	},
	Expr::Sub(a,b,r)=>{
	    Value::value_sub(eval(a,env)?,eval(b,env)?,r.clone())
	},
	Expr::Mul(a,b,r)=>{
	    Value::value_mul(eval(a,env)?,eval(b,env)?,r.clone())
	},
	Expr::Div(a,b,r)=>{
	    Value::value_div(eval(a,env)?,eval(b,env)?,r.clone())
	},
	Expr::Bool(b)=>{
	    Ok(Value::Bool(*b))
	},
	Expr::Eq(a,b,r)=>{
	    Value::value_eq(eval(a,env)?,eval(b,env)?,r.clone())
	},
	Expr::Neq(a,b,r)=>{
	    Value::value_neq(eval(a,env)?,eval(b,env)?,r.clone())
	},
	Expr::Gt(a,b,r)=>{
	    Value::value_gt(eval(a,env)?,eval(b,env)?,r.clone())
	},
	Expr::Lt(a,b,r)=>{
	    Value::value_lt(eval(a,env)?,eval(b,env)?,r.clone())
	},
	Expr::Ge(a,b,r)=>{
	    Value::value_ge(eval(a,env)?,eval(b,env)?,r.clone())
	},
	Expr::Le(a,b,r)=>{
	    Value::value_le(eval(a,env)?,eval(b,env)?,r.clone())
	},
	Expr::And(a,b,r)=>{
	    Value::value_and(eval(a,env)?,eval(b,env)?,r.clone())
	},
	Expr::Or(a,b,r)=>{
	    Value::value_or(eval(a,env)?,eval(b,env)?,r.clone())
	},
	
	_=>{todo!()}
    }
}

#[derive(Clone)]
struct FucDef{
    args:Vec<String>,
    body:Box<Expr>,
}

#[derive(Clone)]
pub struct Env{
    bindings:Vec<(String,Value)>,
    fuctions:Vec<(String,Rc<FucDef>)>,
}

impl Env{
    pub fn new()->Self{
	Self{
	    bindings:Vec::new(),
	    fuctions:Vec::new(),
	}
    }

    fn get_var(&self,name:&str)->Option<&Value>{
	self.bindings.iter().rev()
	    .find(|(str,_)|{str==name})
	    .map(|(_,value)|{value})
    }

    fn push_var(&mut self,name:&str,value:Value){
	self.bindings.push((name.to_string(),value));
    }
    
    fn pop_var(&mut self){
	self.bindings.pop();
    }

    fn get_func(&self,name:&str)->Option<Rc<FucDef>>{
	self.fuctions.iter().rev()
	    .find(|(str,_)|{str==name})
	    .map(|(_,fuc_def)|{fuc_def.clone()})
    }

    fn push_func(&mut self,name:&str,fuc_def:FucDef){
	self.fuctions.push((name.to_string(),Rc::new(fuc_def)));
    }

    fn pop_func(&mut self){
	self.fuctions.pop();
    }

}

pub fn value_handle(v:Value){
    match v{
	Value::Unit=>{println!("()")},
	Value::Num(f)=>{println!("{}",f)},
	Value::Bool(b)=>{println!("{}",b)},
    }
}

pub fn eval_error_handle(error:EvalError,str:&str){
    let mut colors = ColorGenerator::new();
    let a = colors.next();
    match error{
	EvalError::ErrorMessage { eval_match, range }=>{
	    Report::build(ariadne::ReportKind::Error,("<eval error>",range.clone()))
		.with_message("eval error")
		.with_label(
		    Label::new(("<eval error>",range.clone()))
			.with_message(format!("eval error: error_kind : {} ",eval_match))
			.with_color(a)
		)
		.finish()
		.print(("<eval error>",Source::from(str)))
		.unwrap();
	},
	
    }
    
}
