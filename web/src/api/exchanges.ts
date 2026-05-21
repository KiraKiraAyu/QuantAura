import type {
  CreateExchangePayload,
  CreateExchangeRequest,
  MessagePayload,
  SafeExchangeConfig,
  UpdateExchangeConfigRequest,
} from "@/types/exchanges"
import request from "@/utils/request"

const Api = {
  Root: "/api/exchanges",
  Detail: "/api/exchanges/{id}",
} as const

export function getExchangeConfigsApi() {
  return request.get<SafeExchangeConfig[]>(Api.Root)
}

export function createExchangeApi(data: CreateExchangeRequest) {
  return request.post<CreateExchangePayload>(Api.Root, data)
}

export function updateExchangeConfigsApi(data: UpdateExchangeConfigRequest) {
  return request.put<MessagePayload>(Api.Root, data)
}

export function deleteExchangeApi(id: string) {
  return request.delete<MessagePayload>(Api.Detail.replace("{id}", id))
}
