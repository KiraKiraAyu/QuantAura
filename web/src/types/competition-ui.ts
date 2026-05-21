export interface CompetitionTrader {
  trader_id: string
  name: string
  ai_model: string
  exchange: string
  equity: number
  initial_balance: number
  total_trades?: number
  win_rate?: number
  is_running: boolean
}

export interface EquityPoint {
  time: number
  value: number
}
