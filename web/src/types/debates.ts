import type { JsonValue } from "@/types/json"

export interface CreateDebateRequest {
  name?: string | null
  symbol?: string | null
  max_rounds?: number | null
  prompt_variant?: string | null
  participants?: string[] | null
}

export interface StartDebateRequest {
  model_ids?: string[] | null
}

export interface DebateListPayload {
  debates: JsonValue[]
  count: number
}

export interface DebateDetailPayload {
  debate: JsonValue
}

export interface DebateActionPayload {
  id: string
  message: string
}

export interface DebateMessagePayload {
  message: string
}

export interface DebatePersonalityPayload {
  id: string
  name: string
  description: string
}

export interface DebatePersonalitiesPayload {
  personalities: DebatePersonalityPayload[]
}

export interface DebateExecutionPayload {
  id: string
  final_decision: string
  final_reasoning: string
  message: string
}

export interface DebateMessagesPayload {
  messages: JsonValue[]
  count: number
}

export interface DebateVotesPayload {
  votes: JsonValue
}
