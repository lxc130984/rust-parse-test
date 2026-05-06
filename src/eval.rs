
#[derive(Debug,Clone)]
pub enum Value{
    Unit,
    Num(f64),
    Bool(bool),
    
}

impl Value{
    fn value_add(a_value:Value,b_value:Value)->Result<Value,String>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Num(a+b))},
	    _=>{Err("error when eval_add".to_string())}
	}
    }
    fn value_sub(a_value:Value,b_value:Value)->Result<Value,String>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Num(a-b))},
	    _=>{Err("error when eval_sub".to_string())}
	}
    }
    fn value_mul(a_value:Value,b_value:Value)->Result<Value,String>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Num(a*b))},
	    _=>{Err("error when eval_mul".to_string())}
	}
    }
    fn value_div(a_value:Value,b_value:Value)->Result<Value,String>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Num(a/b))},
	    _=>{Err("error when eval_div".to_string())}
	}
    }
    fn value_neg(a_value:Value)->Result<Value,String>{
	match a_value{
	    Value::Num(a)=>{Ok(Value::Num(-a))},
	    _=>{Err("error when eval_add".to_string())}
	}
    }

    fn value_eq(a_value:Value,b_value:Value)->Result<Value,String>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Bool(a==b))},
	    (Value::Bool(a),Value::Bool(b))=>{Ok(Value::Bool(a==b))},
	    _=>{Err("error when eval_eq".to_string())}
	}
    }
    fn value_neq(a_value:Value,b_value:Value)->Result<Value,String>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Bool(a!=b))},
	    (Value::Bool(a),Value::Bool(b))=>{Ok(Value::Bool(a==b))},
	    _=>{Err("error when eval_neq".to_string())}
	}
    }
    fn value_gt(a_value:Value,b_value:Value)->Result<Value,String>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Bool(a>b))},
	    _=>{Err("error when eval_gt".to_string())}
	}
    }
    fn value_lt(a_value:Value,b_value:Value)->Result<Value,String>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Bool(a<b))},
	    _=>{Err("error when eval_lt".to_string())}
	}
    }
    fn value_ge(a_value:Value,b_value:Value)->Result<Value,String>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Bool(a>=b))},
	    _=>{Err("error when eval_ge".to_string())}
	}
    }
    fn value_le(a_value:Value,b_value:Value)->Result<Value,String>{
	match (a_value,b_value){
	    (Value::Num(a),Value::Num(b))=>{Ok(Value::Bool(a<=b))},
	    _=>{Err("error when eval_le".to_string())}
	}
    }

    fn value_and(a_value:Value,b_value:Value)->Result<Value,String>{
	match (a_value,b_value){
	    (Value::Bool(a),Value::Bool(b))=>{Ok(Value::Bool(a&&b))},
	    _=>{Err("error when eval_and".to_string())}
	}
    }
    fn value_or(a_value:Value,b_value:Value)->Result<Value,String>{
	match (a_value,b_value){
	    (Value::Bool(a),Value::Bool(b))=>{Ok(Value::Bool(a||b))},
	    _=>{Err("error when eval_or".to_string())}
	}
    }
}




#[derive(Debug,Clone)]
pub enum Expr<'src>{
    Num(f64),
    Bool(bool),
    String(&'src str),
    Unit,
    
    Neg(Box<Expr<'src>>),
    Sub(Box<Expr<'src>>,Box<Expr<'src>>),
    Add(Box<Expr<'src>>,Box<Expr<'src>>),
    Mul(Box<Expr<'src>>,Box<Expr<'src>>),
    Div(Box<Expr<'src>>,Box<Expr<'src>>),

    Eq(Box<Expr<'src>>,Box<Expr<'src>>),
    Neq(Box<Expr<'src>>,Box<Expr<'src>>),
    Gt(Box<Expr<'src>>,Box<Expr<'src>>), //>
    Lt(Box<Expr<'src>>,Box<Expr<'src>>), //<
    Ge(Box<Expr<'src>>,Box<Expr<'src>>), //>=
    Le(Box<Expr<'src>>,Box<Expr<'src>>), //<=

    And(Box<Expr<'src>>,Box<Expr<'src>>),
    Or(Box<Expr<'src>>,Box<Expr<'src>>),
    
    Block(Vec<Expr<'src>>),
    Let(String,Box<Expr<'src>>,Box<Expr<'src>>),
    Var(String),

    If(Box<Expr<'src>>,Box<Expr<'src>>,Option<Box<Expr<'src>>>),
    
}

pub fn eval<'src>(
    expr:&'src Expr<'src>,
    env:&mut Env,
)->Result<Value,String>{
    match expr {
	Expr::Num(f)=>{Ok(Value::Num(*f))},
	Expr::Neg(e)=>{
	    Value::value_neg(eval(e,env)?)
	},
	Expr::Add(a,b)=>{
	    Value::value_add(eval(a,env)?,eval(b,env)?)
	},
	Expr::Sub(a,b)=>{
	    Value::value_sub(eval(a,env)?,eval(b,env)?)
	},
	Expr::Mul(a,b)=>{
	    Value::value_mul(eval(a,env)?,eval(b,env)?)
	},
	Expr::Div(a,b)=>{
	    Value::value_div(eval(a,env)?,eval(b,env)?)
	},
	Expr::Bool(b)=>{
	    Ok(Value::Bool(*b))
	},
	Expr::Eq(a,b)=>{
	    Value::value_eq(eval(a,env)?,eval(b,env)?)
	},
	Expr::Neq(a,b)=>{
	    Value::value_neq(eval(a,env)?,eval(b,env)?)
	},
	Expr::Gt(a,b)=>{
	    Value::value_gt(eval(a,env)?,eval(b,env)?)
	},
	Expr::Lt(a,b)=>{
	    Value::value_lt(eval(a,env)?,eval(b,env)?)
	},
	Expr::Ge(a,b)=>{
	    Value::value_ge(eval(a,env)?,eval(b,env)?)
	},
	Expr::Le(a,b)=>{
	    Value::value_le(eval(a,env)?,eval(b,env)?)
	},
	Expr::And(a,b)=>{
	    Value::value_and(eval(a,env)?,eval(b,env)?)
	},
	Expr::Or(a,b)=>{
	    Value::value_or(eval(a,env)?,eval(b,env)?)
	},
	Expr::Var(name)=>{
	    match env.get(&name){
		Some(v)=>{Ok(v)},
		None=>{Err("eval error when var,can not find the var".to_string())}
	    }
	},
	Expr::Block(vec)=>{
	    let mut last = Value::Unit;
	    for e in vec{
		last = eval(e,env)?;
	    }
	    Ok(last)
	},
	Expr::Let(name,value,body)=>{
	    let val = eval(value,env)?;
	    env.push(&name,val);
	    let result = eval(body,env);
	    env.pop();
	    result
	},
	Expr::If(cond,then_expr,else_expr)=>{
	    let cond_value = eval(cond,env)?;
	    match cond_value {
		Value::Bool(true)=>{eval(then_expr,env)},
		Value::Bool(false)=>{
		    match else_expr{
			Some(e)=>{eval(e,env)},
			None=>{Ok(Value::Unit)},
		    }
		},
		_=>{Err("error eval if-else".to_string())},
	    }
	}
	_=>{todo!()}
    }
}

pub struct Env{
    bindings:Vec<(String,Value)>,
}

impl Env{
    pub fn new()->Self{
	Self{
	    bindings:Vec::new(),
	}
    }

    fn get(&self,name:&str)->Option<Value>{
	self.bindings.iter().rev()
	    .find(|(str,_)|{str==name})
	    .map(|(_,value)|{value.clone()})
    }

    fn push(&mut self,name:&str,value:Value){
	self.bindings.push((name.to_string(),value));
    }
    
    fn pop(&mut self){
	self.bindings.pop();
    }

}

pub fn value_handle(v:Value){
    match v{
	Value::Unit=>{println!("()")},
	Value::Num(f)=>{println!("{}",f)},
	Value::Bool(b)=>{println!("{}",b)},
    }
}
