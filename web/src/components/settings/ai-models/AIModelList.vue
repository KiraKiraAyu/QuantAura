<script setup lang="ts">
import Button from "primevue/button"
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
  <div class="mt-8 flex min-h-0 flex-1 flex-col">
    <div class="mb-4 flex items-center justify-between">
      <h3 class="text-lg font-bold text-surface-900 dark:text-white">Models</h3>
      <div class="flex gap-2">
        <Button
          :label="fetchingRemoteModels ? 'Fetching...' : 'Fetch Models'"
          icon="pi pi-cloud-download"
          size="small"
          severity="secondary"
          variant="outlined"
          :disabled="fetchingRemoteModels || !provider.apiKey"
          @click="emit('fetchRemoteModels', provider)"
        />
        <Button
          v-if="!isAddingModel"
          label="Add Model"
          icon="pi pi-plus"
          size="small"
          @click="emit('startAddModel')"
        />
      </div>
    </div>

    <div class="flex-1 overflow-y-auto pr-2 pb-4 flex flex-col gap-3">
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
        class="flex w-full items-center justify-center gap-2 rounded-xl border border-dashed border-surface-300 dark:border-surface-700 bg-transparent p-4 text-sm font-medium text-surface-500 transition-colors hover:bg-surface-100 dark:hover:bg-surface-800"
        @click="emit('startAddModel')"
      >
        <i class="pi pi-plus text-base"></i>
        Add Model
      </button>
    </div>
  </div>
</template>
