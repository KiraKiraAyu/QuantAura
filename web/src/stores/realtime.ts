import { defineStore } from "pinia"
import { computed, ref } from "vue"
import { useAuthStore } from "./auth"

export interface RealtimeEvent {
  type: string
  user_id?: string
  trader_id?: string
  [key: string]: unknown
}

export const useRealtimeStore = defineStore("realtime", () => {
  const source = ref<EventSource | null>(null)
  const connected = ref(false)
  const lastEvent = ref<RealtimeEvent | null>(null)
  const equitySnapshot = ref<Record<string, unknown> | null>(null)
  const positions = ref<unknown[]>([])
  const engineStatus = ref<Record<string, string>>({})

  const isConnected = computed(() => connected.value)

  function connect() {
    const auth = useAuthStore()
    const state = source.value?.readyState
    if (
      !auth.token ||
      state === EventSource.OPEN ||
      state === EventSource.CONNECTING
    ) {
      return
    }

    const eventSource = new EventSource(buildEventsUrl(auth.token))
    source.value = eventSource

    eventSource.onopen = () => {
      if (source.value !== eventSource) return
      connected.value = true
    }

    eventSource.onerror = () => {
      if (source.value !== eventSource) return
      connected.value = false
      if (eventSource.readyState === EventSource.CLOSED) {
        source.value = null
      }
    }

    eventSource.onmessage = (evt) => {
      if (source.value !== eventSource) return
      try {
        const data: RealtimeEvent = JSON.parse(evt.data)
        lastEvent.value = data
        handleEvent(data)
      } catch {
        /* ignore parse errors */
      }
    }
  }

  function buildEventsUrl(token: string) {
    const fallback = new URL("/api/events", window.location.origin)
    const configured = import.meta.env.VITE_EVENTS_URL
    const url = configured ? new URL(configured, fallback) : fallback

    url.searchParams.set("token", token)
    return url.toString()
  }

  function disconnect() {
    source.value?.close()
    source.value = null
    connected.value = false
  }

  function handleEvent(ev: RealtimeEvent) {
    switch (ev.type) {
      case "equity_snapshot":
        equitySnapshot.value = ev as Record<string, unknown>
        break
      case "position_update":
        if (Array.isArray(ev.positions)) positions.value = ev.positions
        break
      case "engine_status":
        if (ev.trader_id && typeof ev.status === "string") {
          engineStatus.value[ev.trader_id as string] = ev.status as string
        }
        break
    }
  }

  return {
    connected,
    isConnected,
    lastEvent,
    equitySnapshot,
    positions,
    engineStatus,
    connect,
    disconnect,
  }
})
