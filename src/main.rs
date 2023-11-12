fn main() {
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let items = prompt_items(&mut rl);

    // Debug contents of items Vec in debug mode.
    #[cfg(debug_assertions)]
    {
        println!("{:?}", items);
    }
}

/// Prompt through STDIN for user string input using rustyline `rl` with prompt `p`.
///
/// # Arguments
/// * `rl` - Rustyline [Editor](rustyline::Editor) used to read user's input.
/// * `p` - User prompt given to the user to signal the need for input.
fn prompt(rl: &mut rustyline::Editor<(), rustyline::history::FileHistory>, p: &str) -> String {
    let readline = rl.readline(p);
    match readline {
        Ok(line) => line,
        Err(..) => {
            println!("Input error.");
            "".to_owned()
        }
    }
}

/// Prompt through STDIN for user integer input using rustyline `rl` with prompt `p`. This function will repeat the prompt until a valid integer is provided.
///
/// # Arguments
/// * `rl` - Rustyline [Editor](rustyline::Editor) used to read user's input.
/// * `p` - User prompt given to the user to signal the need for input.
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

/// Prompt through STDIN for user integer input using rustyline `rl` with prompt `p`. This function will ask the user for the number of items they wish to list. Then, a looping prompt will ask them for the item name, item amount, and item stack size.
///
/// # Arguments
/// * `rl` - Rustyline [Editor](rustyline::Editor) used to read user's input.
///
/// # Returns
/// A [Vec] containing tuples containing:
/// * __0__: The item's name. It does not have to match the item's name inside of Factorio. It is purely for reference.
/// * __1__: The amount of the item used on the factory floor used in an arbitrary amount of time. This amount of time must be equal for all items referenced.
/// * __2__: The item's stack size.
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
