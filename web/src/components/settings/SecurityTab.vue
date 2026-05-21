<script setup lang="ts">
import { ref } from "vue"
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import BaseInput from "@/components/universal/BaseInput.vue"
import { changePasswordApi } from "@/api/auth"
import { useAuthStore } from "@/stores/auth"

const auth = useAuthStore()
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
    pwMsg.value = "✓ Password updated"
    pwForm.value = { current: "", newPw: "", confirm: "" }
  } catch (e: unknown) {
    pwMsg.value = e instanceof Error ? e.message : "Failed"
  }
}

function logout() {
  auth.logout()
}
</script>

<template>
  <div class="flex flex-col gap-4">
    <div>
      <h2 class="font-bold text-sm mb-4">Change Password</h2>
      <div class="flex flex-col gap-3 max-w-sm">
        <div>
          <label>Current Password</label>
          <BaseInput v-model="pwForm.current" type="password" />
        </div>
        <div>
          <label>New Password</label>
          <BaseInput v-model="pwForm.newPw" type="password" />
        </div>
        <div>
          <label>Confirm New Password</label>
          <BaseInput v-model="pwForm.confirm" type="password" />
        </div>
        <p
          v-if="pwMsg"
          class="text-xs"
          :class="pwMsg.startsWith('✓') ? '' : ''"
        >
          {{ pwMsg }}
        </p>
        <BaseButton @click="changePassword" class="w-fit">
          <Icon
            icon="ic:round-lock"
            class="inline-block text-base align-[-0.125em]"
          />
          Update Password
        </BaseButton>
      </div>
    </div>

    <div>
      <h2 class="font-bold text-sm mb-2">Danger Zone</h2>
      <p class="text-xs mb-4 text-[--color-text-muted]">
        These actions are irreversible.
      </p>
      <BaseButton @click="logout" class="text-xs py-1.5 px-4">
        <Icon
          icon="ic:round-logout"
          class="inline-block text-base align-[-0.125em]"
        />
        Sign Out
      </BaseButton>
    </div>
  </div>
</template>
