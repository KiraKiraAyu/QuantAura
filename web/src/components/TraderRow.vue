<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
defineProps<{ trader: Record<string, unknown> }>()
defineEmits(["start", "stop", "sync"])
</script>

<template>
  <div
    class="flex items-center gap-3 py-3 px-3 rounded-xl transition-colors bg-[--color-surface-elevated]"
  >
    <!-- Status indicator -->
    <div
      class="w-2 h-2 rounded-full shrink-0"
      :class="
        trader.is_running
          ? 'bg-success animate-pulse'
          : 'bg-text-muted'
      "
    ></div>

    <!-- Info -->
    <div class="flex-1 min-w-0">
      <p class="text-sm font-semibold truncate">
        {{ trader.name || trader.id }}
      </p>
      <p class="text-xs truncate text-[--color-text-muted]">
        {{ trader.ai_model_id ?? trader.ai_model ?? "" }} ·
        {{ trader.exchange_id ?? "" }}
      </p>
    </div>

    <!-- Actions -->
    <div class="flex items-center gap-1.5 shrink-0">
      <span
        :class="trader.is_running ? '' : ''"
        class="text-[0.65rem]"
      >
        {{ trader.is_running ? "Running" : "Stopped" }}
      </span>

      <!-- Sync balance -->
      <BaseButton
        @click="$emit('sync')"
        class="text-xs px-2 py-1 rounded-lg transition-colors text-[--color-text-muted] bg-[--color-surface-overlay]"
        title="Sync balance">
        <Icon icon="ic:round-refresh" class="inline-block text-base align-[-0.125em]" />

      </BaseButton>

      <!-- Start / Stop -->
      <BaseButton
        v-if="!trader.is_running"
        @click="$emit('start')"
        class="text-xs px-2.5 py-1 rounded-lg font-semibold transition-colors bg-[oklch(0.72_0.17_145/0.15)] text-[--color-success]">
        <Icon icon="ic:round-play-arrow" class="inline-block text-base align-[-0.125em]" />

      </BaseButton>
      <BaseButton
        v-else
        @click="$emit('stop')"
        class="text-xs px-2.5 py-1 rounded-lg font-semibold transition-colors bg-[oklch(0.65_0.21_15/0.15)] text-[--color-error]">
        <Icon icon="ic:round-stop" class="inline-block text-base align-[-0.125em]" />

      </BaseButton>
    </div>
  </div>
</template>
