import type {
  CreateDebateRequest,
  DebateActionPayload,
  DebateDetailPayload,
  DebateExecutionPayload,
  DebateListPayload,
  DebateMessagePayload,
  DebateMessagesPayload,
  DebatePersonalitiesPayload,
  DebateVotesPayload,
  StartDebateRequest,
} from "@/types/debates"
import request from "@/utils/request"

const Api = {
  Root: "/api/debates",
  Personalities: "/api/debates/personalities",
  Detail: "/api/debates/{id}",
  Start: "/api/debates/{id}/start",
  Cancel: "/api/debates/{id}/cancel",
  Execute: "/api/debates/{id}/execute",
  Messages: "/api/debates/{id}/messages",
  Votes: "/api/debates/{id}/votes",
} as const

export function getDebatesApi() {
  return request.get<DebateListPayload>(Api.Root)
}

export function createDebateApi(data: CreateDebateRequest) {
  return request.post<DebateActionPayload>(Api.Root, data)
}

export function getDebatePersonalitiesApi() {
  return request.get<DebatePersonalitiesPayload>(Api.Personalities)
}

export function getDebateApi(id: string) {
  return request.get<DebateDetailPayload>(Api.Detail.replace("{id}", id))
}

export function deleteDebateApi(id: string) {
  return request.delete<DebateMessagePayload>(Api.Detail.replace("{id}", id))
}

export function startDebateApi(id: string, data: StartDebateRequest) {
  return request.post<DebateActionPayload>(Api.Start.replace("{id}", id), data)
}

export function cancelDebateApi(id: string) {
  return request.post<DebateActionPayload>(Api.Cancel.replace("{id}", id))
}

export function executeDebateApi(id: string) {
  return request.post<DebateExecutionPayload>(Api.Execute.replace("{id}", id))
}

export function getDebateMessagesApi(id: string) {
  return request.get<DebateMessagesPayload>(Api.Messages.replace("{id}", id))
}

export function getDebateVotesApi(id: string) {
  return request.get<DebateVotesPayload>(Api.Votes.replace("{id}", id))
}
