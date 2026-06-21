import type { Recipe, RecipeIngredient, NutritionSummary, CostSummary, DRIReference, NRVReference, Ingredient } from '@/types'
import { DRI_VALUES, NRV_VALUES } from '@/types'

export function calculateTotalNutrition(ingredients: RecipeIngredient[]): NutritionSummary {
  const total: NutritionSummary = {
    calories: 0,
    calories_kj: 0,
    protein: 0,
    fat: 0,
    carbs: 0,
    fiber: 0,
    sodium: 0,
    calcium: 0,
    iron: 0,
    vitamin_a: 0,
    vitamin_c: 0
  }

  for (const ri of ingredients) {
    if (!ri.ingredient) continue
    const factor = ri.amount / 100
    total.calories += ri.ingredient.calories * factor
    total.protein += ri.ingredient.protein * factor
    total.fat += ri.ingredient.fat * factor
    total.carbs += ri.ingredient.carbs * factor
    total.fiber += ri.ingredient.fiber * factor
    total.sodium += ri.ingredient.sodium * factor
    total.calcium += ri.ingredient.calcium * factor
    total.iron += ri.ingredient.iron * factor
    total.vitamin_a += ri.ingredient.vitamin_a * factor
    total.vitamin_c += ri.ingredient.vitamin_c * factor
  }

  total.calories_kj = Math.round(total.calories * 4.184)
  return total
}

export function calculatePerServing(total: NutritionSummary, servings: number): NutritionSummary {
  const result: NutritionSummary = { ...total }
  const keys = Object.keys(result) as (keyof NutritionSummary)[]
  for (const key of keys) {
    result[key] = result[key] / servings
  }
  return result
}

export function calculatePer100g(total: NutritionSummary, totalWeight: number): NutritionSummary {
  const result: NutritionSummary = { ...total }
  if (totalWeight <= 0) return result
  
  const factor = 100 / totalWeight
  const keys = Object.keys(result) as (keyof NutritionSummary)[]
  for (const key of keys) {
    result[key] = result[key] * factor
  }
  return result
}

export function calculateTotalWeight(ingredients: RecipeIngredient[]): number {
  return ingredients.reduce((sum, ri) => sum + ri.amount, 0)
}

export function calculateDRIPercentage(nutrition: NutritionSummary, dri: DRIReference = DRI_VALUES): Record<string, { value: number; percentage: number; status: 'normal' | 'warning' | 'excess' }> {
  const result: Record<string, { value: number; percentage: number; status: 'normal' | 'warning' | 'excess' }> = {}
  
  const mapping: Array<{ key: keyof NutritionSummary; driKey: keyof DRIReference; isUpperLimit?: boolean }> = [
    { key: 'calories', driKey: 'calories' },
    { key: 'protein', driKey: 'protein' },
    { key: 'fat', driKey: 'fat', isUpperLimit: true },
    { key: 'carbs', driKey: 'carbs' },
    { key: 'fiber', driKey: 'fiber' },
    { key: 'sodium', driKey: 'sodium', isUpperLimit: true },
    { key: 'calcium', driKey: 'calcium' },
    { key: 'iron', driKey: 'iron' },
    { key: 'vitamin_a', driKey: 'vitamin_a' },
    { key: 'vitamin_c', driKey: 'vitamin_c' }
  ]

  for (const { key, driKey, isUpperLimit } of mapping) {
    const value = nutrition[key]
    const reference = dri[driKey]
    const percentage = (value / reference) * 100
    
    let status: 'normal' | 'warning' | 'excess' = 'normal'
    if (isUpperLimit) {
      if (percentage > 100) status = 'excess'
      else if (percentage < 60) status = 'warning'
    } else {
      if (percentage > 150) status = 'excess'
      else if (percentage < 60) status = 'warning'
    }

    result[key] = { value, percentage, status }
  }

  return result
}

export function calculateNRVPercentage(nutrition: NutritionSummary, nrv: NRVReference = NRV_VALUES): {
  energy: { value: number; nrv: number; percentage: number }
  protein: { value: number; nrv: number; percentage: number }
  fat: { value: number; nrv: number; percentage: number }
  carbs: { value: number; nrv: number; percentage: number }
  sodium: { value: number; nrv: number; percentage: number }
} {
  return {
    energy: {
      value: Math.round(nutrition.calories_kj),
      nrv: nrv.energy,
      percentage: Math.round((nutrition.calories_kj / nrv.energy) * 100)
    },
    protein: {
      value: Number(nutrition.protein.toFixed(1)),
      nrv: nrv.protein,
      percentage: Math.round((nutrition.protein / nrv.protein) * 100)
    },
    fat: {
      value: Number(nutrition.fat.toFixed(1)),
      nrv: nrv.fat,
      percentage: Math.round((nutrition.fat / nrv.fat) * 100)
    },
    carbs: {
      value: Number(nutrition.carbs.toFixed(1)),
      nrv: nrv.carbs,
      percentage: Math.round((nutrition.carbs / nrv.carbs) * 100)
    },
    sodium: {
      value: Number(nutrition.sodium.toFixed(1)),
      nrv: nrv.sodium,
      percentage: Math.round((nutrition.sodium / nrv.sodium) * 100)
    }
  }
}

export function calculateCost(ingredients: RecipeIngredient[], servings: number): CostSummary {
  let total = 0
  for (const ri of ingredients) {
    if (ri.ingredient?.price_per_kg) {
      total += (ri.amount / 1000) * ri.ingredient.price_per_kg
    }
  }
  return {
    total: Number(total.toFixed(2)),
    per_serving: Number((total / servings).toFixed(2))
  }
}

export function scaleRecipeByTarget(
  ingredients: RecipeIngredient[],
  targetNutrient: keyof NutritionSummary,
  targetValue: number,
  currentNutrition: NutritionSummary
): RecipeIngredient[] {
  const currentValue = currentNutrition[targetNutrient]
  if (currentValue <= 0) return ingredients
  
  const factor = targetValue / currentValue
  return ingredients.map(ri => ({
    ...ri,
    amount: Number((ri.amount * factor).toFixed(1))
  }))
}

export function scaleRecipeByServings(
  ingredients: RecipeIngredient[],
  originalServings: number,
  newServings: number
): RecipeIngredient[] {
  if (originalServings <= 0 || newServings <= 0) return ingredients
  
  const factor = newServings / originalServings
  return ingredients.map(ri => ({
    ...ri,
    amount: Number((ri.amount * factor).toFixed(1))
  }))
}

const NUTRIENT_KEYS_FOR_SIMILARITY = [
  'calories', 'protein', 'fat', 'carbs', 'fiber',
  'sodium', 'calcium', 'iron', 'vitamin_a', 'vitamin_c'
] as const

function getIngredientNutrientVector(ingredient: Ingredient): number[] {
  return NUTRIENT_KEYS_FOR_SIMILARITY.map(key => ingredient[key])
}

function getNutrientMaxValues(ingredients: Ingredient[]): number[] {
  const maxValues = new Array(NUTRIENT_KEYS_FOR_SIMILARITY.length).fill(0)
  
  for (const ingredient of ingredients) {
    const vector = getIngredientNutrientVector(ingredient)
    for (let i = 0; i < vector.length; i++) {
      if (vector[i] > maxValues[i]) {
        maxValues[i] = vector[i]
      }
    }
  }
  
  return maxValues.map(v => v === 0 ? 1 : v)
}

function normalizeVector(vector: number[], maxValues: number[]): number[] {
  return vector.map((v, i) => v / maxValues[i])
}

function euclideanDistance(v1: number[], v2: number[]): number {
  let sum = 0
  for (let i = 0; i < v1.length; i++) {
    const diff = v1[i] - v2[i]
    sum += diff * diff
  }
  return Math.sqrt(sum)
}

export interface SimilarIngredient {
  ingredient: Ingredient
  distance: number
  similarity: number
}

export function findSimilarIngredients(
  targetIngredient: Ingredient,
  allIngredients: Ingredient[],
  options: {
    sameCategory?: boolean
    limit?: number
  } = {}
): SimilarIngredient[] {
  const { sameCategory = true, limit = 5 } = options
  
  let candidates = allIngredients.filter(i => i.id !== targetIngredient.id)
  
  if (sameCategory) {
    candidates = candidates.filter(i => i.category === targetIngredient.category)
  }
  
  if (candidates.length === 0) return []
  
  const allForMax = [...candidates, targetIngredient]
  const maxValues = getNutrientMaxValues(allForMax)
  
  const targetVector = normalizeVector(getIngredientNutrientVector(targetIngredient), maxValues)
  
  const results: SimilarIngredient[] = candidates.map(ingredient => {
    const candidateVector = normalizeVector(getIngredientNutrientVector(ingredient), maxValues)
    const distance = euclideanDistance(targetVector, candidateVector)
    return { ingredient, distance, similarity: 0 }
  })
  
  results.sort((a, b) => a.distance - b.distance)
  
  const maxDistance = results.length > 0 ? results[results.length - 1].distance : 1
  const minDistance = results.length > 0 ? results[0].distance : 0
  
  for (const r of results) {
    if (maxDistance === minDistance) {
      r.similarity = 100
    } else {
      r.similarity = Math.round(((maxDistance - r.distance) / (maxDistance - minDistance)) * 100)
    }
  }
  
  if (results.length > 0) {
    results[0].similarity = 100
  }
  
  return results.slice(0, limit)
}

export function calculateReplacementNutritionChange(
  originalIngredient: Ingredient,
  newIngredient: Ingredient,
  amount: number
): {
  original: NutritionSummary
  new: NutritionSummary
  changes: Record<keyof NutritionSummary, { value: number; percentage: number; isIncrease: boolean }>
} {
  const factor = amount / 100
  
  const original: NutritionSummary = {
    calories: originalIngredient.calories * factor,
    calories_kj: Math.round(originalIngredient.calories * factor * 4.184),
    protein: originalIngredient.protein * factor,
    fat: originalIngredient.fat * factor,
    carbs: originalIngredient.carbs * factor,
    fiber: originalIngredient.fiber * factor,
    sodium: originalIngredient.sodium * factor,
    calcium: originalIngredient.calcium * factor,
    iron: originalIngredient.iron * factor,
    vitamin_a: originalIngredient.vitamin_a * factor,
    vitamin_c: originalIngredient.vitamin_c * factor
  }
  
  const newNutrition: NutritionSummary = {
    calories: newIngredient.calories * factor,
    calories_kj: Math.round(newIngredient.calories * factor * 4.184),
    protein: newIngredient.protein * factor,
    fat: newIngredient.fat * factor,
    carbs: newIngredient.carbs * factor,
    fiber: newIngredient.fiber * factor,
    sodium: newIngredient.sodium * factor,
    calcium: newIngredient.calcium * factor,
    iron: newIngredient.iron * factor,
    vitamin_a: newIngredient.vitamin_a * factor,
    vitamin_c: newIngredient.vitamin_c * factor
  }
  
  const changes = {} as Record<keyof NutritionSummary, { value: number; percentage: number; isIncrease: boolean }>
  
  const keys = Object.keys(original) as (keyof NutritionSummary)[]
  for (const key of keys) {
    const diff = newNutrition[key] - original[key]
    const percentage = original[key] === 0 ? 0 : (diff / original[key]) * 100
    changes[key] = {
      value: diff,
      percentage,
      isIncrease: diff > 0
    }
  }
  
  return { original, new: newNutrition, changes }
}
