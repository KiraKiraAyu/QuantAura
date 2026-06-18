<script setup lang="ts">
import { computed } from "vue"
import Button from "primevue/button"
import Dialog from "primevue/dialog"
import CompetitionEquityPanel from "@/components/competition/CompetitionEquityPanel.vue"
import CompetitionLeaderboard from "@/components/competition/CompetitionLeaderboard.vue"
import CompetitionPodium from "@/components/competition/CompetitionPodium.vue"
import PageHeader from "@/components/layout/PageHeader.vue"
import { useCompetitionPage } from "@/composables/useCompetitionPage"

const {
  avatarStyle,
  filtered,
  fmt,
  lastUpdated,
  load,
  loading,
  returnPct,
  search,
  selectedEquity,
  selectedTrader,
  showDetail,
  topThree,
} = useCompetitionPage()

const showDialog = computed({
  get: () => selectedTrader.value !== null,
  set: (val) => {
    if (!val) selectedTrader.value = null
  }
})
</script>

<template>
  <div class="flex flex-col gap-6">
    <PageHeader
      title="Competition"
      description="Live AI trader leaderboard - Updates every scan interval"
    >
      <template #actions>
        <div class="flex items-center gap-4">
          <div class="flex items-center gap-2">
            <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
            <span class="text-xs text-surface-500 font-medium">{{
              lastUpdated
            }}</span>
          </div>
          <Button
            label="Refresh"
            icon="pi pi-refresh"
            size="small"
            severity="secondary"
            variant="outlined"
            @click="load"
          />
        </div>
      </template>
    </PageHeader>

    <CompetitionPodium
      v-if="!loading && topThree.length"
      :traders="topThree"
      :avatar-style="avatarStyle"
      :fmt="fmt"
      :return-pct="returnPct"
    />

    <CompetitionLeaderboard
      v-model="search"
      :traders="filtered"
      :loading="loading"
      :avatar-style="avatarStyle"
      :fmt="fmt"
      :return-pct="returnPct"
      @select="showDetail"
    />

    <Dialog
      v-model:visible="showDialog"
      modal
      :header="selectedTrader ? `${selectedTrader.trader_name || selectedTrader.trader_id} - Equity Curve` : 'Equity Curve'"
      :style="{ width: '50rem' }"
    >
      <CompetitionEquityPanel
        v-if="selectedTrader"
        :trader="selectedTrader"
        :data="selectedEquity"
      />
    </Dialog>
  </div>
</template>
