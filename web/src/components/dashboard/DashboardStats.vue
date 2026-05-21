<script setup lang="ts">
import StatCard from "@/components/StatCard.vue"
import type {
  DashboardEquitySnapshot,
  DashboardTrader,
} from "@/types/dashboard-ui"

defineProps<{
  equity: DashboardEquitySnapshot
  traders: DashboardTrader[]
  loading: boolean
  initialLoadDone: boolean
}>()
</script>

<template>
  <div class="grid grid-cols-2 gap-4 lg:grid-cols-4">
    <StatCard
      title="Total Equity"
      :value="equity.equity"
      prefix="$"
      :loading="!equity.loaded"
    />
    <StatCard
      title="Available Cash"
      :value="equity.available_cash"
      prefix="$"
      :loading="!equity.loaded"
    />
    <StatCard
      title="Unrealized PnL"
      :value="equity.unrealized_pnl"
      prefix="$"
      :signed="true"
      :loading="!equity.loaded"
    />
    <StatCard
      title="Active Traders"
      :value="traders.filter((trader) => trader.is_running).length"
      :loading="!initialLoadDone && loading"
    />
  </div>
</template>
