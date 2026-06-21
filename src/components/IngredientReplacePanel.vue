<template>
  <div class="ingredient-replace-panel">
    <div class="panel-header">
      <div class="header-title">
        <el-icon><Refresh /></el-icon>
        <span>食材替换建议</span>
      </div>
      <el-switch
        v-model="crossCategory"
        active-text="跨分类"
        inactive-text="仅同分类"
        inline-prompt
        @change="loadSimilarIngredients"
      />
    </div>

    <div v-if="selectedIngredient" class="selected-info">
      <div class="selected-label">当前选中：</div>
      <div class="selected-name">
        <el-tag size="small" type="info">{{ selectedIngredient.category }}</el-tag>
        <span>{{ selectedIngredient.name }}</span>
        <span class="amount">({{ amount }} 克)</span>
      </div>
    </div>

    <div v-else class="empty-selection">
      <el-empty description="请选择一个食材查看替换建议" :image-size="80" />
    </div>

    <div v-if="selectedIngredient" class="candidates-list">
      <div v-if="loading" class="loading-state">
        <el-icon class="is-loading"><Loading /></el-icon>
        <span>正在分析...</span>
      </div>

      <div v-else-if="candidates.length === 0" class="no-candidates">
        <el-empty description="未找到合适的替换食材" :image-size="80" />
      </div>

      <div v-else class="candidate-item" v-for="(candidate, index) in candidates" :key="candidate.ingredient.id" @click="showConfirmDialog(candidate)">
        <div class="candidate-header">
          <div class="rank-badge" :class="`rank-${index + 1}`">{{ index + 1 }}</div>
          <div class="candidate-name">
            <span class="name">{{ candidate.ingredient.name }}</span>
            <el-tag v-if="crossCategory" size="small" type="info" class="category-tag">
              {{ candidate.ingredient.category }}
            </el-tag>
          </div>
          <div class="similarity">
            <span class="similarity-value">{{ candidate.similarity }}%</span>
            <span class="similarity-label">相似度</span>
          </div>
        </div>
        <div class="candidate-nutrition-preview">
          <div class="nutrition-change-item" v-for="(change, key) in getPreviewChanges(candidate)" :key="key">
            <span class="nutrient-name">{{ getNutrientShortLabel(key as string) }}</span>
            <span class="nutrient-change" :class="change.isIncrease ? 'increase' : 'decrease'" v-if="change.value !== 0">
              <el-icon v-if="change.isIncrease"><Top /></el-icon>
              <el-icon v-else><Bottom /></el-icon>
              {{ Math.abs(change.percentage).toFixed(0) }}%
            </span>
            <span class="nutrient-change no-change" v-else>-</span>
          </div>
        </div>
        <div class="replace-hint">
          <el-icon><Right /></el-icon>
          点击替换
        </div>
      </div>
    </div>

    <el-dialog
      v-model="confirmDialogVisible"
      title="确认替换食材"
      width="560px"
      :close-on-click-modal="false"
    >
      <div v-if="selectedCandidate && selectedIngredient" class="confirm-content">
        <div class="replace-header">
          <div class="replace-item">
            <div class="item-label">原食材</div>
            <div class="item-name">{{ selectedIngredient.name }}</div>
            <el-tag size="small" type="info">{{ selectedIngredient.category }}</el-tag>
          </div>
          <div class="replace-arrow">
            <el-icon><Right /></el-icon>
          </div>
          <div class="replace-item">
            <div class="item-label">替换为</div>
            <div class="item-name">{{ selectedCandidate.ingredient.name }}</div>
            <el-tag size="small" type="success">{{ selectedCandidate.ingredient.category }}</el-tag>
          </div>
        </div>

        <div class="amount-info">
          用量保持不变：<strong>{{ amount }} 克</strong>
        </div>

        <div class="compare-title">营养成分对比（该食材贡献）</div>
        <el-table :data="compareTableData" size="small" border>
          <el-table-column prop="nutrient" label="营养素" width="100" />
          <el-table-column prop="original" label="替换前" width="100" align="right">
            <template #default="{ row }">
              <span>{{ formatValue(row.nutrientKey, row.originalValue) }}</span>
            </template>
          </el-table-column>
          <el-table-column prop="new" label="替换后" width="100" align="right">
            <template #default="{ row }">
              <span>{{ formatValue(row.nutrientKey, row.newValue) }}</span>
            </template>
          </el-table-column>
          <el-table-column prop="change" label="变化" width="120" align="right">
            <template #default="{ row }">
              <span :class="row.changeObj.isIncrease ? 'increase' : 'decrease'">
                {{ row.changeObj.isIncrease ? '+' : '' }}{{ formatValue(row.nutrientKey, row.changeObj.value) }}
                ({{ row.changeObj.isIncrease ? '+' : '' }}{{ row.changeObj.percentage.toFixed(1) }}%)
              </span>
            </template>
          </el-table-column>
        </el-table>

        <div class="tip">
          <el-icon><InfoFilled /></el-icon>
          替换后将自动刷新配方的营养计算结果和雷达图
        </div>
      </div>

      <template #footer>
        <el-button @click="confirmDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="confirmReplace">确认替换</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { Refresh, Loading, Top, Bottom, Right, InfoFilled } from '@element-plus/icons-vue'
import type { Ingredient } from '@/types'
import { NUTRIENT_LABELS, NUTRIENT_UNITS } from '@/types'
import { findSimilarIngredients, calculateReplacementNutritionChange, type SimilarIngredient } from '@/utils/nutrition'

interface Props {
  selectedIngredient: Ingredient | null
  amount: number
  allIngredients: Ingredient[]
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'replace', newIngredient: Ingredient): void
}>()

const crossCategory = ref(false)
const loading = ref(false)
const candidates = ref<SimilarIngredient[]>([])
const confirmDialogVisible = ref(false)
const selectedCandidate = ref<SimilarIngredient | null>(null)

const PREVIEW_NUTRIENTS = ['calories', 'protein', 'fat', 'carbs']

function getNutrientShortLabel(key: string): string {
  const labels: Record<string, string> = {
    calories: '热量',
    protein: '蛋白质',
    fat: '脂肪',
    carbs: '碳水'
  }
  return labels[key] || key
}

function formatValue(key: string, value: number): string {
  const unit = NUTRIENT_UNITS[key as keyof typeof NUTRIENT_UNITS] || ''
  if (key === 'sodium' || key === 'calcium' || key === 'vitamin_a' || key === 'iron') {
    return `${value.toFixed(1)} ${unit}`
  }
  if (key === 'calories_kj') {
    return `${value.toFixed(0)} ${unit}`
  }
  return `${value.toFixed(1)} ${unit}`
}

function loadSimilarIngredients() {
  if (!props.selectedIngredient) {
    candidates.value = []
    return
  }

  loading.value = true
  try {
    const limit = crossCategory.value ? 10 : 5
    candidates.value = findSimilarIngredients(props.selectedIngredient, props.allIngredients, {
      sameCategory: !crossCategory.value,
      limit
    })
  } finally {
    loading.value = false
  }
}

function getPreviewChanges(candidate: SimilarIngredient) {
  if (!props.selectedIngredient) return {}
  
  const result = calculateReplacementNutritionChange(
    props.selectedIngredient,
    candidate.ingredient,
    props.amount
  )
  
  const preview: Record<string, { value: number; percentage: number; isIncrease: boolean }> = {}
  for (const key of PREVIEW_NUTRIENTS) {
    preview[key] = result.changes[key as keyof typeof result.changes]
  }
  return preview
}

const compareTableData = computed(() => {
  if (!selectedCandidate.value || !props.selectedIngredient) return []

  const result = calculateReplacementNutritionChange(
    props.selectedIngredient,
    selectedCandidate.value.ingredient,
    props.amount
  )

  const nutrientKeys = ['calories', 'protein', 'fat', 'carbs', 'fiber', 'sodium', 'calcium', 'iron', 'vitamin_a', 'vitamin_c']

  return nutrientKeys.map(key => ({
    nutrient: NUTRIENT_LABELS[key as keyof typeof NUTRIENT_LABELS],
    nutrientKey: key,
    originalValue: result.original[key as keyof typeof result.original],
    newValue: result.new[key as keyof typeof result.new],
    changeObj: result.changes[key as keyof typeof result.changes]
  }))
})

function showConfirmDialog(candidate: SimilarIngredient) {
  selectedCandidate.value = candidate
  confirmDialogVisible.value = true
}

function confirmReplace() {
  if (!selectedCandidate.value) return
  
  emit('replace', selectedCandidate.value.ingredient)
  confirmDialogVisible.value = false
  ElMessage.success('食材替换成功')
}

watch(() => props.selectedIngredient, () => {
  loadSimilarIngredients()
}, { immediate: true })
</script>

<style scoped lang="scss">
.ingredient-replace-panel {
  background: #fff;
  border-radius: 8px;
  padding: 16px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.05);

  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
    padding-bottom: 12px;
    border-bottom: 1px solid #ebeef5;

    .header-title {
      display: flex;
      align-items: center;
      gap: 8px;
      font-size: 16px;
      font-weight: 600;
      color: #303133;

      .el-icon {
        color: #409eff;
      }
    }
  }

  .selected-info {
    margin-bottom: 16px;
    padding: 12px;
    background: #f5f7fa;
    border-radius: 6px;

    .selected-label {
      font-size: 12px;
      color: #909399;
      margin-bottom: 6px;
    }

    .selected-name {
      display: flex;
      align-items: center;
      gap: 8px;
      font-size: 15px;
      font-weight: 500;
      color: #303133;

      .amount {
        color: #909399;
        font-weight: normal;
        font-size: 13px;
      }
    }
  }

  .empty-selection {
    padding: 40px 0;
  }

  .candidates-list {
    .loading-state {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 12px;
      padding: 40px 0;
      color: #909399;

      .el-icon {
        font-size: 32px;
        color: #409eff;
      }
    }

    .no-candidates {
      padding: 20px 0;
    }

    .candidate-item {
      padding: 12px;
      margin-bottom: 10px;
      border: 1px solid #ebeef5;
      border-radius: 6px;
      cursor: pointer;
      transition: all 0.2s;

      &:hover {
        border-color: #409eff;
        background: #f0f9ff;
      }

      .candidate-header {
        display: flex;
        align-items: center;
        gap: 10px;
        margin-bottom: 10px;

        .rank-badge {
          width: 24px;
          height: 24px;
          border-radius: 50%;
          display: flex;
          align-items: center;
          justify-content: center;
          font-size: 12px;
          font-weight: 600;
          color: #fff;
          background: #c0c4cc;

          &.rank-1 { background: #f56c6c; }
          &.rank-2 { background: #e6a23c; }
          &.rank-3 { background: #67c23a; }
        }

        .candidate-name {
          flex: 1;
          display: flex;
          align-items: center;
          gap: 6px;

          .name {
            font-weight: 500;
            color: #303133;
          }

          .category-tag {
            font-size: 11px;
          }
        }

        .similarity {
          text-align: right;

          .similarity-value {
            display: block;
            font-size: 18px;
            font-weight: 600;
            color: #67c23a;
            line-height: 1.2;
          }

          .similarity-label {
            font-size: 11px;
            color: #909399;
          }
        }
      }

      .candidate-nutrition-preview {
        display: flex;
        gap: 12px;
        padding: 8px 0;
        border-top: 1px dashed #ebeef5;
        border-bottom: 1px dashed #ebeef5;
        margin-bottom: 8px;

        .nutrition-change-item {
          flex: 1;
          text-align: center;

          .nutrient-name {
            display: block;
            font-size: 11px;
            color: #909399;
            margin-bottom: 2px;
          }

          .nutrient-change {
            font-size: 12px;
            font-weight: 500;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 2px;

            &.increase {
              color: #f56c6c;
            }

            &.decrease {
              color: #67c23a;
            }

            &.no-change {
              color: #c0c4cc;
            }

            .el-icon {
              font-size: 10px;
            }
          }
        }
      }

      .replace-hint {
        display: flex;
        align-items: center;
        justify-content: flex-end;
        gap: 4px;
        font-size: 12px;
        color: #409eff;

        .el-icon {
          font-size: 14px;
        }
      }
    }
  }

  .confirm-content {
    .replace-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: 16px;
      padding: 16px;
      background: #f5f7fa;
      border-radius: 6px;

      .replace-item {
        flex: 1;
        text-align: center;

        .item-label {
          font-size: 12px;
          color: #909399;
          margin-bottom: 6px;
        }

        .item-name {
          font-size: 16px;
          font-weight: 600;
          color: #303133;
          margin-bottom: 6px;
        }
      }

      .replace-arrow {
        padding: 0 20px;
        color: #409eff;
        font-size: 24px;
      }
    }

    .amount-info {
      text-align: center;
      margin-bottom: 16px;
      color: #606266;

      strong {
        color: #409eff;
      }
    }

    .compare-title {
      font-size: 14px;
      font-weight: 600;
      color: #303133;
      margin-bottom: 12px;
    }

    .tip {
      margin-top: 16px;
      padding: 10px 12px;
      background: #ecf5ff;
      border-radius: 4px;
      font-size: 13px;
      color: #409eff;
      display: flex;
      align-items: center;
      gap: 8px;
    }
  }
}

.increase {
  color: #f56c6c;
}

.decrease {
  color: #67c23a;
}
</style>
