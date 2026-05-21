import { computed, onMounted, ref } from "vue"
import { getCompetitionApi, getEquityHistoryApi } from "@/api/competition"
import type { CompetitionTrader, EquityPoint } from "@/types/competition-ui"

export function useCompetitionPage() {
  const traders = ref<CompetitionTrader[]>([])
  const loading = ref(true)
  const search = ref("")
  const selectedTrader = ref<CompetitionTrader | null>(null)
  const selectedEquity = ref<EquityPoint[]>([])
  const lastUpdated = ref("-")
  const avatarColors = ["#e2528a", "#9b6dae", "#5b8dee", "#2ecc71", "#e67e22"]
  const topThree = computed(() => traders.value.slice(0, 3))
  const filtered = computed(() => {
    if (!search.value) return traders.value
    const query = search.value.toLowerCase()
    return traders.value.filter(
      (trader) =>
        (trader.name || trader.trader_id).toLowerCase().includes(query) ||
        trader.ai_model.toLowerCase().includes(query),
    )
  })

  function fmt(value: number) {
    return (value ?? 0).toLocaleString("en-US", {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    })
  }

  function returnPct(trader: CompetitionTrader) {
    const initial = trader.initial_balance || 1000
    return ((trader.equity - initial) / initial) * 100
  }

  function avatarStyle(id: string) {
    const color = avatarColors[id.charCodeAt(0) % avatarColors.length]
    return `background-color:${color}22;color:${color};`
  }

  async function load() {
    loading.value = true
    try {
      const data = await getCompetitionApi()
      const list = (data.traders ?? []).map((item: any) => ({
        ...item,
        name: item.trader_name ?? item.name,
        equity: item.total_equity ?? item.equity ?? 0,
        initial_balance:
          item.initial_balance ??
          ((item.total_equity ?? 0) - (item.total_pnl ?? 0) || 1000),
      }))
      traders.value = [...list].sort(
        (left: CompetitionTrader, right: CompetitionTrader) =>
          returnPct(right) - returnPct(left),
      )
      lastUpdated.value = new Date().toLocaleTimeString()
    } catch {
      /* keep previous list */
    } finally {
      loading.value = false
    }
  }

  async function showDetail(trader: CompetitionTrader) {
    selectedTrader.value = trader
    selectedEquity.value = []
    try {
      const points = await getEquityHistoryApi({ trader_id: trader.trader_id })
      selectedEquity.value = points.map((point: any) => ({
        time:
          point.ts ??
          (point.timestamp
            ? Math.floor(new Date(point.timestamp).getTime() / 1000)
            : 0),
        value: point.total_equity ?? point.equity ?? point.value ?? 0,
      }))
    } catch {
      /* keep empty detail chart */
    }
  }

  onMounted(load)

  return {
    avatarStyle,
    filtered,
    fmt,
    lastUpdated,
    load,
    loading,
    returnPct,
    search,
    selectedEquity,
    selectedTrader,
    showDetail,
    topThree,
  }
}
