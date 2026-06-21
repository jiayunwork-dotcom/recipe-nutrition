<template>
  <div class="recipe-detail-page">
    <el-row :gutter="24">
      <el-col :span="16">
        <div class="page-container">
          <div class="recipe-header">
            <div class="recipe-basic">
              <el-input
              v-model="recipeForm.name"
              placeholder="配方名称"
              class="recipe-name-input"
              @change="saveBasicInfo"
            />
            </div>
            <div class="recipe-meta">
              <el-select
                v-model="recipeForm.category"
                placeholder="选择分类"
                @change="saveBasicInfo"
              >
                <el-option
                  v-for="cat in RECIPE_CATEGORIES"
                  :key="cat"
                  :label="cat"
                  :value="cat"
                />
              </el-select>
              <div class="servings-control">
                <span class="label">制作份数:</span>
                <el-input-number
                  v-model="recipeForm.servings"
                  :min="1"
                  :max="100"
                  @change="handleServingsChange"
                />
              </div>
              <el-icon
                class="favorite-btn large"
                :class="{ active: currentRecipe?.is_favorite }"
                @click="toggleFavorite"
              >
                <StarFilled v-if="currentRecipe?.is_favorite" />
                <Star v-else />
              </el-icon>
            </div>
          </div>

          <div class="scale-section">
            <div class="section-title">配方缩放</div>
            <div class="scale-options">
              <div class="scale-option">
                <span class="label">目标缩放：</span>
                <el-select v-model="targetNutrient" style="width: 140px; margin-right: 12px;">
                  <el-option label="蛋白质" value="protein" />
                  <el-option label="热量" value="calories" />
                  <el-option label="脂肪" value="fat" />
                  <el-option label="碳水化合物" value="carbs" />
                  <el-option label="膳食纤维" value="fiber" />
                </el-select>
                <el-input-number
                  v-model="targetValue"
                  :min="0"
                  :precision="1"
                  placeholder="目标值"
                  style="width: 140px; margin-right: 12px;"
                />
                <span style="margin-right: 12px;">
                  {{ getNutrientUnit(targetNutrient) }}
                </span>
                <el-button type="primary" @click="applyTargetScale">
                  <el-icon><RefreshRight /></el-icon>
                  应用缩放
                </el-button>
              </div>
            </div>
          </div>

          <div class="ingredients-section">
            <div class="section-header">
              <div class="section-title">食材清单</div>
              <el-button type="primary" @click="openAddIngredientDialog">
                <el-icon><Plus /></el-icon>
                添加食材
              </el-button>
            </div>
            <el-table
              ref="ingredientTableRef"
              :data="currentRecipe?.ingredients || []"
              stripe
              highlight-current-row
              @row-click="handleIngredientRowClick"
              style="width: 100%; cursor: pointer;"
            >
              <el-table-column prop="ingredient.name" label="食材" min-width="120">
                <template #default="{ row }">
                  <div class="ingredient-name">
                    <el-tag size="small" style="margin-right: 8px;">
                      {{ row.ingredient?.category }}
                    </el-tag>
                    {{ row.ingredient?.name }}
                  </div>
                </template>
              </el-table-column>
              <el-table-column label="用量" width="180">
                <template #default="{ row }">
                  <div class="ingredient-amount">
                    <el-input-number
                      v-model="row.amount"
                      :min="0.1"
                      :precision="1"
                      @change="(val: number) => updateIngredientAmount(row.id, val)"
                    />
                    <span class="unit">克</span>
                  </div>
                </template>
              </el-table-column>
              <el-table-column label="每100g营养" width="200">
                <template #default="{ row }">
                  <div class="mini-nutrition">
                    <span>热: {{ row.ingredient?.calories.toFixed(0) }}kcal</span>
                    <span>蛋: {{ row.ingredient?.protein.toFixed(1) }}g</span>
                  </div>
                </template>
              </el-table-column>
              <el-table-column label="贡献热量" width="120">
                <template #default="{ row }">
                  {{ calculateIngredientNutrition(row).calories.toFixed(0) }} kcal
                </template>
              </el-table-column>
              <el-table-column label="成本" width="100">
                <template #default="{ row }">
                  <span v-if="row.ingredient?.price_per_kg">
                    ¥{{ calculateIngredientCost(row).toFixed(2) }}
                  </span>
                  <span v-else style="color: #c0c4cc;">-</span>
                </template>
              </el-table-column>
              <el-table-column label="操作" width="80" fixed="right">
                <template #default="{ row }">
                  <el-button
                    type="danger"
                    link
                    @click="removeIngredient(row.id)"
                  >
                    <el-icon><Delete /></el-icon>
                  </el-button>
                </template>
              </el-table-column>
            </el-table>
            <div class="ingredients-summary">
              <div class="summary-item">
                <span class="label">总重量:</span>
                <span class="value">{{ nutrition?.totalWeight.toFixed(1) }} 克</span>
              </div>
              <div class="summary-item">
                <span class="label">总成本:</span>
                <span class="value cost">¥{{ nutrition?.cost.total.toFixed(2) }}</span>
              </div>
              <div class="summary-item">
                <span class="label">每份成本:</span>
                <span class="value cost">¥{{ nutrition?.cost.per_serving.toFixed(2) }}</span>
              </div>
            </div>
          </div>

          <div class="notes-section">
            <div class="section-title">备注</div>
            <el-input
              v-model="recipeForm.notes"
              type="textarea"
              :rows="3"
              placeholder="添加制作说明或备注..."
              @change="saveBasicInfo"
            />
          </div>
        </div>
      </el-col>

      <el-col :span="8">
        <IngredientReplacePanel
          :selected-ingredient="selectedIngredientForReplace"
          :amount="selectedIngredientAmount"
          :all-ingredients="ingredientStore.ingredients"
          @replace="handleIngredientReplace"
          style="margin-bottom: 20px;"
        />

        <div class="nutrition-panel">
          <el-tabs v-model="activeTab">
            <el-tab-pane label="营养分析" name="analysis">
              <div class="nutrition-summary">
                <div class="tab-subtitle">每份营养成分 ({{ recipeForm.servings }} 份中的1份)</div>
                <div class="nutrition-grid">
                  <div class="nutrition-card calories">
                    <div class="label">热量</div>
                    <div class="value">
                      {{ nutrition?.perServing.calories.toFixed(0) }}
                      <span class="unit">kcal</span>
                    </div>
                    <div class="sub-value">{{ nutrition?.perServing.calories_kj.toFixed(0) }} kJ</div>
                  </div>
                  <div class="nutrition-card protein">
                    <div class="label">蛋白质</div>
                    <div class="value">
                      {{ nutrition?.perServing.protein.toFixed(1) }}
                      <span class="unit">g</span>
                    </div>
                  </div>
                  <div class="nutrition-card fat">
                    <div class="label">脂肪</div>
                    <div class="value">
                      {{ nutrition?.perServing.fat.toFixed(1) }}
                      <span class="unit">g</span>
                    </div>
                  </div>
                  <div class="nutrition-card carbs">
                    <div class="label">碳水化合物</div>
                    <div class="value">
                      {{ nutrition?.perServing.carbs.toFixed(1) }}
                      <span class="unit">g</span>
                    </div>
                  </div>
                </div>

                <div class="nutrition-table">
                  <div class="nutrient-row" v-for="(item, key) in nutrition?.driComparison" :key="key">
                    <span class="name">{{ getNutrientLabel(key as string) }}</span>
                    <span class="value">{{ formatNutrientValue(key as string, item.value) }}</span>
                    <span class="percentage">{{ item.percentage.toFixed(0) }}%</span>
                    <span class="status-badge" :class="item.status">
                      {{ item.status === 'normal' ? '正常' : item.status === 'warning' ? '不足' : '超标' }}
                    </span>
                  </div>
                </div>

                <div class="per100g-section">
                  <div class="section-subtitle">每100克营养密度</div>
                  <div class="per100g-grid">
                    <div class="per100g-item">
                      <span class="label">热量</span>
                      <span class="value">{{ nutrition?.per100g.calories.toFixed(0) }} kcal</span>
                    </div>
                    <div class="per100g-item">
                      <span class="label">蛋白质</span>
                      <span class="value">{{ nutrition?.per100g.protein.toFixed(1) }} g</span>
                    </div>
                    <div class="per100g-item">
                      <span class="label">脂肪</span>
                      <span class="value">{{ nutrition?.per100g.fat.toFixed(1) }} g</span>
                    </div>
                    <div class="per100g-item">
                      <span class="label">碳水</span>
                      <span class="value">{{ nutrition?.per100g.carbs.toFixed(1) }} g</span>
                    </div>
                  </div>
                </div>
              </div>
            </el-tab-pane>

            <el-tab-pane label="DRI评估" name="dri">
              <div class="dri-section">
              <div class="section-subtitle">与中国居民膳食营养素参考摄入量对比</div>
              <DRIRadarChart
                v-if="nutrition?.driComparison"
                :driComparison="nutrition.driComparison"
              />
              <div class="dri-legend">
                <div class="legend-item">
                  <span class="legend-color normal"></span>
                  <span>正常 (60%-150%)</span>
                </div>
                <div class="legend-item">
                  <span class="legend-color warning"></span>
                  <span>不足 (<60%)</span>
                </div>
                <div class="legend-item">
                  <span class="legend-color excess"></span>
                  <span>超标 (>100%上限 / >150%)</span>
                </div>
              </div>
              </div>
            </el-tab-pane>

            <el-tab-pane label="营养标签" name="label">
              <div class="label-section">
                <div class="section-subtitle">食品包装营养成分表</div>
                <div class="label-preview">
                  <NutritionLabel
                    ref="nutritionLabelRef"
                    :nrvData="nutrition?.nrvComparison"
                    :servingSize="nutrition?.per100g ? 100 : 0"
                  />
                </div>
                <el-button
                  type="primary"
                  style="margin-top: 16px; width: 100%;"
                  @click="exportLabelImage"
                >
                  <el-icon><Download /></el-icon>
                  导出标签图片
                </el-button>
              </div>
            </el-tab-pane>
          </el-tabs>

          <div class="export-section">
            <div class="section-title">导出</div>
            <el-row :gutter="12">
              <el-col :span="12">
                <el-button type="success" style="width: 100%;" @click="exportPDF">
                  <el-icon><Document /></el-icon>
                  导出 PDF
                </el-button>
              </el-col>
              <el-col :span="12">
                <el-button type="warning" style="width: 100%;" @click="exportExcel">
                  <el-icon><Grid /></el-icon>
                  导出 Excel
                </el-button>
              </el-col>
            </el-row>
          </div>
        </div>
      </el-col>
    </el-row>

    <el-dialog
      v-model="addIngredientDialogVisible"
      title="添加食材"
      width="600px"
    >
      <div class="add-ingredient-toolbar">
        <el-input
          v-model="ingredientSearch"
          placeholder="搜索食材..."
          clearable
          style="width: 200px; margin-right: 12px;"
        >
          <template #prefix>
            <el-icon><Search /></el-icon>
          </template>
        </el-input>
        <el-select
          v-model="ingredientCategoryFilter"
          placeholder="分类筛选"
          clearable
          style="width: 140px;"
        >
          <el-option
            v-for="cat in ingredientStore.categories"
            :key="cat"
            :label="cat"
            :value="cat"
          />
        </el-select>
      </div>
      <el-table
        ref="ingredientTableRef"
        :data="filteredIngredientsForAdd"
        max-height="400"
        @row-click="handleIngredientSelect"
        highlight-current-row
        style="margin-top: 16px;"
      >
        <el-table-column prop="name" label="食材名称" min-width="120" />
        <el-table-column prop="category" label="分类" width="100" />
        <el-table-column label="每100g" width="200">
          <template #default="{ row }">
            热:{{ row.calories.toFixed(0) }}kcal 蛋:{{ row.protein.toFixed(1) }}g 脂:{{ row.fat.toFixed(1) }}g
          </template>
        </el-table-column>
      </el-table>
      <div class="amount-input-section">
        <span>用量:</span>
        <el-input-number
          v-model="newIngredientAmount"
          :min="0.1"
          :precision="1"
          placeholder="输入用量"
          style="width: 160px;"
        />
        <span class="unit">克</span>
      </div>
      <template #footer>
        <el-button @click="addIngredientDialogVisible = false">取消</el-button>
        <el-button type="primary" :disabled="!selectedIngredientForAdd || newIngredientAmount <= 0" @click="confirmAddIngredient">
          添加
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  Star, StarFilled, Plus, Delete, Search, Download, Document, Grid, RefreshRight } from '@element-plus/icons-vue'
import { useRecipeStore } from '@/stores/recipe'
import { useIngredientStore } from '@/stores/ingredient'
import { RECIPE_CATEGORIES, NUTRIENT_LABELS, NUTRIENT_UNITS, type NutritionSummary } from '@/types'
import DRIRadarChart from '@/components/DRIRadarChart.vue'
import NutritionLabel from '@/components/NutritionLabel.vue'
import IngredientReplacePanel from '@/components/IngredientReplacePanel.vue'
import type { RecipeIngredient, Ingredient } from '@/types'
import html2canvas from 'html2canvas'
import { save } from '@tauri-apps/plugin-dialog'
import { writeFile } from '@tauri-apps/plugin-fs'
import * as XLSX from 'xlsx'
import jsPDF from 'jspdf'

const route = useRoute()
const router = useRouter()
const recipeStore = useRecipeStore()
const ingredientStore = useIngredientStore()

const activeTab = ref('analysis')
const addIngredientDialogVisible = ref(false)
const nutritionLabelRef = ref<InstanceType<typeof NutritionLabel>>()
const ingredientTableRef = ref<any>(null)

const recipeForm = reactive({
  name: '',
  category: '',
  servings: 1,
  notes: ''
})

const ingredientSearch = ref('')
const ingredientCategoryFilter = ref<string | undefined>(undefined)
const selectedIngredientForAdd = ref<any>(null)
const newIngredientAmount = ref<number>(100)

const targetNutrient = ref<keyof NutritionSummary>('protein')
const targetValue = ref<number>(30)

const selectedIngredientForReplace = ref<Ingredient | null>(null)
const selectedIngredientAmount = ref<number>(0)
const selectedRecipeIngredientId = ref<number | null>(null)

const currentRecipe = computed(() => recipeStore.currentRecipe)
const nutrition = computed(() => recipeStore.nutritionCalculations)

const filteredIngredientsForAdd = computed(() => {
  let result = ingredientStore.ingredients
  
  if (ingredientCategoryFilter.value) {
    result = result.filter(i => i.category === ingredientCategoryFilter.value)
  }
  
  if (ingredientSearch.value) {
    const keyword = ingredientSearch.value.toLowerCase()
    result = result.filter(i => i.name.toLowerCase().includes(keyword))
  }
  
  return result.sort((a, b) => a.name.localeCompare(b.name, 'zh-CN'))
})

onMounted(async () => {
  await ingredientStore.loadIngredients()
  
  const recipeId = route.params.id
  if (recipeId && recipeId !== 'new') {
    await recipeStore.loadRecipe(Number(recipeId))
    if (currentRecipe.value) {
      recipeForm.name = currentRecipe.value.name
      recipeForm.category = currentRecipe.value.category
      recipeForm.servings = currentRecipe.value.servings
      recipeForm.notes = currentRecipe.value.notes
    }
  } else {
    router.replace('/recipes')
    ElMessage.error('请先创建配方')
  }
})

watch(
  () => currentRecipe.value,
  (newVal) => {
    if (newVal) {
      recipeForm.name = newVal.name
      recipeForm.category = newVal.category
      recipeForm.servings = newVal.servings
      recipeForm.notes = newVal.notes
    }
  },
  { deep: true }
)

watch(
  () => currentRecipe.value?.ingredients,
  () => {
    if (selectedRecipeIngredientId.value !== null && currentRecipe.value) {
      const ri = currentRecipe.value.ingredients.find(i => i.id === selectedRecipeIngredientId.value)
      if (ri) {
        selectedIngredientAmount.value = ri.amount
      }
    }
  },
  { deep: true }
)

async function saveBasicInfo() {
  if (!currentRecipe.value) return
  
  await recipeStore.updateRecipe(currentRecipe.value.id, {
    name: recipeForm.name,
    category: recipeForm.category,
    notes: recipeForm.notes
  })
}

async function handleServingsChange(newVal: number) {
  if (!currentRecipe.value) return
  
  const oldVal = currentRecipe.value.servings
  recipeStore.scaleByServings(newVal)
  
  for (const ri of currentRecipe.value.ingredients) {
    await recipeStore.updateIngredientAmount(ri.id, ri.amount)
  }
  
  await recipeStore.updateRecipe(currentRecipe.value.id, { servings: newVal })
}

async function toggleFavorite() {
  if (!currentRecipe.value) return
  await recipeStore.toggleFavorite(currentRecipe.value.id)
}

function openAddIngredientDialog() {
  ingredientSearch.value = ''
  ingredientCategoryFilter.value = undefined
  selectedIngredientForAdd.value = null
  newIngredientAmount.value = 100
  addIngredientDialogVisible.value = true
}

function handleIngredientSelect(row: any) {
  selectedIngredientForAdd.value = row
}

async function confirmAddIngredient() {
  if (!currentRecipe.value || !selectedIngredientForAdd.value || newIngredientAmount.value <= 0) return
  
  await recipeStore.addIngredient(
    currentRecipe.value.id,
    selectedIngredientForAdd.value.id,
    newIngredientAmount.value
  )
  
  addIngredientDialogVisible.value = false
  ElMessage.success('食材添加成功')
}

async function updateIngredientAmount(id: number, amount: number) {
  await recipeStore.updateIngredientAmount(id, amount)
}

async function removeIngredient(id: number) {
  try {
    await ElMessageBox.confirm(
      '确定要移除此食材吗？',
      '确认移除',
      { type: 'warning' }
    )
    await recipeStore.removeIngredient(id)
    
    if (selectedRecipeIngredientId.value === id) {
      selectedIngredientForReplace.value = null
      selectedIngredientAmount.value = 0
      selectedRecipeIngredientId.value = null
    }
    
    ElMessage.success('已移除')
  } catch (error) {
    // 用户取消
  }
}

function handleIngredientRowClick(row: RecipeIngredient) {
  if (row.ingredient) {
    selectedIngredientForReplace.value = row.ingredient
    selectedIngredientAmount.value = row.amount
    selectedRecipeIngredientId.value = row.id
  }
}

async function handleIngredientReplace(newIngredient: Ingredient) {
  if (!currentRecipe.value || selectedRecipeIngredientId.value === null) return

  const recipeIngredient = currentRecipe.value.ingredients.find(ri => ri.id === selectedRecipeIngredientId.value)
  if (!recipeIngredient) return

  const amount = recipeIngredient.amount
  
  await recipeStore.removeIngredient(selectedRecipeIngredientId.value)
  await recipeStore.addIngredient(currentRecipe.value.id, newIngredient.id, amount)

  selectedIngredientForReplace.value = newIngredient
  selectedIngredientAmount.value = amount

  const newRi = currentRecipe.value.ingredients.find(ri => ri.ingredient_id === newIngredient.id)
  if (newRi) {
    selectedRecipeIngredientId.value = newRi.id
  }
}

function calculateIngredientNutrition(row: RecipeIngredient) {
  if (!row.ingredient) return { calories: 0 }
  
  const factor = row.amount / 100
  return {
    calories: row.ingredient.calories * factor
  }
}

function calculateIngredientCost(row: RecipeIngredient) {
  if (row.ingredient?.price_per_kg) {
    return (row.amount / 1000) * row.ingredient.price_per_kg
  }
  return 0
}

function getNutrientLabel(key: string) {
  return NUTRIENT_LABELS[key as keyof typeof NUTRIENT_LABELS] || key
}

function getNutrientUnit(key: string) {
  return NUTRIENT_UNITS[key as keyof typeof NUTRIENT_UNITS] || ''
}

function formatNutrientValue(key: string, value: number) {
  const unit = getNutrientUnit(key)
  if (key === 'sodium' || key === 'calcium' || key === 'vitamin_a') {
    return `${value.toFixed(0)} ${unit}`
  }
  return `${value.toFixed(1)} ${unit}`
}

function applyTargetScale() {
  if (!nutrition.value) return
  
  recipeStore.scaleByTargetNutrient(targetNutrient.value, targetValue.value)
  
  if (currentRecipe.value) {
    for (const ri of currentRecipe.value.ingredients) {
      recipeStore.updateIngredientAmount(ri.id, ri.amount)
    }
  }
  
  ElMessage.success('已按目标值缩放')
}

function handleExportError(error: unknown, type: string) {
  console.error(`导出${type}失败:`, error)
  let message = `导出${type}失败`
  if (error instanceof Error) {
    const errMsg = error.message.toLowerCase()
    if (errMsg.includes('forbidden') || errMsg.includes('not allowed') || errMsg.includes('scope') || errMsg.includes('permission') || errMsg.includes('not permitted')) {
      message = '保存路径权限不足。请尝试保存到以下位置：桌面、文档、下载、图片、主目录下的文件夹'
    } else if (errMsg.includes('path') && (errMsg.includes('not found') || errMsg.includes('no such'))) {
      message = '保存路径不存在，请检查路径是否正确'
    } else if (errMsg.includes('denied') || errMsg.includes('permission denied')) {
      message = '权限被拒绝，请尝试保存到桌面或文档文件夹'
    } else if (errMsg.includes('cancelled') || errMsg.includes('canceled')) {
      return
    } else {
      message = `导出${type}失败: ${error.message}`
    }
  } else {
    message = `导出${type}失败: ${String(error)}`
  }
  ElMessage({
    message,
    type: 'error',
    duration: 5000,
    showClose: true
  })
}

async function exportLabelImage() {
  if (!nutritionLabelRef.value) {
    ElMessage.warning('营养标签未准备好')
    return
  }
  const element = nutritionLabelRef.value.getElement()
  if (!element) {
    ElMessage.warning('营养标签未准备好')
    return
  }
  try {
    const filePath = await save({
      defaultPath: `${recipeForm.name}_营养标签.png`,
      filters: [{ name: 'PNG', extensions: ['png'] }]
    })
    
    if (filePath) {
      const canvas = await html2canvas(element, {
        backgroundColor: '#ffffff',
        scale: 2,
        useCORS: true
      })
      const dataUrl = canvas.toDataURL('image/png')
      const base64Data = dataUrl.split(',')[1]
      const binaryData = Uint8Array.from(atob(base64Data), c => c.charCodeAt(0))
      await writeFile(filePath, binaryData)
      ElMessage.success('标签图片导出成功')
    }
  } catch (error) {
    handleExportError(error, '标签图片')
  }
}

async function exportPDF() {
  if (!currentRecipe.value || !nutrition.value) {
    ElMessage.warning('请先创建配方并添加食材')
    return
  }
  
  try {
    const filePath = await save({
      defaultPath: `${recipeForm.name}_配方.pdf`,
      filters: [{ name: 'PDF', extensions: ['pdf'] }]
    })
    
    if (filePath) {
      const doc = new jsPDF()
      
      doc.setFontSize(18)
      doc.text(recipeForm.name, 20, 20)
      
      doc.setFontSize(12)
      doc.text(`分类: ${recipeForm.category}`, 20, 35)
      doc.text(`份数: ${recipeForm.servings} 份`, 20, 45)
      doc.text(`总重量: ${nutrition.value.totalWeight.toFixed(1)} 克`, 20, 55)
      
      doc.setFontSize(14)
      doc.text('食材清单:', 20, 75)
      
      let y = 85
      doc.setFontSize(10)
      
      for (const ri of currentRecipe.value.ingredients) {
        if (ri.ingredient) {
          doc.text(`${ri.ingredient.name}: ${ri.amount} 克`, 25, y)
          y += 8
        }
      }
      
      y += 10
      doc.setFontSize(14)
      doc.text('营养成分 (每份):', 20, y)
      
      y += 10
      doc.setFontSize(10)
      
      const nutrients = [
        { name: '热量', value: `${nutrition.value.perServing.calories.toFixed(0)} kcal (${nutrition.value.perServing.calories_kj.toFixed(0)} kJ)` },
        { name: '蛋白质', value: `${nutrition.value.perServing.protein.toFixed(1)} g` },
        { name: '脂肪', value: `${nutrition.value.perServing.fat.toFixed(1)} g` },
        { name: '碳水化合物', value: `${nutrition.value.perServing.carbs.toFixed(1)} g` },
        { name: '膳食纤维', value: `${nutrition.value.perServing.fiber.toFixed(1)} g` },
        { name: '钠', value: `${nutrition.value.perServing.sodium.toFixed(0)} mg` },
        { name: '钙', value: `${nutrition.value.perServing.calcium.toFixed(0)} mg` },
        { name: '铁', value: `${nutrition.value.perServing.iron.toFixed(1)} mg` },
        { name: '维生素A', value: `${nutrition.value.perServing.vitamin_a.toFixed(0)} μgRAE` },
        { name: '维生素C', value: `${nutrition.value.perServing.vitamin_c.toFixed(1)} mg` },
      ]
      
      for (const n of nutrients) {
        doc.text(`${n.name}: ${n.value}`, 25, y)
        y += 7
      }
      
      if (recipeForm.notes) {
        y += 10
        doc.setFontSize(12)
        doc.text('备注:', 20, y)
        y += 8
        doc.setFontSize(10)
        doc.text(recipeForm.notes, 25, y, { maxWidth: 160 })
      }
      
      const pdfArrayBuffer = doc.output('arraybuffer')
      await writeFile(filePath, new Uint8Array(pdfArrayBuffer))
      ElMessage.success('PDF 导出成功')
    }
  } catch (error) {
    handleExportError(error, 'PDF')
  }
}

async function exportExcel() {
  if (!currentRecipe.value || !nutrition.value) {
    ElMessage.warning('请先创建配方并添加食材')
    return
  }
  
  try {
    const filePath = await save({
      defaultPath: `${recipeForm.name}_配方.xlsx`,
      filters: [{ name: 'Excel', extensions: ['xlsx'] }]
    })
    
    if (filePath) {
      const data = [
        ['配方名称', recipeForm.name],
        ['分类', recipeForm.category],
        ['份数', recipeForm.servings],
        ['总重量', `${nutrition.value.totalWeight.toFixed(1)} 克`],
        ['总成本', `¥${nutrition.value.cost.total.toFixed(2)}`],
        ['每份成本', `¥${nutrition.value.cost.per_serving.toFixed(2)}`],
        [],
        ['食材清单'],
        ['食材名称', '分类', '用量(克)', '单价(元/kg)', '成本(元)'],
        ...currentRecipe.value.ingredients.map(ri => [
          ri.ingredient?.name || '',
          ri.ingredient?.category || '',
          ri.amount,
          ri.ingredient?.price_per_kg || '',
          ri.ingredient?.price_per_kg ? ((ri.amount / 1000) * ri.ingredient.price_per_kg).toFixed(2) : ''
        ]),
        [],
        ['营养成分 (每份)'],
        ['营养素', '含量', '单位', '占DRI%'],
        ['热量', nutrition.value.perServing.calories.toFixed(0), 'kcal', `${(nutrition.value.perServing.calories / 2000 * 100).toFixed(0)}%`],
        ['能量', nutrition.value.perServing.calories_kj.toFixed(0), 'kJ', ''],
        ['蛋白质', nutrition.value.perServing.protein.toFixed(1), 'g', `${(nutrition.value.perServing.protein / 60 * 100).toFixed(0)}%`],
        ['脂肪', nutrition.value.perServing.fat.toFixed(1), 'g', `${(nutrition.value.perServing.fat / 65 * 100).toFixed(0)}%`],
        ['碳水化合物', nutrition.value.perServing.carbs.toFixed(1), 'g', `${(nutrition.value.perServing.carbs / 300 * 100).toFixed(0)}%`],
        ['膳食纤维', nutrition.value.perServing.fiber.toFixed(1), 'g', `${(nutrition.value.perServing.fiber / 25 * 100).toFixed(0)}%`],
        ['钠', nutrition.value.perServing.sodium.toFixed(0), 'mg', `${(nutrition.value.perServing.sodium / 2000 * 100).toFixed(0)}%`],
        ['钙', nutrition.value.perServing.calcium.toFixed(0), 'mg', `${(nutrition.value.perServing.calcium / 800 * 100).toFixed(0)}%`],
        ['铁', nutrition.value.perServing.iron.toFixed(1), 'mg', `${(nutrition.value.perServing.iron / 12 * 100).toFixed(0)}%`],
        ['维生素A', nutrition.value.perServing.vitamin_a.toFixed(0), 'μgRAE', `${(nutrition.value.perServing.vitamin_a / 800 * 100).toFixed(0)}%`],
        ['维生素C', nutrition.value.perServing.vitamin_c.toFixed(1), 'mg', `${(nutrition.value.perServing.vitamin_c / 100 * 100).toFixed(0)}%`],
      ]
      
      const ws = XLSX.utils.aoa_to_sheet(data)
      const wb = XLSX.utils.book_new()
      XLSX.utils.book_append_sheet(wb, ws, '配方')
      
      const excelBuffer = XLSX.write(wb, { bookType: 'xlsx', type: 'array' })
      await writeFile(filePath, new Uint8Array(excelBuffer))
      
      ElMessage.success('Excel 导出成功')
    }
  } catch (error) {
    handleExportError(error, 'Excel')
  }
}
</script>

<style scoped lang="scss">
.recipe-detail-page {
  .recipe-header {
    margin-bottom: 24px;
    
    .recipe-name-input {
      :deep(.el-input__inner) {
        font-size: 24px;
        font-weight: 600;
        height: 48px;
      }
    }
    
    .recipe-meta {
      display: flex;
      align-items: center;
      gap: 16px;
      margin-top: 16px;
      
      .label {
        margin-right: 8px;
        color: #606266;
      }
      
      .servings-control {
        display: flex;
        align-items: center;
        
        .label {
          color: #606266;
        }
      }
      
      .favorite-btn.large {
        font-size: 24px;
        margin-left: auto;
      }
    }
  }
  
  .scale-section {
    margin-bottom: 24px;
    
    .scale-options {
      .scale-option {
        display: flex;
        align-items: center;
        
        .label {
          margin-right: 8px;
          color: #606266;
        }
      }
    }
  }
  
  .ingredients-section {
    margin-bottom: 24px;
    
    .section-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 16px;
      
      .section-title {
        margin: 0;
        border: none;
        padding: 0;
      }
    }
    
    .ingredient-name {
      display: flex;
      align-items: center;
    }
    
    .ingredient-amount {
      display: flex;
      align-items: center;
      gap: 8px;
      
      .unit {
        color: #606266;
      }
    }
    
    .mini-nutrition {
      font-size: 12px;
      color: #606266;
      
      span {
        margin-right: 8px;
      }
    }
    
    .ingredients-summary {
      display: flex;
      gap: 32px;
      margin-top: 16px;
      padding-top: 16px;
      border-top: 1px solid #ebeef5;
      
      .summary-item {
        display: flex;
        align-items: center;
        gap: 8px;
        
        .label {
          color: #606266;
        }
        
        .value {
          font-weight: 600;
          color: #303133;
          
          &.cost {
            color: #f56c6c;
          }
        }
      }
    }
  }
  
  .notes-section {
    margin-bottom: 24px;
  }
  
  .nutrition-panel {
    background: #fff;
    border-radius: 8px;
    padding: 20px;
    box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.05);
    
    .tab-subtitle {
      font-size: 14px;
      color: #606266;
      margin-bottom: 16px;
    }
    
    .section-subtitle {
      font-size: 14px;
      font-weight: 600;
      color: #303133;
      margin: 16px 0;
      padding-bottom: 8px;
      border-bottom: 1px solid #ebeef5;
    }
    
    .nutrition-table {
      margin-top: 16px;
      
      .nutrient-row {
        display: flex;
        align-items: center;
        padding: 8px 0;
        border-bottom: 1px solid #f0f0f0;
        
        .name {
          flex: 1;
          color: #606266;
        }
        
        .value {
          width: 100px;
          text-align: right;
          font-weight: 500;
        }
        
        .percentage {
          width: 60px;
          text-align: right;
          color: #909399;
        }
      }
    }
    
    .per100g-section {
      margin-top: 20px;
      
      .per100g-grid {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 12px;
        
        .per100g-item {
          display: flex;
          justify-content: space-between;
          padding: 8px 12px;
          background: #f5f7fa;
          border-radius: 4px;
          
          .label {
            color: #606266;
          }
          
          .value {
            font-weight: 600;
            color: #409eff;
          }
        }
      }
    }
    
    .dri-section {
      .dri-legend {
        display: flex;
        justify-content: center;
        gap: 24px;
        margin-top: 16px;
        
        .legend-item {
          display: flex;
          align-items: center;
          gap: 8px;
          font-size: 12px;
          color: #606266;
          
          .legend-color {
            width: 12px;
            height: 12px;
            border-radius: 2px;
            
            &.normal { background: #67c23a; }
            &.warning { background: #e6a23c; }
            &.excess { background: #f56c6c; }
          }
        }
      }
    }
    
    .label-section {
      .label-preview {
        display: flex;
        justify-content: center;
        padding: 20px 0;
      }
    }
    
    .export-section {
      margin-top: 20px;
      padding-top: 20px;
      border-top: 1px solid #ebeef5;
      
      .section-title {
        margin-bottom: 16px;
        padding-bottom: 12px;
        border-bottom: 1px solid #ebeef5;
      }
    }
  }
  
  .add-ingredient-toolbar {
    display: flex;
    align-items: center;
  }
  
  .amount-input-section {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-top: 16px;
    padding-top: 16px;
    border-top: 1px solid #ebeef5;
    
    .unit {
      margin-left: 8px;
      color: #606266;
    }
  }
}
</style>
