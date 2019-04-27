pub enum Operator {Add, Sub,Mul,}
pub enum Token {Operator(Operator),Operand(isize),}

pub fn eval(tokens:&[Token])->Option<isize>{
	let mut v:Vec<isize> = vec![];
	for i in tokens {
		use Token::*;
		match *i {
			Operand(x) => v.push(x),
			Operator(ref opps) => {
				if v.len() < 2  {
					return None;
				}
				else {
				use Operator::*;
				match *opps {
					Add => {let (b,a) = (v.pop(),v.pop()); 
						v.push(a.unwrap()+b.unwrap());},

					Sub => {let (b,a) = (v.pop(),v.pop()); 
						v.push(a.unwrap()-b.unwrap());},

					Mul => {let (b,a) = (v.pop(),v.pop()); 
						v.push(a.unwrap()*b.unwrap());},
				}
			}},
		}
	}
	if v.len() > 1 {
		return None;
	}
	else {
		return v.pop();
	}
}
