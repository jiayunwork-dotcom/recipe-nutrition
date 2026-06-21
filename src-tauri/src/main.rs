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

fn calculate_similarity(a: &ExportedIngredient, b: &Ingredient) -> f64 {
    let mut score = 0.0;
    let nutrients = [
        (a.calories, b.calories, 200.0),
        (a.protein, b.protein, 20.0),
        (a.fat, b.fat, 20.0),
        (a.carbs, b.carbs, 50.0),
        (a.fiber, b.fiber, 10.0),
    ];
    for (va, vb, scale) in nutrients {
        let diff = (va - vb).abs();
        score += 1.0 / (1.0 + diff / scale);
    }
    score / nutrients.len() as f64
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

#[tauri::command]
fn get_nutrition_presets(state: State<AppState>) -> Result<Vec<NutritionPreset>, String> {
    state.db.get_nutrition_presets().map_err(|e| e.to_string())
}

#[tauri::command]
fn get_nutrition_preset(state: State<AppState>, id: i64) -> Result<NutritionPreset, String> {
    state.db.get_nutrition_preset(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn create_nutrition_preset(
    state: State<AppState>,
    data: CreateNutritionPreset,
) -> Result<NutritionPreset, String> {
    state.db.create_nutrition_preset(data).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_nutrition_preset(
    state: State<AppState>,
    id: i64,
    data: UpdateNutritionPreset,
) -> Result<NutritionPreset, String> {
    state.db
        .update_nutrition_preset(id, data)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_nutrition_preset(state: State<AppState>, id: i64) -> Result<(), String> {
    state.db.delete_nutrition_preset(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn set_recipe_nutrition_preset(
    state: State<AppState>,
    recipe_id: i64,
    preset_id: Option<i64>,
) -> Result<(), String> {
    state.db
        .set_recipe_nutrition_preset(recipe_id, preset_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_recipe_preset_id(state: State<AppState>, recipe_id: i64) -> Result<Option<i64>, String> {
    state.db
        .get_recipe_nutrition_preset_id(recipe_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn create_version_snapshot(
    state: State<AppState>,
    recipe_id: i64,
    summary: String,
) -> Result<RecipeVersion, String> {
    let recipe = state.db.get_recipe(recipe_id).map_err(|e| e.to_string())?;
    
    let snapshot_items: Vec<IngredientSnapshotItem> = recipe
        .ingredients
        .iter()
        .filter_map(|ri| ri.ingredient.as_ref().map(|i| IngredientSnapshotItem {
            ingredient_id: i.id,
            ingredient_name: i.name.clone(),
            category: i.category.clone(),
            amount: ri.amount,
            calories: i.calories,
            protein: i.protein,
            fat: i.fat,
            carbs: i.carbs,
            fiber: i.fiber,
            sodium: i.sodium,
            calcium: i.calcium,
            iron: i.iron,
            vitamin_a: i.vitamin_a,
            vitamin_c: i.vitamin_c,
        }))
        .collect();

    let ingredients_snapshot = serde_json::to_string(&snapshot_items)
        .map_err(|e| e.to_string())?;

    let total = nutrition::calculate_total_nutrition(&recipe.ingredients);
    let nutrition_snapshot = serde_json::to_string(&total)
        .map_err(|e| e.to_string())?;

    state.db
        .create_recipe_version(
            recipe_id,
            &summary,
            &ingredients_snapshot,
            &nutrition_snapshot,
            false,
            None,
        )
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_recipe_versions(
    state: State<AppState>,
    recipe_id: i64,
) -> Result<Vec<RecipeVersion>, String> {
    state.db.get_recipe_versions(recipe_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_version_diff(
    _state: State<AppState>,
    version_id: i64,
    previous_version_id: Option<i64>,
    versions_json: String,
) -> Result<VersionDiffSummary, String> {
    let versions: Vec<RecipeVersion> = serde_json::from_str(&versions_json)
        .map_err(|e| e.to_string())?;

    let current = versions.iter().find(|v| v.id == version_id)
        .ok_or_else(|| "Version not found".to_string())?;

    let current_ingredients: Vec<IngredientSnapshotItem> =
        serde_json::from_str(&current.ingredients_snapshot)
            .map_err(|e| e.to_string())?;

    let prev_ingredients: Vec<IngredientSnapshotItem> = if let Some(prev_id) = previous_version_id {
        let prev = versions.iter().find(|v| v.id == prev_id)
            .ok_or_else(|| "Previous version not found".to_string())?;
        serde_json::from_str(&prev.ingredients_snapshot)
            .map_err(|e| e.to_string())?
    } else {
        Vec::new()
    };

    let current_map: std::collections::HashMap<i64, &IngredientSnapshotItem> = current_ingredients
        .iter()
        .map(|i| (i.ingredient_id, i))
        .collect();
    let prev_map: std::collections::HashMap<i64, &IngredientSnapshotItem> = prev_ingredients
        .iter()
        .map(|i| (i.ingredient_id, i))
        .collect();

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut modified = Vec::new();

    for (id, curr) in &current_map {
        if let Some(prev) = prev_map.get(id) {
            if (curr.amount - prev.amount).abs() > 0.01 {
                modified.push(format!(
                    "{}: {:.1}g → {:.1}g",
                    curr.ingredient_name, prev.amount, curr.amount
                ));
            }
        } else {
            added.push(format!("{} ({:.1}g)", curr.ingredient_name, curr.amount));
        }
    }

    for (id, prev) in &prev_map {
        if !current_map.contains_key(id) {
            removed.push(format!("{} ({:.1}g)", prev.ingredient_name, prev.amount));
        }
    }

    Ok(VersionDiffSummary { added, removed, modified })
}

#[tauri::command]
fn rollback_to_version(state: State<AppState>, version_id: i64) -> Result<Recipe, String> {
    state.db.rollback_to_version(version_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_templates(
    state: State<AppState>,
    keyword: Option<String>,
    tag: Option<String>,
) -> Result<Vec<RecipeTemplate>, String> {
    state.db
        .get_templates(keyword.as_deref(), tag.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_template(state: State<AppState>, id: i64) -> Result<RecipeTemplate, String> {
    state.db.get_template(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn create_template(
    state: State<AppState>,
    data: CreateTemplate,
) -> Result<RecipeTemplate, String> {
    state.db.create_template(data).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_template(
    state: State<AppState>,
    id: i64,
    data: UpdateTemplate,
) -> Result<RecipeTemplate, String> {
    state.db.update_template(id, data).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_template(state: State<AppState>, id: i64) -> Result<(), String> {
    state.db.delete_template(id).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_all_template_tags(state: State<AppState>) -> Result<Vec<String>, String> {
    state.db.get_all_template_tags().map_err(|e| e.to_string())
}

#[tauri::command]
fn create_recipe_from_template(
    state: State<AppState>,
    template_id: i64,
    new_recipe_name: String,
) -> Result<Recipe, String> {
    state.db
        .create_recipe_from_template(template_id, &new_recipe_name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn export_recipe_json(
    state: State<AppState>,
    recipe_id: i64,
) -> Result<String, String> {
    let (recipe, preset_id_opt) = state.db
        .get_recipe_extended(recipe_id)
        .map_err(|e| e.to_string())?;

    let preset = if let Some(preset_id) = preset_id_opt {
        state.db.get_nutrition_preset(preset_id).ok()
    } else {
        None
    };

    let exported_ingredients: Vec<ExportedIngredient> = recipe
        .ingredients
        .iter()
        .filter_map(|ri| ri.ingredient.as_ref().map(|i| ExportedIngredient {
            name: i.name.clone(),
            category: i.category.clone(),
            amount: ri.amount,
            calories: i.calories,
            protein: i.protein,
            fat: i.fat,
            carbs: i.carbs,
            fiber: i.fiber,
            sodium: i.sodium,
            calcium: i.calcium,
            iron: i.iron,
            vitamin_a: i.vitamin_a,
            vitamin_c: i.vitamin_c,
            price_per_kg: i.price_per_kg,
        }))
        .collect();

    let exported_preset = preset.map(|p| ExportedNutritionPreset {
        name: p.name,
        description: p.description,
        max_calories: p.max_calories,
        min_calories: p.min_calories,
        max_protein: p.max_protein,
        min_protein: p.min_protein,
        max_fat: p.max_fat,
        min_fat: p.min_fat,
        max_carbs: p.max_carbs,
        min_carbs: p.min_carbs,
        max_fiber: p.max_fiber,
        min_fiber: p.min_fiber,
        max_sodium: p.max_sodium,
        min_sodium: p.min_sodium,
    });

    let export = RecipeExportFormat {
        format_version: "1.0".to_string(),
        exported_at: chrono::Utc::now().to_rfc3339(),
        recipe: ExportedRecipeInfo {
            name: recipe.name,
            category: recipe.category,
            servings: recipe.servings,
            notes: recipe.notes,
        },
        ingredients: exported_ingredients,
        nutrition_preset: exported_preset,
    };

    serde_json::to_string_pretty(&export).map_err(|e| e.to_string())
}

#[tauri::command]
fn preview_import_json(
    state: State<AppState>,
    json_content: String,
) -> Result<ImportPreview, String> {
    let import: RecipeExportFormat = serde_json::from_str(&json_content)
        .map_err(|e| format!("JSON 解析失败: {}", e))?;

    let mut ingredient_matches = Vec::new();
    let all_ingredients = state.db.get_ingredients(None, None)
        .map_err(|e| e.to_string())?;

    for exported in &import.ingredients {
        let exact_match = state.db
            .find_ingredient_exact_match(&exported.name, &exported.category)
            .map_err(|e| e.to_string())?;

        let (matched_local, action, requires_confirmation) = if let Some(m) = &exact_match {
            (Some(m.clone()), "use_exact".to_string(), false)
        } else {
            (None, "needs_review".to_string(), true)
        };

        let mut candidates: Vec<IngredientMatchCandidate> = all_ingredients
            .iter()
            .filter(|i| i.category.to_lowercase() == exported.category.to_lowercase())
            .map(|i| IngredientMatchCandidate {
                local_ingredient: i.clone(),
                similarity_score: calculate_similarity(exported, i),
            })
            .collect();
        candidates.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap_or(std::cmp::Ordering::Equal));
        candidates.truncate(5);

        ingredient_matches.push(ImportIngredientMatch {
            exported: exported.clone(),
            matched_local,
            candidates,
            requires_confirmation,
            action,
        });
    }

    let preset_exists = if let Some(ref preset) = import.nutrition_preset {
        state.db
            .get_nutrition_preset_by_name(&preset.name)
            .map_err(|e| e.to_string())?
            .is_some()
    } else {
        false
    };

    Ok(ImportPreview {
        recipe: import.recipe.clone(),
        ingredient_matches,
        nutrition_preset: import.nutrition_preset.clone(),
        preset_exists,
    })
}

#[tauri::command]
fn execute_import_json(
    state: State<AppState>,
    json_content: String,
    confirmed_items: Vec<ImportConfirmedItem>,
) -> Result<ImportResult, String> {
    let import: RecipeExportFormat = serde_json::from_str(&json_content)
        .map_err(|e| format!("JSON 解析失败: {}", e))?;

    let mut new_ingredients_added = 0;

    let mut ingredient_map: std::collections::HashMap<String, i64> = std::collections::HashMap::new();

    for confirmed in &confirmed_items {
        let exported = import
            .ingredients
            .iter()
            .find(|i| i.name == confirmed.exported_name)
            .ok_or_else(|| format!("未找到食材: {}", confirmed.exported_name))?;

        match confirmed.action.as_str() {
            "use_exact" | "use_candidate" => {
                if let Some(id) = confirmed.selected_ingredient_id {
                    ingredient_map.insert(confirmed.exported_name.clone(), id);
                } else {
                    let exact = state.db
                        .find_ingredient_exact_match(&exported.name, &exported.category)
                        .map_err(|e| e.to_string())?;
                    if let Some(i) = exact {
                        ingredient_map.insert(confirmed.exported_name.clone(), i.id);
                    }
                }
            }
            "add_new" => {
                let create_data = CreateIngredient {
                    name: exported.name.clone(),
                    category: exported.category.clone(),
                    calories: exported.calories,
                    protein: exported.protein,
                    fat: exported.fat,
                    carbs: exported.carbs,
                    fiber: exported.fiber,
                    sodium: exported.sodium,
                    calcium: exported.calcium,
                    iron: exported.iron,
                    vitamin_a: exported.vitamin_a,
                    vitamin_c: exported.vitamin_c,
                    price_per_kg: exported.price_per_kg,
                };
                let new_ing = state.db.create_ingredient(create_data)
                    .map_err(|e| e.to_string())?;
                ingredient_map.insert(confirmed.exported_name.clone(), new_ing.id);
                new_ingredients_added += 1;
            }
            _ => {}
        }
    }

    let (preset_created, preset_id) = if let Some(ref preset_info) = import.nutrition_preset {
        let existing = state.db
            .get_nutrition_preset_by_name(&preset_info.name)
            .map_err(|e| e.to_string())?;
        if let Some(p) = existing {
            (false, Some(p.id))
        } else {
            let create_data = CreateNutritionPreset {
                name: preset_info.name.clone(),
                description: preset_info.description.clone(),
                max_calories: preset_info.max_calories,
                min_calories: preset_info.min_calories,
                max_protein: preset_info.max_protein,
                min_protein: preset_info.min_protein,
                max_fat: preset_info.max_fat,
                min_fat: preset_info.min_fat,
                max_carbs: preset_info.max_carbs,
                min_carbs: preset_info.min_carbs,
                max_fiber: preset_info.max_fiber,
                min_fiber: preset_info.min_fiber,
                max_sodium: preset_info.max_sodium,
                min_sodium: preset_info.min_sodium,
            };
            let new_preset = state.db.create_nutrition_preset(create_data)
                .map_err(|e| e.to_string())?;
            (true, Some(new_preset.id))
        }
    } else {
        (false, None)
    };

    let create_recipe_data = CreateRecipe {
        name: format!("{} (导入)", import.recipe.name),
        category: import.recipe.category.clone(),
        servings: import.recipe.servings,
        is_favorite: false,
        notes: import.recipe.notes.clone(),
    };
    let new_recipe = state.db.create_recipe(create_recipe_data)
        .map_err(|e| e.to_string())?;

    if let Some(pid) = preset_id {
        state.db
            .set_recipe_nutrition_preset(new_recipe.id, Some(pid))
            .map_err(|e| e.to_string())?;
    }

    for exported in &import.ingredients {
        if let Some(&ingredient_id) = ingredient_map.get(&exported.name) {
            state.db
                .add_recipe_ingredient(new_recipe.id, ingredient_id, exported.amount)
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(ImportResult {
        recipe_id: new_recipe.id,
        new_ingredients_added,
        preset_created,
        preset_id,
    })
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
            get_nutrition_presets,
            get_nutrition_preset,
            create_nutrition_preset,
            update_nutrition_preset,
            delete_nutrition_preset,
            set_recipe_nutrition_preset,
            get_recipe_preset_id,
            create_version_snapshot,
            get_recipe_versions,
            get_version_diff,
            rollback_to_version,
            get_templates,
            get_template,
            create_template,
            update_template,
            delete_template,
            get_all_template_tags,
            create_recipe_from_template,
            export_recipe_json,
            preview_import_json,
            execute_import_json,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
