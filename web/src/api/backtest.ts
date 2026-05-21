import type {
  BacktestDecisionsPayload,
  BacktestEquityPayload,
  BacktestExportPayload,
  BacktestLabelRequest,
  BacktestMessagePayload,
  BacktestMetricsPayload,
  BacktestQueryParams,
  BacktestRunActionPayload,
  BacktestRunIdRequest,
  BacktestRunsPayload,
  BacktestStartRequest,
  BacktestStatusPayload,
  BacktestTracePayload,
  BacktestTradesPayload,
  KlinePayload,
  KlinesQuery,
} from "@/types/backtest"
import request from "@/utils/request"

const Api = {
  Start: "/api/backtest/start",
  Pause: "/api/backtest/pause",
  Resume: "/api/backtest/resume",
  Stop: "/api/backtest/stop",
  Label: "/api/backtest/label",
  Delete: "/api/backtest/delete",
  Status: "/api/backtest/status",
  Runs: "/api/backtest/runs",
  Equity: "/api/backtest/equity",
  Trades: "/api/backtest/trades",
  Metrics: "/api/backtest/metrics",
  Trace: "/api/backtest/trace",
  Decisions: "/api/backtest/decisions",
  Export: "/api/backtest/export",
  Klines: "/api/backtest/klines",
} as const

export function startBacktestApi(data: BacktestStartRequest) {
  return request.post<BacktestRunActionPayload>(Api.Start, data)
}

export function pauseBacktestApi(data: BacktestRunIdRequest) {
  return request.post<BacktestRunActionPayload>(Api.Pause, data)
}

export function resumeBacktestApi(data: BacktestRunIdRequest) {
  return request.post<BacktestRunActionPayload>(Api.Resume, data)
}

export function stopBacktestApi(data: BacktestRunIdRequest) {
  return request.post<BacktestRunActionPayload>(Api.Stop, data)
}

export function labelBacktestApi(data: BacktestLabelRequest) {
  return request.post<BacktestMessagePayload>(Api.Label, data)
}

export function deleteBacktestApi(data: BacktestRunIdRequest) {
  return request.post<BacktestMessagePayload>(Api.Delete, data)
}

export function getBacktestStatusApi(params?: BacktestQueryParams) {
  return request.get<BacktestStatusPayload>(Api.Status, { params })
}

export function getBacktestRunsApi(params?: BacktestQueryParams) {
  return request.get<BacktestRunsPayload>(Api.Runs, { params })
}

export function getBacktestEquityApi(params?: BacktestQueryParams) {
  return request.get<BacktestEquityPayload>(Api.Equity, { params })
}

export function getBacktestTradesApi(params?: BacktestQueryParams) {
  return request.get<BacktestTradesPayload>(Api.Trades, { params })
}

export function getBacktestMetricsApi(params?: BacktestQueryParams) {
  return request.get<BacktestMetricsPayload>(Api.Metrics, { params })
}

export function getBacktestTraceApi(params?: BacktestQueryParams) {
  return request.get<BacktestTracePayload>(Api.Trace, { params })
}

export function getBacktestDecisionsApi(params?: BacktestQueryParams) {
  return request.get<BacktestDecisionsPayload>(Api.Decisions, { params })
}

export function exportBacktestApi(params?: BacktestQueryParams) {
  return request.get<BacktestExportPayload>(Api.Export, { params })
}

export function getBacktestKlinesApi(params: KlinesQuery) {
  return request.get<KlinePayload[]>(Api.Klines, { params })
}
