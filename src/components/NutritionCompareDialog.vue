<template>
  <el-dialog
    v-model="visible"
    title="营养素对比"
    width="900px"
    :close-on-click-modal="false"
    @open="handleOpen"
  >
    <div class="compare-dialog">
      <div class="selected-ingredients">
        <span class="label">已选食材 ({{ ingredients.length }}/4)：</span>
        <el-tag
          v-for="ing in ingredients"
          :key="ing.id"
          closable
          type="primary"
          style="margin-right: 8px;"
          @close="handleRemove(ing.id)"
        >
          {{ ing.name }}
        </el-tag>
      </div>

      <div class="chart-container">
        <div ref="chartRef" style="width: 100%; height: 400px;"></div>
      </div>

      <div class="compare-table-section">
        <div class="section-title">详细数据对比</div>
        <el-table :data="compareTableData" size="small" border stripe>
          <el-table-column prop="nutrient" label="营养素" width="100" fixed />
          <el-table-column
            v-for="(ing, index) in ingredients"
            :key="ing.id"
            :label="ing.name"
            align="right"
            min-width="120"
          >
            <template #default="{ row }">
              <span :class="{ 'max-value': isMaxValue(row.nutrientKey, index) }">
                {{ formatNutrientValue(row.nutrientKey, row.values[index]) }}
              </span>
            </template>
          </el-table-column>
          <el-table-column
            v-if="ingredients.length >= 2"
            label="差异 (最高 vs 最低)"
            align="right"
            width="160"
            fixed="right"
          >
            <template #default="{ row }">
              <span class="diff-text">
                +{{ formatNutrientValue(row.nutrientKey, row.diffValue) }}
                (+{{ row.diffPercentage.toFixed(1) }}%)
              </span>
            </template>
          </el-table-column>
        </el-table>
      </div>
    </div>

    <template #footer>
      <el-button @click="visible = false">关闭</el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onBeforeUnmount } from 'vue'
import * as echarts from 'echarts'
import type { Ingredient } from '@/types'
import { NUTRIENT_LABELS, NUTRIENT_UNITS } from '@/types'

interface Props {
  modelValue: boolean
  ingredients: Ingredient[]
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'remove', id: number): void
}>()

const chartRef = ref<HTMLElement | null>(null)
let chartInstance: echarts.ECharts | null = null

const visible = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val)
})

const NUTRIENT_KEYS = [
  'calories', 'protein', 'fat', 'carbs', 'fiber',
  'sodium', 'calcium', 'iron', 'vitamin_a', 'vitamin_c'
]

const CHART_COLORS = ['#409eff', '#67c23a', '#e6a23c', '#f56c6c']

const compareTableData = computed(() => {
  return NUTRIENT_KEYS.map(key => {
    const values = props.ingredients.map(ing => ing[key as keyof Ingredient] as number)
    const maxVal = Math.max(...values)
    const minVal = Math.min(...values)
    const diffValue = maxVal - minVal
    const diffPercentage = minVal === 0 ? (maxVal === 0 ? 0 : 100) : ((maxVal - minVal) / minVal) * 100

    return {
      nutrient: NUTRIENT_LABELS[key as keyof typeof NUTRIENT_LABELS],
      nutrientKey: key,
      values,
      maxValue: maxVal,
      minValue: minVal,
      diffValue,
      diffPercentage
    }
  })
})

function formatNutrientValue(key: string, value: number): string {
  const unit = NUTRIENT_UNITS[key as keyof typeof NUTRIENT_UNITS] || ''
  if (key === 'sodium' || key === 'calcium' || key === 'vitamin_a') {
    return `${value.toFixed(0)} ${unit}`
  }
  if (key === 'iron') {
    return `${value.toFixed(1)} ${unit}`
  }
  return `${value.toFixed(1)} ${unit}`
}

function isMaxValue(nutrientKey: string, index: number): boolean {
  const row = compareTableData.value.find(r => r.nutrientKey === nutrientKey)
  if (!row) return false
  return row.values[index] === row.maxValue && row.maxValue > 0
}

function initChart() {
  if (!chartRef.value) return
  
  chartInstance = echarts.init(chartRef.value)
  updateChart()
}

function updateChart() {
  if (!chartInstance || props.ingredients.length === 0) return

  const indicators = NUTRIENT_KEYS.map(key => 
    NUTRIENT_LABELS[key as keyof typeof NUTRIENT_LABELS]
  )

  const series = props.ingredients.map((ing, index) => ({
    name: ing.name,
    type: 'bar',
    data: NUTRIENT_KEYS.map(key => Number((ing[key as keyof Ingredient] as number).toFixed(2))),
    itemStyle: {
      color: CHART_COLORS[index % CHART_COLORS.length]
    },
    barMaxWidth: 40
  }))

  const option: echarts.EChartsOption = {
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        type: 'shadow'
      },
      formatter: (params: any) => {
        if (!params || params.length === 0) return ''
        const nutrientName = params[0].axisValue
        let result = `<strong>${nutrientName}</strong><br/>`
        for (const p of params) {
          result += `${p.marker} ${p.seriesName}: ${p.value} ${getUnit(p.axisValue)}<br/>`
        }
        return result
      }
    },
    legend: {
      data: props.ingredients.map(ing => ing.name),
      top: 0
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      top: 50,
      containLabel: true
    },
    xAxis: {
      type: 'category',
      data: indicators,
      axisLabel: {
        interval: 0,
        rotate: 30,
        fontSize: 12
      }
    },
    yAxis: {
      type: 'value',
      name: '含量 (每100g)',
      nameTextStyle: {
        fontSize: 12
      }
    },
    series
  }

  chartInstance.setOption(option, true)
}

function getUnit(nutrientName: string): string {
  const key = NUTRIENT_KEYS.find(k => 
    NUTRIENT_LABELS[k as keyof typeof NUTRIENT_LABELS] === nutrientName
  )
  if (!key) return ''
  return NUTRIENT_UNITS[key as keyof typeof NUTRIENT_UNITS] || ''
}

function handleOpen() {
  nextTick(() => {
    if (!chartInstance) {
      initChart()
    } else {
      updateChart()
      chartInstance.resize()
    }
  })
}

function handleRemove(id: number) {
  emit('remove', id)
}

function handleResize() {
  chartInstance?.resize()
}

watch(() => props.ingredients, () => {
  nextTick(() => {
    updateChart()
  })
}, { deep: true })

watch(() => props.modelValue, (newVal) => {
  if (newVal) {
    nextTick(() => {
      if (!chartInstance) {
        initChart()
      } else {
        updateChart()
        chartInstance.resize()
      }
    })
  }
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', handleResize)
  if (chartInstance) {
    chartInstance.dispose()
    chartInstance = null
  }
})
</script>

<style scoped lang="scss">
.compare-dialog {
  .selected-ingredients {
    margin-bottom: 16px;
    padding: 12px;
    background: #f5f7fa;
    border-radius: 6px;

    .label {
      color: #606266;
      margin-right: 8px;
    }
  }

  .chart-container {
    margin-bottom: 20px;
    padding: 10px;
    border: 1px solid #ebeef5;
    border-radius: 6px;
  }

  .compare-table-section {
    .section-title {
      font-size: 14px;
      font-weight: 600;
      color: #303133;
      margin-bottom: 12px;
      padding-bottom: 8px;
      border-bottom: 1px solid #ebeef5;
    }
  }

  .max-value {
    font-weight: 600;
    color: #409eff;
  }

  .diff-text {
    color: #f56c6c;
    font-weight: 500;
  }
}
</style>
