<template>
  <div class="radar-chart-container">
    <div ref="chartRef" style="width: 100%; height: 400px;"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, onBeforeUnmount } from 'vue'
import * as echarts from 'echarts'
import type { DRIReference } from '@/types'
import { DRI_VALUES, NUTRIENT_LABELS } from '@/types'

interface Props {
  driComparison: Record<string, {
    value: number
    percentage: number
    status: 'normal' | 'warning' | 'excess'
  }>
}

const props = defineProps<Props>()

const chartRef = ref<HTMLElement | null>(null)
let chartInstance: echarts.ECharts | null = null

function getStatusColor(status: 'normal' | 'warning' | 'excess'
): string {
  switch (status) {
    case 'normal':
      return '#67c23a'
    case 'warning':
      return '#e6a23c'
    case 'excess':
      return '#f56c6c'
    default:
      return '#67c23a'
  }
}

function initChart() {
  if (!chartRef.value) return
  
  chartInstance = echarts.init(chartRef.value)
  updateChart()
}

function updateChart() {
  if (!chartInstance) return

  const nutrientKeys = ['calories', 'protein', 'fat', 'carbs', 'fiber', 'sodium', 'calcium', 'iron', 'vitamin_a', 'vitamin_c']
  
  const indicators = nutrientKeys.map(key => ({
    name: NUTRIENT_LABELS[key as keyof typeof NUTRIENT_LABELS] || key,
    max: 200
  }))

  const values = nutrientKeys.map(key => {
    const data = props.driComparison[key]
    return data ? Math.min(data.percentage, 200) : 0
  })

  const itemColors = nutrientKeys.map(key => {
    const data = props.driComparison[key]
    return data ? getStatusColor(data.status) : '#67c23a'
  })

  const option: echarts.EChartsOption = {
    tooltip: {
      trigger: 'item',
      formatter: (params: any) => {
        const key = nutrientKeys[params.dataIndex]
        const data = props.driComparison[key]
        if (!data) return ''
        const statusText = data.status === 'normal' ? '正常' : data.status === 'warning' ? '不足' : '超标'
        return `${NUTRIENT_LABELS[key as keyof typeof NUTRIENT_LABELS]}<br/>
          含量: ${data.value.toFixed(1)} ${key === 'sodium' || key === 'calcium' || key === 'iron' ? 'mg' : key === 'vitamin_a' ? 'μgRAE' : 'g'}<br/>
          占DRI: ${data.percentage.toFixed(1)}%<br/>
          状态: <span style="color: ${getStatusColor(data.status)}">${statusText}</span>`
      }
    },
    legend: {
      data: ['营养素占DRI比例'],
      bottom: 10
    },
    radar: {
      indicator: indicators,
      shape: 'polygon',
      splitNumber: 4,
      axisName: {
        color: '#333',
        fontSize: 12
      },
      splitLine: {
        lineStyle: {
          color: ['#e5e5e5', '#ddd', '#bbb', '#999']
        }
      },
      splitArea: {
        show: true,
        areaStyle: {
          color: ['rgba(250, 250, 250)', 'rgba(245, 245, 245)', 'rgba(235, 235, 235)', 'rgba(225, 225, 225)']
        }
      },
      axisLine: {
        lineStyle: {
          color: '#ccc'
        }
      }
    },
    series: [
      {
        name: '营养素占DRI比例',
        type: 'radar',
        data: [
          {
            value: values,
            name: '营养素占DRI比例',
            symbol: 'circle',
            symbolSize: 8,
            lineStyle: {
              color: '#409eff',
              width: 2
            },
            areaStyle: {
              color: 'rgba(64, 158, 255, 0.3)'
            },
            itemStyle: {
              color: (params: any) => itemColors[params.dataIndex]
            }
          }
        ]
      }
    ]
  }

  chartInstance.setOption(option)
}

watch(() => props.driComparison, () => {
  updateChart()
}, { deep: true })

onMounted(() => {
  initChart()
  window.addEventListener('resize', handleResize)
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', handleResize)
  if (chartInstance) {
    chartInstance.dispose()
  }
})

function handleResize() {
  chartInstance?.resize()
}

defineExpose({
  getChart: () => chartInstance
})
</script>

<style scoped>
.radar-chart-container {
  width: 100%;
  height: 100%;
}
</style>
