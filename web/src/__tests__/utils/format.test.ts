import { describe, it, expect } from "vitest"
import { formatDate, formatDateTime } from "@/utils/format"

describe("format utils", () => {
  it("formats backend second timestamps as dates", () => {
    expect(formatDate(1_700_000_000, { timeZone: "UTC" })).toBe("11/14/2023")
  })

  it("normalizes second and millisecond timestamps to the same instant", () => {
    const options = { timeZone: "UTC" }

    expect(formatDateTime(1_700_000_000, options)).toBe(
      formatDateTime(1_700_000_000_000, options),
    )
  })

  it("supports timestamp strings", () => {
    expect(formatDate("1700000000", { timeZone: "UTC" })).toBe("11/14/2023")
  })

  it("returns a fallback for invalid values", () => {
    expect(formatDate("not-a-date", { fallback: "Invalid" })).toBe("Invalid")
  })
})
