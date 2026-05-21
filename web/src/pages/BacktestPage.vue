<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
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
        <BaseButton @click="startRun" :disabled="running">
          <Icon
            :icon="running ? 'ic:round-hourglass-empty' : 'ic:round-play-arrow'"
            class="inline-block text-base align-[-0.125em]"
          />
          {{ running ? "Running..." : "Start Backtest" }}
        </BaseButton>
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
