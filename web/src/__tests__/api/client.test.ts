import { describe, it, expect, vi, beforeEach } from "vitest"
import { setActivePinia, createPinia } from "pinia"
import axios from "axios"
import request from "@/utils/request"
import { useAuthStore } from "@/stores/auth"

vi.mock("axios", async () => {
  const actual = await vi.importActual<typeof import("axios")>("axios")
  const mockAxiosInstance = {
    interceptors: {
      request: {
        use: vi.fn<(onFulfilled?: (config: any) => any) => void>(
          (onFulfilled?: (config: any) => any) => {
            mockAxiosInstance.requestInterceptor = onFulfilled
          },
        ),
      },
      response: { use: vi.fn<() => void>() },
    },
    requestInterceptor: undefined as ((config: any) => any) | undefined,
    get: vi.fn<() => Promise<unknown>>(),
    post: vi.fn<() => Promise<unknown>>(),
    put: vi.fn<() => Promise<unknown>>(),
    patch: vi.fn<() => Promise<unknown>>(),
    delete: vi.fn<() => Promise<unknown>>(),
  }

  return {
    ...actual,
    default: {
      ...actual.default,
      create: vi.fn<() => typeof mockAxiosInstance>(() => mockAxiosInstance),
    },
  }
})

vi.mock("@/router", () => ({
  default: {
    currentRoute: { value: { path: "/dashboard" } },
    push: vi.fn<() => Promise<void>>(),
  },
}))

describe("Request Client", () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it("unwraps successful API response data for GET", async () => {
    const mockData = { id: "trader_1", name: "Alpha Trader" }
    const axiosInstance = axios.create()
    vi.mocked(axiosInstance.get).mockResolvedValue({
      data: {
        success: true,
        data: mockData,
        message: null,
        error: null,
      },
    })

    const result = await request.get("/api/traders/1")
    expect(result).toEqual(mockData)
    expect(vi.mocked(axiosInstance.get).mock.calls[0]?.[0]).toBe(
      "/api/traders/1",
    )
  })

  it("unwraps successful API response data for POST", async () => {
    const postPayload = { name: "New Trader", balance: 1000 }
    const responseData = { id: "trader_2", ...postPayload }
    const axiosInstance = axios.create()
    vi.mocked(axiosInstance.post).mockResolvedValue({
      data: {
        success: true,
        data: responseData,
        message: "Created",
        error: null,
      },
    })

    const result = await request.post("/api/traders", postPayload)
    expect(result).toEqual(responseData)
    expect(vi.mocked(axiosInstance.post).mock.calls[0]?.[0]).toBe(
      "/api/traders",
    )
    expect(vi.mocked(axiosInstance.post).mock.calls[0]?.[1]).toEqual(
      postPayload,
    )
  })

  it("unwraps successful API response data for DELETE", async () => {
    const axiosInstance = axios.create()
    vi.mocked(axiosInstance.delete).mockResolvedValue({
      data: {
        success: true,
        data: { message: "Deleted" },
        message: null,
        error: null,
      },
    })

    const result = await request.delete("/api/traders/1")
    expect(result).toEqual({ message: "Deleted" })
    expect(vi.mocked(axiosInstance.delete).mock.calls[0]?.[0]).toBe(
      "/api/traders/1",
    )
  })

  it("handles request interceptor authorization header injection", () => {
    const axiosInstance = axios.create() as unknown as {
      requestInterceptor?: (config: {
        headers?: Record<string, string>
      }) => { headers?: Record<string, string> }
    }
    expect(axiosInstance.requestInterceptor).toBeDefined()
    const interceptor = axiosInstance.requestInterceptor!

    const authStore = useAuthStore()

    // 1. When logged out, no Authorization header is injected
    authStore.setToken("")
    const configLoggedOut = interceptor({ headers: {} })
    const loggedOutAuth = (configLoggedOut.headers as any)?.get
      ? (configLoggedOut.headers as any).get("Authorization")
      : configLoggedOut.headers?.Authorization
    expect(loggedOutAuth).toBeUndefined()

    // 2. When logged in, Authorization header is injected as Bearer <token>
    authStore.setToken("my-jwt-token")
    const configLoggedIn = interceptor({ headers: {} })
    const loggedInAuth = (configLoggedIn.headers as any)?.get
      ? (configLoggedIn.headers as any).get("Authorization")
      : configLoggedIn.headers?.Authorization
    expect(loggedInAuth).toBe("Bearer my-jwt-token")
  })
})
