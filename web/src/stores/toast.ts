import { defineStore } from "pinia"

export const useToast = defineStore("toast", () => {
  const success = (_message: string) => {}
  const warning = (_message: string) => {}
  const error = (_message: string) => {}

  return { success, warning, error }
})
