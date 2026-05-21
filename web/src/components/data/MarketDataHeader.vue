<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"

defineProps<{
  symbols: string[]
}>()

const activeSymbol = defineModel<string>("symbol", { required: true })
const activeInterval = defineModel<string>("interval", { required: true })

const emit = defineEmits<{
  refresh: []
}>()

const intervals = ["1m", "5m", "15m", "1h", "4h", "1d"] as const
</script>

<template>
  <div class="flex items-center justify-between flex-wrap gap-4">
    <div>
      <h1 class="text-2xl font-black">Market Data</h1>
      <p class="text-sm mt-0.5 text-[--color-text-muted]">
        Live candlestick charts and symbol data
      </p>
    </div>
    <div class="flex items-center gap-2">
      <select
        v-model="activeSymbol"
        class="py-1.5 min-w-30"
        @change="emit('refresh')"
      >
        <option v-for="sym in symbols" :key="sym" :value="sym">
          {{ sym }}
        </option>
      </select>
      <select
        v-model="activeInterval"
        class="py-1.5 min-w-20"
        @change="emit('refresh')"
      >
        <option v-for="interval in intervals" :key="interval" :value="interval">
          {{ interval }}
        </option>
      </select>
      <BaseButton @click="emit('refresh')" class="py-1.5 text-xs text-accent">
        <Icon
          icon="ic:round-refresh"
          class="inline-block text-base align-[-0.125em]"
        />
        Refresh
      </BaseButton>
    </div>
  </div>
</template>
