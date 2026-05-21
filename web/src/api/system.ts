import type { HealthResponse, SystemConfigResponse } from "@/types/system"
import request from "@/utils/request"

const Api = {
  Health: "/api/health",
  Config: "/api/config",
} as const

export function getHealthApi() {
  return request.get<HealthResponse>(Api.Health)
}

export function getSystemConfigApi() {
  return request.get<SystemConfigResponse>(Api.Config)
}
