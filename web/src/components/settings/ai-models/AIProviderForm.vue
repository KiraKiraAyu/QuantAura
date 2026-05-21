<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import BaseInput from "@/components/universal/BaseInput.vue"
import type { ApiCategoryOption, LlmProvider } from "@/types/ai-models-ui"

defineProps<{
  provider: LlmProvider
  apiCategories: ApiCategoryOption[]
  checkingProvider: boolean
  savingModels: boolean
  checkMessage: string
}>()

const emit = defineEmits<{
  checkProvider: [provider: LlmProvider]
  removeProvider: []
  saveModels: []
}>()
</script>

<template>
  <BaseInput v-model="provider.name" label="Provider Name" />

  <div>
    <label>API Category</label>
    <select v-model="provider.providerType">
      <option
        v-for="category in apiCategories"
        :key="category.value"
        :value="category.value"
      >
        {{ category.label }}
      </option>
    </select>
  </div>

  <BaseInput
    v-model="provider.baseUrl"
    label="API URL"
    placeholder="https://api.example.com/v1"
  />

  <div class="grid grid-cols-[minmax(0,1fr)_auto] items-end gap-2">
    <BaseInput
      v-model="provider.apiKey"
      type="password"
      label="API Key"
      placeholder="sk-..."
    />
    <BaseButton
      @click="emit('checkProvider', provider)"
      class="mb-2 text-xs px-3 py-2"
      :disabled="checkingProvider || !provider.apiKey"
    >
      <Icon
        icon="ic:round-check-circle"
        class="inline-block text-base align-[-0.125em]"
      />
      {{ checkingProvider ? "Testing..." : "Test" }}
    </BaseButton>
  </div>

  <p v-if="checkMessage" class="px-2 text-xs text-[--color-text-secondary]">
    {{ checkMessage }}
  </p>

  <div class="mt-3 flex flex-wrap items-center gap-2">
    <label
      class="flex cursor-pointer items-center gap-2 text-xs text-[--color-text-secondary]"
    >
      <input
        v-model="provider.enabled"
        type="checkbox"
        class="h-4 w-4 accent-pink-500"
      />
      Enabled
    </label>
    <div class="ml-auto flex gap-2">
      <BaseButton
        @click="emit('removeProvider')"
        class="text-xs px-3 py-1.5 text-[--color-error] bg-[oklch(0.65_0.21_15/0.1)]"
      >
        <Icon
          icon="ic:round-delete"
          class="inline-block text-base align-[-0.125em]"
        />
        Delete
      </BaseButton>
      <BaseButton
        @click="emit('saveModels')"
        class="text-xs px-4 py-1.5"
        :disabled="savingModels"
      >
        <Icon
          icon="ic:round-save"
          class="inline-block text-base align-[-0.125em]"
        />
        {{ savingModels ? "Saving..." : "Save" }}
      </BaseButton>
    </div>
  </div>
</template>
