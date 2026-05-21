import type { JsonValue } from "@/types/json"

export interface CreateStrategyRequest {
  name: string
  description?: string
  config: JsonValue
}

export interface UpdateStrategyRequest {
  name?: string
  description?: string
  config: JsonValue
}

export interface DuplicateStrategyRequest {
  name?: string
}

export interface DefaultStrategyConfigQuery {
  lang?: string | null
}

export interface PreviewPromptRequest {
  config: JsonValue
  account_equity?: number | null
  prompt_variant?: string | null
}

export interface StrategyTestRunRequest {
  config: JsonValue
  prompt_variant?: string | null
  ai_model_id?: string | null
  run_real_ai?: boolean | null
}

export interface StrategyPayload {
  id: string
  name: string
  description: string
  author_email: string
  is_active: boolean
  is_default: boolean
  config: JsonValue
  created_at: string
  updated_at: string
}

export interface StrategyListPayload {
  strategies: StrategyPayload[]
  count: number
}

export interface StrategyCreatedPayload {
  id: string
  message: string
}

export interface StrategyMessagePayload {
  message: string
}

export interface StrategyDefaultConfigPayload {
  language: string
  config: JsonValue
}

export interface StrategyConfigSummaryPayload {
  coin_source: string
  primary_tf: string
  btc_eth_leverage: number
  altcoin_leverage: number
  max_positions: number
}

export interface PreviewPromptPayload {
  system_prompt: string
  prompt_variant: string
  config_summary: StrategyConfigSummaryPayload
}

export interface StrategyTestRunPayload {
  system_prompt: string
  user_prompt: string
  prompt_variant: string
  ai_model_id: string
  ai_response: string
  decisions: JsonValue
  reasoning: string
  duration_ms: number
  used_real_ai: boolean
}
