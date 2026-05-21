import type {
  AvailableModelListPayload,
  MessagePayload,
  ModelConfigPayload,
  ModelProviderProbeRequest,
  ProviderAvailabilityPayload,
  ProviderAvailabilityRequest,
  UpdateModelConfigRequest,
} from "@/types/models"
import request from "@/utils/request"

const Api = {
  Root: "/api/models",
  List: "/api/models/list",
  CheckProvider: "/api/models/check-provider",
} as const

export function getModelConfigsApi() {
  return request.get<ModelConfigPayload>(Api.Root)
}

export function updateModelConfigsApi(data: UpdateModelConfigRequest) {
  return request.put<MessagePayload>(Api.Root, data)
}

export function listAvailableModelsApi(data: ModelProviderProbeRequest) {
  return request.post<AvailableModelListPayload>(Api.List, data)
}

export function checkProviderAvailabilityApi(
  data: ProviderAvailabilityRequest,
) {
  return request.post<ProviderAvailabilityPayload>(Api.CheckProvider, data)
}
