pub const MASS_UNITS: [&str; 10] = ["oz", "lb", "st", "tn", "t", "mcg", "mg", "g", "kg", "mt"];

pub fn check_mass(arg_1: &str, arg_2: &str) -> bool {
    return MASS_UNITS.contains(&arg_1) && MASS_UNITS.contains(&arg_2);
}

// TODO: handle _ in match properly and check correctness of formulas
// Function returns a tuple where 0 is the success of the conversion
// and 1 is the conversion itself
pub fn mass_convert(from: &str, to: &str, val: f64) -> (bool, f64) {
    let mut success = true;
    let mut conversion = 0.0;

    if from == "oz" {
        match to {
            "oz" => conversion = val,
            "lb" => conversion = val / 16.0,
            "st" => conversion = val / 224.0,
            "tn" => conversion = val / 32_000.0,
            "t" => conversion = val / 35_840.0,
            "mcg" => conversion = val * 28_350_000.0,
            "mg" => conversion = val * 28_350.0,
            "g" => conversion = val * 28.35,
            "kg" => conversion = val / 35.274,
            "mt" => conversion = val / 35_270.0,
            _ => success = false,
        }
    } else if from == "lb" {
        match to {
            "oz" => conversion = val * 16.0,
            "lb" => conversion = val,
            "st" => conversion = val / 14.0,
            "tn" => conversion = val / 2_000.0,
            "t" => conversion = val / 2_240.0,
            "mcg" => conversion = val * 453_600_000.0,
            "mg" => conversion = val * 453_600.0,
            "g" => conversion = val * 453.6,
            "kg" => conversion = val / 2.205,
            "mt" => conversion = val / 2_205.0,
            _ => success = false,
        }
    } else if from == "st" {
        match to {
            "oz" => conversion = val * 224.0,
            "lb" => conversion = val * 14.0,
            "st" => conversion = val,
            "tn" => conversion = val / 142.9,
            "t" => conversion = val / 160.0,
            "mcg" => conversion = val * 6_350_000_000.0,
            "mg" => conversion = val * 6_350_000.0,
            "g" => conversion = val * 6_350.0,
            "kg" => conversion = val * 6.35,
            "mt" => conversion = val / 157.5,
            _ => success = false,
        }
    } else if from == "tn" {
        match to {
            "oz" => conversion = val * 32_000.0,
            "lb" => conversion = val * 2_000.0,
            "st" => conversion = val * 142.9,
            "tn" => conversion = val,
            "t" => conversion = val / 1.12,
            "mcg" => conversion = val * 907_200_000_000.0,
            "mg" => conversion = val * 907_200_000.0,
            "g" => conversion = val * 907_200.0,
            "kg" => conversion = val * 907.2,
            "mt" => conversion = val / 1.102,
            _ => success = false,
        }
    } else if from == "t" {
        match to {
            "oz" => conversion = val * 35_840.0,
            "lb" => conversion = val * 2_240.0,
            "st" => conversion = val * 160.0,
            "tn" => conversion = val * 1.12,
            "t" => conversion = val,
            "mcg" => conversion = val * 1_016_000_000_000.0,
            "mg" => conversion = val * 1_016_000_000.0,
            "g" => conversion = val * 1_016_000.0,
            "kg" => conversion = val * 1_016.0,
            "mt" => conversion = val * 1.016,
            _ => success = false,
        }
    } else if from == "mcg" {
        match to {
            "oz" => conversion = val / 28_350_000.0,
            "lb" => conversion = val / 453_600_000.0,
            "st" => conversion = val / 6_350_000_000.0,
            "tn" => conversion = val / 907_200_000_000.0,
            "t" => conversion = val / 1_016_000_000_000.0,
            "mcg" => conversion = val,
            "mg" => conversion = val / 1_000.0,
            "g" => conversion = val / 1_000_000.0,
            "kg" => conversion = val / 1_000_000_000.0,
            "mt" => conversion = val / 1_000_000_000_000.0,
            _ => success = false,
        }
    } else if from == "mg" {
        match to {
            "oz" => conversion = val / 28_350.0,
            "lb" => conversion = val / 453_600.0,
            "st" => conversion = val / 6_350_000.0,
            "tn" => conversion = val / 907_200_000.0,
            "t" => conversion = val / 1_016_000_000.0,
            "mcg" => conversion = val * 1_000.0,
            "mg" => conversion = val,
            "g" => conversion = val / 1_000.0,
            "kg" => conversion = val / 1_000_000.0,
            "mt" => conversion = val / 1_000_000_000.0,
            _ => success = false,
        }
    } else if from == "g" {
        match to {
            "oz" => conversion = val / 28.35,
            "lb" => conversion = val / 453.6,
            "st" => conversion = val / 6_350.0,
            "tn" => conversion = val / 907_200.0,
            "t" => conversion = val / 1_016_000.0,
            "mcg" => conversion = val * 1_000_000.0,
            "mg" => conversion = val * 1_000.0,
            "g" => conversion = val,
            "kg" => conversion = val / 1_000.0,
            "mt" => conversion = val / 1_000_000.0,
            _ => success = false,
        }
    } else if from == "kg" {
        match to {
            "oz" => conversion = val * 35.274,
            "lb" => conversion = val * 2.205,
            "st" => conversion = val / 6.35,
            "tn" => conversion = val / 907.2,
            "t" => conversion = val / 1_016.0,
            "mcg" => conversion = val * 1_000_000_000.0,
            "mg" => conversion = val * 1_000_000.0,
            "g" => conversion = val * 1_000.0,
            "kg" => conversion = val,
            "mt" => conversion = val / 1_000.0,
            _ => success = false,
        }
    } else if from == "mt" {
        match to {
            "oz" => conversion = val * 35_270.0,
            "lb" => conversion = val * 2_205.0,
            "st" => conversion = val * 157.5,
            "tn" => conversion = val * 1.102,
            "t" => conversion = val / 1.016,
            "mcg" => conversion = val * 1_000_000_000_000.0,
            "mg" => conversion = val * 1_000_000_000.0,
            "g" => conversion = val * 1_000_000.0,
            "kg" => conversion = val * 1_000.0,
            "mt" => conversion = val,
            _ => success = false,
        }
    }

    return (success, conversion);
}
