<script setup lang="ts">
import CreateTraderModal from "@/components/CreateTraderModal.vue"
import DashboardHeader from "@/components/dashboard/DashboardHeader.vue"
import DashboardStats from "@/components/dashboard/DashboardStats.vue"
import EquityCurvePanel from "@/components/dashboard/EquityCurvePanel.vue"
import LiveEventsPanel from "@/components/dashboard/LiveEventsPanel.vue"
import OpenPositionsPanel from "@/components/dashboard/OpenPositionsPanel.vue"
import TradersPanel from "@/components/dashboard/TradersPanel.vue"
import { useDashboardPage } from "@/composables/useDashboardPage"

const {
  activeChart,
  closePosition,
  connected,
  equity,
  equityHistory,
  events,
  handleTraderCreated,
  initialLoadDone,
  loadAll,
  loadError,
  loading,
  positions,
  selectEquityTrader,
  showCreateTrader,
  startTrader,
  stopTrader,
  syncBalance,
  traderIdOptions,
  traderName,
  traders,
} = useDashboardPage()
</script>

<template>
  <div class="flex flex-col">
    <DashboardHeader :connected="connected" @refresh="loadAll" />

    <DashboardStats
      :equity="equity"
      :traders="traders"
      :loading="loading"
      :initial-load-done="initialLoadDone"
    />

    <div
      v-if="loadError"
      class="text-xs px-3 py-2 rounded-lg text-[--color-error] bg-[oklch(0.65_0.21_15/0.1)]"
    >
      {{ loadError }}
    </div>

    <EquityCurvePanel
      :trader-ids="traderIdOptions"
      :active-trader-id="activeChart"
      :data="equityHistory"
      :trader-name="traderName"
      @select="selectEquityTrader"
    />

    <div class="grid grid-cols-1 gap-6">
      <TradersPanel
        :traders="traders"
        :loading="loading"
        :initial-load-done="initialLoadDone"
        @create="showCreateTrader = true"
        @start="startTrader"
        @stop="stopTrader"
        @sync="syncBalance"
      />
      <OpenPositionsPanel
        :positions="positions"
        :trader-name="traderName"
        @close="closePosition"
      />
    </div>

    <LiveEventsPanel :events="events" />
  </div>

  <CreateTraderModal
    v-if="showCreateTrader"
    @close="showCreateTrader = false"
    @created="handleTraderCreated"
  />
</template>
