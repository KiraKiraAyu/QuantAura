import type { PublicCompetitionTraderPayload } from "@/types/public"

export type CompetitionTrader = PublicCompetitionTraderPayload

export interface EquityPoint {
  time: number
  value: number
}
