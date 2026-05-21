import type {
  CryptoConfigPayload,
  CryptoPublicKeyPayload,
} from "@/types/public"
import request from "@/utils/request"

const Api = {
  Config: "/api/crypto/config",
  PublicKey: "/api/crypto/public-key",
  Decrypt: "/api/crypto/decrypt",
} as const

export function getCryptoConfigApi() {
  return request.get<CryptoConfigPayload>(Api.Config)
}

export function getCryptoPublicKeyApi() {
  return request.get<CryptoPublicKeyPayload>(Api.PublicKey)
}

export function decryptCryptoApi() {
  return request.post<CryptoConfigPayload>(Api.Decrypt)
}
