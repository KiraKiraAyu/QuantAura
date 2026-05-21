<script setup lang="ts">
import BaseInput from "@/components/universal/BaseInput.vue"
import type { BacktestConfig, BacktestModelOption } from "@/types/backtest-ui"

defineProps<{
  modelOptions: BacktestModelOption[]
}>()

const cfg = defineModel<BacktestConfig>({ required: true })
const intervals = ["1m", "5m", "15m", "1h", "4h", "1d"]
</script>

<template>
  <div>
    <h2 class="font-bold text-sm mb-4">Configuration</h2>
    <div class="grid grid-cols-2 gap-4 md:grid-cols-4">
      <div>
        <label>Symbols</label>
        <BaseInput v-model="cfg.symbols" placeholder="BTC,ETH" />
      </div>
      <div>
        <label>Interval</label>
        <select v-model="cfg.interval">
          <option
            v-for="interval in intervals"
            :key="interval"
            :value="interval"
          >
            {{ interval }}
          </option>
        </select>
      </div>
      <div>
        <label>Start Date</label>
        <BaseInput v-model="cfg.startDate" type="date" />
      </div>
      <div>
        <label>End Date</label>
        <BaseInput v-model="cfg.endDate" type="date" />
      </div>
      <div>
        <label>Initial Balance ($)</label>
        <BaseInput
          v-model.number="cfg.initial_balance"
          type="number"
          min="100"
        />
      </div>
      <div>
        <label>Fee (bps)</label>
        <BaseInput v-model.number="cfg.fee_bps" type="number" min="0" />
      </div>
      <div>
        <label>Slippage (bps)</label>
        <BaseInput v-model.number="cfg.slippage_bps" type="number" min="0" />
      </div>
      <div>
        <label>AI Model</label>
        <select v-model="cfg.ai_model_id">
          <option value="">Select model...</option>
          <option
            v-for="model in modelOptions"
            :key="model.id"
            :value="model.id"
          >
            {{ model.label }}
          </option>
        </select>
      </div>
    </div>
  </div>
</template>
