use std::collections::HashMap;
use std::fmt::Display;
use std::convert::TryInto;

struct UnparsedCommand {
	pub command_name : String,
	pub parameters : Vec<String>
}

impl UnparsedCommand {
	fn from_line(line: String, command_parameter_num_map: &HashMap<String, u8>) -> Option<Self> {
		let words : Vec<&str> = line.split_ascii_whitespace().collect();
		if command_parameter_num_map.contains_key(&(**words.get(0).unwrap()).to_string().to_uppercase()) {
			let command_name = (**words.get(0).unwrap()).to_string();
			let param_num = command_parameter_num_map.get(&command_name).unwrap();
			let parameters = Vec::from(&words[1..=(*param_num as usize)]).iter().map(|s| (**s).to_string()).collect();
			Some(UnparsedCommand {command_name, parameters})
		} else {
			None
		}
	}
}

#[derive(Clone, PartialEq, PartialOrd)]
enum Variable {
	Natural(u32),
	Int(i32),
	Float(f32),
	Char(char),
	Bool(bool),
	Str(String),
	List(Vec<Variable>)
}

impl Variable {
	pub fn to_float(&self) -> f32 {
		match self {
			Variable::Natural(n) => *n as f32,
			Variable::Int(i) => *i as f32,
			Variable::Float(f) => *f,
			_ => panic!()
		}
	}
}

impl std::fmt::Debug for Variable {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self)
	}
}

impl Display for Variable {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Variable::Natural(n) => write!(f, "{}", n),
			Variable::Int(i) => write!(f, "{}", i),
			Variable::Float(float) => write!(f, "{}", float),
			Variable::Char(c) => write!(f, "{}", c),
			Variable::Bool(b) => write!(f, "{}", b),
			Variable::Str(s) => write!(f, "{}", s),
			Variable::List(l) => write!(f, "{:?}", l)
		}
	}
}

#[derive(Clone)]
enum VarType {
	Natural,
	Integer,
	Float,
	Character,
	Boolean,
	Str,
	List
}

#[derive(Clone)]
struct Label(usize);

#[derive(Clone)]
enum Number {
	Natural(u32),
	Integer(i32),
	Float(f32)
}

impl Number {
	pub fn from_var(var: Variable) -> Self {
		match var {
			Variable::Natural(n) => Number::Natural(n),
			Variable::Int(i) => Number::Integer(i),
			Variable::Float(f) => Number::Float(f),
			_ => panic!()
		}
	}

	pub fn to_float(&self) -> f32 {
		match self {
			Number::Natural(n) => *n as f32,
			Number::Integer(i) => *i as f32,
			Number::Float(f) => *f
		}
	}
}

enum Command<'a> {
	Add(&'a mut Variable, Variable, Variable),
	Sub(&'a mut Variable, Number, Number),
	Mul(&'a mut Variable, Number, Number),
	Div(&'a mut Variable, Number, Number),
	Mod(&'a mut Variable, Number, Number),
	Round(&'a mut Variable, f32),
	Floor(&'a mut Variable, f32),
	Ceil(&'a mut Variable, f32),
	And(&'a mut Variable, bool, bool),
	Or(&'a mut Variable, bool, bool),
	Xor(&'a mut Variable, bool, bool),
	Not(&'a mut Variable, bool),
	Decl(String, VarType),
	Set(&'a mut Variable, Variable),
	Free(String),
	Label(String),
	Jmp(Label),
	Jeq(Label, Variable, Variable),
	Jgt(Label, Variable, Variable),
	Jlt(Label, Variable, Variable),
	Jne(Label, Variable, Variable),
	Print(String),
	Input(&'a mut Variable),
	Convert(&'a mut Variable, Variable),
	Slice(&'a mut Variable, Vec<Variable>, u32, u32),
	Index(&'a mut Variable, Vec<Variable>, u32),
	Len(&'a mut Variable, Vec<Variable>),
	Insert(&'a mut Vec<Variable>, u32, Variable)
}

enum CommandResponse {
	Declare(String, VarType),
	Label(String),
	Free(String),
	Jump(Label),
	Nothing
}

impl<'a> Command<'a> {

	pub fn run(&mut self) -> CommandResponse {
		match self {
			Command::Add(ref mut l, o1, o2) => Self::add(l, o1.clone(), o2.clone()),
			Command::Sub(ref mut l, o1, o2) => Self::sub(l, o1.clone(), o2.clone()),
			Command::Mul(ref mut l, o1, o2) => Self::mul(l, o1.clone(), o2.clone()),
			Command::Div(ref mut l, o1, o2) => Self::div(l, o1.clone(), o2.clone()),
			Command::Mod(ref mut l, o1, o2) => Self::modulo(l, o1.clone(), o2.clone()),
			Command::Round(ref mut l, o1) => Self::round(l, *o1),
			Command::Floor(ref mut l, o1) => Self::floor(l, *o1),
			Command::Ceil(ref mut l, o1) => Self::ceil(l, *o1),
			Command::And(ref mut l, o1, o2) => Self::and(l, *o1, *o2),
			Command::Or(ref mut l, o1, o2) => Self::or(l, *o1, *o2),
			Command::Xor(ref mut l, o1, o2) => Self::xor(l, *o1, *o2),
			Command::Not(ref mut l, o1) => Self::not(l, *o1),
			Command::Decl(name, var_type) => return Self::decl((**name).to_string(), var_type.clone()),
			Command::Set(ref mut l, literal) => Self::set(l, literal.clone()),
			Command::Free(var_name) => return Self::free((**var_name).to_string()),
			Command::Label(name) => return Self::label((**name).to_string()),
			Command::Jmp(label) => return Self::jmp(label.clone()),
			Command::Jeq(label, o1, o2) => return Self::jeq(label.clone(), o1.clone(), o2.clone()),
			Command::Jgt(label, o1, o2) => return Self::jgt(label.clone(), o1.clone(), o2.clone()),
			Command::Jlt(label, o1, o2) => return Self::jlt(label.clone(), o1.clone(), o2.clone()),
			Command::Jne(label, o1, o2) => return Self::jne(label.clone(), o1.clone(), o2.clone()),
			Command::Print(text) => Self::print((**text).to_string()),
			Command::Input(ref mut location) => Self::input(location),
			Command::Convert(ref mut location, variable) => Self::convert(location, variable),
			Command::Slice(ref mut location, list, start, end) => Self::slice(location, list.to_vec(), *start, *end),
			Command::Index(ref mut location, list, index) => Self::index(location, list.to_vec(), *index),
			Command::Len(ref mut location, list) => Self::len(location, list.to_vec()),
			Command::Insert(ref mut list, index, item) => Self::insert(list, *index, item.clone())
		};
		CommandResponse::Nothing
	}

	fn add(location: &mut Variable, op1: Variable, op2: Variable) {
		if let Variable::List(ref mut location) = location {
			if let Variable::List(ref op1) = op1 {
				if let Variable::List(ref op2) = op2 {
					location.clear();
					location.append(&mut op1.clone());
					location.append(&mut op2.clone());
				} else {
					location.clear();
					location.append(&mut op1.clone());
					location.push(op2.clone());
				}
			} else {
				location.clear();
				location.push(op1.clone());
				location.push(op2.clone());
			}
		} else if let Variable::Str(ref mut string) = location {
			string.clear();
			string.push_str(&op1.to_string());
			string.push_str(&op2.to_string());
		} else if let Variable::Natural(ref mut num) = location {
			*num = (op1.to_float() + op2.to_float()).round().abs() as u32;
		} else if let Variable::Int(ref mut num) = location {
			*num = (op1.to_float() + op2.to_float()).round() as i32;
		} else if let Variable::Float(ref mut num) = location {
			*num = op1.to_float() + op2.to_float();
		} else {
			panic!();
		}
	}

	fn sub(location: &mut Variable, op1: Number, op2: Number) {
		if let Variable::Natural(ref mut n) = location {
			*n = (op1.to_float() + op2.to_float()).round().abs() as u32;
		} else if let Variable::Int(ref mut n) = location {
			*n = (op1.to_float() + op2.to_float()).round() as i32;
		} else if let Variable::Float(ref mut n) = location {
			*n = op1.to_float() + op2.to_float();
		} else {
			panic!();
		}
	}

	fn mul(location: &mut Variable, op1: Number, op2: Number) {
		if let Variable::Natural(ref mut n) = location {
			*n = (op1.to_float() * op2.to_float()).round().abs() as u32;
		} else if let Variable::Int(ref mut n) = location {
			*n = (op1.to_float() * op2.to_float()).round() as i32;
		} else if let Variable::Float(ref mut n) = location {
			*n = op1.to_float() * op2.to_float();
		} else {
			panic!();
		}
	}

	fn div(location: &mut Variable, op1: Number, op2: Number) {
		if let Variable::Natural(ref mut n) = location {
			*n = (op1.to_float() / op2.to_float()).round().abs() as u32;
		} else if let Variable::Int(ref mut n) = location {
			*n = (op1.to_float() / op2.to_float()).round() as i32;
		} else if let Variable::Float(ref mut n) = location {
			*n = op1.to_float() / op2.to_float();
		} else {
			panic!();
		}
	}

	fn modulo(location: &mut Variable, op1: Number, op2: Number) {
		if let Variable::Natural(ref mut n) = location {
			*n = (op1.to_float() % op2.to_float()).round().abs() as u32;
		} else if let Variable::Int(ref mut n) = location {
			*n = (op1.to_float() % op2.to_float()).round() as i32;
		} else if let Variable::Float(ref mut n) = location {
			*n = op1.to_float() % op2.to_float();
		} else {
			panic!();
		}
	}

	fn round(location: &mut Variable, op1: f32) {
		if let Variable::Natural(ref mut n) = location {
			*n = op1.round().abs() as u32;
		} else if let Variable::Int(ref mut n) = location {
			*n = op1.round() as i32;
		} else if let Variable::Float(ref mut n) = location {
			*n = op1.round();
		} else {
			panic!();
		}
	}

	fn floor(location: &mut Variable, op1:f32) {
		if let Variable::Natural(ref mut n) = location {
			*n = op1.floor().abs() as u32;
		} else if let Variable::Int(ref mut n) = location {
			*n = op1.floor() as i32;
		} else if let Variable::Float(ref mut n) = location {
			*n = op1.floor();
		} else {
			panic!();
		}
	}

	fn ceil(location: &mut Variable, op1: f32) {
		if let Variable::Natural(ref mut n) = location {
			*n = op1.ceil() as u32;
		} else if let Variable::Int(ref mut n) = location {
			*n = op1.ceil() as i32;
		} else if let Variable::Float(ref mut n) = location {
			*n = op1.ceil();
		} else {
			panic!();
		}
	}

	fn and(location: &mut Variable, op1: bool, op2: bool) {
		if let Variable::Bool(ref mut b) = location {
			*b = op1 && op2;
		} else {
			panic!()
		}
	}

	fn or(location: &mut Variable, op1: bool, op2: bool) {
		if let Variable::Bool(ref mut b) = location {
			*b = op1 || op2;
		} else {
			panic!()
		}
	}

	fn xor(location: &mut Variable, op1: bool, op2: bool) {
		if let Variable::Bool(ref mut b) = location {
			*b = op1 != op2;
		} else {
			panic!()
		}
	}

	fn not(location: &mut Variable, op1: bool) {
		if let Variable::Bool(ref mut b) = location {
			*b = !op1;
		} else {
			panic!()
		}
	}

	fn decl(var_name: String, var_type: VarType) -> CommandResponse {
		CommandResponse::Declare(var_name, var_type)
	}

	fn set(location: &mut Variable, literal: Variable) {
		if let Variable::Bool(ref mut b) = location {
			if let Variable::Bool(nb) = literal {
				*b = nb;
			} else {
				panic!();
			}
		} else if let Variable::Char(ref mut c) = location {
			if let Variable::Char(nc) = literal {
				*c = nc;
			} else {
				panic!();
			}
		} else if let Variable::Float(ref mut f) = location {
			if let Variable::Float(nf) = literal {
				*f = nf;
			} else {
				panic!();
			}
		} else if let Variable::Int(ref mut i) = location {
			if let Variable::Int(ni) = literal {
				*i = ni;
			} else {
				panic!();
			}
		} else if let Variable::List(ref mut l) = location {
			if let Variable::List(nl) = literal {
				*l = nl.clone();
			} else {
				panic!();
			}
		} else if let Variable::Natural(ref mut n) = location {
			if let Variable::Natural(nn) = literal {
				*n = nn;
			} else {
				panic!();
			}
		} else if let Variable::Str(ref mut s) = location {
			if let Variable::Str(ns) = literal {
				*s = ns.clone();
			} else {
				panic!();
			}
		}
	}

	fn free(location: String) -> CommandResponse {
		CommandResponse::Free(location.clone())
	}

	fn label(name: String) -> CommandResponse {
		CommandResponse::Label(name.clone())
	}

	fn jmp(label: Label) -> CommandResponse {
		CommandResponse::Jump(label)
	}

	fn jeq(label: Label, o1: Variable, o2: Variable) -> CommandResponse {
		if o1 == o2 {
			CommandResponse::Jump(label)
		} else {
			CommandResponse::Nothing
		}
	}

	fn jgt(label: Label, o1: Variable, o2: Variable) -> CommandResponse {
		if o1 > o2 {
			CommandResponse::Jump(label)
		} else {
			CommandResponse::Nothing
		}
	}

	fn jlt(label: Label, o1: Variable, o2: Variable) -> CommandResponse {
		if o1 < o2 {
			CommandResponse::Jump(label)
		} else {
			CommandResponse::Nothing
		}
	}

	fn jne(label: Label, o1: Variable, o2: Variable) -> CommandResponse {
		if o1 != o2 {
			CommandResponse::Jump(label)
		} else {
			CommandResponse::Nothing
		}
	}

	fn print(string: String) {
		print!("{}", string);
	}

	fn input(location: &mut Variable) {
		if let Variable::Str(ref mut s) = location {
			let reader = std::io::stdin();
			s.clear();
			reader.read_line(s);
		}
	}

	// TODO convert to match
	fn convert(location: &mut Variable, variable: &Variable) {
		if let Variable::Bool(ref mut b) = location {
			if let Variable::Bool(b2) = variable {
				*b = *b2;
			} else if let Variable::Char(c) = variable {
				if *c == 'f' || *c == 'F' {
					*b = false;
				} else {
					*b = true;
				}
			} else if let Variable::Float(f) = variable {
				if *f == 0.0 {
					*b = false;
				} else {
					*b = true;
				}
			} else if let Variable::Int(i) = variable {
				if *i == 0 {
					*b = false;
				} else {
					*b = true;
				}
			} else if let Variable::List(l) = variable {
				if l.is_empty() {
					*b = false;
				} else {
					*b = true;
				}
			} else if let Variable::Natural(n) = variable {
				if *n == 0 {
					*b = false;
				} else {
					*b = true;
				}
			} else if let Variable::Str(s) = variable {
				if s.is_empty() {
					*b = false;
				} else {
					*b = true;
				}
			}
		} else if let Variable::Char(ref mut c) = location {
			if let Variable::Bool(b) = variable {
				if *b {
					*c = 't';
				} else {
					*c = 'f';
				}
			} else if let Variable::Char(oc) = variable {
				*c = *oc;
			} else {
				panic!();
			}
		} else if let Variable::Float(ref mut f) = location {
			if let Variable::Bool(b) = variable {
				if *b {
					*f = 1.0;
				} else {
					*f = 0.0;
				}
			} else if let Variable::Float(of2) = variable {
				*f = *of2;
			} else if let Variable::Int(i) = variable {
				*f = *i as f32;
			} else if let Variable::Natural(n) = variable {
				*f = *n as f32;
			} else if let Variable::Str(s) = variable {
				*f = s.parse().unwrap();
			} else {
				panic!()
			}
		} else if let Variable::Int(ref mut i) = location {
			match variable {
				Variable::Bool(b) => *i = if *b {1} else {0},
				Variable::Float(f) => *i = f.round() as i32,
				Variable::Int(i2) => *i = *i2,
				Variable::Natural(n) => *i = *n as i32,
				Variable::Str(s) => *i = s.parse().unwrap(),
				_ => panic!()
			}
		} else if let Variable::List(ref mut l) = location {
			match variable {
				Variable::List(l2) => *l = l2.clone(),
				Variable::Str(s) => {
					l.clear();
					for character in s.chars() {
						l.push(Variable::Char(character))
					}
				},
				_ => *l = vec![variable.clone()]
			}
		} else if let Variable::Natural(ref mut n) = location {
			match variable {
				Variable::Float(f) => *n = f.round().abs() as u32,
				Variable::Int(i) => *n = i.abs() as u32,
				Variable::Natural(n2) => *n = *n2,
				Variable::Bool(b) => *n = if *b {1} else {0},
				Variable::Str(s) => *n = s.parse().unwrap(),
				_ => panic!()
			}
		} else if let Variable::Str(ref mut s) = location {
			*s = format!("{}", variable);
		}
	}

	fn slice(location: &mut Variable, list: Vec<Variable>, start: u32, end: u32) {
		if let Variable::List(ref mut l) = location {
			*l = list[start as usize..end as usize].iter().map(|v| v.clone()).collect();
		} else {panic!()}
	}

	fn index(location: &mut Variable, list: Vec<Variable>, index: u32) {
		let value = list[index as usize].clone();
		Self::set(location, value);
	}

	fn len(location: &mut Variable, list: Vec<Variable>) {
		match location {
			Variable::Float(ref mut f) => *f = list.len() as f32,
			Variable::Int(ref mut i) => *i = list.len() as i32,
			Variable::Natural(ref mut n) => *n = list.len() as u32,
			_ => panic!()
		}
	}

	fn insert(location: &mut Vec<Variable>, index: u32, item: Variable) {
		if index as usize == location.len() {
			location.push(item);
		} else {
			location.insert(index as usize, item);
		}
	}
}

#[derive(Clone)]
struct Program {
	program: String,
	vars: HashMap<String, Variable>,
	labels: HashMap<String, Label>,
	current_line: usize
}

impl Program {

	pub fn new(program: String) -> Self {
		Program {
			program,
			vars: HashMap::new(),
			labels: HashMap::new(),
			current_line: 0
		}
	}

	fn parse_literal(&self, literal: String) -> Variable {
		let literal = literal.trim().to_string();
		if self.vars.contains_key(&literal) {
			self.vars.get(&literal).unwrap().clone()
		} else if literal.starts_with('-') {
			Variable::Int(literal.parse().unwrap())
		} else if literal.starts_with('+') {
			Variable::Int(literal.split_at(1).1.parse().unwrap())
		} else if literal.starts_with('\"') {
			Variable::Str(literal.split_at(1).1.to_string())
		} else if literal == "TRUE" {
			Variable::Bool(true)
		} else if literal == "FALSE" {
			Variable::Bool(false)
		} else if literal.starts_with('[') {
			let items : Vec<String> = literal.split_terminator(',').map(|s| s.to_string()).collect();
			Variable::List(items.iter().map(|i| self.parse_literal(i.to_string())).collect())
		} else if literal.starts_with('\'') {
			Variable::Char(literal.chars().nth(2).unwrap())
		} else {
			Variable::Natural(literal.parse().unwrap())
		}
	}

	fn get_mut_var(&mut self, name: String) -> &mut Variable {
		self.vars.get_mut(&name).unwrap()
	}

	fn get_var(&self, name: String) -> Variable {
		self.vars.get(&name).unwrap().clone()
	}

	fn get_num_var(&self, name: String) -> Number {
		Number::from_var(self.get_var(name).clone())
	}

	fn get_nat_var(&self, name: String) -> u32 {
		self.get_num_var(name).to_float().round().abs() as u32
	}

	fn get_float_var(&self, name: String) -> f32 {
		self.get_num_var(name).to_float()
	}

	fn get_bool_var(&self, name: String) -> bool {
		let var = self.get_var(name);
		if let Variable::Bool(b) = var {
			b
		} else {
			panic!()
		}
	}

	fn get_str_var(&self, name: String) -> String {
		let var = self.get_var(name);
		if let Variable::Str(string) = var {
			string.to_string()
		} else {
			panic!()
		}
	}

	fn get_list_var(&self, name: String) -> Vec<Variable> {
		let var = self.get_var(name);
		if let Variable::List(l) = var {
			l.clone()
		} else {
			panic!()
		}
	}

	fn get_label(&self, name: String) -> Label {
		self.labels.get(&name).unwrap().clone()
	}

	fn run_line(&mut self, line: String, command_parameter_num_map: &HashMap<String, u8>) {
		let command = UnparsedCommand::from_line(line, command_parameter_num_map);
		if let Some(command) = command {
			match match command.command_name.as_str() {
				"ADD" => {
					let var1 = self.get_var(command.parameters[1].clone());
					let var2 = self.get_var(command.parameters[2].clone());
					Command::Add(self.get_mut_var(command.parameters[0].clone()), var1, var2).run()
				},
				"SUB" => {
					let var1 = self.get_num_var(command.parameters[1].clone());
					let var2 = self.get_num_var(command.parameters[2].clone());
					Command::Sub(self.get_mut_var(command.parameters[0].clone()), var1, var2).run()
				},
				"MUL" => {
					let var1 = self.get_num_var(command.parameters[1].clone());
					let var2 = self.get_num_var(command.parameters[2].clone());
					Command::Mul(self.get_mut_var(command.parameters[0].clone()), var1, var2).run()
				},
				"DIV" => {
					let var1 = self.get_num_var(command.parameters[1].clone());
					let var2 = self.get_num_var(command.parameters[2].clone());
					Command::Div(self.get_mut_var(command.parameters[0].clone()), var1, var2).run()
				},
				"MOD" => {
					let var1 = self.get_num_var(command.parameters[1].clone());
					let var2 = self.get_num_var(command.parameters[2].clone());
					Command::Mod(self.get_mut_var(command.parameters[0].clone()), var1, var2).run()
				},
				"ROUND" => {
					let var = self.get_float_var(command.parameters[1].clone());
					Command::Round(self.get_mut_var(command.parameters[0].clone()), var).run()
				},
				"FLOOR" => {
					let var = self.get_float_var(command.parameters[1].clone());
					Command::Floor(self.get_mut_var(command.parameters[0].clone()), var).run()
				},
				"CEIL" => {
					let var = self.get_float_var(command.parameters[1].clone());
					Command::Ceil(self.get_mut_var(command.parameters[0].clone()), var).run()
				},
				"AND" => {
					let var1 = self.get_bool_var(command.parameters[1].clone());
					let var2 = self.get_bool_var(command.parameters[2].clone());
					Command::And(self.get_mut_var(command.parameters[0].clone()), var1, var2).run()
				},
				"OR" => {
					let var1 = self.get_bool_var(command.parameters[1].clone());
					let var2 = self.get_bool_var(command.parameters[2].clone());
					Command::Or(self.get_mut_var(command.parameters[0].clone()), var1, var2).run()
				},
				"XOR" => {
					let var1 = self.get_bool_var(command.parameters[1].clone());
					let var2 = self.get_bool_var(command.parameters[2].clone());
					Command::Xor(self.get_mut_var(command.parameters[0].clone()), var1, var2).run()
				},
				"NOT" => {
					let var = self.get_bool_var(command.parameters[1].clone());
					Command::Not(self.get_mut_var(command.parameters[0].clone()), var).run()
				},
				"DECL" => {
					Command::Label(command.parameters[0].clone()).run()
				},
				"SET" => {
					let literal = self.parse_literal(command.parameters[1].clone());
					Command::Set(self.get_mut_var(command.parameters[0].clone()), literal).run()
				},
				"FREE" => {
					Command::Free(command.parameters[0].clone()).run()
				},
				"LABEL" => Command::Label(command.parameters[0].clone()).run(),
				"JMP" => Command::Jmp(self.get_label(command.parameters[0].clone())).run(),
				"JEQ" => Command::Jeq(self.get_label(command.parameters[0].clone()), self.get_var(command.parameters[1].clone()), self.get_var(command.parameters[2].clone())).run(),
				"JNE" => Command::Jne(self.get_label(command.parameters[0].clone()), self.get_var(command.parameters[1].clone()), self.get_var(command.parameters[2].clone())).run(),
				"JGT" => Command::Jgt(self.get_label(command.parameters[0].clone()), self.get_var(command.parameters[1].clone()), self.get_var(command.parameters[2].clone())).run(),
				"JLT" => Command::Jeq(self.get_label(command.parameters[0].clone()), self.get_var(command.parameters[1].clone()), self.get_var(command.parameters[2].clone())).run(),
				"PRINT" => Command::Print(self.get_str_var(command.parameters[0].clone())).run(),
				"INPUT" => Command::Input(self.get_mut_var(command.parameters[0].clone())).run(),
				"CONVERT" => {
					let var = self.get_var(command.parameters[1].clone());
					Command::Convert(self.get_mut_var(command.parameters[0].clone()), var).run()
				},
				"SLICE" => {
					let list = self.get_list_var(command.parameters[1].clone());
					let start = self.get_nat_var(command.parameters[2].clone());
					let end = self.get_nat_var(command.parameters[3].clone());
					Command::Slice(self.get_mut_var(command.parameters[0].clone()), list, start, end).run()
				},
				"INDEX" => {
					let list = self.get_list_var(command.parameters[1].clone());
					let index = self.get_nat_var(command.parameters[2].clone());
					Command::Index(self.get_mut_var(command.parameters[0].clone()), list, index).run()
				},
				"LEN" => {
					let list = self.get_list_var(command.parameters[1].clone());
					Command::Len(self.get_mut_var(command.parameters[0].clone()), list).run()
				}
				"INSERT" => {
					let index = self.get_nat_var(command.parameters[1].clone());
					let item = self.get_var(command.parameters[2].clone());
					let list: &mut Vec<Variable> = if let Variable::List(ref mut l) = self.get_mut_var(command.parameters[0].clone()) {
						l
					} else {panic!()};
					Command::Insert(list, index, item).run()
				}
				_ => panic!()
			} {
				CommandResponse::Declare(s, t) => {self.vars.insert(s, match t {
						VarType::Boolean => Variable::Bool(false),
						VarType::Character => Variable::Char('\0'),
						VarType::Float => Variable::Float(0.0),
						VarType::Integer => Variable::Int(0),
						VarType::List => Variable::List(vec![]),
						VarType::Natural => Variable::Natural(0),
						VarType::Str => Variable::Str(String::new())
					});},
				CommandResponse::Free(s) => {self.vars.remove(&s).unwrap();},
				CommandResponse::Jump(label) => {self.current_line = label.0;},
				CommandResponse::Label(name) => {self.labels.insert(name, Label(self.current_line));},
				CommandResponse::Nothing => ()
			}
		}
	}

	pub fn run_program(&mut self) {
		let file = self.program.clone();
		let lines : Vec<&str> = file.lines().collect();
		let command_parameter_num_map = command_parameter_num_map();
		self.current_line = 0;
		self.labels = HashMap::new();
		self.vars = HashMap::new();
		while self.current_line < lines.len() {
			self.run_line((**lines.get(self.current_line).unwrap()).to_string(), &command_parameter_num_map);
			self.current_line += 1;
		}
	}
}

fn command_parameter_num_map() -> HashMap<String, u8> {
	let mut map = HashMap::new();
	map.insert("ADD".to_string(), 3);
	map.insert("SUB".to_string(), 3);
	map.insert("MUL".to_string(), 3);
	map.insert("DIV".to_string(), 3);
	map.insert("MOD".to_string(), 3);
	map.insert("ROUND".to_string(), 2);
	map.insert("FLOOR".to_string(), 2);
	map.insert("CEIL".to_string(), 2);
	map.insert("AND".to_string(), 3);
	map.insert("OR".to_string(), 3);
	map.insert("XOR".to_string(), 3);
	map.insert("NOT".to_string(), 2);
	map.insert("DECL".to_string(), 2);
	map.insert("SET".to_string(), 2);
	map.insert("FREE".to_string(), 1);
	map.insert("LABEL".to_string(), 1);
	map.insert("JMP".to_string(), 1);
	map.insert("JEQ".to_string(), 3);
	map.insert("JGT".to_string(), 3);
	map.insert("JLT".to_string(), 3);
	map.insert("JGT".to_string(), 3);
	map.insert("JNE".to_string(), 3);
	map.insert("PRINT".to_string(), 1);
	map.insert("INPUT".to_string(), 1);
	map.insert("CONVERT".to_string(), 2);
	map.insert("SLICE".to_string(), 4);
	map.insert("INDEX".to_string(), 3);
	map.insert("LEN".to_string(), 2);
	map
}

fn main() {
	if let Some(filename) = std::env::args().nth(1) {
		let file = std::fs::read_to_string(filename).unwrap();
		Program::new(file).run_program();
	} else {
		println!("Please give a filename");
	}
}
