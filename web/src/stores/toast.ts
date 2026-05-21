import { defineStore } from "pinia"

export const useToast = defineStore("toast", () => {
  const success = (message: string) => {}
  const warning = (message: string) => {}
  const error = (message: string) => {}

  return { success, warning, error }
})
