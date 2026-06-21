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
}

export interface MessagePayload {
  message: string
}

export interface CreateExchangePayload {
  message: string
  id: string
}
