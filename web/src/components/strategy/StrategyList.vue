<script setup lang="ts">
import type { EditableStrategy } from "@/types/strategy-ui"

defineProps<{
  strategies: EditableStrategy[]
  selectedId?: string
  loading: boolean
}>()

const emit = defineEmits<{
  select: [strategy: EditableStrategy]
}>()

function fmtDate(value: string) {
  return new Date(value).toLocaleDateString()
}
</script>

<template>
  <div class="flex flex-col gap-3">
    <div
      v-for="strategy in strategies"
      :key="strategy.id"
      @click="emit('select', strategy)"
      class="cursor-pointer transition-all duration-150"
      :class="
        selectedId === strategy.id
          ? 'ring-2 ring-accent'
          : 'hover:border-border'
      "
    >
      <div class="flex items-center justify-between mb-2">
        <span class="font-semibold text-sm truncate">{{ strategy.name }}</span>
        <span v-if="strategy.is_active">Active</span>
      </div>
      <p class="text-xs line-clamp-2 text-[--color-text-muted]">
        {{ strategy.description || "No description" }}
      </p>
      <p class="text-xs mt-2 text-[--color-border]">
        Updated {{ fmtDate(strategy.updated_at) }}
      </p>
    </div>
    <div
      v-if="!loading && strategies.length === 0"
      class="text-center py-10 text-sm text-[--color-text-muted]"
    >
      No strategies yet
    </div>
    <div
      v-if="loading"
      class="text-center py-10 text-sm text-[--color-text-muted]"
    >
      Loading...
    </div>
  </div>
</template>
