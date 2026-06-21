<script setup lang="ts">
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import BaseInput from "@/components/universal/BaseInput.vue"
import type { AuthMode } from "@/types/auth-ui"

defineProps<{
  loading: boolean
  error: string
  emailError: string
  passwordError: string
}>()

const mode = defineModel<AuthMode>("mode", { required: true })
const email = defineModel<string>("email", { required: true })
const password = defineModel<string>("password", { required: true })

const emit = defineEmits<{
  submit: []
  validate: [field: "email" | "password"]
}>()

function toggleMode() {
  mode.value = mode.value === "Register" ? "Login" : "Register"
}

function submitAs(nextMode: AuthMode) {
  mode.value = nextMode
  emit("submit")
}
</script>

<template>
  <div
    class="min-h-screen flex items-center justify-center p-4 bg-[radial-gradient(circle_at_top_left,oklch(0.32_0.11_301),transparent_34%),radial-gradient(circle_at_bottom_right,oklch(0.24_0.11_225),transparent_32%),oklch(0.145_0.018_285)]"
  >
    <div
      class="relative w-full max-w-4xl overflow-hidden rounded-4xl border border-white/10 bg-[oklch(0.19_0.018_285)] shadow-2xl shadow-black/50 min-h-155"
    >
      <div
        class="absolute left-0 top-0 hidden h-full w-1/2 transition-transform duration-700 ease-in-out md:block z-20"
        :class="{ 'translate-x-full': mode === 'Register' }"
      >
        <form
          class="flex h-full flex-col items-center justify-center bg-[oklch(0.18_0.018_285)] px-12 text-center text-white"
          @submit.prevent="submitAs('Login')"
        >
          <div
            class="mb-5 inline-flex h-14 w-14 items-center justify-center rounded-2xl bg-linear-to-br from-reisa-pink-500 to-reisa-lilac-500 shadow-lg shadow-reisa-pink-500/20"
          >
            <span class="text-2xl font-black text-white">A</span>
          </div>
          <h1 class="mb-2 text-3xl font-black text-white">Welcome back</h1>
          <p class="mb-6 text-sm text-white/55">Sign in to QuantAura</p>

          <BaseInput
            v-model="email"
            id="login-email"
            label="Email"
            type="email"
            autocomplete="email"
            required
            class="w-full border-white/15 bg-white/5 text-white"
            :error="emailError"
            @blur="emit('validate', 'email')"
          />
          <BaseInput
            v-model="password"
            id="login-password"
            label="Password"
            type="password"
            autocomplete="current-password"
            required
            class="w-full border-white/15 bg-white/5 text-white"
            :error="passwordError"
            @blur="emit('validate', 'password')"
          />

          <a
            href="#"
            class="my-4 border-b border-transparent text-sm text-white/45 transition-colors hover:border-white/45 hover:text-white"
          >
            Forgot password?
          </a>

          <p
            v-if="error"
            class="mb-3 rounded-lg px-3 py-2 text-xs text-[--color-error] bg-[oklch(0.65_0.21_15/0.1)]"
          >
            {{ error }}
          </p>
          <BaseButton
            type="submit"
            variant="emphasis"
            class="mt-2 min-w-36 justify-center font-semibold uppercase tracking-wider"
            :disabled="loading"
          >
            <Icon
              :icon="loading ? 'ic:round-hourglass-empty' : 'ic:round-login'"
              class="inline-block text-base align-[-0.125em]"
            />
            {{ loading ? "Please wait..." : "Log in" }}
          </BaseButton>
        </form>
      </div>

      <div
        class="absolute left-0 top-0 hidden h-full w-1/2 transition-all duration-700 ease-in-out md:block"
        :class="
          mode === 'Register'
            ? 'translate-x-full opacity-100 z-30'
            : 'opacity-0 z-10'
        "
      >
        <form
          class="flex h-full flex-col items-center justify-center bg-[oklch(0.18_0.018_285)] px-12 text-center text-white"
          @submit.prevent="submitAs('Register')"
        >
          <div
            class="mb-5 inline-flex h-14 w-14 items-center justify-center rounded-2xl bg-linear-to-br from-reisa-pink-500 to-reisa-lilac-500 shadow-lg shadow-reisa-pink-500/20"
          >
            <span class="text-2xl font-black text-white">A</span>
          </div>
          <h1 class="mb-2 text-3xl font-black text-white">Create account</h1>
          <p class="mb-6 text-sm text-white/55">
            Start with email and password
          </p>

          <BaseInput
            v-model="email"
            id="register-email"
            label="Email"
            type="email"
            autocomplete="email"
            required
            class="w-full border-white/15 bg-white/5 text-white"
            :error="emailError"
            @blur="emit('validate', 'email')"
          />
          <BaseInput
            v-model="password"
            id="register-password"
            label="Password"
            type="password"
            autocomplete="new-password"
            required
            class="w-full border-white/15 bg-white/5 text-white"
            :error="passwordError"
            @blur="emit('validate', 'password')"
          />

          <p
            v-if="error"
            class="mb-3 rounded-lg px-3 py-2 text-xs text-[--color-error] bg-[oklch(0.65_0.21_15/0.1)]"
          >
            {{ error }}
          </p>
          <BaseButton
            type="submit"
            variant="emphasis"
            class="mt-2 min-w-36 justify-center font-semibold uppercase tracking-wider"
            :disabled="loading"
          >
            <Icon
              :icon="loading ? 'ic:round-hourglass-empty' : 'ic:round-add'"
              class="inline-block text-base align-[-0.125em]"
            />
            {{ loading ? "Please wait..." : "Sign up" }}
          </BaseButton>
        </form>
      </div>

      <div
        class="absolute left-1/2 top-0 hidden h-full w-1/2 overflow-hidden transition-transform duration-700 ease-in-out md:block z-40"
        :class="{ '-translate-x-full': mode === 'Register' }"
      >
        <div
          class="relative -left-full flex h-full w-[200%] flex-row bg-[radial-gradient(circle_at_30%_20%,oklch(0.58_0.18_350),transparent_30%),linear-gradient(140deg,oklch(0.2_0.07_285),oklch(0.27_0.12_260),oklch(0.18_0.06_225))] text-white transition-transform duration-700 ease-in-out"
          :class="{ 'translate-x-1/2': mode === 'Register' }"
        >
          <div
            class="flex h-full w-1/2 flex-col items-center justify-center px-10 text-center"
          >
            <h2 class="mb-4 text-4xl font-black">Already have an account?</h2>
            <p class="mb-8 max-w-xs text-sm text-white/85">
              Return to your trading terminal and live strategy workspace.
            </p>
            <BaseButton
              variant="outline"
              class="border-white/70 text-white hover:bg-white hover:text-[oklch(0.18_0.018_285)] font-semibold uppercase tracking-wider"
              :disabled="loading"
              @click="toggleMode"
            >
              Go to log in
            </BaseButton>
          </div>

          <div
            class="flex h-full w-1/2 flex-col items-center justify-center px-10 text-center"
          >
            <h2 class="mb-4 text-4xl font-black">New to QuantAura?</h2>
            <p class="mb-8 max-w-xs text-sm text-white/85">
              Create an account to configure models, exchanges, and strategies.
            </p>
            <BaseButton
              variant="outline"
              class="border-white/70 text-white hover:bg-white hover:text-[oklch(0.18_0.018_285)] font-semibold uppercase tracking-wider"
              :disabled="loading"
              @click="toggleMode"
            >
              Go to sign up
            </BaseButton>
          </div>
        </div>
      </div>

      <div
        class="flex min-h-155 flex-col justify-center bg-surface-overlay px-6 py-10 text-white md:hidden"
      >
        <div class="mb-8 text-center">
          <div
            class="mb-4 inline-flex h-14 w-14 items-center justify-center rounded-2xl bg-linear-to-br from-reisa-pink-500 to-reisa-lilac-500 shadow-lg shadow-reisa-pink-500/20"
          >
            <span class="text-2xl font-black text-white">A</span>
          </div>
          <h1 class="text-3xl font-black text-white">
            {{ mode === "Register" ? "Create account" : "Welcome back" }}
          </h1>
          <p class="mt-2 text-sm text-white/55">
            {{
              mode === "Register"
                ? "Start with email and password"
                : "Sign in to QuantAura"
            }}
          </p>
        </div>

        <form
          class="flex flex-col"
          @submit.prevent="submitAs(mode === 'Register' ? 'Register' : 'Login')"
        >
          <BaseInput
            v-model="email"
            id="mobile-email"
            label="Email"
            type="email"
            autocomplete="email"
            required
            class="w-full border-white/15 bg-white/5 text-white"
            :error="emailError"
            @blur="emit('validate', 'email')"
          />
          <BaseInput
            v-model="password"
            id="mobile-password"
            label="Password"
            type="password"
            :autocomplete="
              mode === 'Register' ? 'new-password' : 'current-password'
            "
            required
            class="w-full border-white/15 bg-white/5 text-white"
            :error="passwordError"
            @blur="emit('validate', 'password')"
          />
          <p
            v-if="error"
            class="mb-3 rounded-lg px-3 py-2 text-xs text-[--color-error] bg-[oklch(0.65_0.21_15/0.1)]"
          >
            {{ error }}
          </p>
          <BaseButton
            type="submit"
            variant="emphasis"
            class="mt-2 w-full justify-center font-semibold uppercase tracking-wider"
            :disabled="loading"
          >
            <Icon
              :icon="
                loading
                  ? 'ic:round-hourglass-empty'
                  : mode === 'Register'
                    ? 'ic:round-add'
                    : 'ic:round-login'
              "
              class="inline-block text-base align-[-0.125em]"
            />
            {{
              loading
                ? "Please wait..."
                : mode === "Register"
                  ? "Create account"
                  : "Log in"
            }}
          </BaseButton>
        </form>

        <BaseButton
          variant="outline"
          class="mt-4 w-full justify-center"
          :disabled="loading"
          @click="toggleMode"
        >
          {{ mode === "Register" ? "Go to log in" : "Go to sign up" }}
        </BaseButton>
      </div>
    </div>
  </div>
</template>
