#[derive(Debug)]
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

    Block(Vec<Expr<'src>>),
    
}

pub fn eval<'src>(
    expr:&'src Expr<'src>,
)->Result<Value,String>{
    match expr {
	Expr::Num(f)=>{Ok(Value::Num(*f))},
	Expr::Neg(e)=>{
	    Value::value_neg(eval(e)?)
	},
	Expr::Add(a,b)=>{
	    Value::value_add(eval(a)?,eval(b)?)
	},
	Expr::Sub(a,b)=>{
	    Value::value_sub(eval(a)?,eval(b)?)
	},
	Expr::Mul(a,b)=>{
	    Value::value_mul(eval(a)?,eval(b)?)
	},
	Expr::Div(a,b)=>{
	    Value::value_div(eval(a)?,eval(b)?)
	},
	Expr::Bool(b)=>{
	    Ok(Value::Bool(*b))
	},
	Expr::Block(vec)=>{
	    let mut last = Value::Unit;
	    for e in vec{
		last = eval(e)?;
	    }
	    Ok(last)
	}
	
	
	_=>{todo!()}
    }
}

pub fn value_handle(v:Value){
    match v{
	Value::Unit=>{println!("()")},
	Value::Num(f)=>{println!("{}",f)},
	Value::Bool(b)=>{println!("{}",b)},
    }
}
