import {
  getAnalyticsSummary,
  getCsvTemplate,
  importPropertiesCsv,
} from "@/lib/tauri";

export interface DashboardAnalyticsSummary {
  activeProperties: number;
  totalGenerations: number;
  averageLatencyMs: number | null;
  successRate: number | null;
  totalCostCents: number;
}

export function createFallbackAnalyticsSummary(
  propertyCount: number,
): DashboardAnalyticsSummary {
  return {
    activeProperties: propertyCount,
    totalGenerations: 0,
    averageLatencyMs: null,
    successRate: null,
    totalCostCents: 0,
  };
}

export async function getDashboardAnalyticsSummary(
  fallback: DashboardAnalyticsSummary,
): Promise<{
  summary: DashboardAnalyticsSummary;
  source: "live" | "fallback";
}> {
  try {
    const summary = await getAnalyticsSummary();
    return {
      source: "live",
      summary: {
        activeProperties: fallback.activeProperties,
        totalGenerations: summary.totalGenerations,
        averageLatencyMs: summary.averageLatencyMs,
        successRate: summary.successRate,
        totalCostCents: summary.totalCostCents,
      },
    };
  } catch {
    return { summary: fallback, source: "fallback" };
  }
}

export const importDashboardCsv = importPropertiesCsv;
export const getDashboardCsvTemplate = getCsvTemplate;
