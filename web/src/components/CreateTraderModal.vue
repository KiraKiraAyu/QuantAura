<script setup lang="ts">
import BaseInput from "@/components/universal/BaseInput.vue"
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import { ref, onMounted } from "vue"
import { getExchangeConfigsApi } from "@/api/exchanges"
import { getModelConfigsApi } from "@/api/models"
import { getStrategiesApi } from "@/api/strategies"
import { createTraderApi } from "@/api/trading"
import type { SafeExchangeConfig } from "@/types/exchanges"
import type { ModelConfigPayload } from "@/types/models"
import type { StrategyListPayload, StrategyPayload } from "@/types/strategies"
import type { CreateTraderRequest } from "@/types/trading"

const emit = defineEmits(["close", "created"])

const loading = ref(false)
const error = ref("")
const exchanges = ref<SafeExchangeConfig[]>([])
const strategies = ref<StrategyPayload[]>([])
const models = ref<{ id: string; label: string }[]>([])
const form = ref({
  name: "",
  exchange_id: "",
  ai_model_id: "",
  strategy_id: "",
  scan_interval_minutes: 60,
  initial_balance: 1000,
  show_in_competition: false,
})

async function submit() {
  loading.value = true
  error.value = ""
  if (!form.value.ai_model_id) {
    error.value = "Select an AI model first"
    loading.value = false
    return
  }
  try {
    const payload: CreateTraderRequest = {
      ...form.value,
      strategy_id: form.value.strategy_id || undefined,
    }
    await createTraderApi(payload)
    emit("created")
  } catch (e: unknown) {
    error.value = e instanceof Error ? e.message : "Failed to create trader"
  } finally {
    loading.value = false
  }
}

onMounted(async () => {
  const [exRes, stRes, modelRes] = await Promise.all([
    getExchangeConfigsApi().catch((): SafeExchangeConfig[] => []),
    getStrategiesApi().catch(
      (): StrategyListPayload => ({ strategies: [], count: 0 }),
    ),
    getModelConfigsApi().catch((): ModelConfigPayload => ({ providers: [] })),
  ])
  exchanges.value = exRes
  strategies.value = stRes.strategies

  const providerItems = modelRes.providers
  const enabledModels = providerItems.flatMap(
    (provider: {
      name?: string
      enabled?: boolean
      models?: { id?: string; name?: string; enabled?: boolean }[]
    }) =>
      (provider.models ?? [])
        .filter(
          (model) => (provider.enabled ?? true) && (model.enabled ?? true),
        )
        .map((model) => ({
          id: model.id ?? "",
          label: `${provider.name ?? "Provider"} / ${model.name ?? model.id ?? ""}`,
        })),
  )

  models.value = enabledModels
  if (
    !form.value.ai_model_id ||
    !enabledModels.some(
      (model: { id: string }) => model.id === form.value.ai_model_id,
    )
  ) {
    form.value.ai_model_id = enabledModels[0]?.id ?? ""
  }
})
</script>

<template>
  <div
    class="fixed inset-0 flex items-center justify-center z-50 p-4 bg-black/60"
  >
    <div class="w-full max-w-lg">
      <h2 class="font-bold mb-5">Create Trader</h2>
      <form @submit.prevent="submit" class="flex flex-col gap-4">
        <div class="grid grid-cols-1 gap-3 md:grid-cols-2">
          <div>
            <label>Name</label>
            <BaseInput
              v-model="form.name"
             
              placeholder="My AI Trader"
              required
            />
          </div>
          <div>
            <label>Exchange</label>
            <select v-model="form.exchange_id" required>
              <option value="">Select exchange…</option>
              <option
                v-for="ex in exchanges"
                :key="ex.id"
                :value="ex.id"
              >
                {{ ex.account_name || ex.exchange_type }}
              </option>
            </select>
          </div>
          <div>
            <label>AI Model</label>
            <select v-model="form.ai_model_id">
              <option value="">Select model…</option>
              <option v-for="m in models" :key="m.id" :value="m.id">
                {{ m.label }}
              </option>
            </select>
          </div>
          <div>
            <label>Strategy</label>
            <select v-model="form.strategy_id">
              <option value="">No strategy</option>
              <option
                v-for="s in strategies"
                :key="s.id"
                :value="s.id"
              >
                {{ s.name }}
              </option>
            </select>
          </div>
          <div>
            <label>Scan Interval (min)</label>
            <BaseInput
              v-model.number="form.scan_interval_minutes"
              type="number"
             
              min="1"
              max="1440"
            />
          </div>
          <div>
            <label>Initial Balance (USDT)</label>
            <BaseInput
              v-model.number="form.initial_balance"
              type="number"
             
              min="10"
            />
          </div>
        </div>

        <div class="flex items-center gap-3">
          <label class="flex items-center gap-2 cursor-pointer">
            <BaseInput type="checkbox" v-model="form.show_in_competition" />
            <span class="text-sm">Show in leaderboard</span>
          </label>
        </div>

        <p v-if="error" class="text-xs text-[--color-error]">
          {{ error }}
        </p>

        <div class="flex gap-3 mt-1">
          <BaseButton type="submit" class="flex-1" :disabled="loading">
            <Icon icon="ic:round-check" class="inline-block text-base align-[-0.125em]" />
            {{ loading ? "Creating…" : " Create Trader" }}
          </BaseButton>
          <BaseButton type="button" @click="$emit('close')">
            <Icon icon="ic:round-close" class="inline-block text-base align-[-0.125em]" /> Cancel
          </BaseButton>
        </div>
      </form>
    </div>
  </div>
</template>
