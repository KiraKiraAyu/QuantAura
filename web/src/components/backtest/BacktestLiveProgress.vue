<script setup lang="ts">
import type { BacktestLiveProgress } from "@/types/backtest-ui"

defineProps<{
  progress: BacktestLiveProgress
  progressPct: number
}>()
</script>

<template>
  <div class="border border-[--color-primary]">
    <h3 class="font-bold text-sm mb-3">
      Live Progress - {{ progress.run_id?.toString().slice(0, 8) }}...
    </h3>
    <div class="flex items-center gap-4 text-sm">
      <div>
        <span class="text-[--color-text-muted]">State: </span>
        <span class="font-semibold text-[--color-primary]">
          {{ progress.state }}
        </span>
      </div>
      <div>
        <span class="text-[--color-text-muted]">Bar: </span>
        <span class="font-mono">
          {{ progress.bar_index }}/{{ progress.total_bars }}
        </span>
      </div>
      <div>
        <span class="text-[--color-text-muted]">Equity: </span>
        <span class="font-mono">
          ${{ (progress.equity ?? 0).toLocaleString() }}
        </span>
      </div>
    </div>
    <div
      class="mt-3 h-1.5 rounded-full overflow-hidden bg-[--color-surface-overlay]"
    >
      <div
        class="h-full rounded-full transition-all duration-300 bg-[linear-gradient(90deg,var(--color-reisa-pink-500),var(--color-reisa-lilac-400))]"
        :style="{ width: progressPct + '%' }"
      ></div>
    </div>
  </div>
</template>
