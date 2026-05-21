export interface DebateSession {
  id: string
  name: string
  symbol: string
  status: string
  max_rounds: number
  current_round: number
  created_at: string
  final_decision?: string
  final_reasoning?: string
}

export interface DebateMessage {
  round: number
  personality: string
  content: string
  vote: string
}

export interface DebateDraft {
  name: string
  symbol: string
  max_rounds: number
  participants: string[]
}
