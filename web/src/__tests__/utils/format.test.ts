import { describe, it, expect } from "vitest";

describe("Format Utils", () => {
  describe("formatCurrency", () => {
    it("should format positive numbers", () => {
      const formatCurrency = (value: number) => {
        return new Intl.NumberFormat("en-US", {
          style: "currency",
          currency: "USD",
        }).format(value);
      };

      expect(formatCurrency(1234.56)).toBe("$1,234.56");
    });

    it("should format negative numbers", () => {
      const formatCurrency = (value: number) => {
        return new Intl.NumberFormat("en-US", {
          style: "currency",
          currency: "USD",
        }).format(value);
      };

      expect(formatCurrency(-1234.56)).toBe("-$1,234.56");
    });

    it("should handle zero", () => {
      const formatCurrency = (value: number) => {
        return new Intl.NumberFormat("en-US", {
          style: "currency",
          currency: "USD",
        }).format(value);
      };

      expect(formatCurrency(0)).toBe("$0.00");
    });
  });

  describe("formatPercentage", () => {
    it("should format percentage with 2 decimals", () => {
      const formatPercentage = (value: number) => {
        return `${(value * 100).toFixed(2)}%`;
      };

      expect(formatPercentage(0.1234)).toBe("12.34%");
    });

    it("should handle negative percentages", () => {
      const formatPercentage = (value: number) => {
        return `${(value * 100).toFixed(2)}%`;
      };

      expect(formatPercentage(-0.05)).toBe("-5.00%");
    });
  });
});
