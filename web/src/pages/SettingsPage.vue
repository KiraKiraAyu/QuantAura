<script setup lang="ts">
import Tabs from "primevue/tabs"
import TabList from "primevue/tablist"
import Tab from "primevue/tab"
import TabPanels from "primevue/tabpanels"
import TabPanel from "primevue/tabpanel"
import AIModelsTab from "@/components/settings/AIModelsTab.vue"
import ExchangeAccountsTab from "@/components/settings/ExchangeAccountsTab.vue"
import SecurityTab from "@/components/settings/SecurityTab.vue"
</script>

<template>
  <div class="flex flex-col gap-6">
    <div>
      <h1 class="text-2xl font-black text-surface-900 dark:text-white">Settings</h1>
      <p class="text-sm mt-0.5 text-surface-500 font-medium tracking-wide">
        Configure exchanges, AI models, and account security
      </p>
    </div>

    <Tabs value="0">
      <TabList>
        <Tab value="0">AI Models</Tab>
        <Tab value="1">Exchanges</Tab>
        <Tab value="2">Security</Tab>
      </TabList>
      <TabPanels>
        <TabPanel value="0">
          <AIModelsTab />
        </TabPanel>
        <TabPanel value="1">
          <ExchangeAccountsTab />
        </TabPanel>
        <TabPanel value="2">
          <SecurityTab />
        </TabPanel>
      </TabPanels>
    </Tabs>
  </div>
</template>
