<template>
  <div class="recipes-page">
    <div class="toolbar">
      <div class="toolbar-left">
        <el-select
          v-model="selectedCategory"
          placeholder="选择分类"
          clearable
          style="width: 140px; margin-right: 12px;"
          @change="handleFilterChange"
        >
          <el-option
            v-for="cat in RECIPE_CATEGORIES"
            :key="cat"
            :label="cat"
            :value="cat"
          />
        </el-select>
        <el-radio-group v-model="sortBy" @change="handleFilterChange">
          <el-radio-button value="recent">最近编辑</el-radio-button>
          <el-radio-button value="name">名称排序</el-radio-button>
        </el-radio-group>
        <el-button
          :type="onlyFavorites ? 'warning' : 'default'"
          @click="toggleFavorites"
        >
          <el-icon :class="{ active: onlyFavorites }"><Star /></el-icon>
          {{ onlyFavorites ? '已收藏' : '全部' }}
        </el-button>
      </div>
      <div class="toolbar-right">
        <el-button type="primary" @click="handleCreate">
          <el-icon><Plus /></el-icon>
          新建配方
        </el-button>
      </div>
    </div>

    <div class="content" v-loading="recipeStore.loading">
      <el-empty
        v-if="Object.keys(recipeStore.groupedRecipes).length === 0"
        description="暂无配方，点击右上角新建一个吧"
      />
      
      <template v-else>
        <div v-for="(recipes, category) in recipeStore.groupedRecipes" :key="category" class="category-group">
          <h3 class="category-title">
            <el-icon><Food /></el-icon>
            {{ category }}
            <span class="count">({{ recipes.length }})</span>
          </h3>
          <div class="recipe-grid">
            <div
              v-for="recipe in recipes"
              :key="recipe.id"
              class="recipe-card card-hover"
              @click="handleEdit(recipe.id)"
            >
              <div class="card-header">
                <h4 class="recipe-name">{{ recipe.name }}</h4>
                <el-icon
                  class="favorite-btn"
                  :class="{ active: recipe.is_favorite }"
                  @click.stop="handleToggleFavorite(recipe.id)"
                >
                  <StarFilled v-if="recipe.is_favorite" />
                  <Star v-else />
                </el-icon>
              </div>
              <div class="card-body">
                <div class="info-row">
                  <span class="label">份数:</span>
                  <span class="value">{{ recipe.servings }} 份</span>
                </div>
                <div class="info-row">
                  <span class="label">食材:</span>
                  <span class="value">{{ recipe.ingredients.length }} 种</span>
                </div>
                <div class="nutrition-preview" v-if="getRecipePreview(recipe.id)">
                  <div class="nutrition-item">
                    <span class="nutrition-label">热量</span>
                    <span class="nutrition-value">
                      {{ getRecipePreview(recipe.id)?.perServing.calories.toFixed(0) }}
                      <span class="unit">kcal</span>
                    </span>
                  </div>
                  <div class="nutrition-item">
                    <span class="nutrition-label">蛋白质</span>
                    <span class="nutrition-value">
                      {{ getRecipePreview(recipe.id)?.perServing.protein.toFixed(1) }}
                      <span class="unit">g</span>
                    </span>
                  </div>
                </div>
              </div>
              <div class="card-footer">
                <span class="update-time">
                  更新于 {{ formatDate(recipe.updated_at) }}
                </span>
                <div class="card-actions">
                  <el-button
                    type="primary"
                    link
                    size="small"
                    @click.stop="handleDuplicate(recipe)"
                  >
                    复制
                  </el-button>
                  <el-button
                    type="danger"
                    link
                    size="small"
                    @click.stop="handleDelete(recipe)"
                  >
                    删除
                  </el-button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </template>
    </div>

    <el-dialog
      v-model="createDialogVisible"
      title="新建配方"
      width="480px"
    >
      <el-form
        ref="createFormRef"
        :model="createFormData"
        :rules="createFormRules"
        label-width="80px"
      >
        <el-form-item label="配方名称" prop="name">
          <el-input v-model="createFormData.name" placeholder="请输入配方名称" />
        </el-form-item>
        <el-form-item label="分类" prop="category">
          <el-select v-model="createFormData.category" style="width: 100%;">
            <el-option
              v-for="cat in RECIPE_CATEGORIES"
              :key="cat"
              :label="cat"
              :value="cat"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="制作份数">
          <el-input-number v-model="createFormData.servings" :min="1" :max="100" />
        </el-form-item>
        <el-form-item label="备注">
          <el-input
            v-model="createFormData.notes"
            type="textarea"
            :rows="3"
            placeholder="可选：填写制作说明等"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="createDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleCreateSubmit">创建并编辑</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus'
import { Plus, Star, StarFilled, Food } from '@element-plus/icons-vue'
import { useRecipeStore } from '@/stores/recipe'
import { useIngredientStore } from '@/stores/ingredient'
import { RECIPE_CATEGORIES } from '@/types'
import type { Recipe } from '@/types'
import { calculateTotalNutrition, calculatePerServing } from '@/utils/nutrition'

const router = useRouter()
const recipeStore = useRecipeStore()
const ingredientStore = useIngredientStore()

const selectedCategory = ref<string | undefined>(undefined)
const sortBy = ref<'recent' | 'name'>('recent')
const onlyFavorites = ref(false)
const createDialogVisible = ref(false)
const createFormRef = ref<FormInstance>()

const createFormData = reactive({
  name: '',
  category: '',
  servings: 1,
  notes: ''
})

const createFormRules: FormRules = {
  name: [{ required: true, message: '请输入配方名称', trigger: 'blur' }],
  category: [{ required: true, message: '请选择分类', trigger: 'change' }]
}

const recipePreviews = ref<Map<number, any>>(new Map())

onMounted(async () => {
  await ingredientStore.loadIngredients()
  await loadRecipes()
})

async function loadRecipes() {
  recipeStore.setCategory(selectedCategory.value)
  recipeStore.setSortBy(sortBy.value)
  recipeStore.setOnlyFavorites(onlyFavorites.value)
  await recipeStore.loadRecipes()
  precomputePreviews()
}

function precomputePreviews() {
  recipePreviews.value.clear()
  for (const recipe of recipeStore.recipes) {
    const ingredients = recipe.ingredients.map(ri => ({
      ...ri,
      ingredient: ingredientStore.ingredients.find(i => i.id === ri.ingredient_id)
    }))
    const total = calculateTotalNutrition(ingredients)
    const perServing = calculatePerServing(total, recipe.servings)
    recipePreviews.value.set(recipe.id, { total, perServing })
  }
}

function getRecipePreview(recipeId: number) {
  return recipePreviews.value.get(recipeId)
}

function handleFilterChange() {
  loadRecipes()
}

function toggleFavorites() {
  onlyFavorites.value = !onlyFavorites.value
  loadRecipes()
}

function handleCreate() {
  createFormData.name = ''
  createFormData.category = ''
  createFormData.servings = 1
  createFormData.notes = ''
  createDialogVisible.value = true
}

async function handleCreateSubmit() {
  if (!createFormRef.value) return
  
  await createFormRef.value.validate(async (valid) => {
    if (valid) {
      try {
        const newRecipe = await recipeStore.createRecipe(createFormData)
        createDialogVisible.value = false
        router.push(`/recipes/${newRecipe.id}`)
      } catch (error) {
        ElMessage.error('创建失败')
      }
    }
  })
}

function handleEdit(id: number) {
  router.push(`/recipes/${id}`)
}

async function handleToggleFavorite(id: number) {
  try {
    await recipeStore.toggleFavorite(id)
    ElMessage.success('操作成功')
  } catch (error) {
    ElMessage.error('操作失败')
  }
}

async function handleDuplicate(recipe: Recipe) {
  try {
    const newName = `${recipe.name} - 副本`
    const newRecipe = await recipeStore.duplicateRecipe(recipe.id, newName)
    ElMessage.success('复制成功')
    loadRecipes()
  } catch (error) {
    ElMessage.error('复制失败')
  }
}

async function handleDelete(recipe: Recipe) {
  try {
    await ElMessageBox.confirm(
      `确定要删除配方"${recipe.name}"吗？此操作不可恢复。`,
      '确认删除',
      { type: 'warning' }
    )
    await recipeStore.deleteRecipe(recipe.id)
    ElMessage.success('删除成功')
    precomputePreviews()
  } catch (error) {
    // 用户取消
  }
}

function formatDate(dateStr: string) {
  const date = new Date(dateStr)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  
  if (diff < 60000) return '刚刚'
  if (diff < 3600000) return `${Math.floor(diff / 60000)} 分钟前`
  if (diff < 86400000) return `${Math.floor(diff / 3600000)} 小时前`
  if (diff < 604800000) return `${Math.floor(diff / 86400000)} 天前`
  
  return `${date.getMonth() + 1}/${date.getDate()}`
}
</script>

<style scoped lang="scss">
.recipes-page {
  .toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
    padding: 16px;
    background: #fff;
    border-radius: 8px;
    
    .toolbar-left, .toolbar-right {
      display: flex;
      align-items: center;
      gap: 12px;
    }
  }
  
  .content {
    .category-group {
      margin-bottom: 32px;
      
      .category-title {
        display: flex;
        align-items: center;
        gap: 8px;
        font-size: 18px;
        font-weight: 600;
        color: #303133;
        margin-bottom: 16px;
        
        .count {
          font-size: 14px;
          font-weight: 400;
          color: #909399;
        }
      }
    }
    
    .recipe-grid {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
      gap: 16px;
    }
    
    .recipe-card {
      background: #fff;
      border-radius: 8px;
      padding: 20px;
      cursor: pointer;
      border: 1px solid #ebeef5;
      
      .card-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        margin-bottom: 16px;
        
        .recipe-name {
          margin: 0;
          font-size: 16px;
          font-weight: 600;
          color: #303133;
          flex: 1;
        }
      }
      
      .card-body {
        margin-bottom: 16px;
        
        .info-row {
          display: flex;
          justify-content: space-between;
          margin-bottom: 8px;
          font-size: 13px;
          
          .label {
            color: #909399;
          }
          
          .value {
            color: #303133;
            font-weight: 500;
          }
        }
        
        .nutrition-preview {
          display: flex;
          gap: 24px;
          margin-top: 12px;
          padding-top: 12px;
          border-top: 1px dashed #ebeef5;
          
          .nutrition-item {
            .nutrition-label {
              display: block;
              font-size: 12px;
              color: #909399;
              margin-bottom: 4px;
            }
            
            .nutrition-value {
              font-size: 16px;
              font-weight: 600;
              color: #409eff;
              
              .unit {
                font-size: 12px;
                font-weight: 400;
                color: #909399;
                margin-left: 2px;
              }
            }
          }
        }
      }
      
      .card-footer {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding-top: 12px;
        border-top: 1px solid #f0f0f0;
        
        .update-time {
          font-size: 12px;
          color: #c0c4cc;
        }
        
        .card-actions {
          display: flex;
          gap: 8px;
        }
      }
    }
  }
}
</style>
