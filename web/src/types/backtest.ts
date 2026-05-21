import type { JsonValue } from "@/types/json"

export type { KlinePayload } from "@/types/public"

export interface BacktestStartRequest {
  run_id?: string | null
  symbols?: string[] | null
  start_ts?: number | null
  end_ts?: number | null
  initial_balance?: number | null
  fee_bps?: number | null
  slippage_bps?: number | null
  ai_model_id?: string | null
  prompt_variant?: string | null
  btc_eth_leverage?: number | null
  altcoin_leverage?: number | null
  interval?: string | null
  decision_every?: number | null
}

export interface BacktestRunIdRequest {
  run_id: string
}

export interface BacktestLabelRequest {
  run_id: string
  label: string
}

export interface BacktestQueryParams {
  run_id?: string | null
  limit?: number | null
}

export interface KlinesQuery {
  symbol: string
  interval?: string | null
  limit?: number | null
}

export interface BacktestRunActionPayload {
  run_id: string
  message: string
}

export interface BacktestMessagePayload {
  message: string
}

export interface BacktestStatusPayload {
  status: JsonValue
}

export interface BacktestRunsPayload {
  runs: JsonValue[]
  count: number
}

export interface BacktestEquityPayload {
  points: JsonValue[]
  count: number
}

export interface BacktestTradesPayload {
  trades: JsonValue[]
  count: number
}

export interface BacktestTracePayload {
  trace: JsonValue[]
}

export interface BacktestDecisionsPayload {
  decisions: JsonValue[]
  count: number
}

export interface BacktestMetricsPayload {
  metrics: JsonValue
}

export interface BacktestExportPayload {
  run_id: string
  trades: JsonValue[]
  equity: JsonValue[]
  exported_at: number
}
