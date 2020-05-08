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

#[derive(Clone)]
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

enum VarType<'a> {
	Natural,
	Integer,
	Float,
	Character,
	Boolean,
	Str,
	List(&'a VarType<'a>)
}

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

enum Command<'a, T> {
	Add(&'a mut Variable, &'a Variable, &'a Variable),
	Sub(&'a mut Variable, Number, Number),
	Mul(&'a mut Variable, Number, Number),
	Div(&'a mut Variable, Number, Number),
	Mod(&'a mut Variable, Number, Number),
	Round(&'a mut Variable, f32),
	Floor(&'a mut Variable, f32),
	Ceil(&'a mut Variable, f32),
	And(&'a mut Variable, T, T),
	Or(&'a mut Variable, T, T),
	Xor(&'a mut Variable, T, T),
	Not(&'a mut Variable, T),
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
	ToNat(&'a mut Variable, String),
	ToInt(&'a mut Variable, String),
	ToFloat(&'a mut Variable, String),
	ToStr(&'a mut Variable, &'a mut Variable),
	Slice(&'a mut Variable, Vec<Variable>, u32, u32),
	Index(&'a mut Variable, Vec<Variable>, u32),
	Len(&'a mut Variable, Vec<Variable>)
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
	println!("{}", 200i32 as u32);
}
