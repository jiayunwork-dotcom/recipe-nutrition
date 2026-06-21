<template>
  <div class="templates-page">
    <div class="toolbar">
      <el-input
        v-model="searchKeyword"
        placeholder="搜索模板名称或描述..."
        clearable
        style="width: 280px; margin-right: 12px;"
        @keyup.enter="loadTemplates"
      >
        <template #prefix>
          <el-icon><Search /></el-icon>
        </template>
      </el-input>
      <el-select
        v-model="selectedTag"
        placeholder="标签筛选"
        clearable
        style="width: 180px; margin-right: 12px;"
        @change="loadTemplates"
      >
        <el-option
          v-for="tag in allTags"
          :key="tag"
          :label="tag"
          :value="tag"
        />
      </el-select>
      <el-button type="primary" @click="loadTemplates">
        <el-icon><Refresh /></el-icon>
        刷新
      </el-button>
    </div>

    <div v-if="loading" class="loading-wrap">
      <el-icon class="is-loading" :size="32"><Loading /></el-icon>
    </div>

    <el-empty v-else-if="templates.length === 0" description="暂无模板，可在配方详情页保存为模板" />

    <div v-else class="template-grid">
      <div
        v-for="tpl in templates"
        :key="tpl.id"
        class="template-card"
      >
        <div class="card-body">
          <div class="card-header">
            <h3 class="template-name">{{ tpl.name }}</h3>
            <el-dropdown @command="(cmd) => handleDropdownCommand(cmd, tpl)">
              <el-button size="small" link>
                <el-icon><MoreFilled /></el-icon>
              </el-button>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item command="edit">
                    <el-icon><Edit /></el-icon>编辑
                  </el-dropdown-item>
                  <el-dropdown-item command="delete" divided>
                    <el-icon color="#f56c6c"><Delete /></el-icon>删除
                  </el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </div>

          <p v-if="tpl.description" class="template-desc">{{ tpl.description }}</p>

          <div class="template-tags" v-if="tpl.tags.length > 0">
            <el-tag
              v-for="tag in tpl.tags"
              :key="tag"
              size="small"
              type="info"
              effect="plain"
              style="margin-right: 4px; margin-bottom: 4px;"
            >
              {{ tag }}
            </el-tag>
          </div>

          <div class="template-meta">
            <span class="meta-item">
              <el-icon><Collection /></el-icon>
              {{ tpl.category }}
            </span>
            <span class="meta-item">
              <el-icon><User /></el-icon>
              {{ tpl.servings }} 份
            </span>
            <span class="meta-item">
              <el-icon><Goods /></el-icon>
              {{ tpl.ingredients.length }} 种食材
            </span>
          </div>
        </div>

        <div class="card-footer">
          <span class="updated-time">
            更新于 {{ formatDate(tpl.updated_at) }}
          </span>
          <el-button type="primary" size="small" @click="handleUseTemplate(tpl)">
            <el-icon><DocumentCopy /></el-icon>
            使用模板
          </el-button>
        </div>
      </div>
    </div>

    <el-dialog
      v-model="editDialogVisible"
      title="编辑模板"
      width="500px"
      :close-on-click-modal="false"
    >
      <el-form :model="editForm" label-width="80px">
        <el-form-item label="模板名称" required>
          <el-input v-model="editForm.name" maxlength="100" />
        </el-form-item>
        <el-form-item label="描述">
          <el-input v-model="editForm.description" type="textarea" :rows="2" maxlength="200" />
        </el-form-item>
        <el-form-item label="标签">
          <el-input
            v-model="editForm.tagsStr"
            placeholder="多个标签用逗号分隔，如：早餐,减脂,高蛋白"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="editDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSaveEdit">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  Search, Refresh, Loading, MoreFilled, Edit, Delete,
  Collection, User, Goods, DocumentCopy
} from '@element-plus/icons-vue'
import { templateApi, presetApi } from '@/api/tauri'
import type { RecipeTemplate } from '@/types'

const router = useRouter()

const loading = ref(false)
const templates = ref<RecipeTemplate[]>([])
const allTags = ref<string[]>([])
const searchKeyword = ref('')
const selectedTag = ref<string | undefined>(undefined)

const editDialogVisible = ref(false)
const editingTemplate = ref<RecipeTemplate | null>(null)
const editForm = reactive({
  name: '',
  description: '',
  tagsStr: '',
})

function formatDate(s: string): string {
  const d = new Date(s)
  const mm = String(d.getMonth() + 1).padStart(2, '0')
  const dd = String(d.getDate()).padStart(2, '0')
  const hh = String(d.getHours()).padStart(2, '0')
  const mi = String(d.getMinutes()).padStart(2, '0')
  return `${mm}-${dd} ${hh}:${mi}`
}

async function loadTemplates() {
  loading.value = true
  try {
    const [tpls, tags] = await Promise.all([
      templateApi.getAll(
        searchKeyword.value.trim() || undefined,
        selectedTag.value || undefined
      ),
      templateApi.getAllTags(),
    ])
    templates.value = tpls
    allTags.value = tags
  } catch (e: any) {
    ElMessage.error('加载模板失败：' + e)
  } finally {
    loading.value = false
  }
}

async function handleUseTemplate(tpl: RecipeTemplate) {
  try {
    const { value: newName } = await ElMessageBox.prompt(
      `使用模板"${tpl.name}"创建配方，请输入新配方名称：`,
      '从模板创建配方',
      {
        confirmButtonText: '创建',
        cancelButtonText: '取消',
        inputValue: `${tpl.name} 副本`,
        inputPlaceholder: '请输入配方名称',
      }
    )
    if (!newName || !newName.trim()) return
    const presetNames = await presetApi.getAll().catch(() => [])
    const presetMap = new Map(presetNames.map(p => [p.id, p]))
    const newRecipe = await templateApi.createRecipeFromTemplate(tpl.id, newName.trim())
    ElMessage.success('配方已创建')
    router.push(`/recipes/${newRecipe.id}`)
  } catch (e: any) {
    if (e === 'cancel' || e === 'close') return
    ElMessage.error('创建失败：' + (e?.message || e))
  }
}

function handleDropdownCommand(cmd: string, tpl: RecipeTemplate) {
  if (cmd === 'edit') {
    editingTemplate.value = tpl
    editForm.name = tpl.name
    editForm.description = tpl.description
    editForm.tagsStr = tpl.tags.join(', ')
    editDialogVisible.value = true
  } else if (cmd === 'delete') {
    handleDelete(tpl)
  }
}

async function handleDelete(tpl: RecipeTemplate) {
  try {
    await ElMessageBox.confirm(
      `确定要删除模板"${tpl.name}"吗？`,
      '确认删除',
      { type: 'warning' }
    )
    await templateApi.delete(tpl.id)
    ElMessage.success('已删除')
    await loadTemplates()
  } catch (_) {
    // 用户取消
  }
}

async function handleSaveEdit() {
  if (!editingTemplate.value) return
  if (!editForm.name.trim()) {
    ElMessage.warning('请输入模板名称')
    return
  }
  const tags = editForm.tagsStr
    .split(/[,，]/)
    .map(s => s.trim())
    .filter(s => s.length > 0)

  try {
    await templateApi.update(editingTemplate.value.id, {
      name: editForm.name.trim(),
      description: editForm.description,
      tags,
    })
    ElMessage.success('已保存')
    editDialogVisible.value = false
    await loadTemplates()
  } catch (e: any) {
    ElMessage.error('保存失败：' + e)
  }
}

onMounted(loadTemplates)
</script>

<style scoped lang="scss">
.templates-page {
  .toolbar {
    display: flex;
    align-items: center;
    margin-bottom: 20px;
    padding: 16px;
    background: #fff;
    border-radius: 8px;
    box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.05);
  }

  .loading-wrap {
    display: flex;
    justify-content: center;
    padding: 64px 0;
    color: #909399;
  }

  .template-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
    gap: 16px;
  }

  .template-card {
    background: #fff;
    border-radius: 8px;
    box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.05);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    transition: all 0.2s;

    &:hover {
      box-shadow: 0 4px 16px 0 rgba(0, 0, 0, 0.1);
      transform: translateY(-2px);
    }

    .card-body {
      padding: 16px;
      flex: 1;
      display: flex;
      flex-direction: column;
    }

    .card-header {
      display: flex;
      justify-content: space-between;
      align-items: flex-start;
      margin-bottom: 8px;

      .template-name {
        margin: 0;
        font-size: 16px;
        font-weight: 600;
        color: #303133;
        line-height: 1.4;
      }
    }

    .template-desc {
      margin: 0 0 12px 0;
      font-size: 13px;
      color: #606266;
      line-height: 1.5;
      display: -webkit-box;
      -webkit-line-clamp: 2;
      -webkit-box-orient: vertical;
      overflow: hidden;
    }

    .template-tags {
      margin-bottom: 12px;
      min-height: 24px;
    }

    .template-meta {
      display: flex;
      flex-wrap: wrap;
      gap: 12px;
      margin-top: auto;
      padding-top: 12px;

      .meta-item {
        display: flex;
        align-items: center;
        gap: 4px;
        font-size: 12px;
        color: #909399;
      }
    }

    .card-footer {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 12px 16px;
      background: #fafbfc;
      border-top: 1px solid #ebeef5;

      .updated-time {
        font-size: 12px;
        color: #c0c4cc;
      }
    }
  }
}
</style>
