<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"

defineProps<{
  connected: boolean
}>()

const emit = defineEmits<{
  refresh: []
}>()
</script>

<template>
  <div class="flex items-center justify-between flex-wrap gap-3">
    <div>
      <h1 class="text-2xl font-black">Dashboard</h1>
      <p class="text-sm mt-0.5 text-[--color-text-muted]">
        Real-time trading overview
      </p>
    </div>
    <div class="flex items-center gap-3">
      <div
        class="flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs bg-[--color-surface-elevated] border border-[--color-border-subtle]"
      >
        <span
          class="w-1.5 h-1.5 rounded-full"
          :class="connected ? 'bg-success' : 'bg-text-muted'"
        ></span>
        <span class="text-[--color-text-secondary]">
          {{ connected ? "Live" : "Connecting..." }}
        </span>
      </div>
      <BaseButton @click="emit('refresh')" class="text-xs py-1.5 px-3">
        <Icon
          icon="ic:round-refresh"
          class="inline-block text-base align-[-0.125em]"
        />
        Refresh
      </BaseButton>
    </div>
  </div>
</template>
