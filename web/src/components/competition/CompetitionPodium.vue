<script setup lang="ts">
import type { CompetitionTrader } from "@/types/competition-ui"

defineProps<{
  traders: CompetitionTrader[]
  avatarStyle: (id: string) => string
  fmt: (value: number) => string
  returnPct: (trader: CompetitionTrader) => number
}>()
</script>

<template>
  <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-2">
    <div
      v-for="(trader, index) in traders"
      :key="trader.trader_id"
      class="bg-surface-0 dark:bg-surface-900 border border-surface-200 dark:border-surface-800 rounded-2xl p-6 flex flex-col items-center gap-3 relative overflow-hidden transition-transform duration-300 hover:-translate-y-1"
      :class="index === 0 ? 'ring-2 ring-primary-500 dark:ring-primary-400 shadow-md' : 'shadow-sm'"
    >
      <div class="text-5xl mb-2">{{ ["🥇", "🥈", "🥉"][index] }}</div>
      <div
        class="w-16 h-16 rounded-full flex items-center justify-center text-2xl font-black text-white shadow-sm ring-4 ring-surface-0 dark:ring-surface-900 z-10"
        :style="avatarStyle(trader.trader_id)"
      >
        {{ (trader.trader_name || trader.trader_id).charAt(0).toUpperCase() }}
      </div>
      <div class="text-center z-10">
        <p class="font-bold text-lg text-surface-900 dark:text-white truncate max-w-40">
          {{ trader.trader_name || trader.trader_id }}
        </p>
        <p class="text-xs text-surface-500 font-medium tracking-wide">
          {{ trader.ai_model }}
        </p>
      </div>
      <div class="text-center mt-2 z-10">
        <p class="font-black text-2xl font-mono" :class="returnPct(trader) >= 0 ? 'text-emerald-500 dark:text-emerald-400' : 'text-rose-500 dark:text-rose-400'">
          {{ (returnPct(trader) >= 0 ? "+" : "") + returnPct(trader).toFixed(2) }}%
        </p>
        <p class="text-sm text-surface-600 dark:text-surface-400 font-medium">
          ${{ fmt(trader.total_equity) }}
        </p>
      </div>
      <div
        class="absolute inset-0 pointer-events-none opacity-[0.03] dark:opacity-[0.08]"
        :class="index === 0 ? 'bg-[radial-gradient(circle_at_top,var(--p-primary-500),transparent)]' : ''"
      ></div>
    </div>
  </div>
</template>
