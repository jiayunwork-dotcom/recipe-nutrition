import { invoke } from '@tauri-apps/api/core'
import type {
  Ingredient,
  Recipe,
  RecipeIngredient,
  RecipeVersion,
  RecipeTemplate,
  NutritionPreset,
  ImportPreview,
  ImportConfirmedItem,
  ImportResult,
  VersionDiffSummary,
} from '@/types'

export const ingredientApi = {
  getAll: (category?: string, keyword?: string): Promise<Ingredient[]> =>
    invoke('get_ingredients', { category, keyword }),

  getById: (id: number): Promise<Ingredient> =>
    invoke('get_ingredient', { id }),

  create: (data: Omit<Ingredient, 'id' | 'created_at' | 'updated_at' | 'is_custom'>): Promise<Ingredient> =>
    invoke('create_ingredient', { data }),

  update: (id: number, data: Partial<Omit<Ingredient, 'id' | 'created_at' | 'updated_at'>>): Promise<Ingredient> =>
    invoke('update_ingredient', { id, data }),

  delete: (id: number): Promise<void> =>
    invoke('delete_ingredient', { id }),

  getCategories: (): Promise<string[]> =>
    invoke('get_ingredient_categories'),

  batchUpdatePrice: (updates: Array<{ id: number; price_per_kg: number | null }>): Promise<void> =>
    invoke('batch_update_ingredient_prices', { updates })
}

export const recipeApi = {
  getAll: (category?: string, sortBy?: 'recent' | 'name', onlyFavorites?: boolean): Promise<Recipe[]> =>
    invoke('get_recipes', { category, sortBy, onlyFavorites }),

  getById: (id: number): Promise<Recipe> =>
    invoke('get_recipe', { id }),

  create: (data: Omit<Recipe, 'id' | 'created_at' | 'updated_at' | 'ingredients'>): Promise<Recipe> =>
    invoke('create_recipe', { data }),

  update: (id: number, data: Partial<Omit<Recipe, 'id' | 'created_at' | 'updated_at' | 'ingredients'>>): Promise<Recipe> =>
    invoke('update_recipe', { id, data }),

  delete: (id: number): Promise<void> =>
    invoke('delete_recipe', { id }),

  toggleFavorite: (id: number): Promise<Recipe> =>
    invoke('toggle_recipe_favorite', { id }),

  duplicate: (id: number, newName: string): Promise<Recipe> =>
    invoke('duplicate_recipe', { id, newName }),

  addIngredient: (recipeId: number, ingredientId: number, amount: number): Promise<RecipeIngredient> =>
    invoke('add_recipe_ingredient', { recipeId, ingredientId, amount }),

  updateIngredient: (id: number, amount: number): Promise<RecipeIngredient> =>
    invoke('update_recipe_ingredient', { id, amount }),

  removeIngredient: (id: number): Promise<void> =>
    invoke('remove_recipe_ingredient', { id }),

  getCategories: (): Promise<string[]> =>
    invoke('get_recipe_categories'),

  exportPDF: (id: number, filePath: string): Promise<void> =>
    invoke('export_recipe_pdf', { id, filePath }),

  exportExcel: (id: number, filePath: string): Promise<void> =>
    invoke('export_recipe_excel', { id, filePath }),

  exportLabelImage: (id: number, filePath: string): Promise<void> =>
    invoke('export_nutrition_label_image', { id, filePath })
}

export const nutritionApi = {
  calculate: (recipeId: number): Promise<{
    total: any
    perServing: any
    per100g: any
    driComparison: any
    nrvComparison: any
    cost: any
    totalWeight: number
  }> =>
    invoke('calculate_nutrition', { recipeId })
}

export const presetApi = {
  getAll: (): Promise<NutritionPreset[]> =>
    invoke('get_nutrition_presets'),

  getById: (id: number): Promise<NutritionPreset> =>
    invoke('get_nutrition_preset', { id }),

  create: (data: Omit<NutritionPreset, 'id' | 'created_at' | 'updated_at'>): Promise<NutritionPreset> =>
    invoke('create_nutrition_preset', { data }),

  update: (id: number, data: any): Promise<NutritionPreset> =>
    invoke('update_nutrition_preset', { id, data }),

  delete: (id: number): Promise<void> =>
    invoke('delete_nutrition_preset', { id }),

  setRecipePreset: (recipeId: number, presetId: number | null): Promise<void> =>
    invoke('set_recipe_nutrition_preset', { recipeId, presetId }),

  getRecipePresetId: (recipeId: number): Promise<number | null> =>
    invoke('get_recipe_preset_id', { recipeId }),
}

export const versionApi = {
  createSnapshot: (recipeId: number, summary: string): Promise<RecipeVersion> =>
    invoke('create_version_snapshot', { recipeId, summary }),

  getAll: (recipeId: number): Promise<RecipeVersion[]> =>
    invoke('get_recipe_versions', { recipeId }),

  getDiff: (
    versionId: number,
    previousVersionId: number | null,
    versionsJson: string
  ): Promise<VersionDiffSummary> =>
    invoke('get_version_diff', { versionId, previousVersionId, versionsJson }),

  rollback: (versionId: number): Promise<Recipe> =>
    invoke('rollback_to_version', { versionId }),
}

export const templateApi = {
  getAll: (keyword?: string, tag?: string): Promise<RecipeTemplate[]> =>
    invoke('get_templates', { keyword, tag }),

  getById: (id: number): Promise<RecipeTemplate> =>
    invoke('get_template', { id }),

  create: (data: {
    name: string
    description: string
    category: string
    servings: number
    notes: string
    recipe_id: number
    tags: string[]
    nutrition_preset_id: number | null
  }): Promise<RecipeTemplate> =>
    invoke('create_template', { data }),

  update: (id: number, data: any): Promise<RecipeTemplate> =>
    invoke('update_template', { id, data }),

  delete: (id: number): Promise<void> =>
    invoke('delete_template', { id }),

  getAllTags: (): Promise<string[]> =>
    invoke('get_all_template_tags'),

  createRecipeFromTemplate: (templateId: number, newRecipeName: string): Promise<Recipe> =>
    invoke('create_recipe_from_template', { templateId, newRecipeName }),
}

export const exchangeApi = {
  exportJson: (recipeId: number): Promise<string> =>
    invoke('export_recipe_json', { recipeId }),

  previewImport: (jsonContent: string): Promise<ImportPreview> =>
    invoke('preview_import_json', { jsonContent }),

  executeImport: (jsonContent: string, confirmedItems: ImportConfirmedItem[]): Promise<ImportResult> =>
    invoke('execute_import_json', { jsonContent, confirmedItems }),
}
