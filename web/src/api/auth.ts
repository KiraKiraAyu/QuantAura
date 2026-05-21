import type {
  ChangePasswordRequest,
  CurrentUserPayload,
  LoginRequest,
  MessagePayload,
  RegisterRequest,
  TokenPayload,
} from "@/types/auth"
import request from "@/utils/request"

const Api = {
  Register: "/api/register",
  Login: "/api/login",
  Logout: "/api/logout",
  Me: "/api/me",
  ChangePassword: "/api/auth/change-password",
} as const

export function registerApi(data: RegisterRequest) {
  return request.post<TokenPayload>(Api.Register, data)
}

export function loginApi(data: LoginRequest) {
  return request.post<TokenPayload>(Api.Login, data)
}

export function logoutApi() {
  return request.post<MessagePayload>(Api.Logout)
}

export function getCurrentUserApi() {
  return request.get<CurrentUserPayload>(Api.Me)
}

export function changePasswordApi(data: ChangePasswordRequest) {
  return request.post<MessagePayload>(Api.ChangePassword, data)
}
