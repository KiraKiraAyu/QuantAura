import { computed, onMounted, ref, watch } from "vue"
import { getEquityHistoryApi } from "@/api/competition"
import {
  closeTraderPositionApi,
  getPositionsApi,
  getTraderListApi,
  startTraderApi,
  stopTraderApi,
  syncTraderBalanceApi,
} from "@/api/trading"
import { useRealtimeStore } from "@/stores/realtime"
import type {
  DashboardEquitySnapshot,
  DashboardLiveEvent,
  DashboardPosition,
  DashboardTrader,
  EquityChartPoint,
} from "@/types/dashboard-ui"
import type { EquityHistoryPointPayload } from "@/types/public"

export function useDashboardPage() {
  const realtime = useRealtimeStore()
  const loading = ref(true)
  const initialLoadDone = ref(false)
  const loadError = ref("")
  const showCreateTrader = ref(false)

  const traders = ref<DashboardTrader[]>([])
  const positions = ref<DashboardPosition[]>([])
  const equity = ref<DashboardEquitySnapshot>({
    equity: 0,
    available_cash: 0,
    unrealized_pnl: 0,
    loaded: false,
  })
  const events = ref<DashboardLiveEvent[]>([])
  const equityHistory = ref<EquityChartPoint[]>([])
  const activeChart = ref("")
  const connected = computed(() => realtime.isConnected)
  const traderIdOptions = computed(() =>
    traders.value.map((trader) => trader.id).slice(0, 5),
  )

  function traderName(id: string) {
    return (
      (traders.value.find((trader) => trader.id === id)?.name as string) ||
      `${id.slice(0, 8)}...`
    )
  }

  async function loadEquityHistory(traderId: string) {
    try {
      const points = await getEquityHistoryApi({ trader_id: traderId })
      equityHistory.value = points.map(equityPoint)
    } catch {
      equityHistory.value = []
    }
  }

  async function loadAll() {
    loading.value = true
    loadError.value = ""
    try {
      const data = await getTraderListApi()
      traders.value = data.traders

      if (traders.value.length === 0) {
        equity.value = {
          equity: 0,
          available_cash: 0,
          unrealized_pnl: 0,
          loaded: true,
        }
        positions.value = []
        realtime.clearPositions()
        return
      }

      await loadOpenPositions(traders.value.map((trader) => trader.id))

      if (traders.value.length > 0 && !activeChart.value) {
        activeChart.value = traders.value[0]!.id
        await loadEquityHistory(traders.value[0]!.id)
      }
    } catch (error: unknown) {
      traders.value = []
      positions.value = []
      const err = error as {
        response?: { data?: { error?: string }; status?: number }
        message?: string
      }
      const statusMsg = err?.response?.status
        ? `Request failed (${err.response.status})`
        : ""
      loadError.value =
        err?.response?.data?.error ||
        statusMsg ||
        err?.message ||
        "Failed to load dashboard data"
      equity.value = {
        equity: 0,
        available_cash: 0,
        unrealized_pnl: 0,
        loaded: true,
      }
    } finally {
      loading.value = false
      initialLoadDone.value = true
    }
  }

  async function startTrader(id: string) {
    await startTraderApi(id).catch(() => {})
    await loadAll()
  }

  async function stopTrader(id: string) {
    await stopTraderApi(id).catch(() => {})
    await loadAll()
  }

  async function syncBalance(id: string) {
    await syncTraderBalanceApi(id).catch(() => {})
    await loadAll()
  }

  async function closePosition(traderId: string, symbol: string, side: string) {
    if (!confirm(`Close ${symbol} position?`)) return
    await closeTraderPositionApi(traderId, { symbol, side }).catch(() => {})
    realtime.removePosition(traderId, symbol, side)
    positions.value = positions.value.filter(
      (position) =>
        !(
          position.trader_id === traderId &&
          position.symbol === symbol &&
          position.side === side
        ),
    )
  }

  async function selectEquityTrader(traderId: string) {
    activeChart.value = traderId
    await loadEquityHistory(traderId)
  }

  function handleTraderCreated() {
    showCreateTrader.value = false
    void loadAll()
  }

  watch(
    () => realtime.positions,
    (value) => {
      positions.value = value
    },
    { deep: true, immediate: true },
  )
  watch(
    () => realtime.equitySnapshot,
    (value) => {
      if (!value) return
      equity.value = {
        equity: (value.equity as number) ?? 0,
        available_cash: (value.available_cash as number) ?? 0,
        unrealized_pnl: (value.unrealized_pnl as number) ?? 0,
        loaded: true,
      }
    },
  )
  watch(
    () => realtime.lastEvent,
    (event) => {
      if (!event || event.type === "ping") return
      events.value.unshift({
        type: event.type,
        summary: event.trader_id ? `trader:${event.trader_id as string}` : "",
        time: new Date().toLocaleTimeString(),
      })
      if (events.value.length > 50) events.value.pop()
    },
  )

  onMounted(() => {
    void loadAll()
    window.setTimeout(() => {
      if (!initialLoadDone.value) {
        loading.value = false
        initialLoadDone.value = true
        equity.value.loaded = true
        if (!loadError.value) {
          loadError.value = "Dashboard init timed out. Please click Refresh."
        }
      }
    }, 5000)
  })

  function equityPoint(point: EquityHistoryPointPayload): EquityChartPoint {
    return {
      time: Math.floor(new Date(point.timestamp).getTime() / 1000),
      value: point.total_equity,
    }
  }

  async function loadOpenPositions(traderIds: string[]) {
    const entries = await Promise.all(
      traderIds.map(async (traderId) => {
        try {
          const payload = await getPositionsApi({
            trader_id: traderId,
            status: "open",
          })
          return [payload.trader_id, payload.items] as const
        } catch {
          return [traderId, []] as const
        }
      }),
    )

    realtime.replacePositionsByTrader(
      Object.fromEntries(entries) as Record<string, DashboardPosition[]>,
    )
  }

  return {
    activeChart,
    closePosition,
    connected,
    equity,
    equityHistory,
    events,
    handleTraderCreated,
    initialLoadDone,
    loadAll,
    loadError,
    loading,
    positions,
    selectEquityTrader,
    showCreateTrader,
    startTrader,
    stopTrader,
    syncBalance,
    traderIdOptions,
    traderName,
    traders,
  }
}
