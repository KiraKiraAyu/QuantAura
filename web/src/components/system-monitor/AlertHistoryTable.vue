<script setup lang="ts">
import type { RuntimeAlertHistoryItemPayload, RuntimeAlertItemPayload } from "@/types/trading"
import { formatDateTime } from "@/utils/format"

defineProps<{
  alerts: RuntimeAlertHistoryItemPayload[]
  loading: boolean
  parseAlerts: (value: RuntimeAlertItemPayload[]) => string
}>()
</script>

<template>
  <div class="flex-1">
    <div class="flex items-center justify-between mb-4">
      <h2 class="font-bold text-sm">Alert History</h2>
      <span>{{ alerts.length }} total</span>
    </div>
    <div
      v-if="loading"
      class="text-center text-xs py-10 text-[--color-text-muted]"
    >
      Loading...
    </div>
    <div
      v-else-if="alerts.length === 0"
      class="text-center py-10 text-xs text-[--color-text-muted]"
    >
      No alerts logged in the selected window.
    </div>
    <div v-else class="overflow-y-auto max-h-75">
      <table class="w-full text-xs">
        <thead>
          <tr class="text-[--color-text-muted]">
            <th class="text-left font-semibold py-2">Time</th>
            <th class="text-left font-semibold py-2">Severity</th>
            <th class="text-left font-semibold py-2">Details</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="alert in alerts"
            :key="alert.id"
            class="border-t border-[--color-border-subtle]"
          >
            <td class="py-2 text-[--color-text-secondary]">
              {{ formatDateTime(alert.created_at) }}
            </td>
            <td class="py-2">
              <span>{{ alert.severity }}</span>
            </td>
            <td class="py-2 text-[--color-text-primary]">
              Triggers: {{ parseAlerts(alert.alerts) }}
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
