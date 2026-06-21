<template>
  <el-container class="app-container">
    <el-aside width="220px" class="sidebar">
      <div class="logo">
        <el-icon :size="32" color="#409eff">
          <Food />
        </el-icon>
        <span class="title">营养配方管理</span>
      </div>
      <el-menu
        :default-active="activeMenu"
        class="sidebar-menu"
        @select="handleMenuSelect"
      >
        <el-menu-item index="/recipes">
          <el-icon><Notebook /></el-icon>
          <span>配方管理</span>
        </el-menu-item>
        <el-menu-item index="/templates">
          <el-icon><CollectionTag /></el-icon>
          <span>配方模板</span>
        </el-menu-item>
        <el-menu-item index="/ingredients">
          <el-icon><Collection /></el-icon>
          <span>食材库</span>
        </el-menu-item>
        <el-menu-item index="/settings">
          <el-icon><Setting /></el-icon>
          <span>设置</span>
        </el-menu-item>
      </el-menu>
    </el-aside>
    <el-container>
      <el-header class="header">
        <div class="header-content">
          <h2>{{ pageTitle }}</h2>
          <div class="header-actions">
            <slot name="header-actions"></slot>
          </div>
        </div>
      </el-header>
      <el-main class="main-content">
        <router-view v-slot="{ Component }">
          <component :is="Component" />
        </router-view>
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Food, Notebook, Collection, CollectionTag, Setting } from '@element-plus/icons-vue'

const route = useRoute()
const router = useRouter()

const activeMenu = computed(() => {
  if (route.path.startsWith('/recipes')) return '/recipes'
  if (route.path.startsWith('/templates')) return '/templates'
  if (route.path.startsWith('/ingredients')) return '/ingredients'
  if (route.path.startsWith('/settings')) return '/settings'
  return '/recipes'
})

const pageTitle = computed(() => {
  if (route.path === '/recipes' || route.path === '/') return '配方列表'
  if (route.path === '/ingredients') return '食材库管理'
  if (route.path === '/recipes/new') return '新建配方'
  if (route.path.startsWith('/recipes/')) return '配方详情'
  if (route.path === '/templates') return '配方模板库'
  if (route.path === '/settings') return '系统设置'
  return ''
})

function handleMenuSelect(index: string) {
  router.push(index)
}
</script>

<style scoped lang="scss">
.app-container {
  height: 100vh;
  background: #f5f7fa;
}

.sidebar {
  background: #001529;
  color: #fff;
  display: flex;
  flex-direction: column;

  .logo {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 20px 24px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);

    .title {
      font-size: 18px;
      font-weight: 600;
      color: #fff;
    }
  }

  .sidebar-menu {
    flex: 1;
    border-right: none;
    background: transparent;
  }

  :deep(.el-menu-item) {
    color: rgba(255, 255, 255, 0.75);
    
    &.is-active {
      background: #409eff;
      color: #fff;
    }
    
    &:hover {
      background: rgba(64, 158, 255, 0.2);
      color: #fff;
    }
  }

  :deep(.el-menu-item.is-active) {
    background: #409eff;
  }
}

.header {
  background: #fff;
  border-bottom: 1px solid #e4e7ed;
  padding: 0 24px;
  display: flex;
  align-items: center;

  .header-content {
    width: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;

    h2 {
      margin: 0;
      font-size: 20px;
      font-weight: 600;
      color: #303133;
    }
  }
}

.main-content {
  padding: 24px;
  overflow-y: auto;
}
</style>
