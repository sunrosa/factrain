fn main() {
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    println!("{}", prompt_float(&mut rl));
}

fn prompt(rl: &mut rustyline::Editor<(), rustyline::history::FileHistory>) -> String {
    let readline = rl.readline("> ");
    match readline {
        Ok(line) => line,
        Err(_) => {
            println!("No input");
            "".to_owned()
        }
    }
}

fn prompt_float(rl: &mut rustyline::Editor<(), rustyline::history::FileHistory>) -> f32 {
    loop {
        let input_string = prompt(rl);
        match input_string.parse::<f32>() {
            Ok(t) => {
                break t;
            }
            Err(..) => {
                println!("Error parsing input as 32-bit floating point integer.");
                continue;
            }
        }
    }
}
