mod data;

use std::{fmt::Display, str::FromStr};

fn main() {
    // Read environment variables.
    let env_args: Vec<String> = std::env::args().collect();

    #[cfg(debug_assertions)]
    {
        println!("DEBUG main()[env_args] {:?}", env_args);
    }

    // Print help information if requested.
    if env_args.contains(&"-h".into()) {
        println!("-f    Print full output.\n-h    Print help.");
        return;
    }

    // Initialize Rustyline Editor in order to read user input lines.
    let mut rl_editor = rustyline::DefaultEditor::new().unwrap();

    // Prompt user for items to be calculated.
    let ingredients = prompt_ingredients(&mut rl_editor);

    #[cfg(debug_assertions)]
    {
        println!("DEBUG main()[items]: {:?}", ingredients);
    }

    // Total number of items specified in user-given amounts.
    let mut total_items = 0.0;
    for item in &ingredients {
        total_items += item.amount;
    }

    // Total item stacks specified in user-given amounts.
    let mut total_stacks = 0.0;
    for item in &ingredients {
        total_stacks += item.amount / item.stack_size as f64;
    }

    #[cfg(debug_assertions)]
    {
        println!("DEBUG main()[total_stacks]: {:?}", total_stacks);
    }

    // Calculate ingredient ratios.
    let mut item_ratios: Vec<IngredientRatio> = Vec::new();
    for item in &ingredients {
        item_ratios.push(IngredientRatio {
            ingredient: item.clone(),
            ratio: item.amount / total_items,
            stack_ratio: (item.amount / item.stack_size as f64) / total_stacks,
        });
    }

    #[cfg(debug_assertions)]
    {
        println!("DEBUG main()[item_ratios]: {:?}", item_ratios);
    }

    // Print calculated output.
    for item in item_ratios {
        println!(
            "{:=^80}",
            format!(
                " {} x{}/{} ({:.2}%) ",
                item.ingredient.name.to_uppercase(),
                item.ingredient.amount,
                item.ingredient.stack_size,
                item.stack_ratio * 100.0
            )
        );

        println!("{:=^80}", " STACKS ");
        println!(
            "{:>40} ---- {} {} stacks",
            "40 SLOTS (1 car)",
            (item.stack_ratio * 40.0).round() as u32,
            item.ingredient.name
        );

        if env_args.contains(&"-f".into()) {
            println!(
                "{:>40} ---- {} {} stacks",
                "80 SLOTS (2 cars)",
                (item.stack_ratio * 80.0).round() as u32,
                item.ingredient.name
            );

            println!(
                "{:>40} ---- {} {} stacks",
                "120 SLOTS (3 cars)",
                (item.stack_ratio * 120.0).round() as u32,
                item.ingredient.name
            );

            println!(
                "{:>40} ---- {} {} stacks",
                "160 SLOTS (4 cars)",
                (item.stack_ratio * 160.0).round() as u32,
                item.ingredient.name
            );

            println!(
                "{:>40} ---- {} {} stacks",
                "48 SLOTS (1 steel chest)",
                (item.stack_ratio * 48.0).round() as u32,
                item.ingredient.name
            );

            println!(
                "{:>40} ---- {} {} stacks",
                "288 SLOTS (6 steel chests)",
                (item.stack_ratio * 288.0).round() as u32,
                item.ingredient.name
            );

            println!(
                "{:>40} ---- {} {} stacks",
                "624 SLOTS (13 steel chests)",
                (item.stack_ratio * 624.0).round() as u32,
                item.ingredient.name
            );

            println!(
                "{:>40} ---- {} {} stacks",
                "1296 SLOTS (27 steel chests)",
                (item.stack_ratio * 1296.0).round() as u32,
                item.ingredient.name
            );
        }

        println!("{:=^80}", " ITEMS ");
        if env_args.contains(&"-f".into()) {
            println!(
                "{:>40} ---- {} {} items",
                "40 SLOTS (1 car)",
                (item.stack_ratio * 40.0).round() as u32 * item.ingredient.stack_size,
                item.ingredient.name
            );

            // "SC" refers to "same car" filtering. This requires each train car to follow the same filter.
            println!(
                "{:>40} ---- {} {} items",
                "80 SLOTS (SC 2 cars)",
                (item.stack_ratio * 40.0).round() as u32 * item.ingredient.stack_size * 2,
                item.ingredient.name
            );

            // "DC" refers to "different car" filtering. This allows each train car to follow their own unique filter.
            println!(
                "{:>40} ---- {} {} items",
                "80 SLOTS (DC 2 cars)",
                (item.stack_ratio * 80.0).round() as u32 * item.ingredient.stack_size,
                item.ingredient.name
            );

            println!(
                "{:>40} ---- {} {} items",
                "120 SLOTS (SC 3 cars)",
                (item.stack_ratio * 40.0).round() as u32 * item.ingredient.stack_size * 3,
                item.ingredient.name
            );

            println!(
                "{:>40} ---- {} {} items",
                "120 SLOTS (DC 3 cars)",
                (item.stack_ratio * 120.0).round() as u32 * item.ingredient.stack_size,
                item.ingredient.name
            );
        }

        println!(
            "{:>40} ---- {} {} items",
            "160 SLOTS (SC 4 cars)",
            (item.stack_ratio * 40.0).round() as u32 * item.ingredient.stack_size * 4,
            item.ingredient.name
        );

        if env_args.contains(&"-f".into()) {
            println!(
                "{:>40} ---- {} {} items",
                "160 SLOTS (DC 4 cars)",
                (item.stack_ratio * 160.0).round() as u32 * item.ingredient.stack_size,
                item.ingredient.name
            );

            println!(
                "{:>40} ---- {} {} items",
                "48 SLOTS (1 steel chest)",
                (item.stack_ratio * 48.0).round() as u32 * item.ingredient.stack_size,
                item.ingredient.name
            );

            println!(
                "{:>40} ---- {} {} items",
                "288 SLOTS (6 steel chests)",
                (item.stack_ratio * 288.0).round() as u32 * item.ingredient.stack_size,
                item.ingredient.name
            );

            println!(
                "{:>40} ---- {} {} items",
                "624 SLOTS (13 steel chests)",
                (item.stack_ratio * 624.0).round() as u32 * item.ingredient.stack_size,
                item.ingredient.name
            );
        }

        println!(
            "{:>40} ---- {} {} items",
            "1296 SLOTS (27 steel chests)",
            (item.stack_ratio * 1296.0).round() as u32 * item.ingredient.stack_size,
            item.ingredient.name
        );

        println!("{:=^80}", " INSERTERS ");
        if env_args.contains(&"-f".into()) {
            println!(
                "{:>40} ---- {} {} inserters",
                "6 INSERTERS (6 steel chests)",
                (item.ratio * 6.0).round() as u32,
                item.ingredient.name
            );
            println!(
                "{:>40} ---- {} {} inserters",
                "13 INSERTERS (13 steel chests)",
                (item.ratio * 13.0).round() as u32,
                item.ingredient.name
            );
        }

        println!(
            "{:>40} ---- {} {} inserters",
            "27 INSERTERS (27 steel chests)",
            (item.ratio * 27.0).round() as u32,
            item.ingredient.name
        );
    }
}

/// Prompt through STDIN for user string input with a specified prompt. If the user presses CTRL-C or otherwise passes SIGINT, this function will terminate the process on the spot.
///
/// # Parameters
/// * `rl_editor` - Rustyline [Editor](rustyline::Editor) used to read user's input.
/// * `user_prompt` - User prompt given to the user to signal the need for input.
fn prompt(
    rl_editor: &mut rustyline::Editor<(), rustyline::history::FileHistory>,
    user_prompt: &str,
) -> String {
    loop {
        let readline = rl_editor.readline(user_prompt);
        match readline {
            Ok(line) => break line,
            Err(e) => match e {
                rustyline::error::ReadlineError::Interrupted => {
                    // If interrupt signal is caught (with CTRL-C), exit here and now.

                    #[cfg(debug_assertions)]
                    {
                        println!("Interrupt signal caught. Terminating.")
                    }

                    std::process::exit(0);
                }
                _ => {
                    println!("Input error: {}", e);
                    continue;
                }
            },
        }
    }
}

/// Prompt through STDIN for user string input with a specified prompt. Will return [Some]\(input) if the user provides input, or [None] if the user elects to press CTRL-C or otherwise pass SIGINT. This function will _NOT_ kill the running process if the user presses CTRl-C or passes SIGINT.
///
/// # Parameters
/// * `rl_editor` - Rustyline [Editor](rustyline::Editor) used to read user's input.
/// * `user_prompt` - User prompt given to the user to signal the need for input.
fn prompt_or_sigint(
    rl_editor: &mut rustyline::Editor<(), rustyline::history::FileHistory>,
    user_prompt: &str,
) -> Option<String> {
    loop {
        let readline = rl_editor.readline(user_prompt);
        match readline {
            Ok(line) => break Some(line),
            Err(e) => match e {
                rustyline::error::ReadlineError::Interrupted => {
                    // If interrupt signal is caught (with CTRL-C), return as None.
                    return None;
                }
                _ => {
                    println!("Input error: {}", e);
                    continue;
                }
            },
        }
    }
}

/// Prompt and parse through STDIN for user generic input with a specified prompt. This function will repeat the prompt until a valid type is provided.
///
/// # Generic parameters
/// * `T` - The type to parse the user input into. This must implement [FromStr].
///
/// # Parameters
/// * `rl_editor` - Rustyline [Editor](rustyline::Editor) used to read user's input.
/// * `user_prompt` - User prompt given to the user to signal the need for input.
fn prompt_and_parse<T: FromStr<Err = impl Display>>(
    rl_editor: &mut rustyline::Editor<(), rustyline::history::FileHistory>,
    user_prompt: &str,
) -> T {
    loop {
        let input_string = prompt(rl_editor, user_prompt);
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

/// Prompt the user for the ingredients they wish to calculate over.
///
/// # Parameters
/// * `rl_editor` - Rustyline [Editor](rustyline::Editor) used to read user's input.
fn prompt_ingredients(
    rl_editor: &mut rustyline::Editor<(), rustyline::history::FileHistory>,
) -> Vec<Ingredient> {
    // Prompt for all ingredients.
    let mut ingredients: Vec<Ingredient> = Vec::new();
    loop {
        let ingredient_name =
            match prompt_or_sigint(rl_editor, format!("Item name (CTRL-C if done) > ").as_str()) {
                Some(t) => t,
                None => break,
            }
            .to_lowercase();

        let ingredient_amount = prompt_and_parse::<f64>(
            rl_editor,
            format!("{} amount > ", ingredient_name.to_uppercase()).as_str(),
        );

        let ingredient_stack_size = match data::fetch_item_stack_size(ingredient_name.as_str()) {
            Some(stack_size) => stack_size,
            None => prompt_and_parse::<u32>(
                rl_editor,
                format!("{} stack size > ", ingredient_name.to_uppercase()).as_str(),
            ),
        };

        ingredients.push(Ingredient {
            name: ingredient_name,
            amount: ingredient_amount,
            stack_size: ingredient_stack_size,
        });
    }

    ingredients
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
    /// The ratio of the ingredient calculated to be needed. This does _NOT_ take stack size into account. This value is between 0 and 1.
    ratio: f64,
    /// The ratio of the ingredient calculated to be needed. This takes stack size into account. This value is between 0 and 1.
    stack_ratio: f64,
}
