fn run_line(line: String) -> Option<usize> {
    None
}

fn run_program(program: String) {
    let lines : Vec<&str> = program.split_terminator('\n').collect();
    let mut current_line = 0;
    while current_line < lines.len() {
        if let Some(line) = run_line(lines.get(current_line).unwrap().to_string()) {
            current_line = line;
        }
        current_line += 1;
    }
}

fn main() {
    println!("Hello, world!");
}
