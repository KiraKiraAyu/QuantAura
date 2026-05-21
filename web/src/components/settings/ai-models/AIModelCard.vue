<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseInput from "@/components/universal/BaseInput.vue"
import type { LlmModel } from "@/types/ai-models-ui"

defineProps<{
  model: LlmModel
}>()

const emit = defineEmits<{
  saveModels: []
  removeModel: []
}>()
</script>

<template>
  <div
    class="mb-4 rounded-md border border-[--color-border-subtle] bg-[--color-surface-elevated] p-4"
  >
    <div class="mb-2 flex items-center justify-end gap-2">
      <label
        class="mr-auto flex cursor-pointer items-center gap-2 text-xs text-[--color-text-secondary]"
      >
        <input
          v-model="model.enabled"
          type="checkbox"
          class="h-4 w-4 accent-pink-500"
        />
        Enabled
      </label>
      <button
        type="button"
        class="text-[--color-text-muted] transition-colors hover:text-[--color-text-primary]"
        title="Save changes"
        @click="emit('saveModels')"
      >
        <Icon icon="ic:round-save" class="text-base" />
      </button>
      <button
        type="button"
        class="text-[--color-text-muted] transition-colors hover:text-[--color-error]"
        title="Delete model"
        @click="emit('removeModel')"
      >
        <Icon icon="ic:round-delete" class="text-base" />
      </button>
    </div>
    <BaseInput
      v-model="model.name"
      label="Display Name"
      placeholder="GPT-4 Turbo"
    />
    <BaseInput
      v-model="model.modelId"
      label="Model ID"
      placeholder="gpt-4-turbo-preview"
    />
  </div>
</template>
