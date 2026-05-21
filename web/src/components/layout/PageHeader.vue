<script setup lang="ts">
defineProps<{
  title: string
  description?: string
}>()
</script>

<template>
  <div class="flex items-center justify-between flex-wrap gap-3">
    <div>
      <h1 class="text-2xl font-black">{{ title }}</h1>
      <p v-if="description" class="text-sm mt-0.5 text-[--color-text-muted]">
        {{ description }}
      </p>
    </div>
    <div v-if="$slots.actions">
      <slot name="actions" />
    </div>
  </div>
</template>
