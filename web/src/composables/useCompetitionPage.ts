import { computed, onMounted, ref } from "vue"
import { getCompetitionApi, getEquityHistoryApi } from "@/api/competition"
import type { CompetitionTrader, EquityPoint } from "@/types/competition-ui"
import type { EquityHistoryPointPayload } from "@/types/public"

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
        (trader.trader_name || trader.trader_id).toLowerCase().includes(query) ||
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
    return trader.total_pnl_pct
  }

  function avatarStyle(id: string) {
    const color = avatarColors[id.charCodeAt(0) % avatarColors.length]
    return `background-color:${color}22;color:${color};`
  }

  async function load() {
    loading.value = true
    try {
      const data = await getCompetitionApi()
      traders.value = [...data.traders].sort(
        (left, right) => returnPct(right) - returnPct(left),
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
      selectedEquity.value = points.map(equityPoint)
    } catch {
      /* keep empty detail chart */
    }
  }

  onMounted(load)

  function equityPoint(point: EquityHistoryPointPayload): EquityPoint {
    return {
      time: Math.floor(new Date(point.timestamp).getTime() / 1000),
      value: point.total_equity,
    }
  }

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
