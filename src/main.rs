fn main() {
    // Initialize Rustyline editor.
    let mut rl = rustyline::DefaultEditor::new().unwrap();

    // Prompt user for items.
    let items = prompt_items(&mut rl);

    #[cfg(debug_assertions)]
    {
        println!("DEBUG main()[items]: {:?}", items);
    }

    // Total item stacks for averaging against. This number is what all items are divided by in order to get a unit rate.
    let mut total_stacks = 0.0;
    for item in &items {
        total_stacks += item.1 as f64 / item.2 as f64;
    }

    #[cfg(debug_assertions)]
    {
        println!("DEBUG main()[total_stacks]: {:?}", total_stacks);
    }

    // Items ratios as floats between 0 and 1. This number contains the "percentage" of the item desired.
    // * 0: The name of the item.
    // * 1: The ratio of the item.
    // * 2: The stack size of the item.
    let mut item_ratios: Vec<(String, f64, u32)> = Vec::new();
    for item in &items {
        item_ratios.push((
            item.0.clone(),
            (item.1 as f64 / item.2 as f64) / total_stacks,
            item.2,
        ));
    }

    #[cfg(debug_assertions)]
    {
        println!("DEBUG main()[item_ratios]: {:?}", item_ratios);
    }

    for item in item_ratios {
        println!(
            "================ {} ================\n======== STACKS ========",
            item.0
        );
        println!(
            "40 SLOTS (one car)          ---- {:.4} stacks",
            item.1 * 40.0
        );
        println!(
            "80 SLOTS (two cars)         ---- {:.4} stacks",
            item.1 * 80.0
        );
        println!(
            "288 SLOTS (6 steel chests)  ---- {:.4} stacks",
            item.1 * 288.0
        );
        println!(
            "624 SLOTS (13 steel chests) ---- {:.4} stacks",
            item.1 * 624.0
        );

        println!("======== ITEMS =========");
        println!(
            "40 SLOTS (one car)          ---- {:.4} items",
            item.1 * 40.0 * item.2 as f64
        );
        println!(
            "80 SLOTS (two cars)         ---- {:.4} items",
            item.1 * 80.0 * item.2 as f64
        );
        println!(
            "288 SLOTS (6 steel chests)  ---- {:.4} items",
            item.1 * 288.0 * item.2 as f64
        );
        println!(
            "624 SLOTS (13 steel chests) ---- {:.4} items",
            item.1 * 624.0 * item.2 as f64
        );
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
fn prompt_u32(rl: &mut rustyline::Editor<(), rustyline::history::FileHistory>, p: &str) -> u32 {
    loop {
        let input_string = prompt(rl, p);
        match input_string.parse::<u32>() {
            Ok(t) => {
                break t;
            }
            Err(..) => {
                println!("Error parsing input as 32-bit unsigned integer.");
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
) -> Vec<(String, u32, u32)> {
    // Ask the user for the number of items they wish to be prompted for.
    let item_count = loop {
        let count = prompt_u32(rl, "Item count > ");
        if count == 0 {
            println!("Item count must not be zero.");
            continue;
        }
        break count;
    };

    // Prompt for all of the items.
    let mut items: Vec<(String, u32, u32)> = Vec::new();
    for n in 1..(item_count + 1) {
        let item_name = prompt(rl, format!("Item {} name > ", n).as_str());
        let item_amount = prompt_u32(rl, format!("Item {} amount > ", n).as_str());
        let item_stack_size = prompt_u32(rl, format!("Item {} stack size > ", n).as_str());
        items.push((item_name, item_amount, item_stack_size));
    }
    items
}
