<script setup lang="ts">
import Button from "primevue/button"

defineProps<{
  connected: boolean
}>()

const emit = defineEmits<{
  refresh: []
}>()
</script>

<template>
  <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
    <div>
      <h1 class="mt-1 text-3xl font-bold text-surface-900 dark:text-white">Dashboard</h1>
      <p class="mt-1 text-sm text-surface-500 dark:text-surface-400">
        Real-time trading overview and performance metrics
      </p>
    </div>
    <div class="flex items-center gap-3">
      <div
        class="flex items-center gap-2 px-3 h-11 rounded-xl text-xs bg-surface-50 dark:bg-surface-900 border border-surface-200 dark:border-surface-800 transition-colors"
      >
        <span
          class="w-1.5 h-1.5 rounded-full"
          :class="connected ? 'bg-success animate-pulse' : 'bg-surface-400 dark:bg-surface-600'"
        ></span>
        <span class="text-surface-600 dark:text-surface-400 font-semibold">
          {{ connected ? "Live Connection" : "Offline" }}
        </span>
      </div>
      <Button
        icon="pi pi-refresh"
        label="Refresh"
        @click="emit('refresh')"
        class="rounded-xl px-4 h-11"
      />
    </div>
  </div>
</template>
