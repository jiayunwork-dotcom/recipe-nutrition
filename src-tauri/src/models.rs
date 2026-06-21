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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeVersion {
    pub id: i64,
    pub recipe_id: i64,
    pub version_number: i32,
    pub summary: String,
    pub ingredients_snapshot: String,
    pub nutrition_snapshot: String,
    pub is_rollback: bool,
    pub rollback_from_version: Option<i32>,
    pub note: String,
    pub is_starred: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngredientSnapshotItem {
    pub ingredient_id: i64,
    pub ingredient_name: String,
    pub category: String,
    pub amount: f64,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeTemplate {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub category: String,
    pub servings: i32,
    pub notes: String,
    pub nutrition_preset_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub ingredients: Vec<TemplateIngredient>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateIngredient {
    pub ingredient_id: i64,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTemplate {
    pub name: String,
    pub description: String,
    pub category: String,
    pub servings: i32,
    pub notes: String,
    pub recipe_id: i64,
    pub tags: Vec<String>,
    pub nutrition_preset_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTemplate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionPreset {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub max_calories: Option<f64>,
    pub min_calories: Option<f64>,
    pub max_protein: Option<f64>,
    pub min_protein: Option<f64>,
    pub max_fat: Option<f64>,
    pub min_fat: Option<f64>,
    pub max_carbs: Option<f64>,
    pub min_carbs: Option<f64>,
    pub max_fiber: Option<f64>,
    pub min_fiber: Option<f64>,
    pub max_sodium: Option<f64>,
    pub min_sodium: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNutritionPreset {
    pub name: String,
    pub description: String,
    pub max_calories: Option<f64>,
    pub min_calories: Option<f64>,
    pub max_protein: Option<f64>,
    pub min_protein: Option<f64>,
    pub max_fat: Option<f64>,
    pub min_fat: Option<f64>,
    pub max_carbs: Option<f64>,
    pub min_carbs: Option<f64>,
    pub max_fiber: Option<f64>,
    pub min_fiber: Option<f64>,
    pub max_sodium: Option<f64>,
    pub min_sodium: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNutritionPreset {
    pub name: Option<String>,
    pub description: Option<String>,
    pub max_calories: Option<Option<f64>>,
    pub min_calories: Option<Option<f64>>,
    pub max_protein: Option<Option<f64>>,
    pub min_protein: Option<Option<f64>>,
    pub max_fat: Option<Option<f64>>,
    pub min_fat: Option<Option<f64>>,
    pub max_carbs: Option<Option<f64>>,
    pub min_carbs: Option<Option<f64>>,
    pub max_fiber: Option<Option<f64>>,
    pub min_fiber: Option<Option<f64>>,
    pub max_sodium: Option<Option<f64>>,
    pub min_sodium: Option<Option<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeExportFormat {
    pub format_version: String,
    pub exported_at: String,
    pub recipe: ExportedRecipeInfo,
    pub ingredients: Vec<ExportedIngredient>,
    pub nutrition_preset: Option<ExportedNutritionPreset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedRecipeInfo {
    pub name: String,
    pub category: String,
    pub servings: i32,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedIngredient {
    pub name: String,
    pub category: String,
    pub amount: f64,
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
pub struct ExportedNutritionPreset {
    pub name: String,
    pub description: String,
    pub max_calories: Option<f64>,
    pub min_calories: Option<f64>,
    pub max_protein: Option<f64>,
    pub min_protein: Option<f64>,
    pub max_fat: Option<f64>,
    pub min_fat: Option<f64>,
    pub max_carbs: Option<f64>,
    pub min_carbs: Option<f64>,
    pub max_fiber: Option<f64>,
    pub min_fiber: Option<f64>,
    pub max_sodium: Option<f64>,
    pub min_sodium: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngredientMatchCandidate {
    pub local_ingredient: Ingredient,
    pub similarity_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportIngredientMatch {
    pub exported: ExportedIngredient,
    pub matched_local: Option<Ingredient>,
    pub candidates: Vec<IngredientMatchCandidate>,
    pub requires_confirmation: bool,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportPreview {
    pub recipe: ExportedRecipeInfo,
    pub ingredient_matches: Vec<ImportIngredientMatch>,
    pub nutrition_preset: Option<ExportedNutritionPreset>,
    pub preset_exists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportConfirmedItem {
    pub exported_name: String,
    pub action: String,
    pub selected_ingredient_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub recipe_id: i64,
    pub new_ingredients_added: i32,
    pub preset_created: bool,
    pub preset_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDiffSummary {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionSnapshotExport {
    pub id: i64,
    pub recipe_id: i64,
    pub version_number: i32,
    pub summary: String,
    pub ingredients_snapshot: String,
    pub nutrition_snapshot: String,
    pub is_rollback: bool,
    pub rollback_from_version: Option<i32>,
    pub note: String,
    pub is_starred: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionsExportFormat {
    pub format_version: String,
    pub exported_at: String,
    pub recipe_id: i64,
    pub recipe_name: String,
    pub versions: Vec<VersionSnapshotExport>,
}
