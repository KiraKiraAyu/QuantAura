<script setup lang="ts">
defineProps<{
  events: any[]
  loading: boolean
  formatMetadata: (value: unknown) => string
}>()
</script>

<template>
  <div class="flex flex-col h-175">
    <div class="flex items-center justify-between mb-4">
      <h2 class="font-bold text-sm">Runtime Events</h2>
      <span>{{ events.length }} events</span>
    </div>
    <div class="flex-1 overflow-y-auto pr-2 flex flex-col gap-2">
      <div
        v-if="loading"
        class="text-center py-10 text-xs text-text-muted"
      >
        Loading events...
      </div>
      <div
        v-else-if="events.length === 0"
        class="text-center py-10 text-xs text-text-muted"
      >
        No events.
      </div>
      <div
        v-for="event in events"
        v-else
        :key="event.id"
        class="p-3 rounded-xl border flex flex-col gap-1 bg-[--color-surface-elevated] border-[--color-border-subtle]"
      >
        <div class="flex justify-between items-center text-xs mb-1">
          <span class="font-bold">{{ event.event_type }}</span>
          <span class="text-[--color-text-muted]">
            {{ new Date(event.timestamp * 1000).toLocaleTimeString() }}
          </span>
        </div>
        <span
          v-if="event.action_type"
          class="text-[0.65rem] inline-block px-1.5 py-0.5 rounded w-max bg-[--color-surface-overlay] text-[--color-text-muted]"
        >
          {{ event.action_type }}
        </span>
        <pre
          class="text-[10px] whitespace-pre-wrap font-mono overflow-x-auto mt-2 text-[--color-text-secondary]"
          >{{ formatMetadata(event.metadata) }}</pre
        >
      </div>
    </div>
  </div>
</template>
