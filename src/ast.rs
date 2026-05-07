use chumsky::{error::RichReason, prelude::*, util::Maybe};
use ariadne::{ColorGenerator,Source,Label,Report};
use crate::eval::Expr;


fn atom_parse<'src>(
    expr:impl Parser<'src, &'src str ,Expr,extra::Err<Rich<'src,char>>> + 'src + Clone
)->impl Parser<'src, &'src str ,Expr,extra::Err<Rich<'src,char>>>+Clone{
    let int = text::int::<_,extra::Err<Rich<_>>>(10).padded().map(|s:&str|{
	Expr::Num(s.parse::<f64>().unwrap())
    });

    let unary = just('-').padded()
        .repeated()
        .foldr(int,|_,r|{Expr::Neg(Box::new(r))});

    
    let bool = just("false").padded().to(Expr::Bool(false))
        .or(just("true").padded().to(Expr::Bool(true)));

    let var = text::ascii::ident().padded()
        .map(|s:&str|{Expr::Var(s.to_string())});//这里没有筛选关键字
    

    let block = block_parser(expr.clone());
    
    let paren = expr.clone().delimited_by(just('(').padded(),just(')').padded()).padded(); 
    let call = call_parser(expr);
    unary.or(bool).or(paren).or(block).or(call).or(var).boxed()//与关键字有关的var放在最后
}

fn product_parse<'src>(
    atom:impl Parser<'src,&'src str,Expr,extra::Err<Rich<'src,char>>>+'src+Clone
)->impl Parser<'src, &'src str ,Expr,extra::Err<Rich<'src,char>>>+Clone{
    let product = atom.clone().foldl(
	choice((
	    just('*').padded().to(Expr::Mul as fn(_,_)->_),
	    just('/').padded().to(Expr::Div as fn(_,_)->_)
	))
	    .then(atom)
	    .repeated()
	    ,
	|l,(m_or_d,r)|{m_or_d(Box::new(l),Box::new(r))}
    ).boxed();
    product
}

fn sum_parse<'src>(
    product:impl Parser<'src,&'src str,Expr,extra::Err<Rich<'src,char>>>+'src+Clone
)->impl Parser<'src, &'src str ,Expr,extra::Err<Rich<'src,char>>>+Clone{
    let sum = product.clone()
	.foldl(
	    choice((
		just('+').padded().to(Expr::Add as fn(_,_)->_),
		just('-').padded().to(Expr::Sub as fn(_,_)->_)
	    ))
		.then(product)
		.repeated()
		,
	    |l,(a_or_s,r)|{a_or_s(Box::new(l),Box::new(r))}
	).boxed();
    sum
}

fn comparsion_parse<'src>(
    sum:impl Parser<'src,&'src str,Expr,extra::Err<Rich<'src,char>>>+'src+Clone
)->impl Parser<'src, &'src str ,Expr,extra::Err<Rich<'src,char>>>+Clone{
    sum.clone()
        .then(choice((
	    just("==").padded().to(Expr::Eq as fn(_, _) -> _),
	    just("!=").padded().to(Expr::Neq as fn(_, _) -> _),
	    just(">=").padded().to(Expr::Ge as fn(_, _) -> _),
	    just("<=").padded().to(Expr::Le as fn(_, _) -> _),
	    just(">").padded().to(Expr::Gt as fn(_, _) -> _),
	    just("<").padded().to(Expr::Lt as fn(_, _) -> _),
	))
	      .then(sum)
	      .or_not()
	)
        .map(|(l,o_r)|{
	    match o_r{
		Some((op,r))=>{op(Box::new(l),Box::new(r))},
		None=>{l},
	    }
	}).boxed()
}

fn logic_parser<'src>(
    comparsion:impl Parser<'src,&'src str,Expr,extra::Err<Rich<'src,char>>>+Clone+'src
)->impl Parser<'src,&'src str,Expr,extra::Err<Rich<'src,char>>>+Clone{
    
    comparsion.clone()
	.foldl(
	    choice((
		just("&&").padded().to(Expr::And as fn(_,_)->_),
		just("||").padded().to(Expr::Or as fn(_,_)->_),
	    ))
		.then(comparsion)
		.repeated()
		,
	    |l,(op,r)|{op(Box::new(l),Box::new(r))}
	)
}

fn let_parser<'src>(
    expr:impl Parser<'src,&'src str,Expr,extra::Err<Rich<'src,char>>>+Clone+'src
)->impl Parser<'src,&'src str,Expr,extra::Err<Rich<'src,char>>>+Clone{
    let let_parser=text::ascii::keyword("let")
        .padded()
        .ignore_then(text::ascii::ident().padded())
        .then_ignore(just('=').padded())
        .then(expr.clone())
        .then_ignore(just(';').padded())
        .then(expr)
        .map(|((name,value),body)|{
	    Expr::Let{
		name:name.to_string(),
		value:Box::new(value),
		then:Box::new(body)
	    }
	}).boxed();
    let_parser
        
}

fn if_parser<'src>(
    expr:impl Parser<'src,&'src str,Expr,extra::Err<Rich<'src,char>>>+Clone+'src
)->impl Parser<'src,&'src str,Expr,extra::Err<Rich<'src,char>>>+Clone{
    text::ascii::keyword("if")
        .ignore_then(expr.clone())
        .then(block_parser(expr.clone()))
        .then(
	    text::ascii::keyword("else").padded()
		.ignore_then(block_parser(expr))
		.or_not()
	)
        .map(|((cond,then),else_or)|{Expr::If{
	    cond:Box::new(cond),
	    then_expr:Box::new(then),
	    else_expr:else_or.map(Box::new)
	}})
        .boxed()
}


fn block_parser<'src>(
    expr:impl Parser<'src,&'src str,Expr,extra::Err<Rich<'src,char>>>+Clone+'src
)->impl Parser<'src,&'src str,Expr,extra::Err<Rich<'src,char>>>+Clone{
    expr.separated_by(just(';').padded())
	.collect::<Vec<_>>()
	.delimited_by(just('{').padded(),just('}').padded()).padded()
	.map(Expr::Block)
	.boxed()
}

fn func_parser<'src>(
    expr:impl Parser<'src,&'src str,Expr,extra::Err<Rich<'src,char>>>+Clone+'src
)->impl Parser<'src,&'src str,Expr,extra::Err<Rich<'src,char>>>+Clone{
    text::ascii::keyword("fn")
        .ignore_then(text::ascii::ident().padded())
        .then(
	    text::ascii::ident().padded()
		.separated_by(just(',').padded())
		.allow_trailing()
		.collect::<Vec<_>>()
		.delimited_by(just('(').padded(),just(')').padded())
		
	)
        .then(block_parser(expr.clone()))
        .then(expr)
        .map(|(((name,args),body),then)|{
	    Expr::Fn{name:name.to_string(),
		     args:args.iter().map(|s|{s.to_string()}).collect(),
		     body:Box::new(body),then:Box::new(then)}
	})
        .boxed()
}

fn call_parser<'src>(
    expr:impl Parser<'src,&'src str,Expr,extra::Err<Rich<'src,char>>>+Clone+'src
)->impl Parser<'src,&'src str,Expr,extra::Err<Rich<'src,char>>>+Clone{
    text::ascii::ident()
        .then(expr.clone()
              .separated_by(just(',').padded())
              .allow_trailing()
              .collect::<Vec<_>>()
              .delimited_by(just('(').padded(),just(')').padded())
	)
        .map(|(name,args)|{Expr::Call{name:name.to_string(),args:args}})
        .boxed()
	
	
        
}


pub fn parser<'src>()->impl Parser<'src,&'src str,Expr,extra::Err<Rich<'src,char>>>{
    recursive(|expr|{
	let expr=expr.boxed();
	let atom = atom_parse(expr.clone());
	let product = product_parse(atom);
	let sum = sum_parse(product);
	let comparsion = comparsion_parse(sum);

	let logic = logic_parser(comparsion);
	let let_ = let_parser(expr.clone());
	let block = block_parser(expr.clone());
	let if_else = if_parser(expr.clone());
	let func = func_parser(expr.clone());
	
	let_.or(if_else).or(func).or(logic).or(block).boxed() //含有关键字的要在logic前面
    })
}




pub fn parse_error_handle(error:Vec<Rich<char>>,str:&str){
    let mut colors=ColorGenerator::new();
    let a = colors.next();
    for e in error{
	match e.reason(){
	    RichReason::ExpectedFound { expected, found }=>{
		Report::build(ariadne::ReportKind::Error,("<parse error>",e.span().into_range()))
		    .with_message("parse error")
		    .with_label(
			Label::new(("<parse error>",e.span().into_range()))
			    .with_message(format!("parse error, expected:{:?},found:{:?}",
						  expected,found.unwrap_or(Maybe::from(' '))))
			    .with_color(a))
		    .finish()
		    .print(("<parse error>",Source::from(str)))
		    .unwrap();
	    },
	    _=>{},
	}
    }
}
