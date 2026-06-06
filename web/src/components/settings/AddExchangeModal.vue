<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue"
import { Icon } from "@iconify/vue"
import BaseButton from "@/components/universal/BaseButton.vue"
import BaseInput from "@/components/universal/BaseInput.vue"
import { getSupportedExchangesApi } from "@/api/catalog"
import { createExchangeApi } from "@/api/exchanges"
import type { CreateExchangeRequest } from "@/types/exchanges"
import type { SupportedExchangePayload } from "@/types/public"

const emit = defineEmits<{
  close: []
  created: []
}>()

const addingEx = ref(false)
const supportedExchanges = ref<SupportedExchangePayload[]>([])
const newEx = ref<CreateExchangeRequest>({
  exchange_type: "binance",
  account_name: "",
  api_key: "",
  secret_key: "",
  testnet: true,
})
const requiresPassphrase = computed(() =>
  ["okx", "bitget"].includes(newEx.value.exchange_type),
)
const isHyperliquid = computed(
  () => newEx.value.exchange_type === "hyperliquid",
)
const usesApiCredentials = computed(() => !isHyperliquid.value)
const supportsTestnet = computed(() => newEx.value.exchange_type !== "aster")

async function addExchange() {
  addingEx.value = true
  try {
    await createExchangeApi({ ...newEx.value, enabled: true })
    emit("created")
    emit("close")
  } finally {
    addingEx.value = false
  }
}

async function loadSupportedExchanges() {
  try {
    supportedExchanges.value = await getSupportedExchangesApi()
    newEx.value.exchange_type = supportedExchanges.value[0]?.id ?? "binance"
  } catch {
    supportedExchanges.value = []
  }
}

onMounted(loadSupportedExchanges)

watch(
  () => newEx.value.exchange_type,
  (exchangeType) => {
    newEx.value.api_key = ""
    newEx.value.secret_key = ""
    newEx.value.passphrase = ""
    newEx.value.hyperliquid_wallet_addr = ""
    if (exchangeType === "aster") {
      newEx.value.testnet = false
    } else if (newEx.value.testnet == null) {
      newEx.value.testnet = true
    }
  },
)
</script>

<template>
  <div
    class="fixed inset-0 flex items-center justify-center z-50 p-4 bg-black/60"
  >
    <div class="w-full max-w-md">
      <h2 class="font-bold mb-4">Add Exchange</h2>
      <div class="flex flex-col gap-3">
        <div>
          <label>Exchange Type</label>
          <select v-model="newEx.exchange_type">
            <option
              v-for="exchange in supportedExchanges"
              :key="exchange.id"
              :value="exchange.id"
            >
              {{ exchange.name }} ({{ exchange.type }})
            </option>
          </select>
        </div>
        <div>
          <label>Account Name</label>
          <BaseInput v-model="newEx.account_name" placeholder="My Binance" />
        </div>
        <div v-if="usesApiCredentials">
          <label>API Key</label>
          <BaseInput v-model="newEx.api_key" placeholder="api key…" />
        </div>
        <div v-if="usesApiCredentials">
          <label>Secret Key</label>
          <BaseInput
            v-model="newEx.secret_key"
            type="password"
            placeholder="secret…"
          />
        </div>
        <div v-if="requiresPassphrase">
          <label>Passphrase</label>
          <BaseInput
            v-model="newEx.passphrase"
            type="password"
            placeholder="passphrase…"
          />
        </div>
        <div v-if="isHyperliquid">
          <label>Wallet Address</label>
          <BaseInput
            v-model="newEx.hyperliquid_wallet_addr"
            placeholder="0x…"
          />
        </div>
        <div v-if="isHyperliquid">
          <label>Private Key</label>
          <BaseInput
            v-model="newEx.secret_key"
            type="password"
            placeholder="private key…"
          />
        </div>
        <label
          v-if="supportsTestnet"
          class="flex items-center gap-2 text-sm cursor-pointer"
        >
          <BaseInput v-model="newEx.testnet" type="checkbox" />
          <span>Use testnet</span>
        </label>
        <div class="flex gap-3">
          <BaseButton @click="addExchange" class="flex-1" :disabled="addingEx">
            <Icon
              icon="ic:round-add"
              class="inline-block text-base align-[-0.125em]"
            />
            {{ addingEx ? "Adding…" : "Add" }}
          </BaseButton>
          <BaseButton @click="emit('close')">
            <Icon
              icon="ic:round-close"
              class="inline-block text-base align-[-0.125em]"
            />
            Cancel
          </BaseButton>
        </div>
      </div>
    </div>
  </div>
</template>
