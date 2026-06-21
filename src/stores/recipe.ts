import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Recipe, RecipeIngredient } from '@/types'
import { recipeApi } from '@/api/tauri'
import {
  calculateTotalNutrition,
  calculatePerServing,
  calculatePer100g,
  calculateDRIPercentage,
  calculateNRVPercentage,
  calculateCost,
  calculateTotalWeight,
  scaleRecipeByTarget,
  scaleRecipeByServings
} from '@/utils/nutrition'
import type { NutritionSummary } from '@/types'

export const useRecipeStore = defineStore('recipe', () => {
  const recipes = ref<Recipe[]>([])
  const currentRecipe = ref<Recipe | null>(null)
  const loading = ref(false)
  const currentCategory = ref<string | undefined>()
  const sortBy = ref<'recent' | 'name'>('recent')
  const onlyFavorites = ref(false)

  const filteredRecipes = computed(() => {
    let result = [...recipes.value]
    
    if (currentCategory.value) {
      result = result.filter(r => r.category === currentCategory.value)
    }
    
    if (onlyFavorites.value) {
      result = result.filter(r => r.is_favorite)
    }
    
    if (sortBy.value === 'recent') {
      result.sort((a, b) => new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime())
    } else {
      result.sort((a, b) => a.name.localeCompare(b.name, 'zh-CN'))
    }
    
    return result
  })

  const groupedRecipes = computed(() => {
    const groups: Record<string, Recipe[]> = {}
    for (const recipe of filteredRecipes.value) {
      if (!groups[recipe.category]) {
        groups[recipe.category] = []
      }
      groups[recipe.category].push(recipe)
    }
    return groups
  })

  const nutritionCalculations = computed(() => {
    if (!currentRecipe.value) return null
    
    const ingredients = currentRecipe.value.ingredients
    const servings = currentRecipe.value.servings
    
    const total = calculateTotalNutrition(ingredients)
    const perServing = calculatePerServing(total, servings)
    const totalWeight = calculateTotalWeight(ingredients)
    const per100g = calculatePer100g(total, totalWeight)
    const driComparison = calculateDRIPercentage(perServing)
    const nrvComparison = calculateNRVPercentage(per100g)
    const cost = calculateCost(ingredients, servings)
    
    return { total, perServing, per100g, driComparison, nrvComparison, cost, totalWeight }
  })

  async function loadRecipes() {
    loading.value = true
    try {
      recipes.value = await recipeApi.getAll(currentCategory.value, sortBy.value, onlyFavorites.value)
    } finally {
      loading.value = false
    }
  }

  async function loadRecipe(id: number) {
    loading.value = true
    try {
      currentRecipe.value = await recipeApi.getById(id)
      return currentRecipe.value
    } finally {
      loading.value = false
    }
  }

  async function createRecipe(data: any) {
    const newRecipe = await recipeApi.create(data)
    recipes.value.push(newRecipe)
    return newRecipe
  }

  async function updateRecipe(id: number, data: any) {
    const updated = await recipeApi.update(id, data)
    const index = recipes.value.findIndex(r => r.id === id)
    if (index !== -1) {
      recipes.value[index] = updated
    }
    if (currentRecipe.value?.id === id) {
      currentRecipe.value = { ...currentRecipe.value, ...updated }
    }
    return updated
  }

  async function deleteRecipe(id: number) {
    await recipeApi.delete(id)
    recipes.value = recipes.value.filter(r => r.id !== id)
    if (currentRecipe.value?.id === id) {
      currentRecipe.value = null
    }
  }

  async function toggleFavorite(id: number) {
    const updated = await recipeApi.toggleFavorite(id)
    const index = recipes.value.findIndex(r => r.id === id)
    if (index !== -1) {
      recipes.value[index] = updated
    }
    if (currentRecipe.value?.id === id) {
      currentRecipe.value.is_favorite = updated.is_favorite
    }
    return updated
  }

  async function duplicateRecipe(id: number, newName: string) {
    const newRecipe = await recipeApi.duplicate(id, newName)
    recipes.value.push(newRecipe)
    return newRecipe
  }

  async function addIngredient(recipeId: number, ingredientId: number, amount: number) {
    const ri = await recipeApi.addIngredient(recipeId, ingredientId, amount)
    if (currentRecipe.value?.id === recipeId) {
      const ingredient = (await import('./ingredient')).useIngredientStore().ingredients.find(i => i.id === ingredientId)
      currentRecipe.value.ingredients.push({ ...ri, ingredient })
    }
    return ri
  }

  async function updateIngredientAmount(recipeIngredientId: number, amount: number) {
    const ri = await recipeApi.updateIngredient(recipeIngredientId, amount)
    if (currentRecipe.value) {
      const index = currentRecipe.value.ingredients.findIndex(i => i.id === recipeIngredientId)
      if (index !== -1) {
        currentRecipe.value.ingredients[index].amount = amount
      }
    }
    return ri
  }

  async function removeIngredient(recipeIngredientId: number) {
    await recipeApi.removeIngredient(recipeIngredientId)
    if (currentRecipe.value) {
      currentRecipe.value.ingredients = currentRecipe.value.ingredients.filter(
        i => i.id !== recipeIngredientId
      )
    }
  }

  function scaleByServings(newServings: number) {
    if (!currentRecipe.value) return
    
    const originalServings = currentRecipe.value.servings
    currentRecipe.value.ingredients = scaleRecipeByServings(
      currentRecipe.value.ingredients,
      originalServings,
      newServings
    )
    currentRecipe.value.servings = newServings
  }

  function scaleByTargetNutrient(targetNutrient: keyof NutritionSummary, targetValue: number) {
    if (!currentRecipe.value || !nutritionCalculations.value) return
    
    currentRecipe.value.ingredients = scaleRecipeByTarget(
      currentRecipe.value.ingredients,
      targetNutrient,
      targetValue,
      nutritionCalculations.value.total
    )
  }

  function setCategory(category: string | undefined) {
    currentCategory.value = category
  }

  function setSortBy(sort: 'recent' | 'name') {
    sortBy.value = sort
  }

  function setOnlyFavorites(value: boolean) {
    onlyFavorites.value = value
  }

  function setCurrentRecipe(recipe: Recipe | null) {
    currentRecipe.value = recipe
  }

  return {
    recipes,
    currentRecipe,
    loading,
    currentCategory,
    sortBy,
    onlyFavorites,
    filteredRecipes,
    groupedRecipes,
    nutritionCalculations,
    loadRecipes,
    loadRecipe,
    createRecipe,
    updateRecipe,
    deleteRecipe,
    toggleFavorite,
    duplicateRecipe,
    addIngredient,
    updateIngredientAmount,
    removeIngredient,
    scaleByServings,
    scaleByTargetNutrient,
    setCategory,
    setSortBy,
    setOnlyFavorites,
    setCurrentRecipe
  }
})
