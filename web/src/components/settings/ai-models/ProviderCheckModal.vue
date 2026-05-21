<script setup lang="ts">
import { computed, watch } from "vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import BaseModal from "@/components/universal/BaseModal.vue"
import type { LlmProvider } from "@/types/ai-models-ui"

const props = defineProps<{
  open: boolean
  provider: LlmProvider | undefined
  selectedModelId: string
  checking: boolean
}>()

const emit = defineEmits<{
  "update:open": [value: boolean]
  "update:selectedModelId": [value: string]
  confirm: []
}>()

const modelOptions = computed(() => {
  return (
    props.provider?.models
      .map((model) => ({
        label: model.name || model.modelId,
        value: model.modelId.trim(),
      }))
      .filter((model) => model.value) ?? []
  )
})

watch(
  () => props.open,
  (open) => {
    if (!open || props.selectedModelId || modelOptions.value.length === 0) {
      return
    }
    const firstModel = modelOptions.value[0]
    if (firstModel) {
      emit("update:selectedModelId", firstModel.value)
    }
  },
)

function close() {
  emit("update:open", false)
}

function updateSelectedModel(event: Event) {
  emit("update:selectedModelId", (event.target as HTMLSelectElement).value)
}
</script>

<template>
  <BaseModal
    :model-value="open"
    title="Test Provider"
    @update:model-value="emit('update:open', $event)"
  >
    <div class="flex flex-col gap-3">
      <p class="text-sm text-[--color-text-secondary]">
        Select a model to send a minimal test request with this provider.
      </p>

      <div>
        <label>Test Model</label>
        <select
          :value="selectedModelId"
          :disabled="modelOptions.length === 0"
          @change="updateSelectedModel"
        >
          <option
            v-for="model in modelOptions"
            :key="model.value"
            :value="model.value"
          >
            {{ model.label }} ({{ model.value }})
          </option>
        </select>
      </div>

      <p
        v-if="modelOptions.length === 0"
        class="text-xs text-[--color-text-muted]"
      >
        Add at least one model before testing this provider.
      </p>
    </div>

    <template #footer>
      <BaseButton class="text-xs px-4 py-1.5" @click="close">
        Cancel
      </BaseButton>
      <BaseButton
        class="text-xs px-4 py-1.5"
        :disabled="checking || !selectedModelId"
        @click="emit('confirm')"
      >
        {{ checking ? "Testing..." : "Test" }}
      </BaseButton>
    </template>
  </BaseModal>
</template>
