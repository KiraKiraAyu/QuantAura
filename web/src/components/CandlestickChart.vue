<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue"
import {
  CandlestickSeries,
  createChart,
  ColorType,
  type UTCTimestamp,
  type CandlestickData,
} from "lightweight-charts"

const props = withDefaults(
  defineProps<{
    data: CandlestickData[]
    height?: number
  }>(),
  { height: 360 },
)

const chartEl = ref<HTMLElement | null>(null)
// eslint-disable-next-line @typescript-eslint/no-explicit-any
let chart: any = null
// eslint-disable-next-line @typescript-eslint/no-explicit-any
let candles: any = null

onMounted(() => {
  if (!chartEl.value) return
  chart = createChart(chartEl.value, {
    layout: {
      background: { type: ColorType.Solid, color: "transparent" },
      textColor: "#7a7091",
      fontFamily: "Outfit, sans-serif",
      fontSize: 11,
    },
    grid: {
      vertLines: { color: "#ffffff08" },
      horzLines: { color: "#ffffff08" },
    },
    rightPriceScale: { borderColor: "#ffffff10" },
    timeScale: { borderColor: "#ffffff10", timeVisible: true },
  })

  candles = chart.addSeries(CandlestickSeries, {
    upColor: "#2ecc71",
    downColor: "#e74c3c",
    borderUpColor: "#2ecc71",
    borderDownColor: "#e74c3c",
    wickUpColor: "#2ecc71",
    wickDownColor: "#e74c3c",
  })

  if (props.data.length) feed(props.data)
  chart.timeScale().fitContent()
  window.addEventListener("resize", onResize)
})

onUnmounted(() => {
  window.removeEventListener("resize", onResize)
  chart?.remove()
})

function feed(arr: CandlestickData[]) {
  if (!candles) return
  const sorted = [...arr].sort(
    (a, b) => (a.time as number) - (b.time as number),
  )
  candles.setData(sorted.map((d) => ({ ...d, time: d.time as UTCTimestamp })))
  chart?.timeScale().fitContent()
}

function onResize() {
  if (chart && chartEl.value)
    chart.applyOptions({ width: chartEl.value.clientWidth })
}

watch(() => props.data, feed, { deep: true })
</script>

<template>
  <div ref="chartEl" class="w-full" :style="{ height: height + 'px' }"></div>
</template>
