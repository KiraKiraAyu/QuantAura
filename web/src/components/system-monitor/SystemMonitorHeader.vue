<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import type { TraderPayload } from "@/types/trading"

defineProps<{
  traders: TraderPayload[]
}>()

const activeTrader = defineModel<string>({ required: true })

const emit = defineEmits<{
  refresh: []
}>()
</script>

<template>
  <div class="flex items-center justify-between flex-wrap gap-4">
    <div>
      <h1 class="text-2xl font-black">System Monitor</h1>
      <p class="text-sm mt-0.5 text-[--color-text-muted]">
        Runtime metrics, alerts, and system events
      </p>
    </div>
    <div class="flex items-center gap-2">
      <select
        v-model="activeTrader"
        class="py-1.5 min-w-37.5"
        @change="emit('refresh')"
      >
        <option value="">(All System)</option>
        <option v-for="trader in traders" :key="trader.id" :value="trader.id">
          {{ trader.name || trader.id }}
        </option>
      </select>
      <BaseButton @click="emit('refresh')" class="py-1.5 text-xs text-accent">
        <Icon
          icon="ic:round-refresh"
          class="inline-block text-base align-[-0.125em]"
        />
        Refresh
      </BaseButton>
    </div>
  </div>
</template>
