<script setup lang="ts">
import type { DebateSession } from "@/types/debate-ui"
import { formatDateTime } from "@/utils/format"

defineProps<{
  debates: DebateSession[]
  activeId?: string
  loading: boolean
}>()

const emit = defineEmits<{
  select: [debate: DebateSession]
}>()
</script>

<template>
  <div class="flex flex-col gap-3">
    <h2 class="font-bold text-sm text-[--color-text-muted]">Sessions</h2>
    <div
      v-for="debate in debates"
      :key="debate.id"
      @click="emit('select', debate)"
      class="cursor-pointer transition-all duration-150"
      :class="
        activeId === debate.id
          ? 'ring-2 ring-accent'
          : 'hover:border-border'
      "
    >
      <div class="flex items-center justify-between mb-1">
        <span class="font-semibold text-sm truncate">
          {{ debate.name || debate.symbol }}
        </span>
        <span>{{ debate.status }}</span>
      </div>
      <p class="text-xs text-[--color-text-muted]">
        {{ debate.symbol }} · {{ debate.max_rounds }} rounds ·
        {{ debate.current_round }} done
      </p>
      <p class="text-xs mt-1 text-border">
        {{ formatDateTime(debate.created_at) }}
      </p>
    </div>
    <div
      v-if="!loading && debates.length === 0"
      class="text-center py-8 text-sm text-[--color-text-muted]"
    >
      No debates yet. Create one!
    </div>
  </div>
</template>
