export interface RegisterRequest {
  email: string
  password: string
}

export interface LoginRequest {
  email: string
  password: string
}

export interface ChangePasswordRequest {
  current_password: string
  new_password: string
}

export interface TokenPayload {
  token: string
  user_id: string
  email: string
  message: string
}

export interface MessagePayload {
  message: string
}

export interface CurrentUserPayload {
  user_id: string
  email: string
}
