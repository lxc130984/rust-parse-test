use chumsky::{error::RichReason, prelude::*, util::Maybe};
use ariadne::{ColorGenerator,Source,Label,Report};
use crate::eval::Expr;

// fn logical_parse<'src>()->impl Parser<'src, &'src str ,Expr<'src>,extra::Err<Rich<'src,char>>>{
//     comparsion_parse()
//         .or(bool_parser())
// 	.foldl(
// 	    choice((
// 		just("&&").padded().to(Expr::And as fn(_,_)->_),
// 		just("||").padded().to(Expr::Or as fn(_,_)->_),
// 	    ))
// 		.then(comparsion_parse().or(bool_parser()))
// 		.repeated()
// 		,
// 	    |l,(op,r)|{op(Box::new(l),Box::new(r))}
// 	)
	
// }



// fn comparsion_parse<'src>()->impl Parser<'src, &'src str ,Expr<'src>,extra::Err<Rich<'src,char>>>{
//     arithmetic_parse()
//         .or(bool_parser())
//         .then(choice((
// 	    just("==").padded().to(Expr::Eq as fn(_, _) -> _),
//             just("!=").padded().to(Expr::Neq as fn(_, _) -> _),
//             just(">=").padded().to(Expr::Ge as fn(_, _) -> _),
//             just("<=").padded().to(Expr::Le as fn(_, _) -> _),
//             just(">").padded().to(Expr::Gt as fn(_, _) -> _),
//             just("<").padded().to(Expr::Lt as fn(_, _) -> _),
// 	))
// 	      .then(arithmetic_parse().or(bool_parser()))
// 	      .or_not()
// 	)
//         .map(|(l,o_r)|{
// 	    match o_r{
// 		Some((op,r))=>{op(Box::new(l),Box::new(r))},
// 		None=>{l},
// 	    }
// 	})
        
// }

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

fn comparsion_parser<'src>(
    expr:impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>+Clone+'src
)->impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>{
    let comparsion_expected=arithmetic_parse()
            .or(bool_parser())
        .or(block_parser(expr.clone()))
        .boxed();
    comparsion_expected.clone()
            .then(choice((
		just("==").padded().to(Expr::Eq as fn(_, _) -> _),
		just("!=").padded().to(Expr::Neq as fn(_, _) -> _),
		just(">=").padded().to(Expr::Ge as fn(_, _) -> _),
		just("<=").padded().to(Expr::Le as fn(_, _) -> _),
		just(">").padded().to(Expr::Gt as fn(_, _) -> _),
		just("<").padded().to(Expr::Lt as fn(_, _) -> _),
	    ))
		  .then(comparsion_expected)
		  .or_not()
	    )
            .map(|(l,o_r)|{
		match o_r{
		    Some((op,r))=>{op(Box::new(l),Box::new(r))},
		    None=>{l},
		}
	    })
}

fn logic_parser<'src>(
    comparsion:impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>+Clone+'src
)->impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>{
    let comparsion_and_bool=comparsion.or(bool_parser()).boxed();
    comparsion_and_bool.clone()
	    .foldl(
		choice((
		    just("&&").padded().to(Expr::And as fn(_,_)->_),
		    just("||").padded().to(Expr::Or as fn(_,_)->_),
		))
		    .then(comparsion_and_bool)
		    .repeated()
		    ,
		|l,(op,r)|{op(Box::new(l),Box::new(r))}
	    )
}


fn block_parser<'src>(
    expr:impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>
)->impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>{
    expr.separated_by(just(';').padded())
	    .collect::<Vec<_>>()
	    .delimited_by(just('{').padded(),just('}').padded()).padded()
	    .map(Expr::Block)
}

pub fn parser<'src>()->impl Parser<'src,&'src str,Expr<'src>,extra::Err<Rich<'src,char>>>{
    recursive(|expr|{
	let comparsion = comparsion_parser(expr.clone()).boxed();
	let logic = logic_parser(comparsion).boxed();
	let block = block_parser(expr).boxed();
	logic.or(block.clone())
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
