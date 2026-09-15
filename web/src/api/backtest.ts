import type {
  BacktestMessagePayload,
  BacktestQueryParams,
  BacktestRunActionPayload,
  BacktestRunsPayload,
  BacktestStartRequest,
} from "@/types/backtest"
import request, { LONG_RUNNING_REQUEST_TIMEOUT_MS } from "@/utils/request"

const Api = {
  Runs: "/api/backtest/runs",
} as const

export function createBacktestRunApi(data: BacktestStartRequest) {
  return request.post<BacktestRunActionPayload>(Api.Runs, data, {
    timeout: LONG_RUNNING_REQUEST_TIMEOUT_MS,
  })
}

export function stopBacktestRunApi(id: string) {
  return request.post<BacktestRunActionPayload>(
    `${Api.Runs}/${encodeURIComponent(id)}/stop`,
  )
}

export function deleteBacktestRunApi(id: string) {
  return request.delete<BacktestMessagePayload>(
    `${Api.Runs}/${encodeURIComponent(id)}`,
  )
}

export function getBacktestRunsApi(params?: BacktestQueryParams) {
  return request.get<BacktestRunsPayload>(Api.Runs, { params })
}
