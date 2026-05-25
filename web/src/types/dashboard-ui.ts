import type { PositionPayload, TraderPayload } from "@/types/trading"

export type DashboardTrader = TraderPayload
export type DashboardPosition = PositionPayload

export interface DashboardEquitySnapshot {
  equity: number
  available_cash: number
  unrealized_pnl: number
  loaded: boolean
}

export interface DashboardLiveEvent {
  type: string
  summary: string
  time: string
}

export interface EquityChartPoint {
  time: number
  value: number
}
