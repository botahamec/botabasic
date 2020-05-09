use std::collections::HashMap;
use std::fmt::Display;
use std::convert::TryInto;

struct UnparsedCommand {
	command_name : String,
	parameters : Vec<String>
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
enum VarType<'a> {
	Natural,
	Integer,
	Float,
	Character,
	Boolean,
	Str,
	List(&'a VarType<'a>)
}

#[derive(Clone)]
struct Label(usize);

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
	Add(&'a mut Variable, &'a Variable, &'a Variable),
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
	Decl(String, VarType<'a>),
	Set(&'a mut Variable, &'a mut Variable),
	Free(String),
	Label(String),
	Jmp(Label),
	Jeq(Label, &'a Variable, &'a Variable),
	Jgt(Label, &'a Variable, &'a Variable),
	Jlt(Label, &'a Variable, &'a Variable),
	Jne(Label, &'a Variable, &'a Variable),
	Print(String),
	Input(&'a mut Variable),
	Convert(&'a mut Variable, &'a Variable),
	Slice(&'a mut Variable, Vec<Variable>, u32, u32),
	Index(&'a mut Variable, Vec<Variable>, u32),
	Len(&'a mut Variable, Vec<Variable>)
}

enum CommandResponse<'a> {
	Declare(String, VarType<'a>),
	Label(String),
	Free(String),
	Jump(Label),
	Nothing
}

impl<'a> Command<'a> {

	pub fn run(&mut self) -> CommandResponse {
		match self {
			Command::Add(ref mut l, ref o1, ref o2) => Self::add(l, o1, o2),
			Command::Sub(ref mut l, ref o1, ref o2) => Self::sub(l, o1, o2),
			Command::Mul(ref mut l, ref o1, ref o2) => Self::mul(l, o1, o2),
			Command::Div(ref mut l, ref o1, ref o2) => Self::div(l, o1, o2),
			Command::Mod(ref mut l, ref o1, ref o2) => Self::modulo(l, o1, o2),
			Command::Round(ref mut l, o1) => Self::round(l, *o1),
			Command::Floor(ref mut l, o1) => Self::floor(l, *o1),
			Command::Ceil(ref mut l, o1) => Self::ceil(l, *o1),
			Command::And(ref mut l, o1, o2) => Self::and(l, *o1, *o2),
			Command::Or(ref mut l, o1, o2) => Self::or(l, *o1, *o2),
			Command::Xor(ref mut l, o1, o2) => Self::xor(l, *o1, *o2),
			Command::Not(ref mut l, o1) => Self::not(l, *o1),
			Command::Decl(name, var_type) => return Self::decl((**name).to_string(), var_type.clone()),
			Command::Set(ref mut l, literal) => Self::set(l, literal),
			Command::Free(var_name) => return Self::free((**var_name).to_string()),
			Command::Label(name) => return Self::label((**name).to_string()),
			Command::Jmp(label) => return Self::jmp(label.clone()),
			Command::Jeq(label, o1, o2) => return Self::jeq(label.clone(), o1, o2),
			Command::Jgt(label, o1, o2) => return Self::jgt(label.clone(), o1, o2),
			Command::Jlt(label, o1, o2) => return Self::jlt(label.clone(), o1, o2),
			Command::Jne(label, o1, o2) => return Self::jne(label.clone(), o1, o2),
			Command::Print(text) => Self::print((**text).to_string()),
			Command::Input(ref mut location) => Self::input(location),
			Command::Convert(ref mut location, variable) => Self::convert(location, variable),
			Command::Slice(ref mut location, list, start, end) => Self::slice(location, list.to_vec(), *start, *end),
			Command::Index(ref mut location, list, index) => Self::index(location, list.to_vec(), *index),
			Command::Len(ref mut location, list) => Self::len(location, list.to_vec())
		};
		CommandResponse::Nothing
	}

	fn add(location: &mut Variable, op1: &Variable, op2: &Variable) {
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

	fn sub(location: &mut Variable, op1: &Number, op2: &Number) {
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

	fn mul(location: &mut Variable, op1: &Number, op2: &Number) {
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

	fn div(location: &mut Variable, op1: &Number, op2: &Number) {
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

	fn modulo(location: &mut Variable, op1: &Number, op2: &Number) {
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

	fn set(location: &mut Variable, literal: &Variable) {
		if let Variable::Bool(ref mut b) = location {
			if let Variable::Bool(nb) = literal {
				*b = *nb;
			} else {
				panic!();
			}
		} else if let Variable::Char(ref mut c) = location {
			if let Variable::Char(nc) = literal {
				*c = *nc;
			} else {
				panic!();
			}
		} else if let Variable::Float(ref mut f) = location {
			if let Variable::Float(nf) = literal {
				*f = *nf;
			} else {
				panic!();
			}
		} else if let Variable::Int(ref mut i) = location {
			if let Variable::Int(ni) = literal {
				*i = *ni;
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
				*n = *nn;
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

	fn free(location: String) -> CommandResponse<'a> {
		CommandResponse::Free(location.clone())
	}

	fn label(name: String) -> CommandResponse<'a> {
		CommandResponse::Label(name.clone())
	}

	fn jmp(label: Label) -> CommandResponse<'a> {
		CommandResponse::Jump(label)
	}

	fn jeq(label: Label, o1: &Variable, o2: &Variable) -> CommandResponse<'a> {
		if o1 == o2 {
			CommandResponse::Jump(label)
		} else {
			CommandResponse::Nothing
		}
	}

	fn jgt(label: Label, o1: &Variable, o2: &Variable) -> CommandResponse<'a> {
		if o1 > o2 {
			CommandResponse::Jump(label)
		} else {
			CommandResponse::Nothing
		}
	}

	fn jlt(label: Label, o1: &Variable, o2: &Variable) -> CommandResponse<'a> {
		if o1 < o2 {
			CommandResponse::Jump(label)
		} else {
			CommandResponse::Nothing
		}
	}

	fn jne(label: Label, o1: &Variable, o2: &Variable) -> CommandResponse<'a> {
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
		Self::set(location, &value);
	}

	fn len(location: &mut Variable, list: Vec<Variable>) {
		match location {
			Variable::Float(ref mut f) => *f = list.len() as f32,
			Variable::Int(ref mut i) => *i = list.len() as i32,
			Variable::Natural(ref mut n) => *n = list.len() as u32,
			_ => panic!()
		}
	}
}

struct Program {
	file: String,
	vars: HashMap<String, Variable>,
	labels: HashMap<String, Label>,
	current_line: usize
}

impl Program {

	fn get_mut_var(&mut self, name: String) -> &mut Variable {
		self.vars.get_mut(&name).unwrap()
	}

	fn get_var(&self, name: String) -> &Variable {
		self.vars.get(&name).unwrap()
	}

	fn get_num_var(&self, name: String) -> Number {
		Number::from_var(self.get_var(name).clone())
	}

	fn get_bool_var(&self, name: String) -> bool {
		let var = self.get_var(name);
		if let Variable::Bool(b) = var {
			*b
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

	fn run_line(&mut self, line: String) {
	}

	pub fn run_program(&mut self, program: String) {
		let lines : Vec<&str> = program.split_terminator('\n').collect();
		self.current_line = 0;
		while self.current_line < lines.len() {
			self.run_line((**lines.get(self.current_line).unwrap()).to_string());
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
	map.insert("TONAT".to_string(), 2);
	map.insert("TOINT".to_string(), 2);
	map.insert("TOFLOAT".to_string(), 2);
	map.insert("TOSTR".to_string(), 2);
	map.insert("CONCAT".to_string(), 3);
	map.insert("SLICE".to_string(), 4);
	map.insert("LEN".to_string(), 2);
	map
}

fn main() {
	println!("{}", 200i32 as f32);
}
