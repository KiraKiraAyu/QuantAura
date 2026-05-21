<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import type { StrategyPromptPreviewModel } from "@/types/strategy-ui"

defineProps<{
  preview: StrategyPromptPreviewModel
}>()

const emit = defineEmits<{
  close: []
}>()
</script>

<template>
  <div class="mb-4">
    <div class="flex items-center justify-between mb-3">
      <h3 class="font-bold text-sm">Generated System Prompt</h3>
      <BaseButton
        @click="emit('close')"
        class="text-xs text-[--color-text-muted]"
      >
        <Icon
          icon="ic:round-close"
          class="inline-block text-base align-[-0.125em]"
        />
        Close
      </BaseButton>
    </div>
    <pre
      class="whitespace-pre-wrap text-xs p-3 rounded-lg overflow-y-auto max-h-60 font-mono bg-[--color-surface-overlay] text-[--color-text-secondary]"
      >{{ preview.system }}</pre
    >
    <div class="mt-2 text-xs text-[--color-text-muted]">
      Variant:
      <span>{{ preview.variant }}</span>
    </div>
  </div>
</template>
