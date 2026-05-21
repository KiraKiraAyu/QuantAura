<script setup lang="ts">
import CandlestickChart from "@/components/CandlestickChart.vue"
import type { CandlePoint } from "@/types/data-ui"

defineProps<{
  loading: boolean
  data: CandlePoint[]
  activeSymbol: string
}>()
</script>

<template>
  <div class="flex-1 flex flex-col min-h-125">
    <div
      v-if="loading"
      class="flex-1 flex items-center justify-center text-sm text-[--color-text-muted]"
    >
      Loading market data...
    </div>
    <CandlestickChart
      v-else-if="data.length"
      :data="data"
      :height="500"
      class="flex-1"
    />
    <div
      v-else
      class="flex-1 flex items-center justify-center text-sm text-[--color-text-muted]"
    >
      No market data available for {{ activeSymbol }}.
    </div>
  </div>
</template>
