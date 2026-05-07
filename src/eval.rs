
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
pub enum Expr{
    Num(f64),
    Bool(bool),
    String(String),
    Unit,
    
    Neg(Box<Expr>),
    Sub(Box<Expr>,Box<Expr>),
    Add(Box<Expr>,Box<Expr>),
    Mul(Box<Expr>,Box<Expr>),
    Div(Box<Expr>,Box<Expr>),

    Eq(Box<Expr>,Box<Expr>),
    Neq(Box<Expr>,Box<Expr>),
    Gt(Box<Expr>,Box<Expr>), //>
    Lt(Box<Expr>,Box<Expr>), //<
    Ge(Box<Expr>,Box<Expr>), //>=
    Le(Box<Expr>,Box<Expr>), //<=

    And(Box<Expr>,Box<Expr>),
    Or(Box<Expr>,Box<Expr>),
    
    Block(Vec<Expr>),
    Var(String),
    
    Let{
	name:String,
	value:Box<Expr>,
	then:Box<Expr>,
    },

    If{
	cond:Box<Expr>,
	then_expr:Box<Expr>,
	else_expr:Option<Box<Expr>>,
    },

    Fn{
	name:String,
	args:Vec<String>,
	body:Box<Expr>,
	then:Box<Expr>,
    },

    Call{
	name:String,
	args:Vec<Expr>
    }
}

pub fn eval(
    expr:&Expr,
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
	    match env.get_var(&name){
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
	Expr::Let { name, value, then }=>{
	    let val = eval(value,env)?;
	    env.push_var(name,val);
	    let result = eval(then,env);
	    env.pop_var();
	    result
	},
	Expr::If { cond, then_expr, else_expr }=>{
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
	},
	Expr::Fn{name,args,body,then}=>{
	    let fuc_def = FucDef{args:args.clone(),body:body.clone()};
	    env.push_func(name,fuc_def);
	    let result = eval(then,env);
	    env.pop_func();
	    result
	},
	Expr::Call{name,args}=>{
	    let func_find = env
		.get_func(name)
		.ok_or_else(||format!("error eval call {}",name))?
		.clone();
	    if func_find.args.len() != args.len(){
		return Err(format!("error eval call {},error args",name));
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
    fuctions:Vec<(String,FucDef)>,
}

impl Env{
    pub fn new()->Self{
	Self{
	    bindings:Vec::new(),
	    fuctions:Vec::new(),
	}
    }

    fn get_var(&self,name:&str)->Option<Value>{
	self.bindings.iter().rev()
	    .find(|(str,_)|{str==name})
	    .map(|(_,value)|{value.clone()})
    }

    fn push_var(&mut self,name:&str,value:Value){
	self.bindings.push((name.to_string(),value));
    }
    
    fn pop_var(&mut self){
	self.bindings.pop();
    }

    fn get_func(&self,name:&str)->Option<&FucDef>{
	self.fuctions.iter().rev()
	    .find(|(str,_)|{str==name})
	    .map(|(_,fuc_def)|{fuc_def})
    }

    fn push_func(&mut self,name:&str,fuc_def:FucDef){
	self.fuctions.push((name.to_string(),fuc_def));
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
