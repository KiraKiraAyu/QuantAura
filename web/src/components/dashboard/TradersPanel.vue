<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import TraderRow from "@/components/TraderRow.vue"
import type { DashboardTrader } from "@/types/dashboard-ui"

defineProps<{
  traders: DashboardTrader[]
  loading: boolean
  initialLoadDone: boolean
}>()

const emit = defineEmits<{
  create: []
  start: [id: string]
  stop: [id: string]
  sync: [id: string]
}>()
</script>

<template>
  <div>
    <div class="flex items-center justify-between mb-4">
      <h2 class="font-bold text-sm">Traders</h2>
      <BaseButton @click="emit('create')" class="py-1.5 text-xs">
        <Icon
          icon="ic:round-add"
          class="inline-block text-base align-[-0.125em]"
        />
        New Trader
      </BaseButton>
    </div>

    <div
      v-if="!initialLoadDone && loading"
      class="text-center text-sm py-8 text-[--color-text-muted]"
    >
      Loading...
    </div>
    <div v-else-if="traders.length === 0" class="text-center py-8">
      <p class="text-sm text-[--color-text-muted]">
        No traders yet. Create one to start.
      </p>
    </div>
    <div v-else class="flex flex-col gap-2">
      <TraderRow
        v-for="trader in traders"
        :key="trader.id"
        :trader="trader"
        @start="emit('start', trader.id)"
        @stop="emit('stop', trader.id)"
        @sync="emit('sync', trader.id)"
      />
    </div>
  </div>
</template>
