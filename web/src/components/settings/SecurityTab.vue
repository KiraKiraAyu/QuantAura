<script setup lang="ts">
import { ref } from "vue"
import Button from "primevue/button"
import InputText from "primevue/inputtext"
import { changePasswordApi } from "@/api/auth"
import { useAuthStore } from "@/stores/auth"
import { useToast } from "primevue/usetoast"

const auth = useAuthStore()
const toast = useToast()
const pwForm = ref({ current: "", newPw: "", confirm: "" })
const pwMsg = ref("")

async function changePassword() {
  pwMsg.value = ""
  if (pwForm.value.newPw !== pwForm.value.confirm) {
    pwMsg.value = "Passwords do not match"
    return
  }
  try {
    await changePasswordApi({
      current_password: pwForm.value.current,
      new_password: pwForm.value.newPw,
    })
    toast.add({ severity: "success", summary: "Success", detail: "Password updated successfully", life: 3000 })
    pwForm.value = { current: "", newPw: "", confirm: "" }
  } catch (e: unknown) {
    pwMsg.value = e instanceof Error ? e.message : "Failed to update password"
  }
}

function logout() {
  auth.logout()
}
</script>

<template>
  <div class="flex flex-col gap-8 max-w-sm mt-4">
    <div>
      <h2 class="font-bold text-lg text-surface-900 dark:text-white mb-6">Change Password</h2>
      <div class="flex flex-col gap-4">
        <div class="flex flex-col gap-1">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">Current Password</label>
          <InputText v-model="pwForm.current" type="password" />
        </div>
        <div class="flex flex-col gap-1">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">New Password</label>
          <InputText v-model="pwForm.newPw" type="password" />
        </div>
        <div class="flex flex-col gap-1">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">Confirm New Password</label>
          <InputText v-model="pwForm.confirm" type="password" />
        </div>
        <p v-if="pwMsg" class="text-xs font-medium text-rose-500 dark:text-rose-400">
          {{ pwMsg }}
        </p>
        <Button
          label="Update Password"
          icon="pi pi-lock"
          class="mt-2"
          @click="changePassword"
        />
      </div>
    </div>

    <div>
      <h2 class="font-bold text-lg text-surface-900 dark:text-white mb-2">Danger Zone</h2>
      <p class="text-xs mb-4 text-surface-500 font-medium tracking-wide">
        These actions are irreversible.
      </p>
      <Button
        label="Sign Out"
        icon="pi pi-sign-out"
        severity="danger"
        variant="outlined"
        @click="logout"
      />
    </div>
  </div>
</template>
