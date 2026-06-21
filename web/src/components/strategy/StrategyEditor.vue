<script setup lang="ts">
import Card from "primevue/card"
import Button from "primevue/button"
import InputText from "primevue/inputtext"
import InputNumber from "primevue/inputnumber"
import Select from "primevue/select"
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
  <Card class="border border-surface-200 dark:border-surface-800 bg-surface-0 dark:bg-surface-900 shadow-none!">
    <template #content>
      <div class="flex items-center gap-3 mb-6 flex-wrap">
        <h2 class="font-bold text-xl text-surface-900 dark:text-white flex-1 truncate">{{ selected.name }}</h2>
        <div class="flex gap-2">
          <Button
            icon="pi pi-check"
            :label="selected.is_active ? 'Active' : 'Activate'"
            @click="emit('activate')"
            :disabled="selected.is_active"
            class="rounded-xl h-10 cursor-pointer"
          />
          <Button
            icon="pi pi-copy"
            label="Duplicate"
            severity="secondary"
            @click="emit('duplicate')"
            :loading="duplicating"
            class="rounded-xl h-10 cursor-pointer"
          />
          <Button
            icon="pi pi-trash"
            label="Delete"
            severity="danger"
            @click="emit('delete')"
            class="rounded-xl h-10 cursor-pointer bg-rose-500! border-rose-500! text-white!"
          />
        </div>
      </div>

      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 mb-4">
        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-bold text-surface-500">Strategy Name</label>
          <InputText v-model="selected.name" placeholder="Strategy name" class="h-10 rounded-xl" />
        </div>
        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-bold text-surface-500">Description</label>
          <InputText v-model="selected.description" placeholder="Brief description" class="h-10 rounded-xl" />
        </div>
      </div>

      <div class="flex flex-col gap-1.5 mb-4">
        <label class="text-xs font-bold text-surface-500">Trading Symbols (comma-separated)</label>
        <InputText
          :value="getSymbols()"
          @change="setSymbols(($event.target as HTMLInputElement).value)"
          placeholder="BTCUSDT,ETHUSDT"
          class="h-10 rounded-xl font-mono"
        />
      </div>

      <div class="grid grid-cols-2 gap-4 sm:grid-cols-4 mb-6">
        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-bold text-surface-500">Max Positions</label>
          <InputNumber
            v-model="getConfig().max_positions"
            :min="1"
            :max="20"
            showButtons
            class="h-10 rounded-xl"
          />
        </div>
        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-bold text-surface-500">BTC/ETH Leverage</label>
          <InputNumber
            v-model="getConfig().btc_eth_leverage"
            :min="1"
            :max="100"
            showButtons
            class="h-10 rounded-xl"
          />
        </div>
        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-bold text-surface-500">Altcoin Leverage</label>
          <InputNumber
            v-model="getConfig().altcoin_leverage"
            :min="1"
            :max="100"
            showButtons
            class="h-10 rounded-xl"
          />
        </div>
        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-bold text-surface-500">Prompt Variant</label>
          <Select
            v-model="getConfig().prompt_variant"
            :options="['balanced', 'aggressive', 'conservative']"
            placeholder="Select variant"
            class="h-10 rounded-xl flex items-center"
          />
        </div>
      </div>

      <div class="flex gap-3 mt-4 border-t border-surface-200 dark:border-surface-800 pt-4">
        <Button
          icon="pi pi-save"
          label="Save Settings"
          @click="emit('save')"
          :loading="saving"
          class="rounded-xl h-11 cursor-pointer flex-1"
        />
        <Button
          icon="pi pi-sparkles"
          label="Test Run (AI)"
          severity="help"
          @click="emit('test')"
          :loading="testRunLoading"
          class="rounded-xl h-11 cursor-pointer"
        />
        <Button
          icon="pi pi-eye"
          label="Preview Prompt"
          severity="secondary"
          text
          @click="emit('preview')"
          :loading="previewLoading"
          class="rounded-xl h-11 cursor-pointer"
        />
      </div>
    </template>
  </Card>
</template>
