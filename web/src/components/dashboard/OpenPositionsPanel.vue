<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import type { DashboardPosition } from "@/types/dashboard-ui"

defineProps<{
  positions: DashboardPosition[]
}>()

const emit = defineEmits<{
  close: [traderId: string, symbol: string, side: string]
}>()

function fmt(value: number, digits = 2) {
  return (value ?? 0).toLocaleString("en-US", {
    minimumFractionDigits: digits,
    maximumFractionDigits: digits,
  })
}
</script>

<template>
  <div>
    <div class="flex items-center justify-between mb-4">
      <h2 class="font-bold text-sm">Open Positions</h2>
      <span>{{ positions.length }} open</span>
    </div>

    <div v-if="positions.length === 0" class="text-center py-8">
      <p class="text-sm text-[--color-text-muted]">No open positions</p>
    </div>
    <div v-else class="overflow-auto max-h-90">
      <table class="w-full text-xs">
        <thead>
          <tr class="text-[--color-text-muted]">
            <th class="text-left py-2 font-semibold">Symbol</th>
            <th class="text-left py-2 font-semibold">Side</th>
            <th class="text-right py-2 font-semibold">Qty</th>
            <th class="text-right py-2 font-semibold">Entry</th>
            <th class="text-right py-2 font-semibold">PnL</th>
            <th class="text-right py-2 font-semibold"></th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="(position, index) in positions"
            :key="index"
            class="border-t border-[--color-border-subtle]"
          >
            <td class="py-2 font-semibold font-mono">
              {{ position.symbol }}
            </td>
            <td class="py-2">
              <span>{{ position.side }}</span>
            </td>
            <td class="py-2 text-right font-mono">
              {{ fmt(position.qty ?? position.quantity ?? 0, 4) }}
            </td>
            <td class="py-2 text-right font-mono">
              {{ fmt(position.entry_price, 2) }}
            </td>
            <td class="py-2 text-right font-mono">
              {{
                (position.unrealized_pnl >= 0 ? "+" : "") +
                fmt(position.unrealized_pnl, 2)
              }}
            </td>
            <td class="py-2 text-right">
              <BaseButton
                v-if="position.trader_id"
                @click="
                  emit(
                    'close',
                    position.trader_id,
                    position.symbol,
                    position.side,
                  )
                "
                class="text-[0.65rem] px-1.5 py-0.5 rounded transition-colors text-[--color-error] bg-[oklch(0.65_0.21_15/0.1)]"
                title="Close position"
              >
                <Icon
                  icon="ic:round-close"
                  class="inline-block text-base align-[-0.125em]"
                />
              </BaseButton>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
