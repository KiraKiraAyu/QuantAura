<script setup lang="ts">
import { onMounted, ref } from "vue"
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import AddExchangeModal from "@/components/settings/AddExchangeModal.vue"
import { deleteExchangeApi, getExchangeConfigsApi } from "@/api/exchanges"
import type { SafeExchangeConfig } from "@/types/exchanges"

const exchanges = ref<SafeExchangeConfig[]>([])
const showAddExchange = ref(false)

async function loadExchanges() {
  try {
    const data = await getExchangeConfigsApi()
    exchanges.value = Array.isArray(data) ? data : []
  } catch {
    /* ignore */
  }
}

async function deleteExchange(id: string) {
  if (!confirm("Delete this exchange?")) return
  await deleteExchangeApi(id)
  await loadExchanges()
}

onMounted(loadExchanges)
</script>

<template>
  <div class="flex flex-col gap-4">
    <div>
      <div class="flex items-center justify-between mb-4">
        <h2 class="font-bold text-sm">Exchange Accounts</h2>
        <BaseButton @click="showAddExchange = true" class="text-xs py-1.5">
          <Icon
            icon="ic:round-add"
            class="inline-block text-base align-[-0.125em]"
          />
          Add Exchange
        </BaseButton>
      </div>
      <div class="flex flex-col gap-3">
        <div
          v-for="ex in exchanges"
          :key="ex.id"
          class="p-4 rounded-xl bg-[--color-surface-elevated] border border-[--color-border-subtle]"
        >
          <div class="flex items-center justify-between">
            <div>
              <h3 class="font-semibold text-sm">
                {{ ex.account_name || ex.exchange_type }}
              </h3>
              <p class="text-xs text-[--color-text-muted]">
                {{ ex.exchange_type }}
              </p>
            </div>
            <div class="flex items-center gap-2">
              <span :class="ex.enabled ? '' : ''">
                {{ ex.enabled ? "Active" : "Disabled" }}
              </span>
              <BaseButton
                @click="deleteExchange(ex.id)"
                class="text-xs px-2 py-1 rounded text-[--color-error] bg-[oklch(0.65_0.21_15/0.1)]"
              >
                <Icon
                  icon="ic:round-delete"
                  class="inline-block text-base align-[-0.125em]"
                />
                Delete
              </BaseButton>
            </div>
          </div>
        </div>
        <div
          v-if="exchanges.length === 0"
          class="text-center py-8 text-sm text-[--color-text-muted]"
        >
          No exchanges configured. Add one to start trading.
        </div>
      </div>
    </div>
  </div>

  <AddExchangeModal
    v-if="showAddExchange"
    @close="showAddExchange = false"
    @created="loadExchanges"
  />
</template>
