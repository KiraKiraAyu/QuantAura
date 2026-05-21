<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import BaseInput from "@/components/universal/BaseInput.vue"
import type { EditableStrategy, StrategyConfig } from "@/types/strategy-ui"

const selected = defineModel<EditableStrategy>({ required: true })

defineProps<{
  saving: boolean
  duplicating: boolean
  testRunLoading: boolean
  previewLoading: boolean
}>()

const emit = defineEmits<{
  activate: []
  duplicate: []
  delete: []
  save: []
  test: []
  preview: []
}>()

function getConfig(): StrategyConfig {
  if (!selected.value.config) selected.value.config = {}
  return selected.value.config
}

function getSymbols() {
  return (getConfig().trading_symbols as string) ?? "BTCUSDT,ETHUSDT"
}

function setSymbols(value: string) {
  getConfig().trading_symbols = value
}
</script>

<template>
  <div>
    <div class="flex items-center gap-3 mb-4">
      <h2 class="font-bold flex-1">{{ selected.name }}</h2>
      <BaseButton
        @click="emit('activate')"
        class="py-1.5 text-xs"
        :disabled="selected.is_active"
      >
        <Icon
          icon="ic:round-check"
          class="inline-block text-base align-[-0.125em]"
        />
        {{ selected.is_active ? "Active" : "Activate" }}
      </BaseButton>
      <BaseButton
        @click="emit('duplicate')"
        class="py-1.5 text-xs"
        :disabled="duplicating"
      >
        <Icon
          icon="ic:round-content-copy"
          class="inline-block text-base align-[-0.125em]"
        />
        {{ duplicating ? "..." : "Duplicate" }}
      </BaseButton>
      <BaseButton @click="emit('delete')" class="text-error py-1.5 text-xs">
        <Icon
          icon="ic:round-delete"
          class="inline-block text-base align-[-0.125em]"
        />
        Delete
      </BaseButton>
    </div>

    <div class="grid grid-cols-1 gap-3 md:grid-cols-2 mb-4">
      <div>
        <label>Name</label>
        <BaseInput v-model="selected.name" placeholder="Strategy name" />
      </div>
      <div>
        <label>Description</label>
        <BaseInput
          v-model="selected.description"
          placeholder="Brief description"
        />
      </div>
    </div>

    <div class="mb-3">
      <label>Symbols (comma-separated)</label>
      <BaseInput
        :value="getSymbols()"
        @change="setSymbols(($event.target as HTMLInputElement).value)"
        placeholder="BTCUSDT,ETHUSDT"
      />
    </div>

    <div class="grid grid-cols-2 gap-3 md:grid-cols-4">
      <div>
        <label>Max Positions</label>
        <BaseInput
          v-model.number="getConfig().max_positions"
          type="number"
          min="1"
          max="20"
        />
      </div>
      <div>
        <label>BTC/ETH Leverage</label>
        <BaseInput
          v-model.number="getConfig().btc_eth_leverage"
          type="number"
          min="1"
          max="100"
        />
      </div>
      <div>
        <label>Altcoin Leverage</label>
        <BaseInput
          v-model.number="getConfig().altcoin_leverage"
          type="number"
          min="1"
          max="100"
        />
      </div>
      <div>
        <label>Prompt Variant</label>
        <select v-model="getConfig().prompt_variant">
          <option value="balanced">Balanced</option>
          <option value="aggressive">Aggressive</option>
          <option value="conservative">Conservative</option>
        </select>
      </div>
    </div>

    <div class="flex gap-3 mt-4">
      <BaseButton @click="emit('save')" :disabled="saving">
        <Icon
          icon="ic:round-save"
          class="inline-block text-base align-[-0.125em]"
        />
        {{ saving ? "Saving..." : "Save" }}
      </BaseButton>
      <BaseButton @click="emit('test')" :disabled="testRunLoading">
        <Icon
          icon="ic:round-science"
          class="inline-block text-base align-[-0.125em]"
        />
        {{ testRunLoading ? "Running..." : "Test Run (AI)" }}
      </BaseButton>
      <BaseButton @click="emit('preview')" :disabled="previewLoading">
        <Icon
          icon="ic:round-visibility"
          class="inline-block text-base align-[-0.125em]"
        />
        {{ previewLoading ? "Loading..." : "Preview Prompt" }}
      </BaseButton>
    </div>
  </div>
</template>
