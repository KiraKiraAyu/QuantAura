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
  <div class="grid grid-cols-3 gap-4">
    <div
      v-for="(trader, index) in traders"
      :key="trader.trader_id"
      class="flex flex-col items-center py-6 gap-2 relative overflow-hidden"
      :class="index === 0 ? 'ring-2 ring-reisa-pink-400' : ''"
    >
      <div class="text-4xl">{{ ["🥇", "🥈", "🥉"][index] }}</div>
      <div
        class="w-12 h-12 rounded-full flex items-center justify-center text-xl font-black"
        :style="avatarStyle(trader.trader_id)"
      >
        {{ (trader.name || trader.trader_id).charAt(0).toUpperCase() }}
      </div>
      <div class="text-center">
        <p class="font-bold text-sm truncate max-w-30">
          {{ trader.name || trader.trader_id }}
        </p>
        <p class="text-xs text-[--color-text-muted]">
          {{ trader.ai_model }}
        </p>
      </div>
      <div class="text-center mt-1">
        <p class="font-black text-xl font-mono">
          {{
            (returnPct(trader) >= 0 ? "+" : "") + returnPct(trader).toFixed(2)
          }}%
        </p>
        <p class="text-xs text-[--color-text-muted]">
          ${{ fmt(trader.equity) }}
        </p>
      </div>
      <div
        class="absolute inset-0 opacity-5 pointer-events-none"
        :class="
          index === 0
            ? 'bg-[radial-gradient(circle_at_top,oklch(0.72_0.175_356),transparent)]'
            : ''
        "
      ></div>
    </div>
  </div>
</template>
