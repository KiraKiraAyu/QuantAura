<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import type { LlmProvider } from "@/types/ai-models-ui"

defineProps<{
  providers: LlmProvider[]
  selectedProviderIndex: number
  supportedProviderTypeCount: number
  apiCategoryLabel: (providerType: string) => string
  providerKey: (provider: LlmProvider, index: number) => string
  providerLabel: (provider: LlmProvider) => string
}>()

const emit = defineEmits<{
  addProvider: []
  selectProvider: [index: number]
}>()
</script>

<template>
  <div
    class="flex w-64 shrink-0 flex-col gap-2 border-r border-[--color-border-subtle] pr-4"
  >
    <BaseButton
      @click="emit('addProvider')"
      class="text-xs py-2 justify-center"
      :disabled="supportedProviderTypeCount === 0"
    >
      <Icon
        icon="ic:round-add-circle"
        class="inline-block text-base align-[-0.125em]"
      />
      Add Provider
    </BaseButton>

    <div class="mt-2 flex flex-col gap-1">
      <button
        v-for="(provider, providerIndex) in providers"
        :key="providerKey(provider, providerIndex)"
        type="button"
        class="rounded-md px-3 py-2 text-left text-sm transition-colors hover:bg-[--color-surface-elevated]"
        :class="
          selectedProviderIndex === providerIndex
            ? 'bg-[--color-surface-elevated] text-[--color-text-primary]'
            : 'text-[--color-text-secondary]'
        "
        @click="emit('selectProvider', providerIndex)"
      >
        <span class="block truncate font-medium">
          {{ providerLabel(provider) }}
        </span>
        <span class="block truncate text-xs text-[--color-text-muted]">
          {{ apiCategoryLabel(provider.providerType) }} ·
          {{ provider.baseUrl }}
        </span>
      </button>
    </div>
  </div>
</template>
