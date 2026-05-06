use chumsky::{error::RichReason, prelude::*, util::Maybe};
use ariadne::{ColorGenerator,Source,Label,Report};
use crate::eval::Expr;
	

fn atom_parse<'src>(
    expr:impl Parser<'src, &'src str ,Expr<'src>,extra::Err<Rich<'src,char>>> + 'src + Clone
)->impl Parser<'src, &'src str ,Expr<'src>,extra::Err<Rich<'src,char>>>+Clone{
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
    
    let paren = expr.delimited_by(just('(').padded(),just(')').padded()).padded(); 
    
    unary.or(bool).or(paren).or(block).or(var).boxed()//与关键字有关的var放在最后
}

fn product_parse<'src>(
    atom:impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>+'src+Clone
)->impl Parser<'src, &'src str ,Expr<'src>,extra::Err<Rich<'src,char>>>+Clone{
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
    product:impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>+'src+Clone
)->impl Parser<'src, &'src str ,Expr<'src>,extra::Err<Rich<'src,char>>>+Clone{
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
    sum:impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>+'src+Clone
)->impl Parser<'src, &'src str ,Expr<'src>,extra::Err<Rich<'src,char>>>+Clone{
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
    comparsion:impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>+Clone+'src
)->impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>+Clone{
    
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
    logic:impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>+Clone+'src,
    expr:impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>+Clone+'src
)->impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>+Clone{
    let let_parser=text::ascii::keyword("let")
        .padded()
        .ignore_then(text::ascii::ident().padded())
        .then_ignore(just('=').padded())
        .then(logic)
        .then_ignore(just(';').padded())
        .then(expr)
        .map(|((name,value),body)|{
	    Expr::Let(name.to_string(),Box::new(value),Box::new(body))
	}).boxed();
    let_parser
        
}

fn if_parser<'src>(
    expr:impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>+Clone+'src
)->impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>+Clone{
    text::ascii::keyword("if")
        .ignore_then(expr.clone())
        .then(block_parser(expr.clone()))
        .then(
	    text::ascii::keyword("else").padded()
		.ignore_then(block_parser(expr))
		.or_not()
	)
        .map(|((cond,then),else_or)|{Expr::If(Box::new(cond),Box::new(then),else_or.map(Box::new))})
        .boxed()
}


fn block_parser<'src>(
    expr:impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>+Clone+'src
)->impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>+Clone{
    expr.separated_by(just(';').padded())
	    .collect::<Vec<_>>()
	    .delimited_by(just('{').padded(),just('}').padded()).padded()
	.map(Expr::Block)
	.boxed()
}


pub fn parser<'src>()->impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>{
    recursive(|expr|{
	let expr=expr.boxed();
	let atom = atom_parse(expr.clone());
	let product = product_parse(atom);
	let sum = sum_parse(product);
	let comparsion = comparsion_parse(sum);

	let logic = logic_parser(comparsion);
	let let_ = let_parser(logic.clone(),expr.clone());
	let block = block_parser(expr.clone());
	let if_else = if_parser(expr);

	let_.or(if_else).or(logic).or(block).boxed()
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
