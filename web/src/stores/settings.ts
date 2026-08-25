import { defineStore } from "pinia"
import { ref } from "vue"
import { getSettingsApi, updateSettingsApi } from "@/api/settings"
import { detectBrowserTimeZone, setDefaultTimeZone } from "@/utils/format"

export const useSettingsStore = defineStore("settings", () => {
  const timezone = ref<string>(detectBrowserTimeZone())
  const initialized = ref(false)

  /**
   * Load settings from backend. Called on app init when logged in.
   * If no timezone is set yet, detects the browser timezone and saves it.
   */
  async function load() {
    try {
      const res = await getSettingsApi()
      const effectiveTz = res.timezone?.trim() || detectBrowserTimeZone()
      timezone.value = effectiveTz
      setDefaultTimeZone(effectiveTz)

      // First launch: no timezone saved yet → detect & persist
      if (!res.timezone) {
        await save(effectiveTz)
      }
    } catch {
      // If settings API fails, fall back to browser timezone for display
      const browserTz = detectBrowserTimeZone()
      timezone.value = browserTz
      setDefaultTimeZone(browserTz)
    } finally {
      initialized.value = true
    }
  }

  /** Save timezone to backend and update local state. */
  async function save(tz: string) {
    const res = await updateSettingsApi({ timezone: tz })
    const savedTz = res.timezone?.trim() || tz
    timezone.value = savedTz
    setDefaultTimeZone(savedTz)
  }

  return {
    timezone,
    initialized,
    load,
    save,
  }
})
