<template>
  <div ref="labelRef" class="nutrition-label">
    <div class="label-title">营养成分表</div>
    <div class="servings">每份 {{ servingSize }} 克</div>
    
    <div class="calories-section">
      <div class="calories-label">能量</div>
      <div class="calories-value">{{ nrvData.energy.value }} kJ</div>
      <div style="font-size: 11px; margin-top: 4px;">
        ({{ Math.round(nrvData.energy.value / 4.184) }} 千卡)
      </div>
    </div>

    <div class="nutrient-row thick-border">
      <span></span>
      <span><strong>营养素参考值%</strong></span>
    </div>

    <div class="nutrient-row bold">
      <span>蛋白质 {{ nrvData.protein.value }} 克</span>
      <span>{{ nrvData.protein.percentage }}%</span>
    </div>

    <div class="nutrient-row bold">
      <span>脂肪 {{ nrvData.fat.value }} 克</span>
      <span>{{ nrvData.fat.percentage }}%</span>
    </div>

    <div class="nutrient-row bold">
      <span>碳水化合物 {{ nrvData.carbs.value }} 克</span>
      <span>{{ nrvData.carbs.percentage }}%</span>
    </div>

    <div class="nutrient-row bold thick-border">
      <span>钠 {{ nrvData.sodium.value }} 毫克</span>
      <span>{{ nrvData.sodium.percentage }}%</span>
    </div>

    <div class="nrv-note">
      * 营养素参考值(NRV)：能量 8400kJ，蛋白质 60g，脂肪 60g，碳水化合物 300g，钠 2000mg。
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface NRVItem {
  value: number
  nrv: number
  percentage: number
}

interface Props {
  nrvData: {
    energy: NRVItem
    protein: NRVItem
    fat: NRVItem
    carbs: NRVItem
    sodium: NRVItem
  }
  servingSize: number
}

defineProps<Props>()

const labelRef = ref<HTMLElement | null>(null)

defineExpose({
  getElement: () => labelRef.value
})
</script>

<style scoped lang="scss">
.nutrition-label {
  font-family: Arial, sans-serif;
  width: 320px;
  border: 2px solid #000;
  padding: 8px;
  background: #fff;
  color: #000;
  
  .label-title {
    font-size: 22px;
    font-weight: 900;
    text-align: center;
    margin-bottom: 4px;
    border-bottom: 1px solid #000;
    padding-bottom: 4px;
  }
  
  .servings {
    font-size: 11px;
    margin: 8px 0;
  }
  
  .calories-section {
    padding: 8px 0;
    border-bottom: 8px solid #000;
    
    .calories-label {
      font-size: 14px;
      font-weight: 700;
    }
    
    .calories-value {
      font-size: 36px;
      font-weight: 900;
    }
  }
  
  .nutrient-row {
    display: flex;
    justify-content: space-between;
    padding: 4px 0;
    font-size: 11px;
    border-bottom: 1px solid #ccc;
    
    &.bold {
      font-weight: 700;
    }
    
    &.thick-border {
      border-bottom-width: 4px;
      border-bottom-color: #000;
    }
  }
  
  .nrv-note {
    font-size: 10px;
    color: #333;
    margin-top: 8px;
    padding-top: 8px;
    border-top: 1px solid #ccc;
    line-height: 1.4;
  }
}
</style>
