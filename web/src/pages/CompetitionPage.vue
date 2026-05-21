<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
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
</script>

<template>
  <div class="flex flex-col gap-6">
    <PageHeader
      title="Competition"
      description="Live AI trader leaderboard - Updates every scan interval"
    >
      <template #actions>
        <div class="flex items-center gap-2">
          <span class="w-2 h-2 rounded-full bg-success animate-pulse"></span>
          <span class="text-xs text-[--color-text-muted]">{{
            lastUpdated
          }}</span>
          <BaseButton @click="load" class="text-xs py-1.5 px-3">
            <Icon
              icon="ic:round-refresh"
              class="inline-block text-base align-[-0.125em]"
            />
            Refresh
          </BaseButton>
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

    <CompetitionEquityPanel
      v-if="selectedTrader"
      :trader="selectedTrader"
      :data="selectedEquity"
      @close="selectedTrader = null"
    />
  </div>
</template>
