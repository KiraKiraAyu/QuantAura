import router from "@/router";
import { useAuthStore } from "@/stores/auth";
import { useToast } from "@/stores/toast";
import type { ApiResponse } from "@/types/api";

import axios, {
  AxiosHeaders,
  type AxiosError,
  type AxiosInstance,
  type AxiosRequestConfig,
  type AxiosResponse,
  type InternalAxiosRequestConfig,
} from "axios";

const service: AxiosInstance = axios.create({
  timeout: 10000,
  headers: {
    "Content-Type": "application/json",
  },
});

service.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    const authStore = useAuthStore();

    if (authStore.isLoggedIn && authStore.token) {
      const headers = AxiosHeaders.from(config.headers);

      headers.set("Authorization", `Bearer ${authStore.token}`);

      config.headers = headers;
    }

    return config;
  },
  (error: AxiosError) => {
    return Promise.reject(error);
  },
);

service.interceptors.response.use(
  (response: AxiosResponse<ApiResponse<unknown>>): any => {
    const res = response.data

    if (res.success) {
      return res.data
    }

    const message = res.error || res.message || "请求失败"

    const toast = useToast()
    toast.error(message)

    return Promise.reject(new Error(message))
  },
  async (error: AxiosError<ApiResponse>) => {
    const toast = useToast()

    const status = error.response?.status

    const message =
      error.response?.data?.error ||
      error.response?.data?.message ||
      error.message ||
      "网络异常"

    toast.error(message)

    if (status === 401) {
      const authStore = useAuthStore()

      authStore.logout()

      if (router.currentRoute.value.path !== "/login") {
        await router.push("/login")
      }
    }

    return Promise.reject(new Error(message))
  },
)

const request = {
  get<T = unknown>(url: string, config?: AxiosRequestConfig): Promise<T> {
    return service.get<ApiResponse<T>, T>(url, config);
  },

  post<T = unknown>(
    url: string,
    data?: unknown,
    config?: AxiosRequestConfig,
  ): Promise<T> {
    return service.post<ApiResponse<T>, T>(url, data, config);
  },

  put<T = unknown>(
    url: string,
    data?: unknown,
    config?: AxiosRequestConfig,
  ): Promise<T> {
    return service.put<ApiResponse<T>, T>(url, data, config);
  },

  patch<T = unknown>(
    url: string,
    data?: unknown,
    config?: AxiosRequestConfig,
  ): Promise<T> {
    return service.patch<ApiResponse<T>, T>(url, data, config);
  },

  delete<T = unknown>(url: string, config?: AxiosRequestConfig): Promise<T> {
    return service.delete<ApiResponse<T>, T>(url, config);
  },
};

export default request;
