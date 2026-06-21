use crate::models::*;

const DRI_CALORIES: f64 = 2000.0;
const DRI_PROTEIN: f64 = 60.0;
const DRI_FAT: f64 = 65.0;
const DRI_CARBS: f64 = 300.0;
const DRI_FIBER: f64 = 25.0;
const DRI_SODIUM: f64 = 2000.0;
const DRI_CALCIUM: f64 = 800.0;
const DRI_IRON: f64 = 12.0;
const DRI_VITAMIN_A: f64 = 800.0;
const DRI_VITAMIN_C: f64 = 100.0;

const NRV_ENERGY: f64 = 8400.0;
const NRV_PROTEIN: f64 = 60.0;
const NRV_FAT: f64 = 60.0;
const NRV_CARBS: f64 = 300.0;
const NRV_SODIUM: f64 = 2000.0;

pub fn calculate_total_nutrition(ingredients: &[RecipeIngredient]) -> NutritionSummary {
    let mut total = NutritionSummary {
        calories: 0.0,
        calories_kj: 0.0,
        protein: 0.0,
        fat: 0.0,
        carbs: 0.0,
        fiber: 0.0,
        sodium: 0.0,
        calcium: 0.0,
        iron: 0.0,
        vitamin_a: 0.0,
        vitamin_c: 0.0,
    };

    for ri in ingredients {
        if let Some(ing) = &ri.ingredient {
            let factor = ri.amount / 100.0;
            total.calories += ing.calories * factor;
            total.protein += ing.protein * factor;
            total.fat += ing.fat * factor;
            total.carbs += ing.carbs * factor;
            total.fiber += ing.fiber * factor;
            total.sodium += ing.sodium * factor;
            total.calcium += ing.calcium * factor;
            total.iron += ing.iron * factor;
            total.vitamin_a += ing.vitamin_a * factor;
            total.vitamin_c += ing.vitamin_c * factor;
        }
    }

    total.calories_kj = (total.calories * 4.184).round();
    total
}

pub fn calculate_per_serving(total: &NutritionSummary, servings: i32) -> NutritionSummary {
    let servings_f = servings as f64;
    NutritionSummary {
        calories: total.calories / servings_f,
        calories_kj: total.calories_kj / servings_f,
        protein: total.protein / servings_f,
        fat: total.fat / servings_f,
        carbs: total.carbs / servings_f,
        fiber: total.fiber / servings_f,
        sodium: total.sodium / servings_f,
        calcium: total.calcium / servings_f,
        iron: total.iron / servings_f,
        vitamin_a: total.vitamin_a / servings_f,
        vitamin_c: total.vitamin_c / servings_f,
    }
}

pub fn calculate_per_100g(total: &NutritionSummary, total_weight: f64) -> NutritionSummary {
    if total_weight <= 0.0 {
        return NutritionSummary {
            calories: 0.0,
            calories_kj: 0.0,
            protein: 0.0,
            fat: 0.0,
            carbs: 0.0,
            fiber: 0.0,
            sodium: 0.0,
            calcium: 0.0,
            iron: 0.0,
            vitamin_a: 0.0,
            vitamin_c: 0.0,
        };
    }
    
    let factor = 100.0 / total_weight;
    NutritionSummary {
        calories: total.calories * factor,
        calories_kj: total.calories_kj * factor,
        protein: total.protein * factor,
        fat: total.fat * factor,
        carbs: total.carbs * factor,
        fiber: total.fiber * factor,
        sodium: total.sodium * factor,
        calcium: total.calcium * factor,
        iron: total.iron * factor,
        vitamin_a: total.vitamin_a * factor,
        vitamin_c: total.vitamin_c * factor,
    }
}

pub fn calculate_total_weight(ingredients: &[RecipeIngredient]) -> f64 {
    ingredients.iter().map(|ri| ri.amount).sum()
}

pub fn calculate_dri_comparison(nutrition: &NutritionSummary) -> std::collections::HashMap<String, DRIComparisonItem> {
    let mut result = std::collections::HashMap::new();
    
    fn get_status(percentage: f64, is_upper_limit: bool) -> String {
        if is_upper_limit {
            if percentage > 100.0 {
                "excess".to_string()
            } else if percentage < 60.0 {
                "warning".to_string()
            } else {
                "normal".to_string()
            }
        } else {
            if percentage > 150.0 {
                "excess".to_string()
            } else if percentage < 60.0 {
                "warning".to_string()
            } else {
                "normal".to_string()
            }
        }
    }
    
    let items = vec![
        ("calories", nutrition.calories, DRI_CALORIES, false),
        ("protein", nutrition.protein, DRI_PROTEIN, false),
        ("fat", nutrition.fat, DRI_FAT, true),
        ("carbs", nutrition.carbs, DRI_CARBS, false),
        ("fiber", nutrition.fiber, DRI_FIBER, false),
        ("sodium", nutrition.sodium, DRI_SODIUM, true),
        ("calcium", nutrition.calcium, DRI_CALCIUM, false),
        ("iron", nutrition.iron, DRI_IRON, false),
        ("vitamin_a", nutrition.vitamin_a, DRI_VITAMIN_A, false),
        ("vitamin_c", nutrition.vitamin_c, DRI_VITAMIN_C, false),
    ];
    
    for (key, value, reference, is_upper_limit) in items {
        let percentage = (value / reference) * 100.0;
        result.insert(key.to_string(), DRIComparisonItem {
            value,
            percentage,
            status: get_status(percentage, is_upper_limit),
        });
    }
    
    result
}

pub fn calculate_nrv_comparison(nutrition: &NutritionSummary) -> NRVComparison {
    NRVComparison {
        energy: NRVItem {
            value: nutrition.calories_kj.round(),
            nrv: NRV_ENERGY,
            percentage: ((nutrition.calories_kj / NRV_ENERGY) * 100.0).round() as i64,
        },
        protein: NRVItem {
            value: (nutrition.protein * 10.0).round() / 10.0,
            nrv: NRV_PROTEIN,
            percentage: ((nutrition.protein / NRV_PROTEIN) * 100.0).round() as i64,
        },
        fat: NRVItem {
            value: (nutrition.fat * 10.0).round() / 10.0,
            nrv: NRV_FAT,
            percentage: ((nutrition.fat / NRV_FAT) * 100.0).round() as i64,
        },
        carbs: NRVItem {
            value: (nutrition.carbs * 10.0).round() / 10.0,
            nrv: NRV_CARBS,
            percentage: ((nutrition.carbs / NRV_CARBS) * 100.0).round() as i64,
        },
        sodium: NRVItem {
            value: (nutrition.sodium * 10.0).round() / 10.0,
            nrv: NRV_SODIUM,
            percentage: ((nutrition.sodium / NRV_SODIUM) * 100.0).round() as i64,
        },
    }
}

pub fn calculate_cost(ingredients: &[RecipeIngredient], servings: i32) -> CostSummary {
    let mut total = 0.0;
    for ri in ingredients {
        if let Some(ing) = &ri.ingredient {
            if let Some(price) = ing.price_per_kg {
                total += (ri.amount / 1000.0) * price;
            }
        }
    }
    let total = (total * 100.0).round() / 100.0;
    let per_serving = (total / servings as f64 * 100.0).round() / 100.0;
    CostSummary { total, per_serving }
}
