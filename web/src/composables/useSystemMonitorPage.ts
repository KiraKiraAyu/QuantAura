import { onMounted, ref } from "vue"
import {
  getRuntimeAlertHistoryApi,
  getRuntimeEventsApi,
  getRuntimeMetricsApi,
  getTraderListApi,
} from "@/api/trading"
import type {
  RuntimeAlertItemPayload,
  RuntimeAlertHistoryItemPayload,
  RuntimeEventPayload,
  RuntimeMetricsPayload,
  TraderPayload,
} from "@/types/trading"

export function useSystemMonitorPage() {
  const loading = ref(true)
  const traders = ref<TraderPayload[]>([])
  const activeTrader = ref("")
  const metrics = ref<RuntimeMetricsPayload | null>(null)
  const alerts = ref<RuntimeAlertHistoryItemPayload[]>([])
  const events = ref<RuntimeEventPayload[]>([])

  function fmt(value: number) {
    return (value || 0).toFixed(2)
  }

  function formatMetadata(metadata: unknown) {
    if (!metadata) return ""
    try {
      const parsed =
        typeof metadata === "string" ? JSON.parse(metadata) : metadata
      return JSON.stringify(parsed, null, 2)
    } catch {
      return String(metadata)
    }
  }

  function parseAlerts(alerts: RuntimeAlertItemPayload[]) {
    const breached = alerts
      .filter((alert) => alert.breached)
      .map((alert) => alert.key)
    return breached.length ? breached.join(", ") : "None"
  }

  async function loadTraders() {
    try {
      const data = await getTraderListApi()
      traders.value = data.traders ?? []
    } catch {
      traders.value = []
    }
  }

  async function loadAll() {
    loading.value = true
    try {
      const params = activeTrader.value ? { trader_id: activeTrader.value } : {}
      const metricsRequest = getRuntimeMetricsApi(params).catch(() => null)
      const alertsRequest = getRuntimeAlertHistoryApi({
        ...params,
        limit: 50,
      }).catch(() => null)
      const eventsRequest = getRuntimeEventsApi({
        ...params,
        limit: 100,
      }).catch(() => null)

      const [metricsResponse, alertsResponse, eventsResponse] =
        await Promise.all([metricsRequest, alertsRequest, eventsRequest])
      metrics.value = metricsResponse
      alerts.value = alertsResponse?.items ?? []
      events.value = eventsResponse?.items ?? []
    } finally {
      loading.value = false
    }
  }

  onMounted(async () => {
    await loadTraders()
    await loadAll()
  })

  return {
    activeTrader,
    alerts,
    events,
    fmt,
    formatMetadata,
    loadAll,
    loading,
    metrics,
    parseAlerts,
    traders,
  }
}
