<script setup lang="ts">
import { reactive, ref, watch } from "vue"
import { useAuthStore } from "@/stores/auth"
import { useRealtimeStore } from "@/stores/realtime"
import { useRouter } from "vue-router"
import AuthPanel from "@/components/auth/AuthPanel.vue"
import type { AuthMode } from "@/types/auth-ui"

const auth = useAuthStore()
const realtime = useRealtimeStore()
const router = useRouter()

const mode = ref<AuthMode>("Login")
const loading = ref(false)
const form = reactive({ email: "", password: "" })

const emailError = ref("")
const passwordError = ref("")

const validateForm = (field: "email" | "password") => {
  if (mode.value !== "Login") {
    if (field === "email") {
      emailError.value = validateEmail(form.email)
    } else {
      passwordError.value = validatePassword(form.password)
    }
  } else {
    emailError.value = ""
    passwordError.value = ""
  }
}

const validateEmail = (email: string) => {
  if (!email) {
    emailError.value = ""
    return ""
  }
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  if (!emailRegex.test(email)) {
    return "Invalid email address"
  }
  return ""
}

const validatePassword = (password: string) => {
  if (!password) {
    passwordError.value = ""
    return ""
  }
  if (password.length < 8) {
    return "Password must be at least 8 characters long"
  }
  return ""
}

watch(mode, () => {
  validateForm("email")
  validateForm("password")
})

async function submitCredentials() {
  if (emailError.value || passwordError.value) {
    return
  }
  loading.value = true
  try {
    if (mode.value === "Register") {
      await auth.register(form.email, form.password)
    } else {
      await auth.login(form.email, form.password)
    }
    realtime.connect()
    await router.push({ name: "dashboard" })
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <AuthPanel
    v-model:mode="mode"
    v-model:email="form.email"
    v-model:password="form.password"
    :loading="loading"
    :email-error="emailError"
    :password-error="passwordError"
    @validate="validateForm"
    @submit="submitCredentials"
  />
</template>
