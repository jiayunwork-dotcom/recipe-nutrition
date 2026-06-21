use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub calories: f64,
    pub protein: f64,
    pub fat: f64,
    pub carbs: f64,
    pub fiber: f64,
    pub sodium: f64,
    pub calcium: f64,
    pub iron: f64,
    pub vitamin_a: f64,
    pub vitamin_c: f64,
    pub price_per_kg: Option<f64>,
    pub is_custom: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIngredient {
    pub name: String,
    pub category: String,
    pub calories: f64,
    pub protein: f64,
    pub fat: f64,
    pub carbs: f64,
    pub fiber: f64,
    pub sodium: f64,
    pub calcium: f64,
    pub iron: f64,
    pub vitamin_a: f64,
    pub vitamin_c: f64,
    pub price_per_kg: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIngredient {
    pub name: Option<String>,
    pub category: Option<String>,
    pub calories: Option<f64>,
    pub protein: Option<f64>,
    pub fat: Option<f64>,
    pub carbs: Option<f64>,
    pub fiber: Option<f64>,
    pub sodium: Option<f64>,
    pub calcium: Option<f64>,
    pub iron: Option<f64>,
    pub vitamin_a: Option<f64>,
    pub vitamin_c: Option<f64>,
    pub price_per_kg: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub servings: i32,
    pub is_favorite: bool,
    pub notes: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub ingredients: Vec<RecipeIngredient>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeIngredient {
    pub id: i64,
    pub recipe_id: i64,
    pub ingredient_id: i64,
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingredient: Option<Ingredient>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRecipe {
    pub name: String,
    pub category: String,
    pub servings: i32,
    pub is_favorite: bool,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRecipe {
    pub name: Option<String>,
    pub category: Option<String>,
    pub servings: Option<i32>,
    pub is_favorite: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionSummary {
    pub calories: f64,
    pub calories_kj: f64,
    pub protein: f64,
    pub fat: f64,
    pub carbs: f64,
    pub fiber: f64,
    pub sodium: f64,
    pub calcium: f64,
    pub iron: f64,
    pub vitamin_a: f64,
    pub vitamin_c: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostSummary {
    pub total: f64,
    pub per_serving: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DRIComparisonItem {
    pub value: f64,
    pub percentage: f64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NRVItem {
    pub value: f64,
    pub nrv: f64,
    pub percentage: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NRVComparison {
    pub energy: NRVItem,
    pub protein: NRVItem,
    pub fat: NRVItem,
    pub carbs: NRVItem,
    pub sodium: NRVItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceUpdate {
    pub id: i64,
    pub price_per_kg: Option<f64>,
}
