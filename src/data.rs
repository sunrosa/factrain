/// Fetch from hand-written bindings the stack size of an item in the Factorio vanilla game from its in-game American English localized name. These bindings also include common colloquialisms.
///
/// # Returns
/// If it is successful, it will return the stack size of the item as [Some]\(u32), if the stack size is not defined in this function, it will instead return [None].
pub fn fetch_item_stack_size(item_name: &str) -> Option<u32> {
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
