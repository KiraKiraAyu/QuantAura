<script setup lang="ts">
import { computed } from 'vue'

defineProps<{ tabs: string[] }>()
const selectedIndex = defineModel<number>('selected-index', { required: true })

const offset = computed(() => selectedIndex.value * (128 + 16))

function handleTabClick(index: number) {
    selectedIndex.value = index
}
</script>

<template>
    <nav class="flex gap-4">
        <button v-for="(tab, index) in tabs" :key="index" @click="handleTabClick(index)" class="w-32 h-8 z-1 cursor-pointer">
            {{ tab }}
        </button>
        <span class="w-32 h-8 inline-block absolute bg-accent/50 z-0 rounded-full transition-transform duration-300" :style="{ transform: `translateX(${offset}px)` }"></span>
    </nav>
</template>
