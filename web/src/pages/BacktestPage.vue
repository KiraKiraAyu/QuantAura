<script setup lang="ts">
import Button from "primevue/button"
import BacktestConfigPanel from "@/components/backtest/BacktestConfigPanel.vue"
import BacktestLiveProgress from "@/components/backtest/BacktestLiveProgress.vue"
import BacktestRunsTable from "@/components/backtest/BacktestRunsTable.vue"
import PageHeader from "@/components/layout/PageHeader.vue"
import { useBacktestPage } from "@/composables/useBacktestPage"

const {
  cfg,
  liveProgress,
  loadingRuns,
  loadRuns,
  modelOptions,
  progressPct,
  running,
  runs,
  startRun,
  stopRun,
} = useBacktestPage()
</script>

<template>
  <div class="flex flex-col gap-6">
    <PageHeader
      title="Backtest Engine"
      description="Simulate and analyze AI trading strategies"
    >
      <template #actions>
        <Button
          @click="startRun"
          :disabled="running"
          :icon="running ? 'pi pi-spin pi-spinner' : 'pi pi-play'"
          :label="running ? 'Running...' : 'Start Backtest'"
          class="rounded-xl h-11 px-4 cursor-pointer"
        />
      </template>
    </PageHeader>

    <BacktestConfigPanel v-model="cfg" :model-options="modelOptions" />

    <BacktestRunsTable
      :runs="runs"
      :loading="loadingRuns"
      @refresh="loadRuns"
      @stop="stopRun"
    />

    <BacktestLiveProgress
      v-if="liveProgress"
      :progress="liveProgress"
      :progress-pct="progressPct"
    />
  </div>
</template>
