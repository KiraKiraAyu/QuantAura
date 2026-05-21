<script setup lang="ts">
import { computed } from "vue"

const props = defineProps<{
  title: string
  value: number
  prefix?: string
  signed?: boolean
  loading?: boolean
}>()

const formatted = computed(() => {
  const v = props.value ?? 0
  const opts: Intl.NumberFormatOptions = {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }
  if (typeof v === "number") {
    return (props.signed && v >= 0 ? "+" : "") + v.toLocaleString("en-US", opts)
  }
  return String(v)
})

const colorClass = computed(() => {
  if (!props.signed) return ""
  return (props.value ?? 0) >= 0 ? "" : ""
})
</script>

<template>
  <div class="flex flex-col gap-2">
    <p>{{ title }}</p>
    <div
      v-if="loading"
      class="h-8 rounded-lg animate-pulse bg-[--color-surface-elevated]"
    ></div>
    <p v-else class="font-mono" :class="colorClass">
      <span v-if="prefix" class="text-lg text-[--color-text-muted]">{{
        prefix
      }}</span>
      {{ formatted }}
    </p>
  </div>
</template>
