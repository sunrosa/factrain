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
        total_stacks += item.amount as f64 / item.stack_size as f64;
    }

    #[cfg(debug_assertions)]
    {
        println!("DEBUG main()[total_stacks]: {:?}", total_stacks);
    }

    // Items ratios as floats between 0 and 1. This number contains the "percentage" of the item desired.
    // * 0: The name of the item.
    // * 1: The ratio of the item.
    // * 2: The stack size of the item.
    let mut item_ratios: Vec<IngredientRatio> = Vec::new();
    for item in &items {
        item_ratios.push(IngredientRatio {
            ingredient: item.clone(),
            ratio: (item.amount as f64 / item.stack_size as f64) / total_stacks,
        });
    }

    #[cfg(debug_assertions)]
    {
        println!("DEBUG main()[item_ratios]: {:?}", item_ratios);
    }

    // Print calculated output.
    for item in item_ratios {
        println!(
            "================ {} ({:.2}%) ================\n======== STACKS ========",
            item.ingredient.name.to_uppercase(),
            item.ratio * 100.0
        );

        println!(
            "40 SLOTS (1 car)             ---- {} stacks",
            (item.ratio * 40.0).round() as u32
        );

        if env_args.contains(&"-f".to_owned()) {
            println!(
                "80 SLOTS (2 cars)            ---- {} stacks",
                (item.ratio * 80.0).round() as u32
            );

            println!(
                "48 SLOTS (1 steel chest)     ---- {} stacks",
                (item.ratio * 48.0).round() as u32
            );

            println!(
                "288 SLOTS (6 steel chests)   ---- {} stacks",
                (item.ratio * 288.0).round() as u32
            );

            println!(
                "624 SLOTS (13 steel chests)  ---- {} stacks",
                (item.ratio * 624.0).round() as u32
            );

            println!(
                "1296 SLOTS (27 steel chests) ---- {} stacks",
                (item.ratio * 1296.0).round() as u32
            );
        }

        println!("======== ITEMS =========");
        if env_args.contains(&"-f".to_owned()) {
            println!(
                "40 SLOTS (1 car)             ---- {} items",
                (item.ratio * 40.0).round() as u32 * item.ingredient.stack_size
            );

            // "SC" refers to "same car" filtering. This requires each train car to follow the same filter.
            println!(
                "80 SLOTS (SC 2 cars)         ---- {} items",
                (item.ratio * 40.0).round() as u32 * item.ingredient.stack_size * 2
            );

            // "DC" refers to "different car" filtering. This allows each train car to follow their own unique filter.
            println!(
                "80 SLOTS (DC 2 cars)         ---- {} items",
                (item.ratio * 80.0).round() as u32 * item.ingredient.stack_size
            );

            println!(
                "120 SLOTS (SC 3 cars)        ---- {} items",
                (item.ratio * 40.0).round() as u32 * item.ingredient.stack_size * 3
            );

            println!(
                "120 SLOTS (DC 3 cars)        ---- {} items",
                (item.ratio * 120.0).round() as u32 * item.ingredient.stack_size
            );
        }

        println!(
            "160 SLOTS (SC 4 cars)        ---- {} items",
            (item.ratio * 40.0).round() as u32 * item.ingredient.stack_size * 4
        );

        if env_args.contains(&"-f".to_owned()) {
            println!(
                "160 SLOTS (DC 4 cars)        ---- {} items",
                (item.ratio * 160.0).round() as u32 * item.ingredient.stack_size
            );

            println!(
                "48 SLOTS (1 steel chest)     ---- {} items",
                (item.ratio * 48.0).round() as u32 * item.ingredient.stack_size
            );

            println!(
                "288 SLOTS (6 steel chests)   ---- {} items",
                (item.ratio * 288.0).round() as u32 * item.ingredient.stack_size
            );

            println!(
                "624 SLOTS (13 steel chests)  ---- {} items",
                (item.ratio * 624.0).round() as u32 * item.ingredient.stack_size
            );
        }

        println!(
            "1296 SLOTS (27 steel chests) ---- {} items",
            (item.ratio * 1296.0).round() as u32 * item.ingredient.stack_size
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
) -> Vec<Ingredient> {
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
    let mut items: Vec<Ingredient> = Vec::new();
    for n in 1..(item_count + 1) {
        let item_name = prompt(rl, format!("Item {} name > ", n).as_str()).to_lowercase();
        let item_amount = prompt_u32(rl, format!("Item {} amount > ", n).as_str());
        let item_stack_size = match fetch_item_stack_size(item_name.as_str()) {
            Some(stack_size) => stack_size,
            None => prompt_u32(rl, format!("Item {} stack size > ", n).as_str()),
        };
        items.push(Ingredient {
            name: item_name,
            amount: item_amount,
            stack_size: item_stack_size,
        });
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
        "speed module 1" => Some(50),
        "efficiency module 1" => Some(50),
        "productivity module 1" => Some(50),
        "fish" => Some(100),
        "steel" => Some(100),
        "plastic" => Some(100),
        "gear" => Some(100),
        "gear wheel" => Some(100),
        "green circuit" => Some(200),
        "red circuit" => Some(200),
        "blue circuit" => Some(100),
        "engine" => Some(50),
        "electric engine" => Some(50),
        "robot frame" => Some(50),
        "frf" => Some(50),
        "rcu" => Some(10),
        "lds" => Some(10),
        "red science" => Some(200),
        "automation science" => Some(200),
        "green science" => Some(200),
        "logistic science" => Some(200),
        "black science" => Some(200),
        "military science" => Some(200),
        "blue science" => Some(200),
        "chemical science" => Some(200),
        "purple science" => Some(200),
        "production science" => Some(200),
        "yellow science" => Some(200),
        "utility science" => Some(200),
        "white science" => Some(2000),
        "space science" => Some(2000),
        "piercing rounds" => Some(200),
        "uranium rounds" => Some(200),
        _ => None,
    }
}

#[derive(Debug, Clone)]
struct Ingredient {
    name: String,
    amount: u32,
    stack_size: u32,
}

#[derive(Debug, Clone)]
struct IngredientRatio {
    ingredient: Ingredient,
    ratio: f64,
}
