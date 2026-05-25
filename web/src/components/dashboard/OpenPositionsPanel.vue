<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import type { DashboardPosition } from "@/types/dashboard-ui"
import { formatDateTime } from "@/utils/format"

defineProps<{
  positions: DashboardPosition[]
  traderName: (id: string) => string
}>()

const emit = defineEmits<{
  close: [traderId: string, symbol: string, side: string]
}>()

function fmt(value: number | null | undefined, digits = 2) {
  return (value ?? 0).toLocaleString("en-US", {
    minimumFractionDigits: digits,
    maximumFractionDigits: digits,
  })
}

function signed(value: number) {
  return (value >= 0 ? "+" : "") + fmt(value, 2)
}

function liquidationPrice(value: number) {
  return value > 0 ? fmt(value, 2) : "-"
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
    <div v-else class="overflow-auto max-h-110">
      <table class="w-full min-w-360 text-xs">
        <thead>
          <tr class="text-[--color-text-muted]">
            <th class="text-left py-2 pr-4 font-semibold">Trader</th>
            <th class="text-left py-2 pr-4 font-semibold">Position</th>
            <th class="text-left py-2 font-semibold">Symbol</th>
            <th class="text-left py-2 font-semibold">Side</th>
            <th class="text-right py-2 px-3 font-semibold">Qty</th>
            <th class="text-right py-2 px-3 font-semibold">Entry</th>
            <th class="text-right py-2 px-3 font-semibold">Mark</th>
            <th class="text-right py-2 px-3 font-semibold">Liq.</th>
            <th class="text-right py-2 px-3 font-semibold">Lev.</th>
            <th class="text-left py-2 px-3 font-semibold">Margin</th>
            <th class="text-right py-2 px-3 font-semibold">Unrealized</th>
            <th class="text-right py-2 px-3 font-semibold">Realized</th>
            <th class="text-left py-2 px-3 font-semibold">Status</th>
            <th class="text-left py-2 px-3 font-semibold">Opened</th>
            <th class="text-left py-2 px-3 font-semibold">Closed</th>
            <th class="text-left py-2 px-3 font-semibold">Updated</th>
            <th class="text-right py-2 pl-3 font-semibold"></th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="position in positions"
            :key="position.id"
            class="border-t border-[--color-border-subtle]"
          >
            <td class="py-2 pr-4 max-w-40 truncate">
              {{ traderName(position.trader_id) }}
            </td>
            <td class="py-2 pr-4 font-mono text-[--color-text-muted]">
              {{ position.id.slice(0, 8) }}
            </td>
            <td class="py-2 font-semibold font-mono">
              {{ position.symbol }}
            </td>
            <td class="py-2">
              <span>{{ position.side }}</span>
            </td>
            <td class="py-2 px-3 text-right font-mono">
              {{ fmt(position.quantity, 4) }}
            </td>
            <td class="py-2 px-3 text-right font-mono">
              {{ fmt(position.entry_price, 2) }}
            </td>
            <td class="py-2 px-3 text-right font-mono">
              {{ fmt(position.mark_price, 2) }}
            </td>
            <td class="py-2 px-3 text-right font-mono">
              {{ liquidationPrice(position.liquidation_price) }}
            </td>
            <td class="py-2 px-3 text-right font-mono">
              {{ position.leverage }}x
            </td>
            <td class="py-2 px-3 uppercase">
              {{ position.margin_mode }}
            </td>
            <td
              class="py-2 px-3 text-right font-mono"
              :class="
                position.unrealized_pnl >= 0
                  ? 'text-[--color-success]'
                  : 'text-[--color-error]'
              "
            >
              {{ signed(position.unrealized_pnl) }}
            </td>
            <td class="py-2 px-3 text-right font-mono">
              {{ signed(position.realized_pnl) }}
            </td>
            <td class="py-2 px-3">
              <span>{{ position.status }}</span>
            </td>
            <td class="py-2 px-3 whitespace-nowrap text-[--color-text-muted]">
              {{ formatDateTime(position.opened_at) }}
            </td>
            <td class="py-2 px-3 whitespace-nowrap text-[--color-text-muted]">
              {{
                position.closed_at ? formatDateTime(position.closed_at) : "-"
              }}
            </td>
            <td class="py-2 px-3 whitespace-nowrap text-[--color-text-muted]">
              {{ formatDateTime(position.updated_at) }}
            </td>
            <td class="py-2 pl-3 text-right">
              <BaseButton
                v-if="position.status === 'open'"
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
