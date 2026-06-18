<script setup lang="ts">
import Button from "primevue/button"
import Select from "primevue/select"
import type { TraderPayload } from "@/types/trading"

defineProps<{
  traders: TraderPayload[]
}>()

const activeTrader = defineModel<string>({ required: true })

const emit = defineEmits<{
  refresh: []
}>()
</script>

<template>
  <div class="flex items-center justify-between flex-wrap gap-4">
    <div>
      <h1 class="text-2xl font-black text-surface-900 dark:text-white">System Monitor</h1>
      <p class="text-sm mt-0.5 text-surface-500">
        Runtime metrics, alerts, and system events
      </p>
    </div>
    <div class="flex flex-wrap items-center gap-2">
      <Select
        v-model="activeTrader"
        :options="[{ id: '', name: '(All System)' }, ...traders]"
        optionLabel="name"
        optionValue="id"
        class="w-48"
        @change="emit('refresh')"
      />
      <Button 
        label="Refresh" 
        icon="pi pi-refresh" 
        severity="secondary" 
        variant="text" 
        class="shrink-0"
        @click="emit('refresh')" 
      />
    </div>
  </div>
</template>
