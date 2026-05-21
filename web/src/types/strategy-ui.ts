export interface StrategyConfig extends Record<string, unknown> {
  trading_symbols?: string
  max_positions?: number
  btc_eth_leverage?: number
  altcoin_leverage?: number
  prompt_variant?: string
}

export interface EditableStrategy {
  id: string
  name: string
  description: string
  is_active: boolean
  updated_at: string
  config: StrategyConfig
}

export interface StrategyDecision {
  action: string
  symbol: string
  confidence: number
  reasoning: string
}

export interface StrategyTestResult {
  decisions: StrategyDecision[]
  raw_ai_response?: string
  duration_ms: number
}

export interface StrategyPromptPreviewModel {
  system: string
  variant: string
  summary: string
}
