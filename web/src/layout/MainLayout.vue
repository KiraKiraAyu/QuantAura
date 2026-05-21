<script setup lang="ts">
import { RouterView } from 'vue-router'
import SideBar from '@/components/SideBar.vue'
</script>

<template>
    <div class="flex min-h-screen">
        <SideBar class="fixed h-screen w-64 bg-surface border-r border-border-subtle flex flex-col" />

        <main class="ml-64 p-8 flex-1 bg-reisa-lilac-900/50 overflow-auto">
            <RouterView />
        </main>
    </div>
</template>
