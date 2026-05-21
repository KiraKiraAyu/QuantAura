<script setup lang="ts">
import BaseButton from "@/components/universal/BaseButton.vue"
import EquityChart from "@/components/EquityChart.vue"

defineProps<{
  traderIds: string[]
  activeTraderId: string
  data: { time: number; value: number }[]
  traderName: (id: string) => string
}>()

const emit = defineEmits<{
  select: [traderId: string]
}>()
</script>

<template>
  <div>
    <div class="flex items-center justify-between mb-4">
      <h2 class="font-bold text-sm">Equity Curve</h2>
      <div class="flex gap-1">
        <BaseButton
          v-for="traderId in traderIds"
          :key="traderId"
          @click="emit('select', traderId)"
          class="px-2.5 py-1 rounded-md text-xs transition-all"
          :class="
            activeTraderId === traderId
              ? 'bg-surface-overlay text-accent font-semibold'
              : 'text-text-muted hover:text-primary'
          "
        >
          {{ traderName(traderId) }}
        </BaseButton>
      </div>
    </div>
    <EquityChart :data="data" :height="200" />
  </div>
</template>
