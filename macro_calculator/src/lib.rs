use json::JsonValue;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut total_calories = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        let kcal_str = &food.calories[1];
        let kcal_value = kcal_str.trim_end_matches("kcal").parse::<f64>().unwrap_or(0.0);
        total_calories += kcal_value * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
    }

    fn round_to_two_decimal_places(value: f64) -> f64 {
        (value * 100.0).round() / 100.0
    }

    fn format_decimal(value: f64) -> String {
        let rounded_value = round_to_two_decimal_places(value);
        let s = format!("{:.2}", rounded_value);
        if s.ends_with(".00") {
            s[..s.len() - 3].to_string()
        } else if s.ends_with('0') && s.contains('.') {
            s[..s.len() - 1].to_string()
        } else {
            s
        }
    }

    let rounded_calories = format_decimal(total_calories);
    let rounded_carbs = format_decimal(total_carbs);
    let rounded_proteins = format_decimal(total_proteins);
    let rounded_fats = format_decimal(total_fats);

    json::object! {
        "cals": rounded_calories.parse::<f64>().unwrap_or(0.0),
        "carbs": rounded_carbs.parse::<f64>().unwrap_or(0.0),
        "proteins": rounded_proteins.parse::<f64>().unwrap_or(0.0),
        "fats": rounded_fats.parse::<f64>().unwrap_or(0.0),
    }
}