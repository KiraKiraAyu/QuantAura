export interface UpdateExchangeConfigRequest {
  exchanges?: Record<string, ExchangeConfigPatch>
}

export interface ExchangeConfigPatch {
  enabled: boolean
  api_key?: string
  secret_key?: string
  passphrase?: string
  testnet?: boolean
  hyperliquid_wallet_addr?: string
  aster_user?: string
  aster_signer?: string
  aster_private_key?: string
  lighter_wallet_addr?: string
  lighter_private_key?: string
  lighter_api_key_private_key?: string
  lighter_api_key_index?: number
}

export interface CreateExchangeRequest {
  exchange_type: string
  account_name?: string
  enabled?: boolean
  api_key?: string
  secret_key?: string
  passphrase?: string
  testnet?: boolean
  hyperliquid_wallet_addr?: string
  aster_user?: string
  aster_signer?: string
  aster_private_key?: string
  lighter_wallet_addr?: string
  lighter_private_key?: string
  lighter_api_key_private_key?: string
  lighter_api_key_index?: number
}

export interface SafeExchangeConfig {
  id: string
  exchange_type: string
  account_name: string
  name: string
  type: string
  enabled: boolean
  testnet: boolean
  hyperliquidWalletAddr: string
  asterUser: string
  asterSigner: string
  lighterWalletAddr: string
  lighterApiKeyIndex: number
}

export interface MessagePayload {
  message: string
}

export interface CreateExchangePayload {
  message: string
  id: string
}
