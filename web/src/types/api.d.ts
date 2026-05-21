export interface ApiResponse<T = unknown> {
  success: boolean
  data?: T | null
  message?: string | null
  error?: string | null
}
