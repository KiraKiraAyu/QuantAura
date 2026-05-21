<script setup lang="ts">
import AIModelsTab from "@/components/settings/AIModelsTab.vue"
import ExchangeAccountsTab from "@/components/settings/ExchangeAccountsTab.vue"
import SecurityTab from "@/components/settings/SecurityTab.vue"
import HorizontalTab from "@/components/universal/HorizontalTab.vue"
import { ref } from "vue"

const tabs = ["AI Models", "Exchanges", "Security"]
const selectedTabIndex = ref(0)
</script>

<template>
  <div class="flex flex-col gap-6">
    <div>
      <h1 class="text-2xl font-black">Settings</h1>
      <p class="text-sm mt-0.5 text-text-muted">
        Configure exchanges, AI models, and account security
      </p>
    </div>

    <HorizontalTab :tabs="tabs" v-model:selected-index="selectedTabIndex" />

    <AIModelsTab v-if="selectedTabIndex === 0" />
    <ExchangeAccountsTab v-if="selectedTabIndex === 1" />
    <SecurityTab v-if="selectedTabIndex === 2" />
  </div>
</template>
