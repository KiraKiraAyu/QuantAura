import type {
  ExchangeSymbolsPayload,
  KlinePayload,
  KlinesQuery,
  SymbolsQuery,
} from "@/types/public"
import request from "@/utils/request"

const Api = {
  Klines: "/api/market/klines",
  Symbols: "/api/market/symbols",
} as const

export function getKlinesApi(params: KlinesQuery) {
  return request.get<KlinePayload[]>(Api.Klines, { params })
}

export function getExchangeSymbolsApi(params?: SymbolsQuery) {
  return request.get<ExchangeSymbolsPayload>(Api.Symbols, { params })
}
