import type {
  DebateMessageItemPayload,
  DebateSummaryPayload,
} from "@/types/debates"

export type DebateSession = DebateSummaryPayload & {
  final_reasoning?: string
}

export type DebateMessage = DebateMessageItemPayload

export interface DebateDraft {
  name: string
  symbol: string
  max_rounds: number
  participants: string[]
}
