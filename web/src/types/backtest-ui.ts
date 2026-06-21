import type { BacktestRunPayload } from "@/types/backtest"

export interface BacktestConfig {
  symbols: string
  interval: string
  startDate: string
  endDate: string
  initial_balance: number
  fee_bps: number
  slippage_bps: number
  ai_model_id: string
}

export type BacktestRun = BacktestRunPayload

export interface BacktestLiveProgress {
  run_id: string
  state: string
  bar_index: number
  total_bars: number
  equity: number
}

export interface BacktestModelOption {
  id: string
  label: string
}
