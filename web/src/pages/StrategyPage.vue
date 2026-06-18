<script setup lang="ts">
import Button from "primevue/button"
import PageHeader from "@/components/layout/PageHeader.vue"
import StrategyEditor from "@/components/strategy/StrategyEditor.vue"
import StrategyList from "@/components/strategy/StrategyList.vue"
import StrategyPromptPreview from "@/components/strategy/StrategyPromptPreview.vue"
import StrategyTestResult from "@/components/strategy/StrategyTestResult.vue"
import { useStrategyPage } from "@/composables/useStrategyPage"

const {
  activateStrategy,
  createNew,
  deleteStrategy,
  duplicateStrategy,
  duplicating,
  loading,
  previewLoading,
  previewPrompt,
  previewPromptText,
  runTest,
  saveStrategy,
  saving,
  selected,
  strategies,
  testResult,
  testRunLoading,
} = useStrategyPage()
</script>

<template>
  <div class="flex flex-col gap-6">
    <PageHeader
      title="Strategy Studio"
      description="Create and manage trading strategies"
    >
      <template #actions>
        <Button label="New Strategy" icon="pi pi-plus" @click="createNew" />
      </template>
    </PageHeader>

    <div class="grid grid-cols-1 gap-6 lg:grid-cols-3">
      <StrategyList
        :strategies="strategies"
        :selected-id="selected?.id"
        :loading="loading"
        @select="selected = $event"
      />

      <div class="lg:col-span-2">
        <div v-if="!selected" class="flex items-center justify-center h-64">
          <p class="text-sm text-[--color-text-muted]">
            Select or create a strategy
          </p>
        </div>

        <div v-else class="flex flex-col gap-4">
          <StrategyEditor
            v-model="selected"
            :saving="saving"
            :duplicating="duplicating"
            :test-run-loading="testRunLoading"
            :preview-loading="previewLoading"
            @activate="activateStrategy"
            @duplicate="duplicateStrategy"
            @delete="deleteStrategy"
            @save="saveStrategy"
            @test="runTest"
            @preview="previewPrompt"
          />
          <StrategyPromptPreview
            v-if="previewPromptText"
            :preview="previewPromptText"
            @close="previewPromptText = null"
          />
          <StrategyTestResult v-if="testResult" :result="testResult" />
        </div>
      </div>
    </div>
  </div>
</template>
