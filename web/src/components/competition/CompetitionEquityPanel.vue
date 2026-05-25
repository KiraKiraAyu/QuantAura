<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import EquityChart from "@/components/EquityChart.vue"
import type { CompetitionTrader, EquityPoint } from "@/types/competition-ui"

defineProps<{
  trader: CompetitionTrader
  data: EquityPoint[]
}>()

const emit = defineEmits<{
  close: []
}>()
</script>

<template>
  <div>
    <div class="flex items-center justify-between mb-4">
      <h2 class="font-bold text-sm">{{ trader.trader_name }} - Equity Curve</h2>
      <BaseButton
        @click="emit('close')"
        class="text-xs text-[--color-text-muted]"
      >
        <Icon
          icon="ic:round-close"
          class="inline-block text-base align-[-0.125em]"
        />
        Close
      </BaseButton>
    </div>
    <EquityChart :data="data" :height="260" />
  </div>
</template>
