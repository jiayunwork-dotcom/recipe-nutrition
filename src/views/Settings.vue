<template>
  <div class="settings-page">
    <el-card class="section-card">
      <template #header>
        <div class="card-header">
          <span class="section-title">营养目标预设</span>
          <el-button type="primary" @click="openCreateDialog">
            <el-icon><Plus /></el-icon>
            新建预设
          </el-button>
        </div>
      </template>

      <div v-if="loading" class="loading-wrap">
        <el-icon class="is-loading" :size="32"><Loading /></el-icon>
      </div>

      <el-empty v-else-if="presets.length === 0" description="暂无营养预设，点击右上角新建" />

      <div v-else class="preset-list">
        <div
          v-for="preset in presets"
          :key="preset.id"
          class="preset-card"
        >
          <div class="preset-header">
            <div>
              <h3 class="preset-name">{{ preset.name }}</h3>
              <p v-if="preset.description" class="preset-desc">{{ preset.description }}</p>
            </div>
            <div class="preset-actions">
              <el-button size="small" @click="openEditDialog(preset)">
                <el-icon><Edit /></el-icon>
              </el-button>
              <el-button type="danger" size="small" @click="handleDelete(preset)">
                <el-icon><Delete /></el-icon>
              </el-button>
            </div>
          </div>

          <div class="preset-constraints">
            <div
              v-for="nk in NUTRIENT_PRESET_KEYS"
              :key="nk.key"
              class="constraint-item"
            >
              <span class="constraint-label">{{ nk.label }}</span>
              <span class="constraint-range">
                <template v-if="getMin(preset, nk.key) !== null || getMax(preset, nk.key) !== null">
                  <span v-if="getMin(preset, nk.key) !== null">
                    ≥ {{ getMin(preset, nk.key) }}
                  </span>
                  <span v-if="getMin(preset, nk.key) !== null && getMax(preset, nk.key) !== null">
                    &nbsp;/&nbsp;
                  </span>
                  <span v-if="getMax(preset, nk.key) !== null">
                    ≤ {{ getMax(preset, nk.key) }}
                  </span>
                  <span class="constraint-unit">{{ nk.unit }}</span>
                </template>
                <span v-else class="no-constraint">未限制</span>
              </span>
            </div>
          </div>

          <div class="preset-meta">
            创建于 {{ formatDate(preset.created_at) }}
            <span v-if="preset.updated_at !== preset.created_at">
              ，更新于 {{ formatDate(preset.updated_at) }}
            </span>
          </div>
        </div>
      </div>
    </el-card>

    <el-dialog
      v-model="dialogVisible"
      :title="editingPreset ? '编辑预设' : '新建预设'"
      width="600px"
      :close-on-click-modal="false"
    >
      <el-form :model="formData" label-width="100px" ref="formRef">
        <el-form-item label="预设名称" required>
          <el-input v-model="formData.name" placeholder="如：减脂餐目标" maxlength="50" />
        </el-form-item>
        <el-form-item label="描述">
          <el-input v-model="formData.description" type="textarea" :rows="2" maxlength="200" />
        </el-form-item>

        <el-divider content-position="left">营养约束条件</el-divider>

        <div
          v-for="nk in NUTRIENT_PRESET_KEYS"
          :key="nk.key"
          class="constraint-form-row"
        >
          <span class="nk-label">{{ nk.label }}</span>
          <div class="nk-inputs">
            <span class="nk-sub-label">最低</span>
            <el-input-number
              v-model="formData.minValues[nk.key]"
              :min="0"
              :precision="1"
              :controls="false"
              :placeholder="'不限'"
              style="width: 120px;"
            />
            <span class="nk-sub-label" style="margin-left: 16px;">最高</span>
            <el-input-number
              v-model="formData.maxValues[nk.key]"
              :min="0"
              :precision="1"
              :controls="false"
              :placeholder="'不限'"
              style="width: 120px;"
            />
            <span class="nk-unit">{{ nk.unit }}</span>
          </div>
        </div>
      </el-form>

      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSave">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox, type FormInstance } from 'element-plus'
import { Plus, Delete, Edit, Loading } from '@element-plus/icons-vue'
import { presetApi } from '@/api/tauri'
import {
  type NutritionPreset,
  type NutrientKey,
  NUTRIENT_PRESET_KEYS,
} from '@/types'

const loading = ref(false)
const presets = ref<NutritionPreset[]>([])
const dialogVisible = ref(false)
const editingPreset = ref<NutritionPreset | null>(null)
const formRef = ref<FormInstance>()

const formData = reactive({
  name: '',
  description: '',
  minValues: {} as Record<NutrientKey, number | null>,
  maxValues: {} as Record<NutrientKey, number | null>,
})

function resetForm() {
  formData.name = ''
  formData.description = ''
  for (const nk of NUTRIENT_PRESET_KEYS) {
    formData.minValues[nk.key] = null
    formData.maxValues[nk.key] = null
  }
}

function getMin(preset: NutritionPreset, key: NutrientKey): number | null {
  return (preset as any)[`min_${key}`] ?? null
}

function getMax(preset: NutritionPreset, key: NutrientKey): number | null {
  return (preset as any)[`max_${key}`] ?? null
}

function formatDate(s: string): string {
  const d = new Date(s)
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`
}

async function loadPresets() {
  loading.value = true
  try {
    presets.value = await presetApi.getAll()
  } catch (e: any) {
    ElMessage.error('加载预设失败：' + e)
  } finally {
    loading.value = false
  }
}

function openCreateDialog() {
  resetForm()
  editingPreset.value = null
  dialogVisible.value = true
}

function openEditDialog(preset: NutritionPreset) {
  resetForm()
  editingPreset.value = preset
  formData.name = preset.name
  formData.description = preset.description
  for (const nk of NUTRIENT_PRESET_KEYS) {
    formData.minValues[nk.key] = getMin(preset, nk.key)
    formData.maxValues[nk.key] = getMax(preset, nk.key)
  }
  dialogVisible.value = true
}

async function handleSave() {
  if (!formData.name.trim()) {
    ElMessage.warning('请输入预设名称')
    return
  }
  try {
    if (editingPreset.value) {
      const updateData: any = {
        name: formData.name.trim(),
        description: formData.description,
      }
      for (const nk of NUTRIENT_PRESET_KEYS) {
        updateData[`min_${nk.key}`] = formData.minValues[nk.key]
        updateData[`max_${nk.key}`] = formData.maxValues[nk.key]
      }
      await presetApi.update(editingPreset.value.id, updateData)
      ElMessage.success('预设已更新')
    } else {
      const createData: any = {
        name: formData.name.trim(),
        description: formData.description,
      }
      for (const nk of NUTRIENT_PRESET_KEYS) {
        createData[`min_${nk.key}`] = formData.minValues[nk.key]
        createData[`max_${nk.key}`] = formData.maxValues[nk.key]
      }
      await presetApi.create(createData)
      ElMessage.success('预设已创建')
    }
    dialogVisible.value = false
    await loadPresets()
  } catch (e: any) {
    ElMessage.error('保存失败：' + e)
  }
}

async function handleDelete(preset: NutritionPreset) {
  try {
    await ElMessageBox.confirm(
      `确定要删除预设"${preset.name}"吗？配方中引用此预设将被解除关联。`,
      '确认删除',
      { type: 'warning' }
    )
    await presetApi.delete(preset.id)
    ElMessage.success('已删除')
    await loadPresets()
  } catch (_) {
    // 用户取消
  }
}

onMounted(loadPresets)
</script>

<style scoped lang="scss">
.settings-page {
  .section-card {
    margin-bottom: 24px;
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;

    .section-title {
      font-weight: 600;
      font-size: 16px;
    }
  }

  .loading-wrap {
    display: flex;
    justify-content: center;
    padding: 48px 0;
    color: #909399;
  }

  .preset-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
    gap: 16px;
  }

  .preset-card {
    border: 1px solid #ebeef5;
    border-radius: 8px;
    padding: 16px;
    transition: all 0.2s;

    &:hover {
      box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.08);
    }

    .preset-header {
      display: flex;
      justify-content: space-between;
      align-items: flex-start;
      margin-bottom: 12px;
      padding-bottom: 12px;
      border-bottom: 1px solid #f0f0f0;

      .preset-name {
        margin: 0 0 4px 0;
        font-size: 16px;
        color: #303133;
      }

      .preset-desc {
        margin: 0;
        font-size: 13px;
        color: #909399;
      }

      .preset-actions {
        display: flex;
        gap: 4px;
      }
    }

    .preset-constraints {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 8px 16px;
      margin-bottom: 12px;

      .constraint-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        font-size: 13px;

        .constraint-label {
          color: #606266;
        }

        .constraint-range {
          color: #303133;
          font-weight: 500;

          .constraint-unit {
            color: #909399;
            font-weight: normal;
            margin-left: 2px;
            font-size: 12px;
          }

          .no-constraint {
            color: #c0c4cc;
            font-weight: normal;
            font-style: italic;
          }
        }
      }
    }

    .preset-meta {
      font-size: 12px;
      color: #c0c4cc;
    }
  }

  .constraint-form-row {
    display: flex;
    align-items: center;
    padding: 8px 0;
    border-bottom: 1px solid #f5f7fa;

    &:last-of-type {
      border-bottom: none;
    }

    .nk-label {
      width: 100px;
      font-size: 14px;
      color: #606266;
    }

    .nk-inputs {
      display: flex;
      align-items: center;
      flex: 1;

      .nk-sub-label {
        font-size: 13px;
        color: #909399;
        margin-right: 6px;
      }

      .nk-unit {
        font-size: 13px;
        color: #909399;
        margin-left: 8px;
      }
    }
  }
}
</style>
