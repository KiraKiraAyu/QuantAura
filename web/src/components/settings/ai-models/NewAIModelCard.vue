<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import BaseInput from "@/components/universal/BaseInput.vue"
import type { LlmModel } from "@/types/ai-models-ui"

defineProps<{
  model: LlmModel
}>()

const emit = defineEmits<{
  cancelAddModel: []
  saveNewModel: []
}>()
</script>

<template>
  <div
    class="mb-4 rounded-md border border-dashed border-[--color-border-subtle] bg-[--color-surface] p-4"
  >
    <h4 class="mb-2 text-sm font-semibold">New Model</h4>
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
    <div class="mt-2 flex justify-end gap-2">
      <BaseButton class="text-xs px-3 py-1.5" @click="emit('cancelAddModel')">
        <Icon
          icon="ic:round-close"
          class="inline-block text-base align-[-0.125em]"
        />
        Cancel
      </BaseButton>
      <BaseButton class="text-xs px-3 py-1.5" @click="emit('saveNewModel')">
        <Icon
          icon="ic:round-check"
          class="inline-block text-base align-[-0.125em]"
        />
        Save
      </BaseButton>
    </div>
  </div>
</template>
