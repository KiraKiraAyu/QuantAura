import type {
  CompetitionListPayload,
  EquityHistoryBatchPayload,
  EquityHistoryBatchRequest,
  EquityHistoryPointPayload,
  EquityHistoryQuery,
  PublicCompetitionTraderPayload,
  PublicTraderConfigPayload,
} from "@/types/public"
import request from "@/utils/request"

const Api = {
  Competition: "/api/competition",
  TopTraders: "/api/competition/top-traders",
  EquityHistory: "/api/competition/equity-history",
  EquityHistoryBatch: "/api/competition/equity-history-batch",
  PublicTraderConfig: "/api/competition/traders/{id}/public-config",
} as const

export function getCompetitionApi() {
  return request.get<CompetitionListPayload>(Api.Competition)
}

export function getTopTradersApi() {
  return request.get<PublicCompetitionTraderPayload[]>(Api.TopTraders)
}

export function getEquityHistoryApi(params?: EquityHistoryQuery) {
  return request.get<EquityHistoryPointPayload[]>(Api.EquityHistory, { params })
}

export function getEquityHistoryBatchApi(data: EquityHistoryBatchRequest) {
  return request.post<EquityHistoryBatchPayload>(Api.EquityHistoryBatch, data)
}

export function getPublicTraderConfigApi(id: string) {
  return request.get<PublicTraderConfigPayload>(
    Api.PublicTraderConfig.replace("{id}", id),
  )
}
