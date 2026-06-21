<template>
  <div class="ingredients-page">
    <div class="toolbar">
      <div class="toolbar-left">
        <el-input
          v-model="searchKeyword"
          placeholder="搜索食材..."
          clearable
          style="width: 280px; margin-right: 16px;"
          @input="handleSearch"
        >
          <template #prefix>
            <el-icon><Search /></el-icon>
          </template>
        </el-input>
        <el-select
          v-model="selectedCategory"
          placeholder="选择分类"
          clearable
          style="width: 160px;"
          @change="handleCategoryChange"
        >
          <el-option
            v-for="cat in ingredientStore.categories"
            :key="cat"
            :label="cat"
            :value="cat"
          />
        </el-select>
      </div>
      <div class="toolbar-right">
        <el-button 
          type="success" 
          @click="openCompareDialog"
          :disabled="selectedForCompare.length < 2"
        >
          <el-icon><DataAnalysis /></el-icon>
          营养素对比 ({{ selectedForCompare.length }}/4)
        </el-button>
        <el-button type="primary" @click="openBatchPriceDialog">
          <el-icon><Money /></el-icon>
          批量设置价格
        </el-button>
        <el-button type="primary" @click="openAddDialog">
          <el-icon><Plus /></el-icon>
          添加食材
        </el-button>
      </div>
    </div>

    <div class="content">
      <el-table
        ref="ingredientTableRef"
        :data="ingredientStore.filteredIngredients"
        v-loading="ingredientStore.loading"
        stripe
        @selection-change="handleSelectionChange"
        style="width: 100%;"
      >
        <el-table-column type="selection" width="55" :selectable="isSelectable" />
        <el-table-column prop="name" label="食材名称" min-width="120">
          <template #default="{ row }">
            <span>{{ row.name }}</span>
            <el-tag v-if="row.is_custom" size="small" type="info" style="margin-left: 8px;">
              自定义
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="category" label="分类" width="100" />
        <el-table-column prop="calories" label="热量" width="80">
          <template #default="{ row }">
            {{ row.calories.toFixed(1) }}
            <span style="color: #909399; font-size: 12px;">kcal</span>
          </template>
        </el-table-column>
        <el-table-column prop="protein" label="蛋白质" width="80">
          <template #default="{ row }">{{ row.protein.toFixed(1) }}g</template>
        </el-table-column>
        <el-table-column prop="fat" label="脂肪" width="80">
          <template #default="{ row }">{{ row.fat.toFixed(1) }}g</template>
        </el-table-column>
        <el-table-column prop="carbs" label="碳水" width="80">
          <template #default="{ row }">{{ row.carbs.toFixed(1) }}g</template>
        </el-table-column>
        <el-table-column prop="fiber" label="膳食纤维" width="80">
          <template #default="{ row }">{{ row.fiber.toFixed(1) }}g</template>
        </el-table-column>
        <el-table-column prop="sodium" label="钠" width="80">
          <template #default="{ row }">{{ row.sodium.toFixed(0) }}mg</template>
        </el-table-column>
        <el-table-column prop="calcium" label="钙" width="80">
          <template #default="{ row }">{{ row.calcium.toFixed(0) }}mg</template>
        </el-table-column>
        <el-table-column prop="iron" label="铁" width="80">
          <template #default="{ row }">{{ row.iron.toFixed(1) }}mg</template>
        </el-table-column>
        <el-table-column prop="vitamin_a" label="维A" width="80">
          <template #default="{ row }">{{ row.vitamin_a.toFixed(0) }}μg</template>
        </el-table-column>
        <el-table-column prop="vitamin_c" label="维C" width="80">
          <template #default="{ row }">{{ row.vitamin_c.toFixed(1) }}mg</template>
        </el-table-column>
        <el-table-column prop="price_per_kg" label="单价" width="100">
          <template #default="{ row }">
            <span v-if="row.price_per_kg !== null">
              ¥{{ row.price_per_kg.toFixed(2) }}/kg
            </span>
            <span v-else style="color: #c0c4cc;">-</span>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="140" fixed="right">
          <template #default="{ row }">
            <el-button type="primary" link @click="openEditDialog(row)">编辑</el-button>
            <el-button 
              type="danger" 
              link 
              @click="handleDelete(row)"
              :disabled="!row.is_custom"
            >
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <el-dialog
      v-model="dialogVisible"
      :title="isEdit ? '编辑食材' : '添加食材'"
      width="600px"
    >
      <el-form
        ref="formRef"
        :model="formData"
        :rules="formRules"
        label-width="100px"
      >
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="食材名称" prop="name">
              <el-input v-model="formData.name" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="分类" prop="category">
              <el-select v-model="formData.category" style="width: 100%;">
                <el-option
                  v-for="cat in INGREDIENT_CATEGORIES"
                  :key="cat"
                  :label="cat"
                  :value="cat"
                />
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>
        <div class="section-title">每100克营养成分</div>
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="热量" prop="calories">
              <el-input-number v-model="formData.calories" :min="0" :precision="1" style="width: 100%;" />
              <span style="color: #909399; font-size: 12px;">千卡</span>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="蛋白质" prop="protein">
              <el-input-number v-model="formData.protein" :min="0" :precision="1" style="width: 100%;" />
              <span style="color: #909399; font-size: 12px;">克</span>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="脂肪" prop="fat">
              <el-input-number v-model="formData.fat" :min="0" :precision="1" style="width: 100%;" />
              <span style="color: #909399; font-size: 12px;">克</span>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="碳水化合物" prop="carbs">
              <el-input-number v-model="formData.carbs" :min="0" :precision="1" style="width: 100%;" />
              <span style="color: #909399; font-size: 12px;">克</span>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="膳食纤维" prop="fiber">
              <el-input-number v-model="formData.fiber" :min="0" :precision="1" style="width: 100%;" />
              <span style="color: #909399; font-size: 12px;">克</span>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="钠" prop="sodium">
              <el-input-number v-model="formData.sodium" :min="0" :precision="1" style="width: 100%;" />
              <span style="color: #909399; font-size: 12px;">毫克</span>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="钙" prop="calcium">
              <el-input-number v-model="formData.calcium" :min="0" :precision="1" style="width: 100%;" />
              <span style="color: #909399; font-size: 12px;">毫克</span>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="铁" prop="iron">
              <el-input-number v-model="formData.iron" :min="0" :precision="2" style="width: 100%;" />
              <span style="color: #909399; font-size: 12px;">毫克</span>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="维生素A" prop="vitamin_a">
              <el-input-number v-model="formData.vitamin_a" :min="0" :precision="1" style="width: 100%;" />
              <span style="color: #909399; font-size: 12px;">微克RAE</span>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="维生素C" prop="vitamin_c">
              <el-input-number v-model="formData.vitamin_c" :min="0" :precision="1" style="width: 100%;" />
              <span style="color: #909399; font-size: 12px;">毫克</span>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="单价">
              <el-input-number v-model="formData.price_per_kg" :min="0" :precision="2" style="width: 100%;" :controls="false" />
              <span style="color: #909399; font-size: 12px;">元/千克</span>
            </el-form-item>
          </el-col>
        </el-row>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSubmit">确定</el-button>
      </template>
    </el-dialog>

    <NutritionCompareDialog
      v-model="compareDialogVisible"
      :ingredients="selectedForCompare"
      @remove="handleRemoveFromCompare"
    />

    <el-dialog
      v-model="batchPriceDialogVisible"
      title="批量设置食材价格"
      width="800px"
    >
      <el-table
        :data="priceUpdateList"
        style="width: 100%;"
        max-height="500px"
      >
        <el-table-column type="selection" width="55" />
        <el-table-column prop="name" label="食材名称" min-width="150" />
        <el-table-column prop="category" label="分类" width="100" />
        <el-table-column label="当前价格" width="120">
          <template #default="{ row }">
            <span v-if="row.current_price !== null">
              ¥{{ row.current_price.toFixed(2) }}/kg
            </span>
            <span v-else style="color: #c0c4cc;">未设置</span>
          </template>
        </el-table-column>
        <el-table-column label="新价格" width="200">
          <template #default="{ row }">
            <el-input-number
              v-model="row.new_price"
              :min="0"
              :precision="2"
              :controls="false"
              placeholder="输入价格"
              style="width: 100%;"
            />
          </template>
        </el-table-column>
      </el-table>
      <div style="margin-top: 16px;">
        <el-input
          v-model="batchPrice"
          placeholder="批量设置选中食材的价格"
          style="width: 200px; margin-right: 12px;"
        >
          <template #append>元/千克</template>
        </el-input>
        <el-button @click="applyBatchPrice">应用到选中</el-button>
      </div>
      <template #footer>
        <el-button @click="batchPriceDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleBatchPriceSubmit">确定</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus'
import { Search, Plus, Money, DataAnalysis } from '@element-plus/icons-vue'
import { useIngredientStore } from '@/stores/ingredient'
import { INGREDIENT_CATEGORIES } from '@/types'
import type { Ingredient } from '@/types'
import NutritionCompareDialog from '@/components/NutritionCompareDialog.vue'

const ingredientStore = useIngredientStore()

const searchKeyword = ref('')
const selectedCategory = ref<string | undefined>(undefined)
const dialogVisible = ref(false)
const batchPriceDialogVisible = ref(false)
const isEdit = ref(false)
const editId = ref<number | null>(null)
const formRef = ref<FormInstance>()
const batchPrice = ref<number | null>(null)
const ingredientTableRef = ref<any>(null)
const compareDialogVisible = ref(false)
const selectedForCompare = ref<Ingredient[]>([])

const formData = reactive({
  name: '',
  category: '',
  calories: 0,
  protein: 0,
  fat: 0,
  carbs: 0,
  fiber: 0,
  sodium: 0,
  calcium: 0,
  iron: 0,
  vitamin_a: 0,
  vitamin_c: 0,
  price_per_kg: null as number | null
})

const formRules: FormRules = {
  name: [{ required: true, message: '请输入食材名称', trigger: 'blur' }],
  category: [{ required: true, message: '请选择分类', trigger: 'change' }]
}

const priceUpdateList = ref<Array<{
  id: number
  name: string
  category: string
  current_price: number | null
  new_price: number | null
}>>([])

onMounted(() => {
  ingredientStore.loadIngredients()
})

function handleSearch() {
  ingredientStore.setSearchKeyword(searchKeyword.value)
}

function handleCategoryChange() {
  ingredientStore.setCategory(selectedCategory.value)
}

function handleSelectionChange(selection: Ingredient[]) {
  selectedForCompare.value = selection.slice(0, 4)
}

function isSelectable(row: Ingredient): boolean {
  if (selectedForCompare.value.length >= 4) {
    return selectedForCompare.value.some(s => s.id === row.id)
  }
  return true
}

function openCompareDialog() {
  if (selectedForCompare.value.length < 2) {
    ElMessage.warning('请至少选择2种食材进行对比')
    return
  }
  if (selectedForCompare.value.length > 4) {
    ElMessage.warning('最多只能选择4种食材进行对比')
    return
  }
  compareDialogVisible.value = true
}

function handleRemoveFromCompare(id: number) {
  selectedForCompare.value = selectedForCompare.value.filter(i => i.id !== id)
  
  if (ingredientTableRef.value) {
    const table = ingredientTableRef.value
    if (table.toggleRowSelection) {
      const row = ingredientStore.ingredients.find(i => i.id === id)
      if (row) {
        table.toggleRowSelection(row, false)
      }
    }
  }
}

function openAddDialog() {
  isEdit.value = false
  editId.value = null
  Object.assign(formData, {
    name: '',
    category: '',
    calories: 0,
    protein: 0,
    fat: 0,
    carbs: 0,
    fiber: 0,
    sodium: 0,
    calcium: 0,
    iron: 0,
    vitamin_a: 0,
    vitamin_c: 0,
    price_per_kg: null
  })
  dialogVisible.value = true
}

function openEditDialog(row: Ingredient) {
  isEdit.value = true
  editId.value = row.id
  Object.assign(formData, {
    name: row.name,
    category: row.category,
    calories: row.calories,
    protein: row.protein,
    fat: row.fat,
    carbs: row.carbs,
    fiber: row.fiber,
    sodium: row.sodium,
    calcium: row.calcium,
    iron: row.iron,
    vitamin_a: row.vitamin_a,
    vitamin_c: row.vitamin_c,
    price_per_kg: row.price_per_kg
  })
  dialogVisible.value = true
}

async function handleSubmit() {
  if (!formRef.value) return
  
  await formRef.value.validate(async (valid) => {
    if (valid) {
      try {
        if (isEdit.value && editId.value) {
          await ingredientStore.updateIngredient(editId.value, formData)
          ElMessage.success('食材更新成功')
        } else {
          await ingredientStore.createIngredient(formData)
          ElMessage.success('食材添加成功')
        }
        dialogVisible.value = false
      } catch (error) {
        ElMessage.error('操作失败')
      }
    }
  })
}

async function handleDelete(row: Ingredient) {
  try {
    await ElMessageBox.confirm(
      `确定要删除食材"${row.name}"吗？`,
      '确认删除',
      { type: 'warning' }
    )
    await ingredientStore.deleteIngredient(row.id)
    ElMessage.success('删除成功')
  } catch (error) {
    // 用户取消
  }
}

function openBatchPriceDialog() {
  priceUpdateList.value = ingredientStore.ingredients.map(i => ({
    id: i.id,
    name: i.name,
    category: i.category,
    current_price: i.price_per_kg,
    new_price: i.price_per_kg
  }))
  batchPriceDialogVisible.value = true
}

function applyBatchPrice() {
  if (batchPrice.value === null) return
  
  const selectedTable = document.querySelector('.el-table') as any
  const selection = selectedTable?.getSelection?.() || []
  
  if (selection.length === 0) {
    ElMessage.warning('请先选择食材')
    return
  }
  
  for (const item of priceUpdateList.value) {
    if (selection.some((s: any) => s.id === item.id)) {
      item.new_price = batchPrice.value
    }
  }
}

async function handleBatchPriceSubmit() {
  const updates = priceUpdateList.value
    .filter(item => item.new_price !== item.current_price)
    .map(item => ({
      id: item.id,
      price_per_kg: item.new_price
    }))
  
  if (updates.length === 0) {
    ElMessage.info('没有需要更新的价格')
    batchPriceDialogVisible.value = false
    return
  }
  
  try {
    await ingredientStore.batchUpdatePrices(updates)
    ElMessage.success(`成功更新 ${updates.length} 个食材价格`)
    batchPriceDialogVisible.value = false
  } catch (error) {
    ElMessage.error('批量更新失败')
  }
}
</script>

<style scoped lang="scss">
.ingredients-page {
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
    background: #fff;
    border-radius: 8px;
    padding: 20px;
  }
  
  .section-title {
    font-size: 14px;
    font-weight: 600;
    color: #303133;
    margin: 16px 0 12px;
    padding-bottom: 8px;
    border-bottom: 1px solid #ebeef5;
  }
}
</style>
