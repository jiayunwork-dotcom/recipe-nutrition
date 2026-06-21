import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Ingredient } from '@/types'
import { ingredientApi } from '@/api/tauri'

export const useIngredientStore = defineStore('ingredient', () => {
  const ingredients = ref<Ingredient[]>([])
  const loading = ref(false)
  const currentCategory = ref<string | undefined>()
  const searchKeyword = ref('')

  const filteredIngredients = computed(() => {
    let result = [...ingredients.value]
    
    if (currentCategory.value) {
      result = result.filter(i => i.category === currentCategory.value)
    }
    
    if (searchKeyword.value) {
      const keyword = searchKeyword.value.toLowerCase()
      result = result.filter(i => i.name.toLowerCase().includes(keyword))
    }
    
    return result.sort((a, b) => a.name.localeCompare(b.name, 'zh-CN'))
  })

  const categories = computed(() => {
    const cats = new Set(ingredients.value.map(i => i.category))
    return Array.from(cats).sort((a, b) => a.localeCompare(b, 'zh-CN'))
  })

  async function loadIngredients(category?: string, keyword?: string) {
    loading.value = true
    try {
      ingredients.value = await ingredientApi.getAll(category, keyword)
      currentCategory.value = category
      searchKeyword.value = keyword || ''
    } finally {
      loading.value = false
    }
  }

  async function getIngredient(id: number) {
    return await ingredientApi.getById(id)
  }

  async function createIngredient(data: any) {
    const newIngredient = await ingredientApi.create(data)
    ingredients.value.push(newIngredient)
    return newIngredient
  }

  async function updateIngredient(id: number, data: any) {
    const updated = await ingredientApi.update(id, data)
    const index = ingredients.value.findIndex(i => i.id === id)
    if (index !== -1) {
      ingredients.value[index] = updated
    }
    return updated
  }

  async function deleteIngredient(id: number) {
    await ingredientApi.delete(id)
    ingredients.value = ingredients.value.filter(i => i.id !== id)
  }

  async function batchUpdatePrices(updates: Array<{ id: number; price_per_kg: number | null }>) {
    await ingredientApi.batchUpdatePrice(updates)
    for (const update of updates) {
      const ingredient = ingredients.value.find(i => i.id === update.id)
      if (ingredient) {
        ingredient.price_per_kg = update.price_per_kg
      }
    }
  }

  function setCategory(category: string | undefined) {
    currentCategory.value = category
  }

  function setSearchKeyword(keyword: string) {
    searchKeyword.value = keyword
  }

  return {
    ingredients,
    loading,
    currentCategory,
    searchKeyword,
    filteredIngredients,
    categories,
    loadIngredients,
    getIngredient,
    createIngredient,
    updateIngredient,
    deleteIngredient,
    batchUpdatePrices,
    setCategory,
    setSearchKeyword
  }
})
