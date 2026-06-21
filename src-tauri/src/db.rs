use std::sync::Mutex;
use rusqlite::{params, Connection, Result};
use chrono::{Utc, DateTime};
use crate::models::*;

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
}
