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
  StrategyConfig,
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
      strategies.value = (data.strategies ?? []).map((strategy) => ({
        ...(strategy as unknown as EditableStrategy),
        config:
          strategy.config &&
          typeof strategy.config === "object" &&
          !Array.isArray(strategy.config)
            ? (strategy.config as StrategyConfig)
            : {},
      }))
    } finally {
      loading.value = false
    }
  }

  function createNew() {
    selected.value = {
      id: "",
      name: "New Strategy",
      description: "",
      is_active: false,
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
        await updateStrategyApi(selected.value.id, selected.value as any)
      } else {
        const data = await createStrategyApi(selected.value as any)
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
        prompt_variant:
          (selected.value.config?.prompt_variant as string) ?? "balanced",
        run_real_ai: true,
      } as any)
      testResult.value = data as unknown as StrategyTestResult
    } catch (error: unknown) {
      testResult.value = {
        decisions: [],
        raw_ai_response:
          error instanceof Error ? error.message : "Strategy test failed",
        duration_ms: 0,
      } as StrategyTestResult
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
        prompt_variant:
          (selected.value.config?.prompt_variant as string) ?? "balanced",
      } as any)
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
