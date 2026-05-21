<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import AIModelCard from "@/components/settings/ai-models/AIModelCard.vue"
import NewAIModelCard from "@/components/settings/ai-models/NewAIModelCard.vue"
import RemoteModelList from "@/components/settings/ai-models/RemoteModelList.vue"
import type { LlmModel, LlmProvider } from "@/types/ai-models-ui"
import type { AvailableModelPayload } from "@/types/models"

const props = defineProps<{
  provider: LlmProvider
  remoteModels: AvailableModelPayload[]
  fetchingRemoteModels: boolean
  isAddingModel: boolean
  newModel: LlmModel
  hasModel: (provider: LlmProvider, modelId: string) => boolean
}>()

const emit = defineEmits<{
  fetchRemoteModels: [provider: LlmProvider]
  startAddModel: []
  addRemoteModel: [provider: LlmProvider, model: AvailableModelPayload]
  saveModels: []
  removeModel: [provider: LlmProvider, modelIndex: number]
  cancelAddModel: []
  saveNewModel: [provider: LlmProvider]
}>()

function providerHasModel(modelId: string) {
  return props.hasModel(props.provider, modelId)
}
</script>

<template>
  <div class="mt-6 flex min-h-0 flex-1 flex-col">
    <div class="mb-2 flex items-center justify-between">
      <h3 class="text-lg font-semibold">Models</h3>
      <div class="flex gap-2">
        <BaseButton
          @click="emit('fetchRemoteModels', provider)"
          class="text-xs py-1.5 px-3"
          :disabled="fetchingRemoteModels || !provider.apiKey"
        >
          <Icon
            icon="ic:round-cloud-download"
            class="inline-block text-base align-[-0.125em]"
          />
          {{ fetchingRemoteModels ? "Fetching..." : "Fetch Models" }}
        </BaseButton>
        <BaseButton
          v-if="!isAddingModel"
          @click="emit('startAddModel')"
          class="text-xs py-1.5 px-3"
        >
          <Icon
            icon="ic:round-add"
            class="inline-block text-base align-[-0.125em]"
          />
          Add Model
        </BaseButton>
      </div>
    </div>

    <div class="flex-1 overflow-y-auto pr-2">
      <RemoteModelList
        :remote-models="remoteModels"
        :has-model="providerHasModel"
        @add-remote-model="emit('addRemoteModel', provider, $event)"
      />

      <AIModelCard
        v-for="(model, modelIndex) in provider.models"
        :key="model.id ?? `model-${modelIndex}`"
        :model="model"
        @save-models="emit('saveModels')"
        @remove-model="emit('removeModel', provider, modelIndex)"
      />

      <NewAIModelCard
        v-if="isAddingModel"
        :model="newModel"
        @cancel-add-model="emit('cancelAddModel')"
        @save-new-model="emit('saveNewModel', provider)"
      />

      <button
        v-if="!isAddingModel"
        type="button"
        class="flex w-full items-center justify-center gap-2 rounded-md border border-dashed border-[--color-border-subtle] bg-transparent p-2 text-sm text-[--color-text-secondary] transition-colors hover:bg-[--color-surface-elevated]"
        @click="emit('startAddModel')"
      >
        <Icon icon="ic:round-add" class="text-base" />
        Add Model
      </button>
    </div>
  </div>
</template>
