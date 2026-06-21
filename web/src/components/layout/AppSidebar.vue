<script setup lang="ts">
import { computed } from "vue"
import { useRoute, RouterLink } from "vue-router"
import Button from "primevue/button"
import { useAuthStore } from "@/stores/auth"
import { useRealtimeStore } from "@/stores/realtime"

defineProps<{ isDark: boolean }>()
const emit = defineEmits<{ (e: "toggle-theme"): void }>()

const route = useRoute()
const authStore = useAuthStore()
const realtime = useRealtimeStore()

const nav = computed(() => [
  { label: "Dashboard", to: "/", icon: "pi pi-chart-bar" },
  { label: "Market Data", to: "/data", icon: "pi pi-chart-line" },
  { label: "Strategy", to: "/strategy", icon: "pi pi-sliders-h" },
  { label: "Backtest", to: "/backtest", icon: "pi pi-history" },
  { label: "AI Debate", to: "/debate", icon: "pi pi-comments" },
  { label: "Competition", to: "/competition", icon: "pi pi-trophy" },
  { label: "Monitor", to: "/monitor", icon: "pi pi-server" },
  { label: "Settings", to: "/settings", icon: "pi pi-cog" },
])
</script>

<template>
  <aside
    class="fixed inset-y-5 left-5 hidden w-64 flex-col items-stretch rounded-2xl bg-surface-50 dark:bg-surface-900 px-4 py-6 lg:flex transition-colors duration-300"
  >
    <!-- Header -->
    <div class="flex items-center gap-3 px-2">
      <div
        class="grid h-10 w-10 shrink-0 place-items-center rounded-xl bg-primary-500 text-white font-black text-xl shadow-lg shadow-primary-500/20"
        aria-label="QuantAura"
      >
        A
      </div>
      <span class="text-xl font-bold tracking-wide text-surface-900 dark:text-surface-0">QuantAura</span>
    </div>

    <!-- Nav Items -->
    <nav class="mt-8 grid gap-2">
      <RouterLink
        v-for="item in nav"
        :key="item.to"
        :to="item.to"
        class="nav-link"
        :class="{ 'is-active': item.to === '/' ? route.path === '/' : route.path.startsWith(item.to) }"
        :aria-label="item.label"
        :title="item.label"
      >
        <span :class="item.icon" class="text-lg w-6 text-center"></span>
        <span>{{ item.label }}</span>
      </RouterLink>
    </nav>

    <!-- Bottom Actions -->
    <div class="mt-auto flex flex-col gap-3">
      <!-- Connection Status Indicator -->
      <div
        class="h-11 w-full px-4 flex items-center gap-3 rounded-xl transition-all"
        :class="
          realtime.isConnected
            ? 'text-emerald-500 bg-emerald-50 dark:bg-emerald-950/20 border border-emerald-200 dark:border-emerald-900/30'
            : 'text-surface-400 bg-surface-100 dark:bg-surface-800 border border-surface-200 dark:border-surface-700 animate-pulse'
        "
        :title="realtime.isConnected ? 'Connection: Live' : 'Connection: Offline'"
      >
        <span
          class="pi text-sm"
          :class="realtime.isConnected ? 'pi-wifi' : 'pi-exclamation-triangle'"
        ></span>
        <span class="font-medium text-sm">{{ realtime.isConnected ? 'Connected' : 'Offline' }}</span>
      </div>

      <!-- Dark Mode Switcher -->
      <button
        class="cursor-pointer h-11 w-full px-4 flex items-center gap-3 rounded-xl text-surface-600 dark:text-surface-400 hover:bg-surface-100 dark:hover:bg-surface-800 hover:text-surface-900 dark:hover:text-surface-0 transition-colors font-medium text-sm"
        @click="emit('toggle-theme')"
      >
        <span class="pi text-lg w-6 text-center" :class="isDark ? 'pi-sun' : 'pi-moon'"></span>
        <span>{{ isDark ? 'Light Mode' : 'Dark Mode' }}</span>
      </button>

      <!-- User Profile & Logout -->
      <div class="flex items-center gap-3 border-t border-surface-200 dark:border-surface-800 pt-4 mt-1 w-full">
        <div
          class="w-10 h-10 shrink-0 rounded-full bg-primary-100 dark:bg-primary-950/40 text-primary-600 dark:text-primary-400 flex items-center justify-center text-sm font-bold"
          :title="authStore.username"
        >
          {{ (authStore.username || "?").charAt(0).toUpperCase() }}
        </div>
        <div class="flex-1 truncate text-sm font-medium text-surface-700 dark:text-surface-300">
          {{ authStore.username || 'User' }}
        </div>
        <Button
          text
          rounded
          icon="pi pi-sign-out"
          class="h-9 w-9 shrink-0 text-surface-500 hover:text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950/20 transition-colors"
          @click="authStore.logout()"
          title="Sign Out"
        />
      </div>
    </div>
  </aside>
</template>
