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
            item.0.to_uppercase()
        );
        println!(
            "40 SLOTS (one car)          ---- ({} stacks)",
            (item.1 * 40.0).round() as u32
        );
        println!(
            "80 SLOTS (two cars)         ---- {} stacks",
            (item.1 * 80.0).round() as u32
        );
        println!(
            "48 SLOTS (1 steel chest)    ---- {} stacks",
            (item.1 * 48.0).round() as u32
        );
        println!(
            "288 SLOTS (6 steel chests)  ---- {} stacks",
            (item.1 * 288.0).round() as u32
        );
        println!(
            "624 SLOTS (13 steel chests) ---- {} stacks",
            (item.1 * 624.0).round() as u32
        );

        println!("======== ITEMS =========");
        println!(
            "40 SLOTS (one car)          ---- {} items",
            (item.1 * 40.0).round() as u32 * item.2
        );
        println!(
            "80 SLOTS (two cars)         ---- ({} items)",
            (item.1 * 80.0).round() as u32 * item.2
        );
        println!(
            "48 SLOTS (1 steel chest)    ---- {} items",
            (item.1 * 48.0).round() as u32 * item.2
        );
        println!(
            "288 SLOTS (6 steel chests)  ---- {} items",
            (item.1 * 288.0).round() as u32 * item.2
        );
        println!(
            "624 SLOTS (13 steel chests) ---- ({} items)",
            (item.1 * 624.0).round() as u32 * item.2
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
        let item_name = prompt(rl, format!("Item {} name > ", n).as_str()).to_lowercase();
        let item_amount = prompt_u32(rl, format!("Item {} amount > ", n).as_str());
        let item_stack_size = match item_name.as_str() {
            // Intermediate products
            "wood" => 100,
            "coal" => 50,
            "stone" => 50,
            "iron ore" => 50,
            "copper ore" => 50,
            "uranium ore" => 50,
            "raw fish" => 100,
            "iron plate" => 100,
            "copper plate" => 100,
            "solid fuel" => 50,
            "steel plate" => 100,
            "plastic bar" => 100,
            "sulfur" => 50,
            "battery" => 200,
            "explosives" => 50,
            "crude oil barrel" => 10,
            "heavy oil barrel" => 10,
            "light oil barrel" => 10,
            "lubricant barrel" => 10,
            "petroleum gas barrel" => 10,
            "sulfuric acid barrel" => 10,
            "water barrel" => 10,
            "copper cable" => 200,
            "iron stick" => 100,
            "iron gear wheel" => 100,
            "empty barrel" => 10,
            "electronic circuit" => 200,
            "advanced circuit" => 200,
            "processing unit" => 100,
            "engine unit" => 50,
            "electric engine unit" => 50,
            "flying robot frame" => 50,
            "rocket control unit" => 10,
            "low density structure" => 10,
            "rocket fuel" => 10,
            "nuclear fuel" => 10,
            "uranium-235" => 100,
            "uranium-238" => 100,
            "uranium fuel cell" => 50,
            "used-up uranium fuel cell" => 50,
            "automation science pack" => 200,
            "logistic science pack" => 200,
            "military science pack" => 200,
            "chemical science pack" => 200,
            "production science pack" => 200,
            "utility science pack" => 200,
            "space science pack" => 2000,
            // Colloquialisms
            "gear" => 100,
            "green circuit" => 200,
            "red circuit" => 200,
            "blue circuit" => 100,
            "red science" => 200,
            "green science" => 200,
            "black science" => 200,
            "blue science" => 200,
            "purple science" => 200,
            "yellow science" => 200,
            "white science" => 2000,
            _ => prompt_u32(rl, format!("Item {} stack size > ", n).as_str()),
        };
        items.push((item_name, item_amount, item_stack_size));
    }
    items
}
