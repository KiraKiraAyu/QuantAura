import { describe, it, expect, beforeEach, vi } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useAuthStore } from "@/stores/auth";

// Mock router
vi.mock("@/router", () => ({
  default: {
    push: vi.fn(),
  },
}));

// Mock API
vi.mock("@/api/auth", () => ({
  loginApi: vi.fn(),
  logoutApi: vi.fn().mockResolvedValue({}),
  registerApi: vi.fn(),
}));

describe("Auth Store", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    localStorage.clear();
  });

  it("should initialize with default state", () => {
    const store = useAuthStore();
    expect(store.isLoggedIn).toBe(false);
    expect(store.token).toBe("");
    expect(store.userId).toBe("");
    expect(store.email).toBe("");
  });

  it("should set session on setSession", () => {
    const store = useAuthStore();

    store.setSession("test-token", "user-123", "test@example.com");

    expect(store.isLoggedIn).toBe(true);
    expect(store.token).toBe("test-token");
    expect(store.userId).toBe("user-123");
    expect(store.email).toBe("test@example.com");
    expect(localStorage.getItem("quantaura_token")).toBe("test-token");
  });

  it("should clear session on logout", async () => {
    const store = useAuthStore();
    store.setSession("test-token", "user-123", "test@example.com");

    await store.logout();

    expect(store.isLoggedIn).toBe(false);
    expect(store.token).toBe("");
    expect(store.userId).toBe("");
    expect(store.email).toBe("");
    expect(localStorage.getItem("quantaura_token")).toBeNull();
  });
});
