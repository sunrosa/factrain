mod data;

use std::{fmt::Display, str::FromStr};

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
        total_stacks += item.amount / item.stack_size as f64;
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
            ratio: (item.amount / item.stack_size as f64) / total_stacks,
        });
    }

    #[cfg(debug_assertions)]
    {
        println!("DEBUG main()[item_ratios]: {:?}", item_ratios);
    }

    // Print calculated output.
    for item in item_ratios {
        println!(
            "================ {} x{}/{} ({:.2}%) ================\n======== STACKS ========",
            item.ingredient.name.to_uppercase(),
            item.ingredient.amount,
            item.ingredient.stack_size,
            item.ratio * 100.0
        );

        println!(
            "40 SLOTS (1 car)             ---- {} {} stacks",
            (item.ratio * 40.0).round() as u32,
            item.ingredient.name
        );

        if env_args.contains(&"-f".to_owned()) {
            println!(
                "80 SLOTS (2 cars)            ---- {} {} stacks",
                (item.ratio * 80.0).round() as u32,
                item.ingredient.name
            );

            println!(
                "48 SLOTS (1 steel chest)     ---- {} {} stacks",
                (item.ratio * 48.0).round() as u32,
                item.ingredient.name
            );

            println!(
                "288 SLOTS (6 steel chests)   ---- {} {} stacks",
                (item.ratio * 288.0).round() as u32,
                item.ingredient.name
            );

            println!(
                "624 SLOTS (13 steel chests)  ---- {} {} stacks",
                (item.ratio * 624.0).round() as u32,
                item.ingredient.name
            );

            println!(
                "1296 SLOTS (27 steel chests) ---- {} {} stacks",
                (item.ratio * 1296.0).round() as u32,
                item.ingredient.name
            );
        }

        println!("======== ITEMS =========");
        if env_args.contains(&"-f".to_owned()) {
            println!(
                "40 SLOTS (1 car)             ---- {} {} items",
                (item.ratio * 40.0).round() as u32 * item.ingredient.stack_size,
                item.ingredient.name
            );

            // "SC" refers to "same car" filtering. This requires each train car to follow the same filter.
            println!(
                "80 SLOTS (SC 2 cars)         ---- {} {} items",
                (item.ratio * 40.0).round() as u32 * item.ingredient.stack_size * 2,
                item.ingredient.name
            );

            // "DC" refers to "different car" filtering. This allows each train car to follow their own unique filter.
            println!(
                "80 SLOTS (DC 2 cars)         ---- {} {} items",
                (item.ratio * 80.0).round() as u32 * item.ingredient.stack_size,
                item.ingredient.name
            );

            println!(
                "120 SLOTS (SC 3 cars)        ---- {} {} items",
                (item.ratio * 40.0).round() as u32 * item.ingredient.stack_size * 3,
                item.ingredient.name
            );

            println!(
                "120 SLOTS (DC 3 cars)        ---- {} {} items",
                (item.ratio * 120.0).round() as u32 * item.ingredient.stack_size,
                item.ingredient.name
            );
        }

        println!(
            "160 SLOTS (SC 4 cars)        ---- {} {} items",
            (item.ratio * 40.0).round() as u32 * item.ingredient.stack_size * 4,
            item.ingredient.name
        );

        if env_args.contains(&"-f".to_owned()) {
            println!(
                "160 SLOTS (DC 4 cars)        ---- {} {} items",
                (item.ratio * 160.0).round() as u32 * item.ingredient.stack_size,
                item.ingredient.name
            );

            println!(
                "48 SLOTS (1 steel chest)     ---- {} {} items",
                (item.ratio * 48.0).round() as u32 * item.ingredient.stack_size,
                item.ingredient.name
            );

            println!(
                "288 SLOTS (6 steel chests)   ---- {} {} items",
                (item.ratio * 288.0).round() as u32 * item.ingredient.stack_size,
                item.ingredient.name
            );

            println!(
                "624 SLOTS (13 steel chests)  ---- {} {} items",
                (item.ratio * 624.0).round() as u32 * item.ingredient.stack_size,
                item.ingredient.name
            );
        }

        println!(
            "1296 SLOTS (27 steel chests) ---- {} {} items",
            (item.ratio * 1296.0).round() as u32 * item.ingredient.stack_size,
            item.ingredient.name
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

/// Prompt and parse through STDIN for user generic input using rustyline `rl` with prompt `p`. This function will repeat the prompt until a valid type is provided.
///
/// # Generic parameters
/// * `T` - The type to parse the user input into. This must implement [FromStr].
///
/// # Arguments
/// * `rl` - Rustyline [Editor](rustyline::Editor) used to read user's input.
/// * `p` - User prompt given to the user to signal the need for input.
fn prompt_and_parse<T: FromStr<Err = impl Display>>(
    rl: &mut rustyline::Editor<(), rustyline::history::FileHistory>,
    p: &str,
) -> T {
    loop {
        let input_string = prompt(rl, p);
        match input_string.parse::<T>() {
            Ok(t) => {
                break t;
            }
            Err(e) => {
                println!("Error parsing input: {}", e);
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
        let count = prompt_and_parse::<u32>(rl, "Item count > ");
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
        let item_amount = prompt_and_parse::<f64>(
            rl,
            format!("{} amount > ", item_name.to_uppercase()).as_str(),
        );
        let item_stack_size = match data::fetch_item_stack_size(item_name.as_str()) {
            Some(stack_size) => stack_size,
            None => prompt_and_parse::<u32>(
                rl,
                format!("{} stack size > ", item_name.to_uppercase()).as_str(),
            ),
        };
        items.push(Ingredient {
            name: item_name,
            amount: item_amount,
            stack_size: item_stack_size,
        });
    }
    items
}

/// Ingredient as defined by the user.
#[derive(Debug, Clone)]
struct Ingredient {
    /// The name of the ingredient.
    name: String,
    /// The amount of the ingredient needed.
    amount: f64,
    /// The stack size of the ingredient.
    stack_size: u32,
}

/// The ratio of the needed ingredient as an f64 between 0 and 1.
#[derive(Debug, Clone)]
struct IngredientRatio {
    /// Ingredient information.
    ingredient: Ingredient,
    /// The ratio of the ingredient calculated to be needed. This takes stack size into account. This value is between 0 and 1.
    ratio: f64,
}
