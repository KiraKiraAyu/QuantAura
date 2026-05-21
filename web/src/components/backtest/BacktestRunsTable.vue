<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import type { BacktestRun } from "@/types/backtest-ui"

defineProps<{
  runs: BacktestRun[]
  loading: boolean
}>()

const emit = defineEmits<{
  refresh: []
  stop: [runId: string]
}>()

function returnPct(run: BacktestRun) {
  const equity = run.summary?.equity_last ?? 0
  const initial = run.summary?.initial_balance ?? 1000
  return initial > 0 ? ((equity - initial) / initial) * 100 : 0
}
</script>

<template>
  <div>
    <div class="flex items-center justify-between mb-4">
      <h2 class="font-bold text-sm">Backtest Runs</h2>
      <BaseButton @click="emit('refresh')" class="text-xs py-1.5 px-3">
        <Icon
          icon="ic:round-refresh"
          class="inline-block text-base align-[-0.125em]"
        />
        Refresh
      </BaseButton>
    </div>
    <div
      v-if="runs.length === 0 && !loading"
      class="text-center py-8 text-sm text-[--color-text-muted]"
    >
      No backtest runs yet. Start one above.
    </div>
    <div v-else class="overflow-auto">
      <table class="w-full text-xs">
        <thead>
          <tr class="text-[--color-text-muted]">
            <th class="text-left py-2 font-semibold">Run ID</th>
            <th class="text-left py-2 font-semibold">State</th>
            <th class="text-right py-2 font-semibold">Equity</th>
            <th class="text-right py-2 font-semibold">Return</th>
            <th class="text-right py-2 font-semibold">Max DD</th>
            <th class="text-left py-2 font-semibold">Created</th>
            <th class="text-right py-2 font-semibold">Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="run in runs"
            :key="run.run_id"
            class="border-t border-[--color-border-subtle]"
          >
            <td class="py-2 font-mono text-[--color-text-secondary]">
              {{ run.run_id.slice(0, 8) }}...
            </td>
            <td class="py-2">
              <span>{{ run.state }}</span>
            </td>
            <td class="py-2 text-right font-mono">
              ${{
                (run.summary?.equity_last ?? 0).toLocaleString("en-US", {
                  minimumFractionDigits: 2,
                  maximumFractionDigits: 2,
                })
              }}
            </td>
            <td class="py-2 text-right font-mono">
              {{
                (returnPct(run) >= 0 ? "+" : "") + returnPct(run).toFixed(2)
              }}%
            </td>
            <td class="py-2 text-right font-mono">
              {{ (run.summary?.max_drawdown_pct ?? 0).toFixed(2) }}%
            </td>
            <td class="py-2 text-[--color-text-muted]">
              {{ new Date(run.created_at).toLocaleDateString() }}
            </td>
            <td class="py-2 text-right">
              <BaseButton
                v-if="run.state === 'running'"
                @click="emit('stop', run.run_id)"
                class="text-xs px-2 py-1 rounded text-[--color-error] bg-[oklch(0.65_0.21_15/0.1)]"
              >
                Stop
              </BaseButton>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
