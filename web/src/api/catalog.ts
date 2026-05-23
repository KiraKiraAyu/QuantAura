import type {
  SupportedExchangePayload,
  SupportedProviderTypePayload,
} from "@/types/public"
import request from "@/utils/request"

const Api = {
  SupportedProviderTypes: "/api/catalog/supported-provider-types",
  SupportedExchanges: "/api/catalog/supported-exchanges",
} as const

export function getSupportedProviderTypesApi() {
  return request.get<SupportedProviderTypePayload[]>(Api.SupportedProviderTypes)
}

export function getSupportedExchangesApi() {
  return request.get<SupportedExchangePayload[]>(Api.SupportedExchanges)
}
