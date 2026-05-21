export interface DashboardTrader {
  id: string
  name?: string
  is_running?: boolean
  ai_model_id?: string
  ai_model?: string
  exchange_id?: string
  [key: string]: unknown
}

export interface DashboardPosition {
  symbol: string
  side: string
  qty?: number
  quantity?: number
  entry_price: number
  unrealized_pnl: number
  trader_id?: string
}

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
