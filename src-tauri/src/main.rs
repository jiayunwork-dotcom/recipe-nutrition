#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri::{State, AppHandle};
use serde::{Deserialize, Serialize};

mod models;
mod db;
mod nutrition;
mod init_data;

use models::*;
use db::Database;

struct AppState {
    db: Arc<Database>,
}

#[derive(Serialize, Deserialize)]
struct CalculateNutritionResponse {
    total: NutritionSummary,
    per_serving: NutritionSummary,
    per_100g: NutritionSummary,
    dri_comparison: std::collections::HashMap<String, DRIComparisonItem>,
    nrv_comparison: NRVComparison,
    cost: CostSummary,
    total_weight: f64,
}

#[tauri::command]
fn get_ingredients(
    state: State<AppState>,
    category: Option<String>,
    keyword: Option<String>,
) -> Result<Vec<Ingredient>, String> {
    state.db
        .get_ingredients(category.as_deref(), keyword.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_ingredient(state: State<AppState>, id: i64) -> Result<Ingredient, String> {
    state.db.get_ingredient(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn create_ingredient(
    state: State<AppState>,
    data: CreateIngredient,
) -> Result<Ingredient, String> {
    state.db.create_ingredient(data).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_ingredient(
    state: State<AppState>,
    id: i64,
    data: UpdateIngredient,
) -> Result<Ingredient, String> {
    state.db
        .update_ingredient(id, data)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_ingredient(state: State<AppState>, id: i64) -> Result<(), String> {
    state.db
        .delete_ingredient(id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_ingredient_categories(state: State<AppState>) -> Result<Vec<String>, String> {
    state.db
        .get_ingredient_categories()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn batch_update_ingredient_prices(
    state: State<AppState>,
    updates: Vec<PriceUpdate>,
) -> Result<(), String> {
    state.db
        .batch_update_ingredient_prices(&updates)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_recipes(
    state: State<AppState>,
    category: Option<String>,
    sort_by: Option<String>,
    only_favorites: Option<bool>,
) -> Result<Vec<Recipe>, String> {
    state.db
        .get_recipes(category.as_deref(), sort_by.as_deref(), only_favorites)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_recipe(state: State<AppState>, id: i64) -> Result<Recipe, String> {
    state.db.get_recipe(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn create_recipe(state: State<AppState>, data: CreateRecipe) -> Result<Recipe, String> {
    state.db.create_recipe(data).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_recipe(
    state: State<AppState>,
    id: i64,
    data: UpdateRecipe,
) -> Result<Recipe, String> {
    state.db
        .update_recipe(id, data)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_recipe(state: State<AppState>, id: i64) -> Result<(), String> {
    state.db.delete_recipe(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn toggle_recipe_favorite(state: State<AppState>, id: i64) -> Result<Recipe, String> {
    state.db
        .toggle_recipe_favorite(id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn duplicate_recipe(
    state: State<AppState>,
    id: i64,
    new_name: String,
) -> Result<Recipe, String> {
    state.db
        .duplicate_recipe(id, &new_name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn add_recipe_ingredient(
    state: State<AppState>,
    recipe_id: i64,
    ingredient_id: i64,
    amount: f64,
) -> Result<RecipeIngredient, String> {
    state.db
        .add_recipe_ingredient(recipe_id, ingredient_id, amount)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_recipe_ingredient(
    state: State<AppState>,
    id: i64,
    amount: f64,
) -> Result<RecipeIngredient, String> {
    state.db
        .update_recipe_ingredient(id, amount)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn remove_recipe_ingredient(state: State<AppState>, id: i64) -> Result<(), String> {
    state.db
        .remove_recipe_ingredient(id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_recipe_categories(state: State<AppState>) -> Result<Vec<String>, String> {
    state.db
        .get_recipe_categories()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn calculate_nutrition(
    state: State<AppState>,
    recipe_id: i64,
) -> Result<CalculateNutritionResponse, String> {
    let recipe = state.db.get_recipe(recipe_id).map_err(|e| e.to_string())?;

    let total = nutrition::calculate_total_nutrition(&recipe.ingredients);
    let per_serving = nutrition::calculate_per_serving(&total, recipe.servings);
    let total_weight = nutrition::calculate_total_weight(&recipe.ingredients);
    let per_100g = nutrition::calculate_per_100g(&total, total_weight);
    let dri_comparison = nutrition::calculate_dri_comparison(&per_serving);
    let nrv_comparison = nutrition::calculate_nrv_comparison(&per_100g);
    let cost = nutrition::calculate_cost(&recipe.ingredients, recipe.servings);

    Ok(CalculateNutritionResponse {
        total,
        per_serving,
        per_100g,
        dri_comparison,
        nrv_comparison,
        cost,
        total_weight,
    })
}

#[tauri::command]
fn export_recipe_pdf(
    _app: AppHandle,
    _state: State<AppState>,
    _id: i64,
    _file_path: String,
) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
fn export_recipe_excel(
    _app: AppHandle,
    _state: State<AppState>,
    _id: i64,
    _file_path: String,
) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
fn export_nutrition_label_image(
    _app: AppHandle,
    _state: State<AppState>,
    _id: i64,
    _file_path: String,
) -> Result<(), String> {
    Ok(())
}

fn main() {
    let app_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    
    let db_path = app_dir.join("recipe_nutrition.db");
    let db_path_str = db_path.to_string_lossy().to_string();

    let db = Database::new(&db_path_str).expect("Failed to initialize database");
    
    init_data::initialize_database(&db).expect("Failed to initialize database with seed data");

    let app_state = AppState {
        db: Arc::new(db),
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_ingredients,
            get_ingredient,
            create_ingredient,
            update_ingredient,
            delete_ingredient,
            get_ingredient_categories,
            batch_update_ingredient_prices,
            get_recipes,
            get_recipe,
            create_recipe,
            update_recipe,
            delete_recipe,
            toggle_recipe_favorite,
            duplicate_recipe,
            add_recipe_ingredient,
            update_recipe_ingredient,
            remove_recipe_ingredient,
            get_recipe_categories,
            calculate_nutrition,
            export_recipe_pdf,
            export_recipe_excel,
            export_nutrition_label_image,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
