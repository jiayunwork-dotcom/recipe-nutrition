use std::sync::Mutex;
use rusqlite::{params, Connection, Result, Row};
use chrono::{Utc, DateTime};
use crate::models::*;
use serde_json;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Database {
            conn: Mutex::new(conn),
        };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS ingredients (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                category TEXT NOT NULL,
                calories REAL NOT NULL DEFAULT 0,
                protein REAL NOT NULL DEFAULT 0,
                fat REAL NOT NULL DEFAULT 0,
                carbs REAL NOT NULL DEFAULT 0,
                fiber REAL NOT NULL DEFAULT 0,
                sodium REAL NOT NULL DEFAULT 0,
                calcium REAL NOT NULL DEFAULT 0,
                iron REAL NOT NULL DEFAULT 0,
                vitamin_a REAL NOT NULL DEFAULT 0,
                vitamin_c REAL NOT NULL DEFAULT 0,
                price_per_kg REAL,
                is_custom INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                UNIQUE(name, category)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS recipes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                category TEXT NOT NULL,
                servings INTEGER NOT NULL DEFAULT 1,
                is_favorite INTEGER NOT NULL DEFAULT 0,
                notes TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS recipe_ingredients (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                recipe_id INTEGER NOT NULL,
                ingredient_id INTEGER NOT NULL,
                amount REAL NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
                FOREIGN KEY (ingredient_id) REFERENCES ingredients(id) ON DELETE RESTRICT,
                UNIQUE(recipe_id, ingredient_id)
            )",
            [],
        )?;

        conn.execute(
            "ALTER TABLE recipes ADD COLUMN nutrition_preset_id INTEGER REFERENCES nutrition_presets(id) ON DELETE SET NULL",
            [],
        ).ok();

        conn.execute(
            "ALTER TABLE recipe_versions ADD COLUMN note TEXT NOT NULL DEFAULT ''",
            [],
        ).ok();

        conn.execute(
            "ALTER TABLE recipe_versions ADD COLUMN is_starred INTEGER NOT NULL DEFAULT 0",
            [],
        ).ok();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS nutrition_presets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                description TEXT NOT NULL DEFAULT '',
                max_calories REAL,
                min_calories REAL,
                max_protein REAL,
                min_protein REAL,
                max_fat REAL,
                min_fat REAL,
                max_carbs REAL,
                min_carbs REAL,
                max_fiber REAL,
                min_fiber REAL,
                max_sodium REAL,
                min_sodium REAL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS recipe_versions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                recipe_id INTEGER NOT NULL,
                version_number INTEGER NOT NULL,
                summary TEXT NOT NULL,
                ingredients_snapshot TEXT NOT NULL,
                nutrition_snapshot TEXT NOT NULL,
                is_rollback INTEGER NOT NULL DEFAULT 0,
                rollback_from_version INTEGER,
                created_at TEXT NOT NULL,
                FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
                UNIQUE(recipe_id, version_number)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS recipe_templates (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                category TEXT NOT NULL,
                servings INTEGER NOT NULL DEFAULT 1,
                notes TEXT NOT NULL DEFAULT '',
                nutrition_preset_id INTEGER,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (nutrition_preset_id) REFERENCES nutrition_presets(id) ON DELETE SET NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS template_tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS template_tag_relations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                template_id INTEGER NOT NULL,
                tag_id INTEGER NOT NULL,
                FOREIGN KEY (template_id) REFERENCES recipe_templates(id) ON DELETE CASCADE,
                FOREIGN KEY (tag_id) REFERENCES template_tags(id) ON DELETE CASCADE,
                UNIQUE(template_id, tag_id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS template_ingredients (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                template_id INTEGER NOT NULL,
                ingredient_id INTEGER NOT NULL,
                amount REAL NOT NULL,
                FOREIGN KEY (template_id) REFERENCES recipe_templates(id) ON DELETE CASCADE,
                FOREIGN KEY (ingredient_id) REFERENCES ingredients(id) ON DELETE RESTRICT,
                UNIQUE(template_id, ingredient_id)
            )",
            [],
        )?;

        conn.execute("PRAGMA foreign_keys = ON", [])?;

        Ok(())
    }

    pub fn get_ingredients(&self, category: Option<&str>, keyword: Option<&str>) -> Result<Vec<Ingredient>> {
        let conn = self.conn.lock().unwrap();
        let mut sql = "SELECT id, name, category, calories, protein, fat, carbs, fiber, sodium, calcium, iron, vitamin_a, vitamin_c, price_per_kg, is_custom, created_at, updated_at FROM ingredients WHERE 1=1".to_string();
        let mut params_vec: Vec<String> = Vec::new();

        if let Some(cat) = category {
            sql.push_str(" AND category = ?");
            params_vec.push(cat.to_string());
        }

        if let Some(kw) = keyword {
            sql.push_str(" AND LOWER(name) LIKE ?");
            params_vec.push(format!("%{}%", kw.to_lowercase()));
        }

        sql.push_str(" ORDER BY name COLLATE NOCASE");

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
            Ok(Ingredient {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
                calories: row.get(3)?,
                protein: row.get(4)?,
                fat: row.get(5)?,
                carbs: row.get(6)?,
                fiber: row.get(7)?,
                sodium: row.get(8)?,
                calcium: row.get(9)?,
                iron: row.get(10)?,
                vitamin_a: row.get(11)?,
                vitamin_c: row.get(12)?,
                price_per_kg: row.get(13)?,
                is_custom: row.get::<_, i32>(14)? != 0,
                created_at: row.get::<_, String>(15)?.parse().unwrap(),
                updated_at: row.get::<_, String>(16)?.parse().unwrap(),
            })
        })?;

        let mut ingredients = Vec::new();
        for row in rows {
            ingredients.push(row?);
        }

        Ok(ingredients)
    }

    pub fn get_ingredient(&self, id: i64) -> Result<Ingredient> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, category, calories, protein, fat, carbs, fiber, sodium, calcium, iron, vitamin_a, vitamin_c, price_per_kg, is_custom, created_at, updated_at 
             FROM ingredients WHERE id = ?"
        )?;
        
        let row = stmt.query_row(params![id], |row| {
            Ok(Ingredient {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
                calories: row.get(3)?,
                protein: row.get(4)?,
                fat: row.get(5)?,
                carbs: row.get(6)?,
                fiber: row.get(7)?,
                sodium: row.get(8)?,
                calcium: row.get(9)?,
                iron: row.get(10)?,
                vitamin_a: row.get(11)?,
                vitamin_c: row.get(12)?,
                price_per_kg: row.get(13)?,
                is_custom: row.get::<_, i32>(14)? != 0,
                created_at: row.get::<_, String>(15)?.parse().unwrap(),
                updated_at: row.get::<_, String>(16)?.parse().unwrap(),
            })
        })?;

        Ok(row)
    }

    pub fn create_ingredient(&self, data: CreateIngredient) -> Result<Ingredient> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO ingredients (name, category, calories, protein, fat, carbs, fiber, sodium, calcium, iron, vitamin_a, vitamin_c, price_per_kg, is_custom, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?)",
            params![
                data.name,
                data.category,
                data.calories,
                data.protein,
                data.fat,
                data.carbs,
                data.fiber,
                data.sodium,
                data.calcium,
                data.iron,
                data.vitamin_a,
                data.vitamin_c,
                data.price_per_kg,
                now,
                now,
            ],
        )?;

        let id = conn.last_insert_rowid();
        self.get_ingredient(id)
    }

    pub fn update_ingredient(&self, id: i64, data: UpdateIngredient) -> Result<Ingredient> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        let mut sql = "UPDATE ingredients SET updated_at = ?".to_string();
        let mut params_vec: Vec<rusqlite::types::Value> = vec![now.clone().into()];
        let mut param_index = 2;

        macro_rules! update_field {
            ($field:ident, $param:expr) => {
                if let Some(val) = $param {
                    sql.push_str(&format!(", {} = ?{}", stringify!($field), param_index));
                    params_vec.push(val.into());
                    param_index += 1;
                }
            };
        }

        update_field!(name, data.name);
        update_field!(category, data.category);
        update_field!(calories, data.calories);
        update_field!(protein, data.protein);
        update_field!(fat, data.fat);
        update_field!(carbs, data.carbs);
        update_field!(fiber, data.fiber);
        update_field!(sodium, data.sodium);
        update_field!(calcium, data.calcium);
        update_field!(iron, data.iron);
        update_field!(vitamin_a, data.vitamin_a);
        update_field!(vitamin_c, data.vitamin_c);
        update_field!(price_per_kg, data.price_per_kg);

        sql.push_str(" WHERE id = ?");
        params_vec.push(id.into());

        conn.execute(&sql, rusqlite::params_from_iter(params_vec.iter()))?;
        self.get_ingredient(id)
    }

    pub fn delete_ingredient(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM ingredients WHERE id = ? AND is_custom = 1", params![id])?;
        Ok(())
    }

    pub fn get_ingredient_categories(&self) -> Result<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT DISTINCT category FROM ingredients ORDER BY category COLLATE NOCASE"
        )?;
        
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut categories = Vec::new();
        for row in rows {
            categories.push(row?);
        }
        Ok(categories)
    }

    pub fn batch_update_ingredient_prices(&self, updates: &[PriceUpdate]) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        for update in updates {
            conn.execute(
                "UPDATE ingredients SET price_per_kg = ?, updated_at = ? WHERE id = ?",
                params![update.price_per_kg, now, update.id],
            )?;
        }
        Ok(())
    }

    pub fn get_recipes(&self, category: Option<&str>, sort_by: Option<&str>, only_favorites: Option<bool>) -> Result<Vec<Recipe>> {
        let conn = self.conn.lock().unwrap();
        let mut sql = "SELECT id, name, category, servings, is_favorite, notes, created_at, updated_at FROM recipes WHERE 1=1".to_string();
        let mut params_vec: Vec<String> = Vec::new();

        if let Some(cat) = category {
            sql.push_str(" AND category = ?");
            params_vec.push(cat.to_string());
        }

        if only_favorites.unwrap_or(false) {
            sql.push_str(" AND is_favorite = 1");
        }

        match sort_by {
            Some("name") => sql.push_str(" ORDER BY name COLLATE NOCASE"),
            _ => sql.push_str(" ORDER BY updated_at DESC"),
        }

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
            let recipe_id: i64 = row.get(0)?;
            let ingredients = self.get_recipe_ingredients_internal(&conn, recipe_id)?;
            Ok(Recipe {
                id: recipe_id,
                name: row.get(1)?,
                category: row.get(2)?,
                servings: row.get(3)?,
                is_favorite: row.get::<_, i32>(4)? != 0,
                notes: row.get(5)?,
                created_at: row.get::<_, String>(6)?.parse().unwrap(),
                updated_at: row.get::<_, String>(7)?.parse().unwrap(),
                ingredients,
            })
        })?;

        let mut recipes = Vec::new();
        for row in rows {
            recipes.push(row?);
        }

        Ok(recipes)
    }

    pub fn get_recipe(&self, id: i64) -> Result<Recipe> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, category, servings, is_favorite, notes, created_at, updated_at 
             FROM recipes WHERE id = ?"
        )?;
        
        let recipe = stmt.query_row(params![id], |row| {
            let recipe_id: i64 = row.get(0)?;
            let ingredients = self.get_recipe_ingredients_internal(&conn, recipe_id)?;
            Ok(Recipe {
                id: recipe_id,
                name: row.get(1)?,
                category: row.get(2)?,
                servings: row.get(3)?,
                is_favorite: row.get::<_, i32>(4)? != 0,
                notes: row.get(5)?,
                created_at: row.get::<_, String>(6)?.parse().unwrap(),
                updated_at: row.get::<_, String>(7)?.parse().unwrap(),
                ingredients,
            })
        })?;

        Ok(recipe)
    }

    fn get_recipe_ingredients_internal(&self, conn: &Connection, recipe_id: i64) -> Result<Vec<RecipeIngredient>> {
        let mut stmt = conn.prepare(
            "SELECT ri.id, ri.recipe_id, ri.ingredient_id, ri.amount, 
                    i.name, i.category, i.calories, i.protein, i.fat, i.carbs, i.fiber, i.sodium, i.calcium, i.iron, i.vitamin_a, i.vitamin_c, i.price_per_kg, i.is_custom, i.created_at, i.updated_at
             FROM recipe_ingredients ri
             INNER JOIN ingredients i ON ri.ingredient_id = i.id
             WHERE ri.recipe_id = ?
             ORDER BY ri.id"
        )?;

        let rows = stmt.query_map(params![recipe_id], |row| {
            Ok(RecipeIngredient {
                id: row.get(0)?,
                recipe_id: row.get(1)?,
                ingredient_id: row.get(2)?,
                amount: row.get(3)?,
                ingredient: Some(Ingredient {
                    id: row.get(2)?,
                    name: row.get(4)?,
                    category: row.get(5)?,
                    calories: row.get(6)?,
                    protein: row.get(7)?,
                    fat: row.get(8)?,
                    carbs: row.get(9)?,
                    fiber: row.get(10)?,
                    sodium: row.get(11)?,
                    calcium: row.get(12)?,
                    iron: row.get(13)?,
                    vitamin_a: row.get(14)?,
                    vitamin_c: row.get(15)?,
                    price_per_kg: row.get(16)?,
                    is_custom: row.get::<_, i32>(17)? != 0,
                    created_at: row.get::<_, String>(18)?.parse().unwrap(),
                    updated_at: row.get::<_, String>(19)?.parse().unwrap(),
                }),
            })
        })?;

        let mut ingredients = Vec::new();
        for row in rows {
            ingredients.push(row?);
        }
        Ok(ingredients)
    }

    pub fn create_recipe(&self, data: CreateRecipe) -> Result<Recipe> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO recipes (name, category, servings, is_favorite, notes, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![
                data.name,
                data.category,
                data.servings,
                data.is_favorite as i32,
                data.notes,
                now,
                now,
            ],
        )?;

        let id = conn.last_insert_rowid();
        self.get_recipe(id)
    }

    pub fn update_recipe(&self, id: i64, data: UpdateRecipe) -> Result<Recipe> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        let mut sql = "UPDATE recipes SET updated_at = ?".to_string();
        let mut params_vec: Vec<rusqlite::types::Value> = vec![now.clone().into()];
        let mut param_index = 2;

        if let Some(val) = data.name {
            sql.push_str(&format!(", name = ?{}", param_index));
            params_vec.push(val.into());
            param_index += 1;
        }
        if let Some(val) = data.category {
            sql.push_str(&format!(", category = ?{}", param_index));
            params_vec.push(val.into());
            param_index += 1;
        }
        if let Some(val) = data.servings {
            sql.push_str(&format!(", servings = ?{}", param_index));
            params_vec.push(val.into());
            param_index += 1;
        }
        if let Some(val) = data.is_favorite {
            sql.push_str(&format!(", is_favorite = ?{}", param_index));
            params_vec.push((val as i32).into());
            param_index += 1;
        }
        if let Some(val) = data.notes {
            sql.push_str(&format!(", notes = ?{}", param_index));
            params_vec.push(val.into());
            param_index += 1;
        }

        sql.push_str(" WHERE id = ?");
        params_vec.push(id.into());

        conn.execute(&sql, rusqlite::params_from_iter(params_vec.iter()))?;
        self.get_recipe(id)
    }

    pub fn delete_recipe(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM recipe_ingredients WHERE recipe_id = ?", params![id])?;
        conn.execute("DELETE FROM recipes WHERE id = ?", params![id])?;
        Ok(())
    }

    pub fn toggle_recipe_favorite(&self, id: i64) -> Result<Recipe> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "UPDATE recipes SET is_favorite = NOT is_favorite, updated_at = ? WHERE id = ?",
            params![now, id],
        )?;
        self.get_recipe(id)
    }

    pub fn duplicate_recipe(&self, id: i64, new_name: &str) -> Result<Recipe> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        let recipe = self.get_recipe(id)?;
        
        conn.execute(
            "INSERT INTO recipes (name, category, servings, is_favorite, notes, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![
                new_name,
                recipe.category,
                recipe.servings,
                recipe.is_favorite as i32,
                recipe.notes,
                now,
                now,
            ],
        )?;

        let new_id = conn.last_insert_rowid();

        for ri in recipe.ingredients {
            conn.execute(
                "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, amount, created_at, updated_at)
                 VALUES (?, ?, ?, ?, ?)",
                params![new_id, ri.ingredient_id, ri.amount, now, now],
            )?;
        }

        self.get_recipe(new_id)
    }

    pub fn add_recipe_ingredient(&self, recipe_id: i64, ingredient_id: i64, amount: f64) -> Result<RecipeIngredient> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, amount, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?)",
            params![recipe_id, ingredient_id, amount, now, now],
        )?;

        let id = conn.last_insert_rowid();
        
        let mut stmt = conn.prepare(
            "SELECT ri.id, ri.recipe_id, ri.ingredient_id, ri.amount, 
                    i.name, i.category, i.calories, i.protein, i.fat, i.carbs, i.fiber, i.sodium, i.calcium, i.iron, i.vitamin_a, i.vitamin_c, i.price_per_kg, i.is_custom, i.created_at, i.updated_at
             FROM recipe_ingredients ri
             INNER JOIN ingredients i ON ri.ingredient_id = i.id
             WHERE ri.id = ?"
        )?;

        let ri = stmt.query_row(params![id], |row| {
            Ok(RecipeIngredient {
                id: row.get(0)?,
                recipe_id: row.get(1)?,
                ingredient_id: row.get(2)?,
                amount: row.get(3)?,
                ingredient: Some(Ingredient {
                    id: row.get(2)?,
                    name: row.get(4)?,
                    category: row.get(5)?,
                    calories: row.get(6)?,
                    protein: row.get(7)?,
                    fat: row.get(8)?,
                    carbs: row.get(9)?,
                    fiber: row.get(10)?,
                    sodium: row.get(11)?,
                    calcium: row.get(12)?,
                    iron: row.get(13)?,
                    vitamin_a: row.get(14)?,
                    vitamin_c: row.get(15)?,
                    price_per_kg: row.get(16)?,
                    is_custom: row.get::<_, i32>(17)? != 0,
                    created_at: row.get::<_, String>(18)?.parse().unwrap(),
                    updated_at: row.get::<_, String>(19)?.parse().unwrap(),
                }),
            })
        })?;

        Ok(ri)
    }

    pub fn update_recipe_ingredient(&self, id: i64, amount: f64) -> Result<RecipeIngredient> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "UPDATE recipe_ingredients SET amount = ?, updated_at = ? WHERE id = ?",
            params![amount, now, id],
        )?;
        
        let recipe_id: i64 = conn.query_row(
            "SELECT recipe_id FROM recipe_ingredients WHERE id = ?",
            params![id],
            |row| row.get(0),
        )?;

        let ingredients = self.get_recipe_ingredients_internal(&conn, recipe_id)?;
        ingredients.into_iter().find(|ri| ri.id == id).ok_or_else(|| {
            rusqlite::Error::QueryReturnedNoRows
        })
    }

    pub fn remove_recipe_ingredient(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM recipe_ingredients WHERE id = ?", params![id])?;
        Ok(())
    }

    pub fn get_recipe_categories(&self) -> Result<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT DISTINCT category FROM recipes ORDER BY category COLLATE NOCASE"
        )?;
        
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut categories = Vec::new();
        for row in rows {
            categories.push(row?);
        }
        Ok(categories)
    }

    pub fn get_recipe_nutrition_preset_id(&self, recipe_id: i64) -> Result<Option<i64>> {
        let conn = self.conn.lock().unwrap();
        let result = conn.query_row(
            "SELECT nutrition_preset_id FROM recipes WHERE id = ?",
            params![recipe_id],
            |row| row.get::<_, Option<i64>>(0),
        );
        match result {
            Ok(val) => Ok(val),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn set_recipe_nutrition_preset(&self, recipe_id: i64, preset_id: Option<i64>) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE recipes SET nutrition_preset_id = ?, updated_at = ? WHERE id = ?",
            params![preset_id, now, recipe_id],
        )?;
        Ok(())
    }

    pub fn get_nutrition_presets(&self) -> Result<Vec<NutritionPreset>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, max_calories, min_calories, max_protein, min_protein, max_fat, min_fat, max_carbs, min_carbs, max_fiber, min_fiber, max_sodium, min_sodium, created_at, updated_at FROM nutrition_presets ORDER BY name COLLATE NOCASE"
        )?;
        let rows = stmt.query_map([], Self::row_to_nutrition_preset)?;
        let mut presets = Vec::new();
        for row in rows {
            presets.push(row?);
        }
        Ok(presets)
    }

    pub fn get_nutrition_preset(&self, id: i64) -> Result<NutritionPreset> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, max_calories, min_calories, max_protein, min_protein, max_fat, min_fat, max_carbs, min_carbs, max_fiber, min_fiber, max_sodium, min_sodium, created_at, updated_at FROM nutrition_presets WHERE id = ?"
        )?;
        stmt.query_row(params![id], Self::row_to_nutrition_preset)
    }

    pub fn get_nutrition_preset_by_name(&self, name: &str) -> Result<Option<NutritionPreset>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, max_calories, min_calories, max_protein, min_protein, max_fat, min_fat, max_carbs, min_carbs, max_fiber, min_fiber, max_sodium, min_sodium, created_at, updated_at FROM nutrition_presets WHERE name = ?"
        )?;
        match stmt.query_row(params![name], Self::row_to_nutrition_preset) {
            Ok(p) => Ok(Some(p)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn row_to_nutrition_preset(row: &Row) -> Result<NutritionPreset> {
        Ok(NutritionPreset {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            max_calories: row.get(3)?,
            min_calories: row.get(4)?,
            max_protein: row.get(5)?,
            min_protein: row.get(6)?,
            max_fat: row.get(7)?,
            min_fat: row.get(8)?,
            max_carbs: row.get(9)?,
            min_carbs: row.get(10)?,
            max_fiber: row.get(11)?,
            min_fiber: row.get(12)?,
            max_sodium: row.get(13)?,
            min_sodium: row.get(14)?,
            created_at: row.get::<_, String>(15)?.parse().unwrap(),
            updated_at: row.get::<_, String>(16)?.parse().unwrap(),
        })
    }

    pub fn create_nutrition_preset(&self, data: CreateNutritionPreset) -> Result<NutritionPreset> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO nutrition_presets (name, description, max_calories, min_calories, max_protein, min_protein, max_fat, min_fat, max_carbs, min_carbs, max_fiber, min_fiber, max_sodium, min_sodium, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                data.name,
                data.description,
                data.max_calories,
                data.min_calories,
                data.max_protein,
                data.min_protein,
                data.max_fat,
                data.min_fat,
                data.max_carbs,
                data.min_carbs,
                data.max_fiber,
                data.min_fiber,
                data.max_sodium,
                data.min_sodium,
                now,
                now,
            ],
        )?;
        let id = conn.last_insert_rowid();
        self.get_nutrition_preset(id)
    }

    pub fn update_nutrition_preset(&self, id: i64, data: UpdateNutritionPreset) -> Result<NutritionPreset> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        let mut sql = "UPDATE nutrition_presets SET updated_at = ?".to_string();
        let mut params_vec: Vec<rusqlite::types::Value> = vec![now.clone().into()];
        let mut param_index = 2;

        macro_rules! update_opt_field {
            ($field:ident, $param:expr) => {
                if let Some(val) = $param {
                    sql.push_str(&format!(", {} = ?{}", stringify!($field), param_index));
                    match val {
                        Some(v) => params_vec.push(v.into()),
                        None => params_vec.push(rusqlite::types::Value::Null),
                    }
                    param_index += 1;
                }
            };
        }
        macro_rules! update_field {
            ($field:ident, $param:expr) => {
                if let Some(val) = $param {
                    sql.push_str(&format!(", {} = ?{}", stringify!($field), param_index));
                    params_vec.push(val.into());
                    param_index += 1;
                }
            };
        }

        update_field!(name, data.name);
        update_field!(description, data.description);
        update_opt_field!(max_calories, data.max_calories);
        update_opt_field!(min_calories, data.min_calories);
        update_opt_field!(max_protein, data.max_protein);
        update_opt_field!(min_protein, data.min_protein);
        update_opt_field!(max_fat, data.max_fat);
        update_opt_field!(min_fat, data.min_fat);
        update_opt_field!(max_carbs, data.max_carbs);
        update_opt_field!(min_carbs, data.min_carbs);
        update_opt_field!(max_fiber, data.max_fiber);
        update_opt_field!(min_fiber, data.min_fiber);
        update_opt_field!(max_sodium, data.max_sodium);
        update_opt_field!(min_sodium, data.min_sodium);

        sql.push_str(" WHERE id = ?");
        params_vec.push(id.into());
        conn.execute(&sql, rusqlite::params_from_iter(params_vec.iter()))?;
        self.get_nutrition_preset(id)
    }

    pub fn delete_nutrition_preset(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM nutrition_presets WHERE id = ?", params![id])?;
        Ok(())
    }

    pub fn get_recipe_versions(&self, recipe_id: i64) -> Result<Vec<RecipeVersion>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, recipe_id, version_number, summary, ingredients_snapshot, nutrition_snapshot, is_rollback, rollback_from_version, note, is_starred, created_at FROM recipe_versions WHERE recipe_id = ? ORDER BY version_number DESC"
        )?;
        let rows = stmt.query_map(params![recipe_id], Self::row_to_recipe_version)?;
        let mut versions = Vec::new();
        for row in rows {
            versions.push(row?);
        }
        Ok(versions)
    }

    pub fn get_recipe_version(&self, id: i64) -> Result<RecipeVersion> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, recipe_id, version_number, summary, ingredients_snapshot, nutrition_snapshot, is_rollback, rollback_from_version, note, is_starred, created_at FROM recipe_versions WHERE id = ?"
        )?;
        stmt.query_row(params![id], Self::row_to_recipe_version)
    }

    fn row_to_recipe_version(row: &Row) -> Result<RecipeVersion> {
        Ok(RecipeVersion {
            id: row.get(0)?,
            recipe_id: row.get(1)?,
            version_number: row.get(2)?,
            summary: row.get(3)?,
            ingredients_snapshot: row.get(4)?,
            nutrition_snapshot: row.get(5)?,
            is_rollback: row.get::<_, i32>(6)? != 0,
            rollback_from_version: row.get(7)?,
            note: row.get(8)?,
            is_starred: row.get::<_, i32>(9)? != 0,
            created_at: row.get::<_, String>(10)?.parse().unwrap(),
        })
    }

    pub fn get_next_version_number(&self, recipe_id: i64) -> Result<i32> {
        let conn = self.conn.lock().unwrap();
        let result: Option<i32> = conn.query_row(
            "SELECT MAX(version_number) FROM recipe_versions WHERE recipe_id = ?",
            params![recipe_id],
            |row| row.get(0),
        )?;
        Ok(result.unwrap_or(0) + 1)
    }

    pub fn create_recipe_version(
        &self,
        recipe_id: i64,
        summary: &str,
        ingredients_snapshot: &str,
        nutrition_snapshot: &str,
        is_rollback: bool,
        rollback_from_version: Option<i32>,
        note: &str,
        is_starred: bool,
    ) -> Result<RecipeVersion> {
        let conn = self.conn.lock().unwrap();
        let version_number = {
            let result: Option<i32> = conn.query_row(
                "SELECT MAX(version_number) FROM recipe_versions WHERE recipe_id = ?",
                params![recipe_id],
                |row| row.get(0),
            )?;
            result.unwrap_or(0) + 1
        };
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO recipe_versions (recipe_id, version_number, summary, ingredients_snapshot, nutrition_snapshot, is_rollback, rollback_from_version, note, is_starred, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                recipe_id,
                version_number,
                summary,
                ingredients_snapshot,
                nutrition_snapshot,
                is_rollback as i32,
                rollback_from_version,
                note,
                is_starred as i32,
                now,
            ],
        )?;
        let id = conn.last_insert_rowid();
        self.get_recipe_version(id)
    }

    pub fn get_templates(&self, keyword: Option<&str>, tag: Option<&str>) -> Result<Vec<RecipeTemplate>> {
        let conn = self.conn.lock().unwrap();
        let mut sql = String::from(
            "SELECT DISTINCT t.id, t.name, t.description, t.category, t.servings, t.notes, t.nutrition_preset_id, t.created_at, t.updated_at FROM recipe_templates t"
        );
        let mut params_vec: Vec<String> = Vec::new();
        let mut where_clauses: Vec<String> = Vec::new();

        if tag.is_some() {
            sql.push_str(" INNER JOIN template_tag_relations ttr ON t.id = ttr.template_id INNER JOIN template_tags tt ON ttr.tag_id = tt.id");
        }

        if let Some(kw) = keyword {
            where_clauses.push("(LOWER(t.name) LIKE ? OR LOWER(t.description) LIKE ?)".to_string());
            params_vec.push(format!("%{}%", kw.to_lowercase()));
            params_vec.push(format!("%{}%", kw.to_lowercase()));
        }

        if let Some(tg) = tag {
            where_clauses.push("LOWER(tt.name) = ?".to_string());
            params_vec.push(tg.to_lowercase());
        }

        if !where_clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&where_clauses.join(" AND "));
        }

        sql.push_str(" ORDER BY t.updated_at DESC");

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
            self.row_to_recipe_template_internal(&conn, row)
        })?;
        let mut templates = Vec::new();
        for row in rows {
            templates.push(row?);
        }
        Ok(templates)
    }

    pub fn get_template(&self, id: i64) -> Result<RecipeTemplate> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, category, servings, notes, nutrition_preset_id, created_at, updated_at FROM recipe_templates WHERE id = ?"
        )?;
        stmt.query_row(params![id], |row| self.row_to_recipe_template_internal(&conn, row))
    }

    fn row_to_recipe_template_internal(&self, conn: &Connection, row: &Row) -> Result<RecipeTemplate> {
        let id: i64 = row.get(0)?;
        let tags = self.get_template_tags_internal(conn, id)?;
        let ingredients = self.get_template_ingredients_internal(conn, id)?;
        Ok(RecipeTemplate {
            id,
            name: row.get(1)?,
            description: row.get(2)?,
            category: row.get(3)?,
            servings: row.get(4)?,
            notes: row.get(5)?,
            nutrition_preset_id: row.get(6)?,
            created_at: row.get::<_, String>(7)?.parse().unwrap(),
            updated_at: row.get::<_, String>(8)?.parse().unwrap(),
            tags,
            ingredients,
        })
    }

    fn get_template_tags_internal(&self, conn: &Connection, template_id: i64) -> Result<Vec<String>> {
        let mut stmt = conn.prepare(
            "SELECT tt.name FROM template_tags tt INNER JOIN template_tag_relations ttr ON tt.id = ttr.tag_id WHERE ttr.template_id = ? ORDER BY tt.name COLLATE NOCASE"
        )?;
        let rows = stmt.query_map(params![template_id], |row| row.get::<_, String>(0))?;
        let mut tags = Vec::new();
        for row in rows {
            tags.push(row?);
        }
        Ok(tags)
    }

    fn get_template_ingredients_internal(&self, conn: &Connection, template_id: i64) -> Result<Vec<TemplateIngredient>> {
        let mut stmt = conn.prepare(
            "SELECT ingredient_id, amount FROM template_ingredients WHERE template_id = ? ORDER BY id"
        )?;
        let rows = stmt.query_map(params![template_id], |row| {
            Ok(TemplateIngredient {
                ingredient_id: row.get(0)?,
                amount: row.get(1)?,
            })
        })?;
        let mut ingredients = Vec::new();
        for row in rows {
            ingredients.push(row?);
        }
        Ok(ingredients)
    }

    pub fn get_all_template_tags(&self) -> Result<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT name FROM template_tags ORDER BY name COLLATE NOCASE"
        )?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut tags = Vec::new();
        for row in rows {
            tags.push(row?);
        }
        Ok(tags)
    }

    fn get_or_create_tag_internal(&self, conn: &Connection, tag_name: &str) -> Result<i64> {
        let lower_name = tag_name.trim().to_lowercase();
        if lower_name.is_empty() {
            return Err(rusqlite::Error::InvalidParameterName("empty tag".to_string()));
        }
        let existing: Option<i64> = conn.query_row(
            "SELECT id FROM template_tags WHERE LOWER(name) = ?",
            params![lower_name],
            |row| row.get(0),
        ).ok();
        if let Some(id) = existing {
            return Ok(id);
        }
        conn.execute(
            "INSERT INTO template_tags (name) VALUES (?)",
            params![tag_name.trim()],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn create_template(&self, data: CreateTemplate) -> Result<RecipeTemplate> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        let recipe = self.get_recipe_internal(&conn, data.recipe_id)?;

        conn.execute(
            "INSERT INTO recipe_templates (name, description, category, servings, notes, nutrition_preset_id, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                data.name,
                data.description,
                data.category,
                data.servings,
                data.notes,
                data.nutrition_preset_id,
                now,
                now,
            ],
        )?;
        let template_id = conn.last_insert_rowid();

        for tag_name in &data.tags {
            if let Ok(tag_id) = self.get_or_create_tag_internal(&conn, tag_name) {
                conn.execute(
                    "INSERT OR IGNORE INTO template_tag_relations (template_id, tag_id) VALUES (?, ?)",
                    params![template_id, tag_id],
                ).ok();
            }
        }

        for ri in &recipe.ingredients {
            conn.execute(
                "INSERT INTO template_ingredients (template_id, ingredient_id, amount) VALUES (?, ?, ?)",
                params![template_id, ri.ingredient_id, ri.amount],
            )?;
        }

        self.get_template(template_id)
    }

    fn get_recipe_internal(&self, conn: &Connection, id: i64) -> Result<Recipe> {
        let mut stmt = conn.prepare(
            "SELECT id, name, category, servings, is_favorite, notes, created_at, updated_at FROM recipes WHERE id = ?"
        )?;
        stmt.query_row(params![id], |row| {
            let recipe_id: i64 = row.get(0)?;
            let ingredients = self.get_recipe_ingredients_internal(conn, recipe_id)?;
            Ok(Recipe {
                id: recipe_id,
                name: row.get(1)?,
                category: row.get(2)?,
                servings: row.get(3)?,
                is_favorite: row.get::<_, i32>(4)? != 0,
                notes: row.get(5)?,
                created_at: row.get::<_, String>(6)?.parse().unwrap(),
                updated_at: row.get::<_, String>(7)?.parse().unwrap(),
                ingredients,
            })
        })
    }

    pub fn update_template(&self, id: i64, data: UpdateTemplate) -> Result<RecipeTemplate> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        let mut sql = "UPDATE recipe_templates SET updated_at = ?".to_string();
        let mut params_vec: Vec<rusqlite::types::Value> = vec![now.clone().into()];
        let mut param_index = 2;

        if let Some(val) = data.name {
            sql.push_str(&format!(", name = ?{}", param_index));
            params_vec.push(val.into());
            param_index += 1;
        }
        if let Some(val) = data.description {
            sql.push_str(&format!(", description = ?{}", param_index));
            params_vec.push(val.into());
            param_index += 1;
        }

        sql.push_str(" WHERE id = ?");
        params_vec.push(id.into());
        conn.execute(&sql, rusqlite::params_from_iter(params_vec.iter()))?;

        if let Some(tags) = data.tags {
            conn.execute("DELETE FROM template_tag_relations WHERE template_id = ?", params![id])?;
            for tag_name in &tags {
                if let Ok(tag_id) = self.get_or_create_tag_internal(&conn, tag_name) {
                    conn.execute(
                        "INSERT OR IGNORE INTO template_tag_relations (template_id, tag_id) VALUES (?, ?)",
                        params![id, tag_id],
                    ).ok();
                }
            }
        }

        self.get_template(id)
    }

    pub fn delete_template(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM template_ingredients WHERE template_id = ?", params![id])?;
        conn.execute("DELETE FROM template_tag_relations WHERE template_id = ?", params![id])?;
        conn.execute("DELETE FROM recipe_templates WHERE id = ?", params![id])?;
        Ok(())
    }

    pub fn create_recipe_from_template(&self, template_id: i64, new_recipe_name: &str) -> Result<Recipe> {
        let template = self.get_template(template_id)?;
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO recipes (name, category, servings, is_favorite, notes, nutrition_preset_id, created_at, updated_at)
             VALUES (?, ?, ?, 0, ?, ?, ?, ?)",
            params![
                new_recipe_name,
                template.category,
                template.servings,
                template.notes,
                template.nutrition_preset_id,
                now,
                now,
            ],
        )?;
        let recipe_id = conn.last_insert_rowid();

        for ti in &template.ingredients {
            conn.execute(
                "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, amount, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
                params![recipe_id, ti.ingredient_id, ti.amount, now, now],
            )?;
        }

        drop(conn);
        self.get_recipe(recipe_id)
    }

    pub fn find_ingredient_exact_match(&self, name: &str, category: &str) -> Result<Option<Ingredient>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, category, calories, protein, fat, carbs, fiber, sodium, calcium, iron, vitamin_a, vitamin_c, price_per_kg, is_custom, created_at, updated_at FROM ingredients WHERE LOWER(name) = ? AND LOWER(category) = ?"
        )?;
        match stmt.query_row(params![name.to_lowercase(), category.to_lowercase()], |row| {
            Ok(Ingredient {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
                calories: row.get(3)?,
                protein: row.get(4)?,
                fat: row.get(5)?,
                carbs: row.get(6)?,
                fiber: row.get(7)?,
                sodium: row.get(8)?,
                calcium: row.get(9)?,
                iron: row.get(10)?,
                vitamin_a: row.get(11)?,
                vitamin_c: row.get(12)?,
                price_per_kg: row.get(13)?,
                is_custom: row.get::<_, i32>(14)? != 0,
                created_at: row.get::<_, String>(15)?.parse().unwrap(),
                updated_at: row.get::<_, String>(16)?.parse().unwrap(),
            })
        }) {
            Ok(i) => Ok(Some(i)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn find_ingredient_similar(&self, category: &str, exclude_id: Option<i64>) -> Result<Vec<Ingredient>> {
        let conn = self.conn.lock().unwrap();
        let mut sql = String::from(
            "SELECT id, name, category, calories, protein, fat, carbs, fiber, sodium, calcium, iron, vitamin_a, vitamin_c, price_per_kg, is_custom, created_at, updated_at FROM ingredients WHERE LOWER(category) = ?"
        );
        let mut params_vec: Vec<String> = vec![category.to_lowercase()];
        if let Some(eid) = exclude_id {
            sql.push_str(" AND id != ?");
            params_vec.push(eid.to_string());
        }
        sql.push_str(" LIMIT 10");

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
            Ok(Ingredient {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
                calories: row.get(3)?,
                protein: row.get(4)?,
                fat: row.get(5)?,
                carbs: row.get(6)?,
                fiber: row.get(7)?,
                sodium: row.get(8)?,
                calcium: row.get(9)?,
                iron: row.get(10)?,
                vitamin_a: row.get(11)?,
                vitamin_c: row.get(12)?,
                price_per_kg: row.get(13)?,
                is_custom: row.get::<_, i32>(14)? != 0,
                created_at: row.get::<_, String>(15)?.parse().unwrap(),
                updated_at: row.get::<_, String>(16)?.parse().unwrap(),
            })
        })?;
        let mut ingredients = Vec::new();
        for row in rows {
            ingredients.push(row?);
        }
        Ok(ingredients)
    }

    pub fn get_recipe_extended(&self, id: i64) -> Result<(Recipe, Option<i64>)> {
        let recipe = self.get_recipe(id)?;
        let preset_id = self.get_recipe_nutrition_preset_id(id)?;
        Ok((recipe, preset_id))
    }

    pub fn update_recipe_with_version(
        &self,
        recipe_id: i64,
        data: UpdateRecipe,
        version_summary: &str,
        ingredients_snapshot: &str,
        nutrition_snapshot: &str,
    ) -> Result<Recipe> {
        let updated = self.update_recipe(recipe_id, data)?;
        self.create_recipe_version(
            recipe_id,
            version_summary,
            ingredients_snapshot,
            nutrition_snapshot,
            false,
            None,
            "",
            false,
        )?;
        Ok(updated)
    }

    pub fn rollback_to_version(&self, version_id: i64) -> Result<Recipe> {
        self.rollback_to_version_with_keep(version_id, &[])
    }

    pub fn rollback_to_version_with_keep(&self, version_id: i64, keep_ingredient_ids: &[i64]) -> Result<Recipe> {
        let conn = self.conn.lock().unwrap();
        let version = self.get_recipe_version(version_id)?;
        let now = Utc::now().to_rfc3339();

        let snapshot: Vec<IngredientSnapshotItem> = serde_json::from_str(&version.ingredients_snapshot)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?;

        conn.execute("DELETE FROM recipe_ingredients WHERE recipe_id = ?", params![version.recipe_id])?;

        for item in &snapshot {
            conn.execute(
                "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, amount, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
                params![version.recipe_id, item.ingredient_id, item.amount, now, now],
            ).ok();
        }

        for &keep_id in keep_ingredient_ids {
            if !snapshot.iter().any(|s| s.ingredient_id == keep_id) {
                conn.execute(
                    "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, amount, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
                    params![version.recipe_id, keep_id, 100.0, now, now],
                ).ok();
            }
        }

        drop(conn);

        let rollback_summary = format!("回退到版本{}", version.version_number);
        let nutrition_snapshot_copy = version.nutrition_snapshot.clone();
        let ingredients_snapshot_copy = version.ingredients_snapshot.clone();

        self.create_recipe_version(
            version.recipe_id,
            &rollback_summary,
            &ingredients_snapshot_copy,
            &nutrition_snapshot_copy,
            true,
            Some(version.version_number),
            "",
            false,
        )?;

        self.get_recipe(version.recipe_id)
    }

    pub fn update_version_note(&self, version_id: i64, note: &str) -> Result<RecipeVersion> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE recipe_versions SET note = ? WHERE id = ?",
            params![note, version_id],
        )?;
        drop(conn);
        self.get_recipe_version(version_id)
    }

    pub fn toggle_version_star(&self, version_id: i64) -> Result<RecipeVersion> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE recipe_versions SET is_starred = NOT is_starred WHERE id = ?",
            params![version_id],
        )?;
        drop(conn);
        self.get_recipe_version(version_id)
    }

    pub fn get_versions_by_ids(&self, version_ids: &[i64]) -> Result<Vec<RecipeVersion>> {
        let conn = self.conn.lock().unwrap();
        let placeholders: Vec<String> = version_ids.iter().map(|_| "?".to_string()).collect();
        let sql = format!(
            "SELECT id, recipe_id, version_number, summary, ingredients_snapshot, nutrition_snapshot, is_rollback, rollback_from_version, note, is_starred, created_at FROM recipe_versions WHERE id IN ({})",
            placeholders.join(",")
        );
        let mut stmt = conn.prepare(&sql)?;
        let params_vec: Vec<&dyn rusqlite::types::ToSql> = version_ids.iter().map(|id| id as &dyn rusqlite::types::ToSql).collect();
        let rows = stmt.query_map(params_vec.as_slice(), Self::row_to_recipe_version)?;
        let mut versions = Vec::new();
        for row in rows {
            versions.push(row?);
        }
        Ok(versions)
    }

    pub fn import_version_snapshots(&self, recipe_id: i64, snapshots: Vec<VersionSnapshotExport>) -> Result<Vec<RecipeVersion>> {
        let mut result = Vec::new();
        for snap in &snapshots {
            {
                let conn = self.conn.lock().unwrap();
                let version_number = {
                    let max_vn: Option<i32> = conn.query_row(
                        "SELECT MAX(version_number) FROM recipe_versions WHERE recipe_id = ?",
                        params![recipe_id],
                        |row| row.get(0),
                    )?;
                    max_vn.unwrap_or(0) + 1
                };
                conn.execute(
                    "INSERT INTO recipe_versions (recipe_id, version_number, summary, ingredients_snapshot, nutrition_snapshot, is_rollback, rollback_from_version, note, is_starred, created_at)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                    params![
                        recipe_id,
                        version_number,
                        snap.summary,
                        snap.ingredients_snapshot,
                        snap.nutrition_snapshot,
                        snap.is_rollback as i32,
                        snap.rollback_from_version,
                        snap.note,
                        snap.is_starred as i32,
                        snap.created_at,
                    ],
                )?;
            }
            let last_id = {
                let conn = self.conn.lock().unwrap();
                conn.query_row(
                    "SELECT MAX(id) FROM recipe_versions WHERE recipe_id = ?",
                    params![recipe_id],
                    |row| row.get::<_, i64>(0),
                )?
            };
            let v = self.get_recipe_version(last_id)?;
            result.push(v);
        }
        Ok(result)
    }
}
