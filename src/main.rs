fn main() {
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let items = prompt_items(&mut rl);
    println!("{:?}", items);
}

fn prompt(rl: &mut rustyline::Editor<(), rustyline::history::FileHistory>, p: &str) -> String {
    let readline = rl.readline(p);
    match readline {
        Ok(line) => line,
        Err(_) => {
            println!("No input");
            "".to_owned()
        }
    }
}

fn prompt_i32(rl: &mut rustyline::Editor<(), rustyline::history::FileHistory>, p: &str) -> i32 {
    loop {
        let input_string = prompt(rl, p);
        match input_string.parse::<i32>() {
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

fn prompt_items(
    rl: &mut rustyline::Editor<(), rustyline::history::FileHistory>,
) -> Vec<(String, i32, i32)> {
    let item_count = prompt_i32(rl, "Item count > ");
    let mut items: Vec<(String, i32, i32)> = Vec::new();
    for n in 1..(item_count + 1) {
        let item_name = prompt(rl, format!("Item {} name > ", n).as_str());
        let item_amount = prompt_i32(rl, format!("Item {} amount > ", n).as_str());
        let item_stack_size = prompt_i32(rl, format!("Item {} stack size > ", n).as_str());
        items.push((item_name, item_amount, item_stack_size));
    }
    items
}
