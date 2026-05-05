use chumsky::{error::RichReason, prelude::*, util::Maybe};
use ariadne::{ColorGenerator,Source,Label,Report};
use crate::eval::Expr;


fn arithmetic_parse<'src>()->impl Parser<'src, &'src str ,Expr<'src>,extra::Err<Rich<'src,char>>>{
    let int = text::int::<_,extra::Err<Rich<_>>>(10).padded().map(|s:&str|{
	Expr::Num(s.parse::<f64>().unwrap())
    });
    let unary = just('-').padded()
        .repeated()
        .foldr(int,|_,r|{Expr::Neg(Box::new(r))});
    let product = unary.foldl(
	choice((
	    just('*').padded().to(Expr::Mul as fn(_,_)->_),
	    just('/').padded().to(Expr::Div as fn(_,_)->_)
	))
	    .then(unary)
	    .repeated()
	    ,
	|l,(m_or_d,r)|{m_or_d(Box::new(l),Box::new(r))}
    );
    let sum = product.foldl(
	choice((
	    just('+').padded().to(Expr::Add as fn(_,_)->_),
	    just('-').padded().to(Expr::Sub as fn(_,_)->_)
	))
	    .then(product)
	    .repeated()
	    ,
	|l,(a_or_s,r)|{a_or_s(Box::new(l),Box::new(r))}
    );
    sum
}

pub fn bool_parser<'src>()->impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>{
    just("false").padded().to(Expr::Bool(false))
        .or(just("true").padded().to(Expr::Bool(true)))
}

pub fn parser<'src>()->impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>{
    recursive(|expr|{
	
	let block = expr
	    .separated_by(just(';').padded())
	    .collect::<Vec<_>>()
	    .delimited_by(just('{').padded(),just('}').padded()).padded()
	    .map(Expr::Block);

	arithmetic_parse()
	    .or(bool_parser())
	    .or(block)
	    .boxed()
	
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
