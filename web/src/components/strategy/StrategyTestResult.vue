<script setup lang="ts">
import type { StrategyTestResult } from "@/types/strategy-ui"

defineProps<{
  result: StrategyTestResult
}>()
</script>

<template>
  <div>
    <h3 class="font-bold text-sm mb-3">AI Test Run Result</h3>
    <div class="flex flex-col gap-2">
      <div
        v-for="(decision, index) in result.decisions"
        :key="index"
        class="flex items-center gap-3 py-2 px-3 rounded-lg text-sm bg-[--color-surface-elevated]"
      >
        <span>{{ decision.action }}</span>
        <span class="font-mono font-semibold">{{ decision.symbol }}</span>
        <span class="text-xs text-[--color-text-muted]">
          conf {{ decision.confidence }}%
        </span>
        <span class="flex-1 text-xs truncate text-[--color-text-secondary]">
          {{ decision.reasoning }}
        </span>
        <span class="text-xs text-[--color-text-muted]">
          {{ result.duration_ms }}ms
        </span>
      </div>
    </div>
    <div v-if="result.ai_response" class="mt-3">
      <label>Raw AI Response</label>
      <pre
        class="text-xs p-3 rounded-lg overflow-auto max-h-40 font-mono bg-[--color-surface-overlay] text-[--color-text-secondary]"
        >{{ result.ai_response }}</pre
      >
    </div>
  </div>
</template>
