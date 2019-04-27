#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {Add,Sub,Mul,Div,}
#[derive(Debug, PartialEq)]
pub enum InfixToken {Operator(Operator),Operand(isize),LeftParen,RightParen,}
#[derive(Debug, PartialEq)]
pub enum PostfixToken {Operator(Operator),Operand(isize),}
/// Transforms an infix expression to a postfix expression.
///
/// If the infix expression is valid, outputs `Some(_)`; 
/// otherwise, outputs `None`.
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
	
	match tokens[tokens.len()-1] {
		InfixToken::Operator(_) => return None,
		InfixToken::LeftParen => return None,
		_ => {},
	}
	
	match tokens[0] {
		InfixToken::Operator(_) => return None,
		InfixToken::RightParen => return None,
		_ => {},
	}
	
	let mut value:Vec<PostfixToken> = vec![];
    let mut stack:Vec<InfixToken> = vec![];
    
    let mut num1 = 0;
    let mut num2 = 0;

    for i in 0..(tokens.len()-1) {
    	//println!("{:?}",stack);
    	//println!("{:?}",value);

    	match tokens[i] {
    		InfixToken::Operand(x) => {
    			value.push(PostfixToken::Operand(x));
    			
    			match tokens[i+1] {
    				InfixToken::Operand(_) => return None,
    				InfixToken::LeftParen => return None,
    				_ => {},
    			}
    		},
    		
    		InfixToken::LeftParen => {
    			num1 += 1;
    			stack.push(InfixToken::LeftParen);
    			
    			match tokens[i+1] {
    				InfixToken::Operator(_) => return None,
    				InfixToken::RightParen => return None,
    				_ => {},
    			}
    		}, 
    		
    		InfixToken::RightParen => {
    			num2 += 1;
    			
    			match tokens[i+1] {
    				InfixToken::Operand(_) => return None,
    				InfixToken::LeftParen => return None,
    				_ => {},
    			}

    			loop {
    				let a = stack.pop();
    				match a {
    					Some(InfixToken::Operator(x)) => value.push(PostfixToken::Operator(x)),
    					Some(InfixToken::LeftParen) => break,
    					None => return None,
    					_ => return None,
    				}
    			}
    		},

    		InfixToken::Operator(x) => {
    			match tokens[i+1] {
    				InfixToken::Operator(_) => return None,
    				InfixToken::RightParen => return None,
    				_ => {},
    			}

    			if stack.is_empty(){
    				stack.push(InfixToken::Operator(x));
    				println!("{:?}",stack );
    			}

    			else {
    				loop {
    					let a = stack.pop();
    					match a {
    						Some(InfixToken::Operator(b)) => {
    							match b {
    								Operator::Div => value.push(PostfixToken::Operator(Operator::Div)),
    								Operator::Mul => value.push(PostfixToken::Operator(Operator::Mul)),
    								Operator::Add => {
    									match x {
    										Operator::Add => value.push(PostfixToken::Operator(Operator::Add)),
    										Operator::Sub => value.push(PostfixToken::Operator(Operator::Add)),
    										_ => {stack.push(a.unwrap());break;},
    									}
    								},
    								Operator::Sub => {
    									match x {
    										Operator::Add => value.push(PostfixToken::Operator(Operator::Sub)),
    										Operator::Sub => value.push(PostfixToken::Operator(Operator::Sub)),
    										_ => {stack.push(a.unwrap());break;},
    									}
    								},
    							}
    						}

    						None => break,
    						_ => {
    							stack.push(a.unwrap());
    							break;
    						},
    					};
    				}
    				stack.push(InfixToken::Operator(x));
    			};
    		},
    	}
    }






    match tokens[tokens.len()-1] {
    	InfixToken::Operand(x) => {
    		value.push(PostfixToken::Operand(x));
    	},
    	
    	InfixToken::RightParen => {
   			num2 += 1;
   			loop {
   				let a = stack.pop();
   				match a {
   					Some(InfixToken::Operator(x)) => value.push(PostfixToken::Operator(x)),
   					Some(InfixToken::LeftParen) => break,
   					None => return None,
   					_ => return None,
   				}
   			}
   		},

   		_ => {},
   	}

    loop {
    	let a = stack.pop();
    	match a {
    		Some(InfixToken::Operator(x)) => value.push(PostfixToken::Operator(x)),
    		None => break,
    		_ => {},
    	}
    }
    if num1 != num2 {
    	return None;
    }
    return Some(value);
    
}

//fn main() {
//	let x = infix_to_postfix(&[InfixToken::Operand(1),InfixToken::Operator(Operator::Add),InfixToken::Operand(2),InfixToken::Operator(Operator::Mul),InfixToken::Operand(3),InfixToken::Operator(Operator::Add),InfixToken::Operand(4)]);
//	println!("{:?}", x);
//}