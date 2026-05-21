<script setup lang="ts">
import BaseInput from "@/components/universal/BaseInput.vue"
import type { CompetitionTrader } from "@/types/competition-ui"

defineProps<{
  traders: CompetitionTrader[]
  loading: boolean
  avatarStyle: (id: string) => string
  fmt: (value: number) => string
  returnPct: (trader: CompetitionTrader) => number
}>()

const search = defineModel<string>({ required: true })

const emit = defineEmits<{
  select: [trader: CompetitionTrader]
}>()
</script>

<template>
  <div>
    <div class="flex items-center justify-between mb-4">
      <h2 class="font-bold text-sm">Full Rankings</h2>
      <div class="flex items-center gap-2">
        <div class="w-40">
          <BaseInput
            v-model="search"
            class="text-xs py-1.5"
            placeholder="Search..."
          />
        </div>
      </div>
    </div>

    <div
      v-if="loading"
      class="text-center py-12 text-sm text-[--color-text-muted]"
    >
      Loading...
    </div>
    <div
      v-else-if="traders.length === 0"
      class="text-center py-12 text-sm text-[--color-text-muted]"
    >
      No active traders in competition
    </div>
    <div v-else class="overflow-auto">
      <table class="w-full text-xs">
        <thead>
          <tr class="text-[--color-text-muted]">
            <th class="text-left py-2 px-3 font-semibold">#</th>
            <th class="text-left py-2 font-semibold">Trader</th>
            <th class="text-left py-2 font-semibold">Model</th>
            <th class="text-right py-2 font-semibold">Equity</th>
            <th class="text-right py-2 font-semibold">Return</th>
            <th class="text-right py-2 font-semibold">Trades</th>
            <th class="text-right py-2 font-semibold">Win Rate</th>
            <th class="text-right py-2 font-semibold">Status</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="(trader, index) in traders"
            :key="trader.trader_id"
            class="border-t border-[--color-border-subtle] transition-colors cursor-pointer hover:bg-surface-elevated"
            @click="emit('select', trader)"
          >
            <td class="py-3 px-3">
              <span
                class="font-bold"
                :class="index < 3 ? 'text-[--color-accent]' : ''"
              >
                {{ index < 3 ? ["🥇", "🥈", "🥉"][index] : index + 1 }}
              </span>
            </td>
            <td class="py-3">
              <div class="flex items-center gap-2">
                <div
                  class="w-7 h-7 rounded-full flex items-center justify-center text-xs font-bold shrink-0"
                  :style="avatarStyle(trader.trader_id)"
                >
                  {{
                    (trader.name || trader.trader_id).charAt(0).toUpperCase()
                  }}
                </div>
                <div>
                  <p class="font-semibold">
                    {{ trader.name || trader.trader_id.slice(0, 16) }}
                  </p>
                  <p class="text-[0.6rem] text-[--color-text-muted]">
                    {{ trader.exchange }}
                  </p>
                </div>
              </div>
            </td>
            <td class="py-3">
              <span>{{ trader.ai_model }}</span>
            </td>
            <td class="py-3 text-right font-mono">${{ fmt(trader.equity) }}</td>
            <td class="py-3 text-right font-mono font-bold">
              {{
                (returnPct(trader) >= 0 ? "+" : "") +
                returnPct(trader).toFixed(2)
              }}%
            </td>
            <td class="py-3 text-right font-mono">
              {{ trader.total_trades ?? "-" }}
            </td>
            <td class="py-3 text-right font-mono">
              {{
                trader.win_rate != null
                  ? (trader.win_rate * 100).toFixed(1) + "%"
                  : "-"
              }}
            </td>
            <td class="py-3 text-right">
              <span class="text-[0.6rem]">
                {{ trader.is_running ? "Live" : "Stopped" }}
              </span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
