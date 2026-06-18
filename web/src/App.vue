<script setup lang="ts">
import { RouterView } from "vue-router"
import { onMounted, watch } from "vue"
import { useAuthStore } from "@/stores/auth"
import { useRealtimeStore } from "@/stores/realtime"
import Toast from "primevue/toast"
import { useToast } from "@/stores/toast"
import { useToast as usePrimeToast } from "primevue/usetoast"

const auth = useAuthStore()
const realtime = useRealtimeStore()
const toastStore = useToast()
const primeToast = usePrimeToast()

// Watch custom Pinia toast store to dispatch toast messages via PrimeVue Toast globally
watch(
  () => toastStore.toastEvent,
  (evt) => {
    if (evt) {
      primeToast.add({
        severity: evt.severity,
        summary: evt.summary,
        detail: evt.detail,
        life: evt.life,
      })
      toastStore.toastEvent = null // Reset channel
    }
  },
  { deep: true }
)

onMounted(() => {
  if (auth.isLoggedIn) {
    realtime.connect()
  }
})

watch(
  () => auth.token,
  (token) => {
    if (token) {
      realtime.connect()
    } else {
      realtime.disconnect()
    }
  },
)
</script>

<template>
  <Toast />
  <RouterView />
</template>
