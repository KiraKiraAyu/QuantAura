import { defineStore } from "pinia"
import { ref, computed } from "vue"
import { loginApi, logoutApi, registerApi } from "@/api/auth"
import router from "@/router"

export const useAuthStore = defineStore("auth", () => {
  const token = ref<string>(localStorage.getItem("amaryllis_token") ?? "")
  const userId = ref<string>(localStorage.getItem("amaryllis_user_id") ?? "")
  const email = ref<string>(localStorage.getItem("amaryllis_email") ?? "")

  const isLoggedIn = computed(() => !!token.value)
  const username = computed(() => email.value.split("@")[0] || email.value)

  function persist() {
    localStorage.setItem("amaryllis_token", token.value)
    localStorage.setItem("amaryllis_user_id", userId.value)
    localStorage.setItem("amaryllis_email", email.value)
  }

  function setSession(tok: string, uid: string, em: string) {
    token.value = tok
    userId.value = uid
    email.value = em
    persist()
  }

  async function login(emailVal: string, password: string) {
    const data = await loginApi({ email: emailVal, password })
    setSession(data.token, data.user_id, data.email)
    return data
  }

  async function register(emailVal: string, password: string) {
    const data = await registerApi({ email: emailVal, password })
    setSession(data.token, data.user_id, data.email)
    return data
  }

  async function logout() {
    if (token.value) logoutApi().catch(() => {})
    token.value = ""
    userId.value = ""
    email.value = ""
    localStorage.removeItem("amaryllis_token")
    localStorage.removeItem("amaryllis_user_id")
    localStorage.removeItem("amaryllis_email")
    router.push("/login")
  }

  return {
    token,
    userId,
    email,
    isLoggedIn,
    username,
    setSession,
    login,
    register,
    logout,
  }
})
