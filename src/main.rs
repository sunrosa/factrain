fn main() {
    // Read environment variables.
    let env_args: Vec<String> = std::env::args().collect();

    #[cfg(debug_assertions)]
    {
        println!("{:?}", env_args);
    }

    // Print help information if requested.
    if env_args.contains(&"-h".to_owned()) {
        println!("-f    Print full output.\n-h    Print help.");
        return;
    }

    // Initialize Rustyline Editor in order to read user input lines.
    let mut rl = rustyline::DefaultEditor::new().unwrap();

    // Prompt user for items to be calculated.
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

    // Print calculated output.
    for item in item_ratios {
        println!(
            "================ {} ({:.2}%) ================\n======== STACKS ========",
            item.0.to_uppercase(),
            item.1 * 100.0
        );

        println!(
            "40 SLOTS (1 car)             ---- {} stacks",
            (item.1 * 40.0).round() as u32
        );

        if env_args.contains(&"-f".to_owned()) {
            println!(
                "80 SLOTS (2 cars)            ---- {} stacks",
                (item.1 * 80.0).round() as u32
            );

            println!(
                "48 SLOTS (1 steel chest)     ---- {} stacks",
                (item.1 * 48.0).round() as u32
            );

            println!(
                "288 SLOTS (6 steel chests)   ---- {} stacks",
                (item.1 * 288.0).round() as u32
            );

            println!(
                "624 SLOTS (13 steel chests)  ---- {} stacks",
                (item.1 * 624.0).round() as u32
            );

            println!(
                "1296 SLOTS (27 steel chests) ---- {} stacks",
                (item.1 * 1296.0).round() as u32
            );
        }

        println!("======== ITEMS =========");
        if env_args.contains(&"-f".to_owned()) {
            println!(
                "40 SLOTS (1 car)             ---- {} items",
                (item.1 * 40.0).round() as u32 * item.2
            );

            // "SC" refers to "same car" filtering. This requires each train car to follow the same filter.
            println!(
                "80 SLOTS (SC 2 cars)         ---- {} items",
                (item.1 * 40.0).round() as u32 * item.2 * 2
            );

            // "DC" refers to "different car" filtering. This allows each train car to follow their own unique filter.
            println!(
                "80 SLOTS (DC 2 cars)         ---- {} items",
                (item.1 * 80.0).round() as u32 * item.2
            );

            println!(
                "120 SLOTS (SC 3 cars)        ---- {} items",
                (item.1 * 40.0).round() as u32 * item.2 * 3
            );

            println!(
                "120 SLOTS (DC 3 cars)        ---- {} items",
                (item.1 * 120.0).round() as u32 * item.2
            );
        }

        println!(
            "160 SLOTS (SC 4 cars)        ---- {} items",
            (item.1 * 40.0).round() as u32 * item.2 * 4
        );

        if env_args.contains(&"-f".to_owned()) {
            println!(
                "160 SLOTS (DC 4 cars)        ---- {} items",
                (item.1 * 160.0).round() as u32 * item.2
            );

            println!(
                "48 SLOTS (1 steel chest)     ---- {} items",
                (item.1 * 48.0).round() as u32 * item.2
            );

            println!(
                "288 SLOTS (6 steel chests)   ---- {} items",
                (item.1 * 288.0).round() as u32 * item.2
            );

            println!(
                "624 SLOTS (13 steel chests)  ---- {} items",
                (item.1 * 624.0).round() as u32 * item.2
            );
        }

        println!(
            "1296 SLOTS (27 steel chests) ---- {} items",
            (item.1 * 1296.0).round() as u32 * item.2
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
        let item_stack_size = match fetch_item_stack_size(item_name.as_str()) {
            Some(stack_size) => stack_size,
            None => prompt_u32(rl, format!("Item {} stack size > ", n).as_str()),
        };
        items.push((item_name, item_amount, item_stack_size));
    }
    items
}

fn fetch_item_stack_size(item_name: &str) -> Option<u32> {
    match item_name {
        // Logistics
        "stone brick" => Some(100),
        "concrete" => Some(100),
        "hazard concrete" => Some(100),
        "refined concrete" => Some(100),
        "refined hazard concrete" => Some(100),
        "landfill" => Some(100),
        // Production
        "repair pack" => Some(100),
        "speed module" => Some(50),
        "speed module 2" => Some(50),
        "speed module 3" => Some(50),
        "efficiency module" => Some(50),
        "efficiency module 2" => Some(50),
        "efficiency module 3" => Some(50),
        "productivity module" => Some(50),
        "productivity module 2" => Some(50),
        "productivity module 3" => Some(50),
        "satellite" => Some(1),
        // Intermediate products
        "wood" => Some(100),
        "coal" => Some(50),
        "stone" => Some(50),
        "iron ore" => Some(50),
        "copper ore" => Some(50),
        "uranium ore" => Some(50),
        "raw fish" => Some(100),
        "iron plate" => Some(100),
        "copper plate" => Some(100),
        "solid fuel" => Some(50),
        "steel plate" => Some(100),
        "plastic bar" => Some(100),
        "sulfur" => Some(50),
        "battery" => Some(200),
        "explosives" => Some(50),
        "crude oil barrel" => Some(10),
        "heavy oil barrel" => Some(10),
        "light oil barrel" => Some(10),
        "lubricant barrel" => Some(10),
        "petroleum gas barrel" => Some(10),
        "sulfuric acid barrel" => Some(10),
        "water barrel" => Some(10),
        "copper cable" => Some(200),
        "iron stick" => Some(100),
        "iron gear wheel" => Some(100),
        "empty barrel" => Some(10),
        "electronic circuit" => Some(200),
        "advanced circuit" => Some(200),
        "processing unit" => Some(100),
        "engine unit" => Some(50),
        "electric engine unit" => Some(50),
        "flying robot frame" => Some(50),
        "rocket control unit" => Some(10),
        "low density structure" => Some(10),
        "rocket fuel" => Some(10),
        "nuclear fuel" => Some(10),
        "uranium-235" => Some(100),
        "uranium-238" => Some(100),
        "uranium fuel cell" => Some(50),
        "used-up uranium fuel cell" => Some(50),
        "automation science pack" => Some(200),
        "logistic science pack" => Some(200),
        "military science pack" => Some(200),
        "chemical science pack" => Some(200),
        "production science pack" => Some(200),
        "utility science pack" => Some(200),
        "space science pack" => Some(2000),
        // Combat
        "firearm magazine" => Some(200),
        "piercing rounds magazine" => Some(200),
        "uranium rounds magazine" => Some(200),
        "shotgun shells" => Some(200),
        "piercing shotgun shells" => Some(200),
        "cannon shell" => Some(200),
        "explosive cannon shell" => Some(200),
        "uranium cannon shell" => Some(200),
        "explosive uranium cannon shell" => Some(200),
        "artillery shell" => Some(1),
        "rocket" => Some(200),
        "explosive rocket" => Some(200),
        "atomic bomb" => Some(10),
        "flamethrower ammo" => Some(100),
        // Colloquialisms
        "fish" => Some(100),
        "steel" => Some(100),
        "plastic" => Some(100),
        "gear" => Some(100),
        "green circuit" => Some(200),
        "red circuit" => Some(200),
        "blue circuit" => Some(100),
        "engine" => Some(50),
        "electric engine" => Some(50),
        "robot frame" => Some(50),
        "rcu" => Some(10),
        "lds" => Some(10),
        "red science" => Some(200),
        "green science" => Some(200),
        "black science" => Some(200),
        "blue science" => Some(200),
        "purple science" => Some(200),
        "yellow science" => Some(200),
        "white science" => Some(2000),
        _ => None,
    }
}
