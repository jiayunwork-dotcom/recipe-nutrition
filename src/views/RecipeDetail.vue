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

          <div class="preset-section">
            <div class="section-title">营养目标预设</div>
            <el-select
              v-model="selectedPresetId"
              placeholder="选择营养约束预设"
              clearable
              style="width: 100%;"
              @change="handlePresetChange"
            >
              <el-option
                v-for="p in presets"
                :key="p.id"
                :label="p.name"
                :value="p.id"
              />
            </el-select>
            <div v-if="currentPreset" class="preset-info">
              <span class="preset-name">{{ currentPreset.name }}</span>
              <el-button link size="small" @click="goToSettings">
                管理预设
              </el-button>
            </div>
          </div>

          <div class="export-section">
            <div class="section-title">配方分享</div>
            <el-row :gutter="12" style="margin-bottom: 12px;">
              <el-col :span="12">
                <el-button type="primary" plain style="width: 100%;" @click="openSaveTemplateDialog">
                  <el-icon><CollectionTag /></el-icon>
                  保存为模板
                </el-button>
              </el-col>
              <el-col :span="12">
                <el-button type="info" plain style="width: 100%;" @click="openVersionsDrawer">
                  <el-icon><Clock /></el-icon>
                  历史版本
                </el-button>
              </el-col>
            </el-row>
            <el-row :gutter="12" style="margin-bottom: 12px;">
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
            <el-row :gutter="12">
              <el-col :span="12">
                <el-button type="primary" style="width: 100%;" @click="exportJSON">
                  <el-icon><Download /></el-icon>
                  导出 JSON
                </el-button>
              </el-col>
              <el-col :span="12">
                <el-button type="danger" plain style="width: 100%;" @click="handleImportClick">
                  <el-icon><Upload /></el-icon>
                  导入 JSON
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

    <el-drawer
      v-model="versionsDrawerVisible"
      title="配方历史版本"
      direction="rtl"
      size="560px"
    >
      <div class="versions-toolbar">
        <el-button
          type="primary"
          size="small"
          :disabled="!currentRecipe"
          @click="handleCreateSnapshot"
        >
          <el-icon><Plus /></el-icon>
          保存当前版本
        </el-button>
        <el-button
          type="success"
          size="small"
          :disabled="selectedCompareIds.length !== 2"
          @click="openCompareView"
        >
          <el-icon><DataLine /></el-icon>
          对比
        </el-button>
        <el-button
          type="warning"
          size="small"
          :disabled="selectedExportIds.length === 0"
          @click="handleExportVersions"
        >
          <el-icon><Download /></el-icon>
          导出
        </el-button>
        <el-button
          type="info"
          size="small"
          @click="handleImportVersions"
        >
          <el-icon><Upload /></el-icon>
          导入
        </el-button>
      </div>
      <div v-if="compareViewVisible" class="compare-view">
        <div class="compare-header">
          <span class="compare-title">版本对比</span>
          <el-button size="small" link @click="compareViewVisible = false">
            <el-icon><Close /></el-icon> 关闭对比
          </el-button>
        </div>
        <div class="compare-columns">
          <div class="compare-col">
            <div class="compare-col-title">v{{ compareVersionA?.version_number }}</div>
            <div
              v-for="item in compareItemsA"
              :key="item.ingredient_id"
              class="compare-ingredient"
              :class="item.changeType"
            >
              <span class="ci-name">{{ item.ingredient_name }}</span>
              <span class="ci-amount">{{ item.amount.toFixed(1) }}g</span>
              <span v-if="item.changeAmount" class="ci-change">{{ item.changeAmount }}</span>
            </div>
          </div>
          <div class="compare-col">
            <div class="compare-col-title">v{{ compareVersionB?.version_number }}</div>
            <div
              v-for="item in compareItemsB"
              :key="item.ingredient_id"
              class="compare-ingredient"
              :class="item.changeType"
            >
              <span class="ci-name">{{ item.ingredient_name }}</span>
              <span class="ci-amount">{{ item.amount.toFixed(1) }}g</span>
              <span v-if="item.changeAmount" class="ci-change">{{ item.changeAmount }}</span>
            </div>
          </div>
        </div>
        <div class="compare-chart">
          <div class="compare-chart-title">营养素对比</div>
          <div ref="compareChartRef" style="width: 100%; height: 260px;"></div>
        </div>
      </div>
      <div v-if="versionsLoading" class="versions-loading">
        <el-icon class="is-loading" :size="24"><Loading /></el-icon>
        加载中...
      </div>
      <el-empty
        v-else-if="sortedVersions.length === 0"
        description="暂无历史版本，点击上方按钮保存"
      />
      <div v-else class="versions-list">
        <div
          v-for="(ver, idx) in sortedVersions"
          :key="ver.id"
          class="version-item"
          :class="{ active: expandedVersionId === ver.id }"
        >
          <div class="version-header" @click="toggleVersionExpand(ver)">
            <div class="version-info">
              <el-tooltip content="对比（选择2个）" placement="top">
                <el-checkbox
                  :model-value="selectedCompareIds.includes(ver.id)"
                  @change="(val: boolean) => toggleCompareSelect(ver.id, val)"
                  @click.stop
                  size="small"
                  class="compare-checkbox"
                >
                  <el-icon :size="12"><DataLine /></el-icon>
                </el-checkbox>
              </el-tooltip>
              <el-tooltip content="导出（可多选）" placement="top">
                <el-checkbox
                  :model-value="selectedExportIds.includes(ver.id)"
                  @change="(val: boolean) => toggleExportSelect(ver.id, val)"
                  @click.stop
                  size="small"
                  class="export-checkbox"
                >
                  <el-icon :size="12"><Download /></el-icon>
                </el-checkbox>
              </el-tooltip>
              <el-icon
                class="star-btn"
                :class="{ starred: ver.is_starred }"
                @click.stop="handleToggleStar(ver)"
              >
                <StarFilled v-if="ver.is_starred" />
                <Star v-else />
              </el-icon>
              <span class="version-badge">
                v{{ ver.version_number }}
                <el-tag
                  v-if="ver.is_rollback"
                  type="warning"
                  size="small"
                  effect="plain"
                  style="margin-left: 4px;"
                >回退</el-tag>
              </span>
              <span class="version-time">{{ formatVersionTime(ver.created_at) }}</span>
            </div>
            <el-icon class="expand-icon">
              <ArrowDown v-if="expandedVersionId !== ver.id" />
              <ArrowUp v-else />
            </el-icon>
          </div>
          <div class="version-summary" @click="toggleVersionExpand(ver)">{{ ver.summary }}</div>
          <div v-if="ver.note" class="version-note" @click="toggleVersionExpand(ver)">{{ ver.note }}</div>

          <div v-if="expandedVersionId === ver.id" class="version-detail">
            <div class="version-note-edit">
              <el-input
                v-model="versionNoteEdit"
                type="textarea"
                :rows="2"
                maxlength="200"
                show-word-limit
                placeholder="添加版本备注（可选，最多200字）"
                size="small"
              />
              <el-button size="small" type="primary" @click="saveVersionNote(ver)" style="margin-top: 4px;">
                保存备注
              </el-button>
            </div>
            <div v-if="versionDiffs[ver.id]" class="diff-section">
              <div v-if="versionDiffs[ver.id]!.added.length > 0" class="diff-group">
                <div class="diff-title added">新增食材</div>
                <div v-for="item in versionDiffs[ver.id]!.added" :key="item" class="diff-item">
                  <el-icon color="#67c23a"><CirclePlus /></el-icon>{{ item }}
                </div>
              </div>
              <div v-if="versionDiffs[ver.id]!.removed.length > 0" class="diff-group">
                <div class="diff-title removed">移除食材</div>
                <div v-for="item in versionDiffs[ver.id]!.removed" :key="item" class="diff-item">
                  <el-icon color="#f56c6c"><CircleClose /></el-icon>{{ item }}
                </div>
              </div>
              <div v-if="versionDiffs[ver.id]!.modified.length > 0" class="diff-group">
                <div class="diff-title modified">调整用量</div>
                <div v-for="item in versionDiffs[ver.id]!.modified" :key="item" class="diff-item">
                  <el-icon color="#e6a23c"><Edit /></el-icon>{{ item }}
                </div>
              </div>
              <div
                v-if="!versionDiffs[ver.id]!.added.length && !versionDiffs[ver.id]!.removed.length && !versionDiffs[ver.id]!.modified.length"
                class="diff-empty"
              >
                基础信息变更（名称、分类、份数、备注等）
              </div>
            </div>
            <div class="version-nutrition">
              <div class="vn-title">当时营养总量</div>
              <div v-if="versionNutrition[ver.id]" class="vn-grid">
                <div class="vn-item">
                  <span class="label">热量</span>
                  <span class="value">{{ versionNutrition[ver.id]!.calories.toFixed(0) }} kcal</span>
                </div>
                <div class="vn-item">
                  <span class="label">蛋白质</span>
                  <span class="value">{{ versionNutrition[ver.id]!.protein.toFixed(1) }} g</span>
                </div>
                <div class="vn-item">
                  <span class="label">脂肪</span>
                  <span class="value">{{ versionNutrition[ver.id]!.fat.toFixed(1) }} g</span>
                </div>
                <div class="vn-item">
                  <span class="label">碳水</span>
                  <span class="value">{{ versionNutrition[ver.id]!.carbs.toFixed(1) }} g</span>
                </div>
              </div>
            </div>
            <div class="version-actions">
              <el-button
                type="primary"
                size="small"
                @click.stop="handleRollback(ver)"
              >
                <el-icon><RefreshLeft /></el-icon>
                回退到此版本
              </el-button>
            </div>
          </div>
        </div>
      </div>
    </el-drawer>

    <el-dialog
      v-model="conflictDialogVisible"
      title="回退冲突处理"
      width="680px"
      :close-on-click-modal="false"
    >
      <div class="conflict-intro">
        回退到 <strong>v{{ rollbackTargetVersion?.version_number }}</strong> 后，以下食材将会丢失。请选择保留或丢弃：
      </div>
      <div class="conflict-list">
        <div
          v-for="item in conflictItems"
          :key="item.ingredient_id"
          class="conflict-item"
        >
          <div class="conflict-item-info">
            <el-tag size="small" type="info">{{ item.category }}</el-tag>
            <span class="conflict-name">{{ item.ingredient_name }}</span>
            <span class="conflict-amount">{{ item.amount.toFixed(1) }}g</span>
          </div>
          <div class="conflict-item-actions">
            <el-radio-group v-model="item.action" size="small">
              <el-radio-button value="discard">丢弃</el-radio-button>
              <el-radio-button value="keep">保留</el-radio-button>
            </el-radio-group>
          </div>
          <div class="conflict-item-preview">
            <span v-if="item.action === 'discard'" class="preview-discard">
              丢弃后: 热量 {{ (conflictBaseNutrition.calories - item.calories).toFixed(0) }}kcal, 蛋白质 {{ (conflictBaseNutrition.protein - item.protein).toFixed(1) }}g
            </span>
            <span v-else class="preview-keep">
              保留后: 热量 {{ (conflictBaseNutrition.calories).toFixed(0) }}kcal, 蛋白质 {{ (conflictBaseNutrition.protein).toFixed(1) }}g
            </span>
          </div>
        </div>
      </div>
      <template #footer>
        <el-button @click="conflictDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="executeRollbackWithMerge">确认回退</el-button>
      </template>
    </el-dialog>

    <el-dialog
      v-model="importDialogVisible"
      title="导入配方 JSON"
      width="720px"
      :close-on-click-modal="false"
    >
      <template v-if="importStep === 'select'">
        <div class="import-step-intro">
          <p>请选择一个配方 JSON 文件进行导入。导入过程中将智能匹配本地食材库。</p>
          <el-upload
            :auto-upload="false"
            :show-file-list="false"
            :on-change="handleImportFileChange"
            accept=".json"
            drag
          >
            <el-icon class="el-icon--upload"><UploadFilled /></el-icon>
            <div class="el-upload__text">
              将 JSON 文件拖到此处，或<em>点击选择</em>
            </div>
            <template #tip>
              <div class="el-upload__tip">只能上传 .json 格式的配方文件</div>
            </template>
          </el-upload>
        </div>
      </template>

      <template v-else-if="importStep === 'preview' && importPreview">
        <div class="import-preview-header">
          <el-descriptions :column="2" size="small" border>
            <el-descriptions-item label="配方名称">{{ importPreview.recipe.name }}</el-descriptions-item>
            <el-descriptions-item label="分类">{{ importPreview.recipe.category }}</el-descriptions-item>
            <el-descriptions-item label="份数">{{ importPreview.recipe.servings }}</el-descriptions-item>
            <el-descriptions-item label="备注">{{ importPreview.recipe.notes || '-' }}</el-descriptions-item>
          </el-descriptions>

          <div v-if="importPreview.nutrition_preset" class="preset-preview">
            <el-alert
              :title="`营养预设: ${importPreview.nutrition_preset.name}${importPreview.preset_exists ? '（已存在，将关联）' : '（不存在，将新建）'}`"
              :type="importPreview.preset_exists ? 'success' : 'warning'"
              show-icon
              :closable="false"
              style="margin-top: 12px;"
            />
          </div>
        </div>

        <div class="import-match-list">
          <div class="section-sub-title">食材匹配确认（共 {{ importPreview.ingredient_matches.length }} 项）</div>
          <div
            v-for="(match, idx) in importPreview.ingredient_matches"
            :key="idx"
            class="match-item"
            :class="{ 'needs-review': match.requires_confirmation }"
          >
            <div class="match-exported">
              <el-tag size="small" type="info">{{ match.exported.category }}</el-tag>
              <span class="match-name">{{ match.exported.name }}</span>
              <span class="match-amount">({{ match.exported.amount }}g)</span>
              <el-tag
                v-if="match.requires_confirmation"
                size="small"
                type="warning"
                effect="dark"
                style="margin-left: 8px;"
              >需确认</el-tag>
            </div>

            <div class="match-actions">
              <el-radio-group v-model="match.action" size="small">
                <el-radio-button
                  v-if="match.matched_local"
                  value="use_exact"
                >使用匹配</el-radio-button>
                <el-radio-button value="add_new">添加为新食材</el-radio-button>
                <el-radio-button
                  v-if="match.candidates.length > 0"
                  value="use_candidate"
                >选择替代</el-radio-button>
              </el-radio-group>

              <div v-if="match.action === 'use_exact' && match.matched_local" class="match-selected">
                → <el-tag type="success" size="small">{{ match.matched_local.name }}</el-tag>
                <span class="match-meta">({{ match.matched_local.category }})</span>
              </div>

              <div v-if="match.action === 'add_new'" class="match-new-info">
                将添加新食材
                <span class="match-nutrition">
                  热{{ match.exported.calories.toFixed(0) }}/蛋{{ match.exported.protein.toFixed(1) }}/脂{{ match.exported.fat.toFixed(1) }}g
                </span>
              </div>

              <div v-if="match.action === 'use_candidate'" class="candidate-list">
                <el-select
                  v-model="match.selected_ingredient_id"
                  placeholder="选择替代食材"
                  size="small"
                  style="min-width: 200px;"
                >
                  <el-option
                    v-for="c in match.candidates"
                    :key="c.local_ingredient.id"
                    :label="`${c.local_ingredient.name} (相似度 ${(c.similarity_score * 100).toFixed(0)}%)`"
                    :value="c.local_ingredient.id"
                  />
                </el-select>
              </div>
            </div>
          </div>
        </div>
      </template>

      <template #footer>
        <template v-if="importStep === 'select'">
          <el-button @click="importDialogVisible = false">取消</el-button>
        </template>
        <template v-else-if="importStep === 'preview'">
          <el-button @click="importStep = 'select'">返回</el-button>
          <el-button type="primary" :disabled="!canExecuteImport" @click="executeImport">
            确认导入
          </el-button>
        </template>
      </template>
    </el-dialog>

    <el-dialog
      v-model="saveTemplateDialogVisible"
      title="保存为配方模板"
      width="500px"
      :close-on-click-modal="false"
    >
      <el-form :model="templateForm" label-width="80px">
        <el-form-item label="模板名称" required>
          <el-input v-model="templateForm.name" maxlength="100" placeholder="输入模板名称" />
        </el-form-item>
        <el-form-item label="描述">
          <el-input v-model="templateForm.description" type="textarea" :rows="2" maxlength="200" placeholder="模板用途或简介" />
        </el-form-item>
        <el-form-item label="标签">
          <el-input
            v-model="templateForm.tagsStr"
            placeholder="多个标签用逗号分隔，如：早餐,减脂,高蛋白"
          />
        </el-form-item>
        <el-form-item label="关联预设">
          <el-select
            v-model="templateForm.presetId"
            placeholder="关联营养约束预设（可选）"
            clearable
            style="width: 100%;"
          >
            <el-option
              v-for="p in presets"
              :key="p.id"
              :label="p.name"
              :value="p.id"
            />
          </el-select>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="saveTemplateDialogVisible = false">取消</el-button>
        <el-button type="primary" :disabled="!templateForm.name.trim()" @click="executeSaveTemplate">
          保存
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
  Star, StarFilled, Plus, Delete, Search, Download, Document, Grid, RefreshRight,
  Clock, CollectionTag, Upload, Loading, ArrowDown, ArrowUp, CirclePlus, CircleClose,
  RefreshLeft, UploadFilled, Setting, Close, DataLine
} from '@element-plus/icons-vue'
import { useRecipeStore } from '@/stores/recipe'
import { useIngredientStore } from '@/stores/ingredient'
import {
  RECIPE_CATEGORIES, NUTRIENT_LABELS, NUTRIENT_UNITS, NUTRIENT_PRESET_KEYS,
  type NutritionSummary,
  type NutritionPreset,
  type RecipeVersion,
  type VersionDiffSummary,
  type ImportPreview,
  type ImportIngredientMatch,
  type ImportConfirmedItem,
  type NutrientKey,
  type IngredientSnapshotItem,
  type CompareIngredientItem,
} from '@/types'
import DRIRadarChart from '@/components/DRIRadarChart.vue'
import NutritionLabel from '@/components/NutritionLabel.vue'
import IngredientReplacePanel from '@/components/IngredientReplacePanel.vue'
import type { RecipeIngredient, Ingredient, NutritionSummary as NutrSum } from '@/types'
import { presetApi, versionApi, templateApi, exchangeApi } from '@/api/tauri'
import html2canvas from 'html2canvas'
import { save, open } from '@tauri-apps/plugin-dialog'
import { writeFile, readTextFile } from '@tauri-apps/plugin-fs'
import * as XLSX from 'xlsx'
import jsPDF from 'jspdf'
import * as echarts from 'echarts'

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

const presets = ref<NutritionPreset[]>([])
const selectedPresetId = ref<number | null>(null)
const currentPreset = computed(() =>
  selectedPresetId.value ? presets.value.find(p => p.id === selectedPresetId.value) || null : null
)

const versionsDrawerVisible = ref(false)
const versionsLoading = ref(false)
const versions = ref<RecipeVersion[]>([])
const expandedVersionId = ref<number | null>(null)
const versionDiffs = reactive<Record<number, VersionDiffSummary | undefined>>({})
const versionNutrition = reactive<Record<number, NutritionSummary | undefined>>({})
const versionNoteEdit = ref('')
const selectedCompareIds = ref<number[]>([])
const selectedExportIds = ref<number[]>([])
const compareViewVisible = ref(false)
const compareChartRef = ref<HTMLElement | null>(null)
let compareChartInstance: echarts.ECharts | null = null
const conflictDialogVisible = ref(false)
const rollbackTargetVersion = ref<RecipeVersion | null>(null)

interface ConflictItem {
  ingredient_id: number
  ingredient_name: string
  category: string
  amount: number
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
  action: 'discard' | 'keep'
}
const conflictItems = ref<ConflictItem[]>([])

const sortedVersions = computed(() => {
  const compareCreatedAtDesc = (a: RecipeVersion, b: RecipeVersion) => {
    const ta = new Date(a.created_at).getTime()
    const tb = new Date(b.created_at).getTime()
    if (tb !== ta) return tb - ta
    return b.version_number - a.version_number
  }
  const starred = versions.value.filter(v => v.is_starred).sort(compareCreatedAtDesc)
  const unstarred = versions.value.filter(v => !v.is_starred).sort(compareCreatedAtDesc)
  return [...starred, ...unstarred]
})

const compareVersionA = computed(() => {
  if (selectedCompareIds.value.length < 2) return null
  return versions.value.find(v => v.id === selectedCompareIds.value[0]) || null
})

const compareVersionB = computed(() => {
  if (selectedCompareIds.value.length < 2) return null
  return versions.value.find(v => v.id === selectedCompareIds.value[1]) || null
})

function buildCompareItems(version: RecipeVersion, otherVersion: RecipeVersion): CompareIngredientItem[] {
  const curIngs: IngredientSnapshotItem[] = []
  const otherIngs: IngredientSnapshotItem[] = []
  try {
    const c = JSON.parse(version.ingredients_snapshot)
    for (const it of c) curIngs.push(it)
  } catch { /* ignore */ }
  try {
    const o = JSON.parse(otherVersion.ingredients_snapshot)
    for (const it of o) otherIngs.push(it)
  } catch { /* ignore */ }

  const curMap = new Map(curIngs.map(i => [i.ingredient_id, i]))
  const otherMap = new Map(otherIngs.map(i => [i.ingredient_id, i]))
  const allIds = new Set([...curMap.keys(), ...otherMap.keys()])
  const items: CompareIngredientItem[] = []

  for (const id of allIds) {
    const cur = curMap.get(id)
    const other = otherMap.get(id)
    if (cur && !other) {
      items.push({ ingredient_id: cur.ingredient_id, ingredient_name: cur.ingredient_name, category: cur.category, amount: cur.amount, changeType: 'added' })
    } else if (!cur && other) {
      items.push({ ingredient_id: other.ingredient_id, ingredient_name: other.ingredient_name, category: other.category, amount: other.amount, changeType: 'removed' })
    } else if (cur && other) {
      const diff = cur.amount - other.amount
      if (Math.abs(diff) > 0.01) {
        const sign = diff > 0 ? '+' : ''
        items.push({ ingredient_id: cur.ingredient_id, ingredient_name: cur.ingredient_name, category: cur.category, amount: cur.amount, changeType: 'modified', changeAmount: `${sign}${diff.toFixed(1)}g` })
      } else {
        items.push({ ingredient_id: cur.ingredient_id, ingredient_name: cur.ingredient_name, category: cur.category, amount: cur.amount, changeType: 'unchanged' })
      }
    }
  }
  return items
}

const compareItemsA = computed(() => {
  if (!compareVersionA.value || !compareVersionB.value) return []
  return buildCompareItems(compareVersionA.value, compareVersionB.value)
})

const compareItemsB = computed(() => {
  if (!compareVersionA.value || !compareVersionB.value) return []
  return buildCompareItems(compareVersionB.value, compareVersionA.value)
})

const conflictBaseNutrition = computed(() => {
  if (!rollbackTargetVersion.value) return { calories: 0, protein: 0, fat: 0, carbs: 0, fiber: 0, sodium: 0, calcium: 0, iron: 0, vitamin_a: 0, vitamin_c: 0 }
  const nutr = parseVersionNutrition(rollbackTargetVersion.value.nutrition_snapshot)
  let keepCalories = nutr.calories
  let keepProtein = nutr.protein
  let keepFat = nutr.fat
  let keepCarbs = nutr.carbs
  let keepFiber = nutr.fiber
  let keepSodium = nutr.sodium
  let keepCalcium = nutr.calcium
  let keepIron = nutr.iron
  let keepVitA = nutr.vitamin_a
  let keepVitC = nutr.vitamin_c
  for (const item of conflictItems.value) {
    if (item.action === 'keep') {
      keepCalories += item.calories
      keepProtein += item.protein
      keepFat += item.fat
      keepCarbs += item.carbs
      keepFiber += item.fiber
      keepSodium += item.sodium
      keepCalcium += item.calcium
      keepIron += item.iron
      keepVitA += item.vitamin_a
      keepVitC += item.vitamin_c
    }
  }
  return { calories: keepCalories, protein: keepProtein, fat: keepFat, carbs: keepCarbs, fiber: keepFiber, sodium: keepSodium, calcium: keepCalcium, iron: keepIron, vitamin_a: keepVitA, vitamin_c: keepVitC }
})

const importDialogVisible = ref(false)
const importStep = ref<'select' | 'preview'>('select')
const importPreview = ref<ImportPreview | null>(null)
const importRawJson = ref('')

const saveTemplateDialogVisible = ref(false)
const templateForm = reactive({
  name: '',
  description: '',
  tagsStr: '',
  presetId: null as number | null,
})

const currentRecipe = computed(() => recipeStore.currentRecipe)
const nutrition = computed(() => recipeStore.nutritionCalculations)

const presetViolations = computed(() => {
  const result: Record<string, { exceeded: boolean; diff: number; message: string }> = {}
  if (!nutrition.value || !currentPreset.value) return result

  const perServing = nutrition.value.perServing
  const p = currentPreset.value
  for (const nk of NUTRIENT_PRESET_KEYS) {
    const val = (perServing as any)[nk.key] as number
    const max = (p as any)[`max_${nk.key}`] as number | null
    const min = (p as any)[`min_${nk.key}`] as number | null
    if (max !== null && val > max) {
      result[nk.key] = {
        exceeded: true,
        diff: val - max,
        message: `超出上限 ${(val - max).toFixed(1)} ${nk.unit}`
      }
    } else if (min !== null && val < min) {
      result[nk.key] = {
        exceeded: false,
        diff: min - val,
        message: `低于下限 ${(min - val).toFixed(1)} ${nk.unit}`
      }
    }
  }
  return result
})

function getPresetValue(preset: NutritionPreset, key: NutrientKey, type: 'min' | 'max'): number | null {
  return (preset as any)[`${type}_${key}`] ?? null
}

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
  presets.value = await presetApi.getAll().catch(() => [])

  const recipeId = route.params.id
  if (recipeId && recipeId !== 'new') {
    await recipeStore.loadRecipe(Number(recipeId))
    if (currentRecipe.value) {
      recipeForm.name = currentRecipe.value.name
      recipeForm.category = currentRecipe.value.category
      recipeForm.servings = currentRecipe.value.servings
      recipeForm.notes = currentRecipe.value.notes
      try {
        selectedPresetId.value = await presetApi.getRecipePresetId(currentRecipe.value.id)
      } catch (_) { /* ignore */ }
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

async function handlePresetChange(val: number | null) {
  if (!currentRecipe.value) return
  try {
    await presetApi.setRecipePreset(currentRecipe.value.id, val)
    ElMessage.success(val ? '已关联营养预设' : '已取消营养预设关联')
  } catch (e: any) {
    ElMessage.error(e || '预设关联失败')
  }
}

async function openVersionsDrawer() {
  if (!currentRecipe.value) return
  versionsDrawerVisible.value = true
  versionsLoading.value = true
  expandedVersionId.value = null
  selectedCompareIds.value = []
  selectedExportIds.value = []
  compareViewVisible.value = false
  try {
    versions.value = await versionApi.getAll(currentRecipe.value.id)
  } catch (e: any) {
    ElMessage.error(e || '加载版本失败')
    versions.value = []
  } finally {
    versionsLoading.value = false
  }
}

function parseVersionNutrition(raw: string): NutrSum {
  try {
    const obj = JSON.parse(raw)
    return {
      calories: obj.calories ?? 0,
      calories_kj: obj.calories_kj ?? 0,
      protein: obj.protein ?? 0,
      fat: obj.fat ?? 0,
      carbs: obj.carbs ?? 0,
      fiber: obj.fiber ?? 0,
      sodium: obj.sodium ?? 0,
      calcium: obj.calcium ?? 0,
      iron: obj.iron ?? 0,
      vitamin_a: obj.vitamin_a ?? 0,
      vitamin_c: obj.vitamin_c ?? 0,
    }
  } catch {
    return {} as NutrSum
  }
}

function computeVersionDiff(version: RecipeVersion, prevIdx: number): VersionDiffSummary {
  const curIngs: any[] = []
  const prevIngs: any[] = []
  try {
    const c = JSON.parse(version.ingredients_snapshot)
    for (const it of c) curIngs.push({ id: it.ingredient_id ?? it.id, name: it.name, amount: it.amount })
  } catch { /* ignore */ }
  if (prevIdx < versions.value.length) {
    try {
      const p = JSON.parse(versions.value[prevIdx].ingredients_snapshot)
      for (const it of p) prevIngs.push({ id: it.ingredient_id ?? it.id, name: it.name, amount: it.amount })
    } catch { /* ignore */ }
  }

  const curMap = new Map(curIngs.map(i => [i.id, i]))
  const prevMap = new Map(prevIngs.map(i => [i.id, i]))
  const added: string[] = []
  const removed: string[] = []
  const modified: string[] = []

  for (const [id, it] of curMap) {
    const p = prevMap.get(id)
    if (!p) added.push(`${it.name} ${it.amount}g`)
    else if (Math.abs(p.amount - it.amount) > 0.001) modified.push(`${it.name}: ${p.amount}g → ${it.amount}g`)
  }
  for (const [id, it] of prevMap) {
    if (!curMap.has(id)) removed.push(`${it.name} ${it.amount}g`)
  }
  return { added, removed, modified }
}

function toggleVersionExpand(ver: RecipeVersion) {
  const idx = versions.value.findIndex(v => v.id === ver.id)
  if (expandedVersionId.value === ver.id) {
    expandedVersionId.value = null
    return
  }
  expandedVersionId.value = ver.id
  versionNoteEdit.value = ver.note || ''
  versionNutrition[ver.id] = parseVersionNutrition(ver.nutrition_snapshot)
  versionDiffs[ver.id] = computeVersionDiff(ver, idx + 1)
}

function formatVersionTime(t: string) {
  const d = new Date(t)
  const now = new Date()
  const sameDay = d.toDateString() === now.toDateString()
  const hhmm = `${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}`
  if (sameDay) return `今天 ${hhmm}`
  return `${d.getMonth() + 1}月${d.getDate()}日 ${hhmm}`
}

async function handleCreateSnapshot() {
  if (!currentRecipe.value) return
  try {
    const v = await versionApi.createSnapshot(currentRecipe.value.id, '手动保存的快照')
    versions.value = await versionApi.getAll(currentRecipe.value.id)
    ElMessage.success(`已保存版本 v${v.version_number}`)
  } catch (e: any) {
    ElMessage.error(e || '保存版本失败')
  }
}

async function handleRollback(ver: RecipeVersion) {
  if (!currentRecipe.value) return

  const currentIngs = currentRecipe.value.ingredients
  let targetIngs: IngredientSnapshotItem[] = []
  try {
    targetIngs = JSON.parse(ver.ingredients_snapshot)
  } catch { /* ignore */ }
  const targetIds = new Set(targetIngs.map(i => i.ingredient_id))
  const lostIngredients = currentIngs.filter(ri => !targetIds.has(ri.ingredient_id))

  if (lostIngredients.length > 0) {
    rollbackTargetVersion.value = ver
    conflictItems.value = lostIngredients.map(ri => {
      const ing = ri.ingredient
      const factor = ri.amount / 100
      return {
        ingredient_id: ri.ingredient_id,
        ingredient_name: ing?.name || '未知食材',
        category: ing?.category || '',
        amount: ri.amount,
        calories: (ing?.calories || 0) * factor,
        protein: (ing?.protein || 0) * factor,
        fat: (ing?.fat || 0) * factor,
        carbs: (ing?.carbs || 0) * factor,
        fiber: (ing?.fiber || 0) * factor,
        sodium: (ing?.sodium || 0) * factor,
        calcium: (ing?.calcium || 0) * factor,
        iron: (ing?.iron || 0) * factor,
        vitamin_a: (ing?.vitamin_a || 0) * factor,
        vitamin_c: (ing?.vitamin_c || 0) * factor,
        action: 'discard' as const,
      }
    })
    conflictDialogVisible.value = true
    return
  }

  try {
    await ElMessageBox.confirm(
      `确定要回退到 v${ver.version_number}（${formatVersionTime(ver.created_at)}）吗？\n这将创建一条新的版本记录。`,
      '确认回退',
      { type: 'warning' }
    )
    await versionApi.rollback(ver.id)
    versions.value = await versionApi.getAll(currentRecipe.value.id)
    expandedVersionId.value = null
    await recipeStore.loadRecipe(currentRecipe.value.id)
    if (currentRecipe.value) {
      recipeForm.name = currentRecipe.value.name
      recipeForm.category = currentRecipe.value.category
      recipeForm.servings = currentRecipe.value.servings
      recipeForm.notes = currentRecipe.value.notes
    }
  } catch (e) {
    if (e === 'cancel') return
    ElMessage.error('回退失败')
  }
}

async function executeRollbackWithMerge() {
  if (!currentRecipe.value || !rollbackTargetVersion.value) return
  const keepPairs: Array<[number, number]> = conflictItems.value
    .filter(i => i.action === 'keep')
    .map(i => [i.ingredient_id, i.amount] as [number, number])
  try {
    await versionApi.rollbackWithKeep(rollbackTargetVersion.value.id, keepPairs)
    versions.value = await versionApi.getAll(currentRecipe.value.id)
    expandedVersionId.value = null
    conflictDialogVisible.value = false
    await recipeStore.loadRecipe(currentRecipe.value.id)
    if (currentRecipe.value) {
      recipeForm.name = currentRecipe.value.name
      recipeForm.category = currentRecipe.value.category
      recipeForm.servings = currentRecipe.value.servings
      recipeForm.notes = currentRecipe.value.notes
    }
    ElMessage.success('回退成功')
  } catch (e: any) {
    ElMessage.error(e || '回退失败')
  }
}

function toggleCompareSelect(id: number, checked: boolean) {
  if (checked) {
    if (selectedCompareIds.value.length < 2) {
      selectedCompareIds.value.push(id)
    } else {
      selectedCompareIds.value.shift()
      selectedCompareIds.value.push(id)
    }
  } else {
    selectedCompareIds.value = selectedCompareIds.value.filter(i => i !== id)
  }
}

function toggleExportSelect(id: number, checked: boolean) {
  if (checked) {
    selectedExportIds.value.push(id)
  } else {
    selectedExportIds.value = selectedExportIds.value.filter(i => i !== id)
  }
}

async function handleToggleStar(ver: RecipeVersion) {
  try {
    const updated = await versionApi.toggleStar(ver.id)
    const idx = versions.value.findIndex(v => v.id === ver.id)
    if (idx !== -1) {
      versions.value[idx] = updated
    }
  } catch (e: any) {
    ElMessage.error(e || '操作失败')
  }
}

async function saveVersionNote(ver: RecipeVersion) {
  try {
    const updated = await versionApi.updateNote(ver.id, versionNoteEdit.value)
    const idx = versions.value.findIndex(v => v.id === ver.id)
    if (idx !== -1) {
      versions.value[idx] = updated
    }
    ElMessage.success('备注已保存')
  } catch (e: any) {
    ElMessage.error(e || '保存备注失败')
  }
}

function openCompareView() {
  if (selectedCompareIds.value.length !== 2) return
  compareViewVisible.value = true
  setTimeout(() => {
    renderCompareChart()
  }, 100)
}

function renderCompareChart() {
  if (!compareChartRef.value || !compareVersionA.value || !compareVersionB.value) return
  if (compareChartInstance) {
    compareChartInstance.dispose()
  }
  compareChartInstance = echarts.init(compareChartRef.value)

  const nutrA = parseVersionNutrition(compareVersionA.value.nutrition_snapshot)
  const nutrB = parseVersionNutrition(compareVersionB.value.nutrition_snapshot)

  const nutrientKeys: (keyof NutritionSummary)[] = [
    'calories', 'protein', 'fat', 'carbs', 'fiber', 'sodium', 'calcium', 'iron', 'vitamin_a', 'vitamin_c'
  ]

  const labels = nutrientKeys.map(k => NUTRIENT_LABELS[k])
  const dataA = nutrientKeys.map(k => (nutrA as any)[k] as number)
  const dataB = nutrientKeys.map(k => (nutrB as any)[k] as number)

  compareChartInstance.setOption({
    tooltip: { trigger: 'axis' },
    legend: { data: [`v${compareVersionA.value.version_number}`, `v${compareVersionB.value.version_number}`], bottom: 0 },
    grid: { left: 50, right: 20, top: 20, bottom: 40 },
    xAxis: { type: 'category', data: labels, axisLabel: { fontSize: 10, rotate: 30 } },
    yAxis: { type: 'value' },
    series: [
      { name: `v${compareVersionA.value.version_number}`, type: 'line', data: dataA, smooth: true, lineStyle: { color: '#409eff' }, itemStyle: { color: '#409eff' } },
      { name: `v${compareVersionB.value.version_number}`, type: 'line', data: dataB, smooth: true, lineStyle: { color: '#e6a23c' }, itemStyle: { color: '#e6a23c' } },
    ],
  })
}

async function handleExportVersions() {
  if (!currentRecipe.value || selectedExportIds.value.length === 0) return
  try {
    const json = await versionApi.exportVersions(currentRecipe.value.id, selectedExportIds.value)
    const filePath = await save({
      defaultPath: `${recipeForm.name}_versions.json`,
      filters: [{ name: 'JSON', extensions: ['json'] }]
    })
    if (filePath) {
      const encoder = new TextEncoder()
      await writeFile(filePath, encoder.encode(json))
      ElMessage.success(`已导出 ${selectedExportIds.value.length} 个版本`)
    }
  } catch (e: any) {
    handleExportError(e, '版本')
  }
}

async function handleImportVersions() {
  if (!currentRecipe.value) return
  try {
    const filePath = await open({
      filters: [{ name: 'JSON', extensions: ['json'] }],
      multiple: false,
    })
    if (!filePath) return
    const content = await readTextFile(filePath as string)
    const count = await versionApi.importVersions(currentRecipe.value.id, content)
    versions.value = await versionApi.getAll(currentRecipe.value.id)
    ElMessage.success(`已导入 ${count} 个版本`)
  } catch (e: any) {
    if (e?.message?.includes('cancel') || e === 'cancel') return
    ElMessage.error(e || '导入版本失败')
  }
}

function openSaveTemplateDialog() {
  if (!currentRecipe.value) return
  templateForm.name = recipeForm.name
  templateForm.description = recipeForm.notes
  templateForm.tagsStr = ''
  templateForm.presetId = selectedPresetId.value
  saveTemplateDialogVisible.value = true
}

async function executeSaveTemplate() {
  if (!currentRecipe.value || !templateForm.name.trim()) return
  try {
    await templateApi.create({
      recipe_id: currentRecipe.value.id,
      name: templateForm.name.trim(),
      description: templateForm.description,
      category: currentRecipe.value.category,
      servings: currentRecipe.value.servings,
      notes: currentRecipe.value.notes,
      tags: templateForm.tagsStr.split(/[,，]/).map(s => s.trim()).filter(Boolean),
      nutrition_preset_id: templateForm.presetId,
    })
    ElMessage.success('模板已保存')
    saveTemplateDialogVisible.value = false
  } catch (e: any) {
    ElMessage.error(e || '保存模板失败')
  }
}

function openImportDialog() {
  importStep.value = 'select'
  importPreview.value = null
  importRawJson.value = ''
  importDialogVisible.value = true
}

async function handleImportFileChange(file: any) {
  if (!file || !file.raw) return
  try {
    const reader = new FileReader()
    reader.onload = async (ev) => {
      try {
        const text = (ev.target as any).result
        importRawJson.value = text
        const preview = await exchangeApi.previewImport(text)
        importPreview.value = preview
        for (const m of preview.ingredient_matches) {
          if (m.matched_local) {
            m.action = 'use_exact'
            m.requires_confirmation = false
          } else if (m.candidates.length > 0) {
            m.action = 'use_candidate'
            m.selected_ingredient_id = m.candidates[0].local_ingredient.id
            m.requires_confirmation = true
          } else {
            m.action = 'add_new'
            m.requires_confirmation = true
          }
        }
        importStep.value = 'preview'
      } catch (e: any) {
        ElMessage.error(e || '文件解析失败')
      }
    }
    reader.readAsText(file.raw)
  } catch (e: any) {
    ElMessage.error(e || '读取文件失败')
  }
}

const canExecuteImport = computed(() => {
  if (!importPreview.value) return false
  for (const m of importPreview.value.ingredient_matches) {
    if (m.action === 'use_exact' && !m.matched_local) return false
    if (m.action === 'use_candidate' && !m.selected_ingredient_id) return false
  }
  return true
})

async function executeImport() {
  if (!importPreview.value) return
  const confirmed: ImportConfirmedItem[] = []
  for (const m of importPreview.value.ingredient_matches) {
    confirmed.push({
      exported: m.exported,
      action: m.action,
      selected_ingredient_id: m.action === 'use_candidate' ? m.selected_ingredient_id : undefined,
    })
  }
  try {
    const result = await exchangeApi.executeImport(importRawJson.value, confirmed)
    importDialogVisible.value = false
    ElMessage.success(`导入成功，${result.new_ingredients_added}种新食材已添加`)
    router.push(`/recipes/${result.recipe_id}`)
    setTimeout(() => location.reload(), 200)
  } catch (e: any) {
    ElMessage.error(e || '导入失败')
  }
}

async function exportJson() {
  if (!currentRecipe.value) return
  try {
    const json = await exchangeApi.exportJson(currentRecipe.value.id)
    const filePath = await save({
      defaultPath: `${recipeForm.name}_recipe.json`,
      filters: [{ name: 'JSON', extensions: ['json'] }]
    })
    if (filePath) {
      const encoder = new TextEncoder()
      await writeFile(filePath, encoder.encode(json))
      ElMessage.success('JSON 导出成功')
    }
  } catch (e: any) {
    ElMessage.error(e || 'JSON 导出失败')
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

  .preset-section {
    margin-bottom: 20px;
    padding: 16px;
    background: #ecf5ff;
    border-radius: 8px;

    .section-title {
      font-weight: 600;
      color: #409eff;
      margin-bottom: 10px;
    }

    .preset-info {
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 8px;
      margin-top: 10px;

      .preset-item {
        font-size: 12px;
        padding: 4px 8px;
        background: rgba(64, 158, 255, 0.1);
        border-radius: 4px;
        color: #303133;

        &.violation {
          background: rgba(245, 108, 108, 0.15);
          color: #f56c6c;
          font-weight: 500;
        }

        &.warning {
          background: rgba(230, 162, 60, 0.15);
          color: #e6a23c;
          font-weight: 500;
        }
      }
    }
  }

  .sharing-section {
    margin-top: 20px;
    padding-top: 20px;
    border-top: 1px solid #ebeef5;

    .section-title {
      margin-bottom: 16px;
      padding-bottom: 12px;
      border-bottom: 1px solid #ebeef5;
      display: flex;
      align-items: center;
      gap: 8px;
      font-weight: 600;
      color: #303133;
    }

    .sharing-grid {
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 12px;
    }
  }

  .versions-toolbar {
    margin-bottom: 16px;
    padding-bottom: 12px;
    border-bottom: 1px solid #ebeef5;
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .compare-view {
    margin-bottom: 16px;
    border: 1px solid #dcdfe6;
    border-radius: 8px;
    padding: 12px;
    background: #fafbfc;
  }

  .compare-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;

    .compare-title {
      font-weight: 600;
      font-size: 14px;
      color: #303133;
    }
  }

  .compare-columns {
    display: flex;
    gap: 12px;
  }

  .compare-col {
    flex: 1;
    min-width: 0;

    .compare-col-title {
      font-weight: 600;
      font-size: 13px;
      color: #409eff;
      text-align: center;
      margin-bottom: 8px;
      padding: 4px 0;
      background: #ecf5ff;
      border-radius: 4px;
    }
  }

  .compare-ingredient {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 12px;
    margin-bottom: 2px;

    .ci-name { flex: 1; }
    .ci-amount { color: #606266; }
    .ci-change { font-weight: 600; font-size: 11px; }

    &.added { background: #f0f9eb; }
    &.added .ci-change { color: #67c23a; }
    &.removed { background: #fef0f0; }
    &.removed .ci-change { color: #f56c6c; }
    &.modified { background: #fdf6ec; }
    &.modified .ci-change { color: #e6a23c; }
    &.unchanged { background: #f5f7fa; }
  }

  .compare-chart {
    margin-top: 16px;

    .compare-chart-title {
      font-weight: 600;
      font-size: 13px;
      color: #303133;
      margin-bottom: 8px;
    }
  }

  .versions-loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding: 40px 0;
    color: #909399;
  }

  .versions-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .conflict-intro {
    margin-bottom: 16px;
    font-size: 14px;
    color: #606266;
    line-height: 1.6;
  }

  .conflict-list {
    max-height: 400px;
    overflow-y: auto;
  }

  .conflict-item {
    border: 1px solid #ebeef5;
    border-radius: 6px;
    padding: 10px 12px;
    margin-bottom: 8px;

    .conflict-item-info {
      display: flex;
      align-items: center;
      gap: 8px;
      margin-bottom: 8px;

      .conflict-name { font-weight: 600; color: #303133; }
      .conflict-amount { color: #909399; font-size: 12px; }
    }

    .conflict-item-actions {
      margin-bottom: 6px;
    }

    .conflict-item-preview {
      font-size: 12px;

      .preview-discard { color: #f56c6c; }
      .preview-keep { color: #67c23a; }
    }
  }

  .version-item {
    border: 1px solid #ebeef5;
    border-radius: 8px;
    padding: 12px 14px;
    cursor: pointer;
    transition: all 0.2s;

    &:hover {
      border-color: #c0c4cc;
    }

    &.active {
      border-color: #409eff;
      background: #ecf5ff;
    }

    .version-header {
      display: flex;
      justify-content: space-between;
      align-items: center;

      .version-info {
        display: flex;
        align-items: center;
        gap: 6px;

        .version-badge {
          font-weight: 700;
          color: #409eff;
          font-size: 14px;
        }

        .version-time {
          font-size: 12px;
          color: #909399;
        }

        .star-btn {
          color: #c0c4cc;
          font-size: 14px;
          cursor: pointer;
          transition: color 0.2s;

          &:hover { color: #e6a23c; }
          &.starred { color: #e6a23c; }
        }
      }

      .expand-icon {
        color: #909399;
        transition: transform 0.2s;
      }
    }

    .version-summary {
      margin-top: 6px;
      font-size: 13px;
      color: #606266;
      line-height: 1.5;
    }

    .version-note {
      margin-top: 4px;
      font-size: 12px;
      color: #909399;
      line-height: 1.4;
    }

    .version-note-edit {
      margin-bottom: 12px;
    }

    .version-detail {
      margin-top: 12px;
      padding-top: 12px;
      border-top: 1px dashed #ebeef5;
    }

    .diff-section {
      margin-bottom: 14px;

      .diff-group {
        margin-bottom: 8px;

        .diff-title {
          font-size: 12px;
          font-weight: 600;
          margin-bottom: 4px;

          &.added { color: #67c23a; }
          &.removed { color: #f56c6c; }
          &.modified { color: #e6a23c; }
        }

        .diff-item {
          font-size: 12px;
          display: flex;
          align-items: center;
          gap: 4px;
          padding: 2px 0;
          color: #303133;
        }
      }

      .diff-empty {
        font-size: 12px;
        color: #909399;
        font-style: italic;
      }
    }

    .version-nutrition {
      margin-bottom: 12px;

      .vn-title {
        font-size: 12px;
        font-weight: 600;
        color: #303133;
        margin-bottom: 6px;
      }

      .vn-grid {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 6px;

        .vn-item {
          display: flex;
          justify-content: space-between;
          padding: 4px 8px;
          background: #f5f7fa;
          border-radius: 4px;
          font-size: 12px;

          .label { color: #606266; }
          .value { font-weight: 500; color: #409eff; }
        }
      }
    }

    .version-actions {
      display: flex;
      justify-content: flex-end;
    }
  }

  .import-step-intro {
    p {
      margin: 0 0 16px;
      color: #606266;
    }
  }

  .import-preview-header {
    margin-bottom: 20px;
  }

  .section-sub-title {
    font-size: 14px;
    font-weight: 600;
    color: #303133;
    margin: 16px 0 10px;
  }

  .import-match-list {
    max-height: 360px;
    overflow-y: auto;
  }

  .match-item {
    border: 1px solid #ebeef5;
    border-radius: 6px;
    padding: 10px 12px;
    margin-bottom: 8px;

    &.needs-review {
      border-color: #e6a23c;
      background: #fdf6ec;
    }

    .match-exported {
      display: flex;
      align-items: center;
      gap: 6px;
      margin-bottom: 8px;
      flex-wrap: wrap;

      .match-name {
        font-weight: 600;
        color: #303133;
      }

      .match-amount {
        color: #909399;
        font-size: 12px;
      }
    }

    .match-actions {
      display: flex;
      flex-direction: column;
      gap: 8px;

      .match-selected,
      .match-new-info,
      .candidate-list {
        padding-left: 12px;
        font-size: 13px;
      }

      .match-meta {
        color: #909399;
        font-size: 12px;
        margin-left: 4px;
      }

      .match-nutrition {
        color: #606266;
        font-size: 12px;
        margin-left: 6px;
        font-family: monospace;
      }
    }
  }

  .share-card {
    :deep(.el-card__body) {
      padding: 12px;
    }

    .card-title {
      font-weight: 600;
      font-size: 13px;
      color: #303133;
      margin-bottom: 6px;
      display: flex;
      align-items: center;
      gap: 6px;
    }

    .card-desc {
      font-size: 12px;
      color: #606266;
      line-height: 1.5;
      margin-bottom: 10px;
      min-height: 36px;
    }

    .card-actions {
      display: flex;
      gap: 6px;
    }
  }

  .nutrient-warning-icon {
    display: inline-flex;
    margin-left: 4px;
    vertical-align: middle;
    cursor: help;
  }

  .violation-tag {
    display: inline-flex;
    align-items: center;
    margin-left: 6px;
    font-size: 12px;
  }
}
</style>
