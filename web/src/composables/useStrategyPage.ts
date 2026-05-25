import { onMounted, ref } from "vue"
import {
  activateStrategyApi,
  createStrategyApi,
  deleteStrategyApi,
  duplicateStrategyApi,
  getStrategiesApi,
  previewStrategyPromptApi,
  strategyTestRunApi,
  updateStrategyApi,
} from "@/api/strategies"
import type {
  EditableStrategy,
  StrategyPromptPreviewModel,
  StrategyTestResult,
} from "@/types/strategy-ui"

export function useStrategyPage() {
  const strategies = ref<EditableStrategy[]>([])
  const selected = ref<EditableStrategy | null>(null)
  const loading = ref(true)
  const saving = ref(false)
  const testRunLoading = ref(false)
  const testResult = ref<StrategyTestResult | null>(null)
  const previewPromptText = ref<StrategyPromptPreviewModel | null>(null)
  const previewLoading = ref(false)
  const duplicating = ref(false)

  async function load() {
    loading.value = true
    try {
      const data = await getStrategiesApi()
      strategies.value = data.strategies
    } finally {
      loading.value = false
    }
  }

  function createNew() {
    selected.value = {
      id: "",
      name: "New Strategy",
      description: "",
      author_email: "",
      is_active: false,
      is_default: false,
      created_at: "",
      updated_at: "",
      config: {
        trading_symbols: "BTCUSDT,ETHUSDT",
        max_positions: 5,
        btc_eth_leverage: 5,
        altcoin_leverage: 3,
        prompt_variant: "balanced",
      },
    }
  }

  async function saveStrategy() {
    if (!selected.value) return
    saving.value = true
    try {
      if (selected.value.id) {
        await updateStrategyApi(selected.value.id, selected.value)
      } else {
        const data = await createStrategyApi(selected.value)
        selected.value.id = data.id
      }
      await load()
    } finally {
      saving.value = false
    }
  }

  async function activateStrategy() {
    if (!selected.value?.id) return
    await activateStrategyApi(selected.value.id)
    await load()
    selected.value =
      strategies.value.find((strategy) => strategy.id === selected.value?.id) ??
      selected.value
  }

  async function deleteStrategy() {
    if (!selected.value?.id) return
    if (!confirm("Delete this strategy?")) return
    await deleteStrategyApi(selected.value.id)
    selected.value = null
    await load()
  }

  async function runTest() {
    if (!selected.value) return
    testRunLoading.value = true
    testResult.value = null
    try {
      const data = await strategyTestRunApi({
        config: selected.value.config,
        prompt_variant: selected.value.config.prompt_variant ?? "balanced",
        run_real_ai: true,
      })
      testResult.value = data
    } catch (error: unknown) {
      testResult.value = {
        system_prompt: "",
        user_prompt: "",
        prompt_variant: selected.value.config.prompt_variant ?? "balanced",
        ai_model_id: "",
        ai_response:
          error instanceof Error ? error.message : "Strategy test failed",
        decisions: [],
        reasoning: "",
        duration_ms: 0,
        used_real_ai: false,
      }
    } finally {
      testRunLoading.value = false
    }
  }

  async function duplicateStrategy() {
    if (!selected.value?.id) return
    duplicating.value = true
    try {
      const data = await duplicateStrategyApi(selected.value.id, {
        name: `${selected.value.name} Copy`,
      })
      await load()
      selected.value =
        strategies.value.find((strategy) => strategy.id === data.id) ?? null
    } finally {
      duplicating.value = false
    }
  }

  async function previewPrompt() {
    if (!selected.value) return
    previewLoading.value = true
    try {
      const data = await previewStrategyPromptApi({
        config: selected.value.config,
        prompt_variant: selected.value.config.prompt_variant ?? "balanced",
      })
      previewPromptText.value = {
        system: data.system_prompt,
        variant: data.prompt_variant,
        summary: JSON.stringify(data.config_summary, null, 2),
      }
    } finally {
      previewLoading.value = false
    }
  }

  onMounted(load)

  return {
    activateStrategy,
    createNew,
    deleteStrategy,
    duplicateStrategy,
    duplicating,
    loading,
    previewLoading,
    previewPrompt,
    previewPromptText,
    runTest,
    saveStrategy,
    saving,
    selected,
    strategies,
    testResult,
    testRunLoading,
  }
}
