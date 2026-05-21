<script setup lang="ts">
import { RouterView } from "vue-router"
import { onMounted, watch } from "vue"
import { useAuthStore } from "@/stores/auth"
import { useRealtimeStore } from "@/stores/realtime"

const auth = useAuthStore()
const realtime = useRealtimeStore()

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
  <RouterView />
</template>
