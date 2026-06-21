<script setup lang="ts">
import Card from "primevue/card"
import type { EditableStrategy } from "@/types/strategy-ui"
import { formatDate } from "@/utils/format"

defineProps<{
  strategies: EditableStrategy[]
  selectedId?: string
  loading: boolean
}>()

const emit = defineEmits<{
  select: [strategy: EditableStrategy]
}>()
</script>

<template>
  <div class="flex flex-col gap-3">
    <div
      v-for="strategy in strategies"
      :key="strategy.id"
      @click="emit('select', strategy)"
      class="cursor-pointer transition-all duration-200"
    >
      <Card
        class="border transition-all duration-200 shadow-none! select-none"
        :class="
          selectedId === strategy.id
            ? 'border-primary bg-primary-50/10 dark:bg-primary-950/20'
            : 'border-surface-200 dark:border-surface-800 bg-surface-0 dark:bg-surface-900 hover:border-surface-300 dark:hover:border-surface-700'
        "
      >
        <template #content>
          <div class="flex items-center justify-between mb-2">
            <span class="font-bold text-sm text-surface-900 dark:text-white truncate">{{ strategy.name }}</span>
            <span
              v-if="strategy.is_active"
              class="text-[9px] font-black uppercase tracking-wider bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 px-1.5 py-0.5 rounded"
            >
              Active
            </span>
          </div>
          <p class="text-xs line-clamp-2 text-surface-500 dark:text-surface-400">
            {{ strategy.description || "No description provided." }}
          </p>
          <p class="text-[10px] mt-3 text-surface-400 dark:text-surface-500 font-medium">
            Updated {{ formatDate(strategy.updated_at) }}
          </p>
        </template>
      </Card>
    </div>
    <div
      v-if="!loading && strategies.length === 0"
      class="text-center py-12 text-sm text-surface-400 dark:text-surface-500 border border-dashed border-surface-200 dark:border-surface-800 rounded-2xl bg-surface-50/50 dark:bg-surface-950/20"
    >
      No strategies created yet.
    </div>
    <div
      v-if="loading"
      class="text-center py-12 text-sm text-surface-400 dark:text-surface-500"
    >
      <span class="pi pi-spin pi-spinner mr-2"></span>
      Loading strategies...
    </div>
  </div>
</template>
