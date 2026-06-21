export interface Ingredient {
  id: number
  name: string
  category: string
  calories: number
  protein: number
  fat: number
  carbs: number
  fiber: number
  sodium: number
  calcium: number
  iron: number
  vitamin_a: number
  vitamin_c: number
  price_per_kg: number | null
  is_custom: boolean
  created_at: string
  updated_at: string
}

export interface RecipeIngredient {
  id: number
  recipe_id: number
  ingredient_id: number
  amount: number
  ingredient?: Ingredient
}

export interface Recipe {
  id: number
  name: string
  category: string
  servings: number
  is_favorite: boolean
  notes: string
  created_at: string
  updated_at: string
  ingredients: RecipeIngredient[]
}

export interface NutritionSummary {
  calories: number
  calories_kj: number
  protein: number
  fat: number
  carbs: number
  fiber: number
  sodium: number
  calcium: number
  iron: number
  vitamin_a: number
  vitamin_c: number
}

export interface CostSummary {
  total: number
  per_serving: number
}

export interface DRIReference {
  calories: number
  protein: number
  fat: number
  carbs: number
  fiber: number
  sodium: number
  calcium: number
  iron: number
  vitamin_a: number
  vitamin_c: number
}

export interface NRVReference {
  energy: number
  protein: number
  fat: number
  carbs: number
  sodium: number
}

export type IngredientCategory = 
  | '谷物' 
  | '肉类' 
  | '蔬菜' 
  | '水果' 
  | '调味料' 
  | '油脂' 
  | '乳制品' 
  | '水产' 
  | '豆制品'
  | '坚果'
  | '蛋类'
  | '其他'

export type RecipeCategory = 
  | '主食' 
  | '热菜' 
  | '凉菜' 
  | '汤品' 
  | '甜点' 
  | '饮品' 
  | '酱料'

export const INGREDIENT_CATEGORIES: IngredientCategory[] = [
  '谷物', '肉类', '蔬菜', '水果', '调味料', '油脂', '乳制品', '水产', '豆制品', '坚果', '蛋类', '其他'
]

export const RECIPE_CATEGORIES: RecipeCategory[] = [
  '主食', '热菜', '凉菜', '汤品', '甜点', '饮品', '酱料'
]

export const DRI_VALUES: DRIReference = {
  calories: 2000,
  protein: 60,
  fat: 65,
  carbs: 300,
  fiber: 25,
  sodium: 2000,
  calcium: 800,
  iron: 12,
  vitamin_a: 800,
  vitamin_c: 100
}

export const NRV_VALUES: NRVReference = {
  energy: 8400,
  protein: 60,
  fat: 60,
  carbs: 300,
  sodium: 2000
}

export const NUTRIENT_UNITS: Record<keyof NutritionSummary, string> = {
  calories: '千卡',
  calories_kj: '千焦',
  protein: '克',
  fat: '克',
  carbs: '克',
  fiber: '克',
  sodium: '毫克',
  calcium: '毫克',
  iron: '毫克',
  vitamin_a: '微克RAE',
  vitamin_c: '毫克'
}

export const NUTRIENT_LABELS: Record<keyof NutritionSummary, string> = {
  calories: '热量',
  calories_kj: '能量',
  protein: '蛋白质',
  fat: '脂肪',
  carbs: '碳水化合物',
  fiber: '膳食纤维',
  sodium: '钠',
  calcium: '钙',
  iron: '铁',
  vitamin_a: '维生素A',
  vitamin_c: '维生素C'
}
