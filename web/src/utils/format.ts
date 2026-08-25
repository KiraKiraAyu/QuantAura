export type DateInput = Date | number | string | null

interface FormatDateOptions {
  fallback?: string
  locale?: string | string[]
  timeZone?: string
}

const DEFAULT_LOCALE = "en-US"
const TIMESTAMP_MS_THRESHOLD = 1_000_000_000_000

/** Detect the browser's local IANA timezone (e.g. "Asia/Shanghai"), falling back to "UTC". */
export function detectBrowserTimeZone(): string {
  try {
    return Intl.DateTimeFormat().resolvedOptions().timeZone || "UTC"
  } catch {
    return "UTC"
  }
}

/** Global default timezone, set by the settings store on app init. Guaranteed string. */
let defaultTimeZone: string = detectBrowserTimeZone()

/** Set the global default timezone used by all format* functions. */
export function setDefaultTimeZone(tz: string): void {
  const trimmed = tz.trim()
  if (trimmed) {
    defaultTimeZone = trimmed
  }
}

/** Reset global default timezone to browser's local timezone. */
export function resetDefaultTimeZone(): void {
  defaultTimeZone = detectBrowserTimeZone()
}

/** Get the current global default timezone. */
export function getDefaultTimeZone(): string {
  return defaultTimeZone
}

/** Format a USD amount with 2 decimal places and thousands separators. */
export function fmtUsd(value: number | null): string {
  if (value === null || Number.isNaN(value)) return "0.00"
  return value.toLocaleString("en-US", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  })
}

/** Normalize a position side string to "Long" or "Short". */
export function normalizeSide(side: string): string {
  const s = side.toLowerCase()
  if (s === "buy" || s === "long") return "Long"
  if (s === "sell" || s === "short") return "Short"
  return side
}

/** Check whether a position side represents a long position. */
export function isLongSide(side: string): boolean {
  const s = side.toLowerCase()
  return s === "buy" || s === "long"
}

export function formatDate(value: DateInput, options?: FormatDateOptions) {
  return formatDateInput(
    value,
    {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
    },
    options,
  )
}

export function formatDateTime(value: DateInput, options?: FormatDateOptions) {
  return formatDateInput(
    value,
    {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
    },
    options,
  )
}

export function formatTime(value: DateInput, options?: FormatDateOptions) {
  return formatDateInput(
    value,
    {
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    },
    options,
  )
}

function formatDateInput(
  value: DateInput,
  dateOptions: Intl.DateTimeFormatOptions,
  options: FormatDateOptions = {},
) {
  const date = toDate(value)
  if (!date) return options.fallback ?? "-"

  const tz = options.timeZone || defaultTimeZone

  return new Intl.DateTimeFormat(options.locale || DEFAULT_LOCALE, {
    ...dateOptions,
    timeZone: tz,
  }).format(date)
}

function toDate(value: DateInput) {
  if (value == null || value === "") return null

  const date =
    value instanceof Date
      ? value
      : typeof value === "number"
        ? new Date(normalizeTimestamp(value))
        : dateFromString(value)

  return Number.isNaN(date.getTime()) ? null : date
}

function normalizeTimestamp(value: number) {
  return Math.abs(value) < TIMESTAMP_MS_THRESHOLD ? value * 1000 : value
}

function dateFromString(value: string) {
  const trimmed = value.trim()
  if (/^-?\d+(\.\d+)?$/.test(trimmed)) {
    return new Date(normalizeTimestamp(Number(trimmed)))
  }
  return new Date(trimmed)
}
