<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import CreateDebateModal from "@/components/debate/CreateDebateModal.vue"
import DebateDetail from "@/components/debate/DebateDetail.vue"
import DebateSessionsList from "@/components/debate/DebateSessionsList.vue"
import PageHeader from "@/components/layout/PageHeader.vue"
import { useDebatePage } from "@/composables/useDebatePage"

const {
  activeDebate,
  cancelDebate,
  createDebate,
  creatingDebate,
  debates,
  loadingDebates,
  messages,
  newDebate,
  personalities,
  personalityEmoji,
  selectDebate,
  showCreate,
  startDebate,
  togglePersonality,
} = useDebatePage()
</script>

<template>
  <div class="flex flex-col gap-6">
    <PageHeader
      title="AI Debate Arena"
      description="Multiple AI personalities debate trading decisions"
    >
      <template #actions>
        <BaseButton @click="showCreate = true">
          <Icon
            icon="ic:round-add"
            class="inline-block text-base align-[-0.125em]"
          />
          New Debate
        </BaseButton>
      </template>
    </PageHeader>

    <div class="grid grid-cols-1 gap-6 lg:grid-cols-3">
      <DebateSessionsList
        :debates="debates"
        :active-id="activeDebate?.id"
        :loading="loadingDebates"
        @select="selectDebate"
      />

      <div class="lg:col-span-2">
        <div v-if="!activeDebate" class="flex items-center justify-center h-64">
          <p class="text-sm text-[--color-text-muted]">
            Select a debate session to view
          </p>
        </div>

        <DebateDetail
          v-else
          :debate="activeDebate"
          :messages="messages"
          @start="startDebate"
          @cancel="cancelDebate"
        />
      </div>
    </div>
  </div>

  <CreateDebateModal
    v-if="showCreate"
    v-model="newDebate"
    :personalities="personalities"
    :creating="creatingDebate"
    :personality-emoji="personalityEmoji"
    @create="createDebate"
    @close="showCreate = false"
    @toggle-personality="togglePersonality"
  />
</template>
