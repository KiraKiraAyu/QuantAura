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

export interface BacktestRun {
  run_id: string
  state: string
  summary: Record<string, number>
  created_at: string
}

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
