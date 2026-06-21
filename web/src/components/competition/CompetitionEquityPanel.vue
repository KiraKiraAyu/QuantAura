<script setup lang="ts">
import EquityChart from "@/components/EquityChart.vue"
import type { CompetitionTrader, EquityPoint } from "@/types/competition-ui"

defineProps<{
  trader: CompetitionTrader
  data: EquityPoint[]
}>()
</script>

<template>
  <div class="pt-2">
    <EquityChart :data="data" :height="320" />
  </div>
</template>
