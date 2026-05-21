<script lang="ts" setup>
import { computed, useAttrs, useId } from "vue"

defineOptions({
  inheritAttrs: false,
})

interface Props {
  variant?: "form" | "search"
  label?: string
  id?: string
  error?: string
}

const props = withDefaults(defineProps<Props>(), {
  variant: "form",
  label: "",
})

const attrs = useAttrs()
const fallbackId = useId()

const inputId = computed(() => props.id || fallbackId)
const inputPlaceholder = computed(() => {
  if (props.label) return " "

  const placeholder = attrs.placeholder
  return typeof placeholder === "string" ? placeholder : ""
})

const model = defineModel<string | number | boolean>()
</script>

<template>
  <div class="group p-2">
    <label
      v-if="label"
      :for="inputId"
      class="z-1 block text-left transition-transform duration-300 relative translate-x-4 translate-y-8 pointer-events-none group-focus-within:translate-y-0 group-focus-within:translate-x-1 group-focus-within:scale-95 group-has-[input:not(:placeholder-shown)]:translate-y-0 group-has-[input:not(:placeholder-shown)]:translate-x-1 group-has-[input:not(:placeholder-shown)]:scale-95"
    >
      {{ label }}
    </label>
    <input
      :id="inputId"
      v-model="model"
      v-bind="$attrs"
      :placeholder="inputPlaceholder"
      class="focus:outline-none w-full rounded-full h-10 p-4 text-md border border-reisa-lilac-600"
    />
    <p v-show="error" class="text-red-500 text-sm">{{ error }}</p>
  </div>
</template>
