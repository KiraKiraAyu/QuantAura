<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import BaseInput from "@/components/universal/BaseInput.vue"
import type { DebateDraft } from "@/types/debate-ui"

defineProps<{
  personalities: string[]
  creating: boolean
  personalityEmoji: (personality: string) => string
}>()

const draft = defineModel<DebateDraft>({ required: true })

const emit = defineEmits<{
  create: []
  close: []
  togglePersonality: [personality: string]
}>()
</script>

<template>
  <div
    class="fixed inset-0 flex items-center justify-center z-50 p-4 bg-black/60"
  >
    <div class="w-full max-w-md">
      <h2 class="font-bold mb-4">Create Debate</h2>
      <div class="flex flex-col gap-3">
        <div>
          <label>Name</label>
          <BaseInput v-model="draft.name" placeholder="BTC Bull/Bear Debate" />
        </div>
        <div>
          <label>Symbol</label>
          <BaseInput v-model="draft.symbol" placeholder="BTCUSDT" />
        </div>
        <div>
          <label>Max Rounds</label>
          <BaseInput
            v-model.number="draft.max_rounds"
            type="number"
            min="1"
            max="10"
          />
        </div>
        <div>
          <label>Participants</label>
          <div class="flex flex-wrap gap-2">
            <BaseButton
              v-for="personality in personalities"
              :key="personality"
              @click="emit('togglePersonality', personality)"
              class="px-3 py-1.5 rounded-lg text-xs font-semibold transition-all"
              :class="
                draft.participants.includes(personality)
                  ? 'bg-surface-overlay text-accent border border-accent'
                  : 'border border-border-subtle text-text-muted'
              "
            >
              {{ personalityEmoji(personality) }} {{ personality }}
            </BaseButton>
          </div>
        </div>
        <div class="flex gap-3 mt-2">
          <BaseButton
            @click="emit('create')"
            class="flex-1"
            :disabled="creating"
          >
            <Icon
              icon="ic:round-check"
              class="inline-block text-base align-[-0.125em]"
            />
            {{ creating ? "Creating..." : "Create" }}
          </BaseButton>
          <BaseButton @click="emit('close')">
            <Icon
              icon="ic:round-close"
              class="inline-block text-base align-[-0.125em]"
            />
            Cancel
          </BaseButton>
        </div>
      </div>
    </div>
  </div>
</template>
