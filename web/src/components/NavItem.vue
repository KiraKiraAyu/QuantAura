<script setup lang="ts">
import { Icon } from "@iconify/vue"
defineProps<{
  to: string
  icon: string
  label: string
}>()
</script>

<template>
  <router-link
    :to="to"
    class="flex items-center h-12 gap-3 px-4 text-sm no-underline bg-clip-text transition-color duration-300"
  >
    <Icon class="text-lg text-accent" :icon="icon" />
    <span>{{ label }}</span>
  </router-link>
</template>
