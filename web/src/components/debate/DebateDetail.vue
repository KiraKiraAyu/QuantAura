<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import type { DebateMessage, DebateSession } from "@/types/debate-ui"

defineProps<{
  debate: DebateSession
  messages: DebateMessage[]
}>()

const emit = defineEmits<{
  start: [id: string]
  cancel: [id: string]
}>()

function personalityEmoji(personality: string) {
  const icons: Record<string, string> = {
    bull: "🐂",
    bear: "🐻",
    analyst: "🔬",
    contrarian: "🎭",
    risk_manager: "🛡️",
  }
  return icons[personality] ?? "🤖"
}

function msgStyle(personality: string) {
  const colors: Record<string, string> = {
    bull: "oklch(0.72 0.17 145 / 0.08)",
    bear: "oklch(0.65 0.21 15 / 0.08)",
    analyst: "oklch(0.66 0.058 301 / 0.1)",
    contrarian: "oklch(0.82 0.16 85 / 0.08)",
    risk_manager: "oklch(0.65 0.21 236 / 0.08)",
  }
  return `background-color:${colors[personality] ?? "var(--color-surface-elevated)"};border:1px solid oklch(0.4 0.04 301 / 0.3)`
}
</script>

<template>
  <div class="flex flex-col gap-4">
    <div>
      <div class="flex items-center gap-3 flex-wrap">
        <div class="flex-1">
          <h2 class="font-bold">{{ debate.name || debate.symbol }}</h2>
          <p class="text-xs mt-0.5 text-[--color-text-muted]">
            Round {{ debate.current_round }}/{{ debate.max_rounds }} ·
            <span>{{ debate.status }}</span>
          </p>
        </div>
        <BaseButton
          v-if="debate.status === 'pending' || debate.status === 'failed'"
          @click="emit('start', debate.id)"
          class="text-xs py-1.5"
        >
          <Icon
            icon="ic:round-play-arrow"
            class="inline-block text-base align-[-0.125em]"
          />
          Start
        </BaseButton>
        <BaseButton
          v-if="debate.status === 'running'"
          @click="emit('cancel', debate.id)"
          class="text-xs py-1.5"
        >
          <Icon
            icon="ic:round-stop"
            class="inline-block text-base align-[-0.125em]"
          />
          Cancel
        </BaseButton>
      </div>
    </div>

    <div v-if="debate.final_decision" class="border border-[--color-accent]">
      <h3 class="font-bold text-sm mb-3">Final Decision</h3>
      <div class="flex items-center gap-3">
        <span class="text-2xl font-black">{{ debate.final_decision }}</span>
        <span class="text-xs text-[--color-text-muted]">
          {{ debate.final_reasoning }}
        </span>
      </div>
    </div>

    <div>
      <h3 class="font-bold text-sm mb-3">Debate Thread</h3>
      <div class="flex flex-col gap-3 max-h-125 overflow-auto pr-1">
        <div
          v-if="messages.length === 0"
          class="text-center py-8 text-sm text-[--color-text-muted]"
        >
          No messages yet. Start the debate to begin.
        </div>
        <div
          v-for="(message, index) in messages"
          :key="index"
          class="p-3 rounded-xl text-sm"
          :style="msgStyle(message.personality)"
        >
          <div class="flex items-center gap-2 mb-1.5">
            <span class="text-base">
              {{ personalityEmoji(message.personality) }}
            </span>
            <span class="font-bold capitalize">{{ message.personality }}</span>
            <span class="text-xs ml-auto text-[--color-text-muted]">
              Round {{ message.round }}
            </span>
            <span v-if="message.vote">{{ message.vote }}</span>
          </div>
          <p class="text-xs leading-relaxed text-[--color-text-secondary]">
            {{ message.content }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>
