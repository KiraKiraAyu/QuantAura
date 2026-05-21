<script setup lang="ts">
import { computed } from 'vue';

interface Props {
  variant?: 'outline' | 'emphasis' | 'square'
  text?: string
  type?: 'button' | 'submit' | 'reset'
}

const { variant = 'outline', text = '', type = 'button' } = defineProps<Props>()
const candiate = {
  outline: 'rounded-full py-2 px-4 text-sm bg-transparent border border-border text-text-secondary hover:bg-surface-elevated hover:text-text-primary',
  emphasis:
      'rounded-full py-2 px-4 text-sm border-0 bg-linear-to-br from-reisa-lilac-500 to-reisa-lilac-600 text-white hover:from-reisa-lilac-400 hover:to-reisa-lilac-500',
  square: 'rounded-lg  aspect-square justify-center',
}

const classes = computed(() => {
  return candiate[variant]
})
</script>

<template>
    <button :type="type" class="cursor-pointer" :class="classes">
        <template v-if="text">
            <div v-if="$slots.default">
                <slot />
            </div>
            <div class="relative overflow-hidden inline-flex">
                <span
                    class="inline-block transition-transform duration-400 ease-[cubic-bezier(.17,.67,.39,1.35)] group-hover:-transform-y-8"
                >
                    {{ text }}
                </span>
                <span
                    class="inline-block absolute inset-0 transition-transform duration-400 ease-[cubic-bezier(.17,.67,.39,1.35)] translate-y-8 group-hover:translate-y-0"
                >
                    {{ text }}
                </span>
            </div>
        </template>
        <slot v-else />
    </button>
</template>
