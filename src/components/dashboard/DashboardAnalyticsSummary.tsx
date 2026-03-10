import {
  ArrowUpRight,
  BarChart3,
  Clock3,
  DollarSign,
  Loader2,
  Sparkles,
  ShieldCheck,
} from "lucide-react";
import type { DashboardAnalyticsSummary as DashboardAnalyticsSnapshot } from "./dashboardBridge";

interface DashboardAnalyticsSummaryProps {
  summary: DashboardAnalyticsSnapshot;
  isLoading: boolean;
  source: "live" | "fallback";
  onRefresh: () => void;
}

function formatCurrency(cents: number) {
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
    maximumFractionDigits: 0,
  }).format(cents / 100);
}

function formatLatency(latencyMs: number | null) {
  if (latencyMs === null) return "Waiting on data";
  if (latencyMs < 1000) return `${Math.round(latencyMs)} ms avg`;
  return `${(latencyMs / 1000).toFixed(1)} s avg`;
}

export function DashboardAnalyticsSummary({
  summary,
  isLoading,
  source,
  onRefresh,
}: DashboardAnalyticsSummaryProps) {
  const cards = [
    {
      label: "Active properties",
      value: summary.activeProperties.toString(),
      detail:
        summary.activeProperties > 0
          ? "Ready for listing work"
          : "Start with one pilot property",
      icon: BarChart3,
      tone: "bg-sky-50 text-sky-700 border-sky-100",
    },
    {
      label: "AI generations",
      value: summary.totalGenerations.toString(),
      detail: "Tracks listings, social posts, and emails created",
      icon: Sparkles,
      tone: "bg-emerald-50 text-emerald-700 border-emerald-100",
    },
    {
      label: "Success rate",
      value:
        summary.successRate === null
          ? "No data"
          : `${Math.round(summary.successRate)}%`,
      detail: "Based on recorded generation outcomes",
      icon: ShieldCheck,
      tone: "bg-amber-50 text-amber-700 border-amber-100",
    },
    {
      label: "Estimated AI spend",
      value: formatCurrency(summary.totalCostCents),
      detail: formatLatency(summary.averageLatencyMs),
      icon: DollarSign,
      tone: "bg-violet-50 text-violet-700 border-violet-100",
    },
  ];

  const headline =
    summary.activeProperties > 0
      ? `${summary.activeProperties} active properties are in motion, with ${summary.totalGenerations} total AI outputs tracked so far.`
      : "The dashboard is ready for your first property or a CSV batch import.";

  return (
    <section className="rounded-2xl border border-slate-200 bg-white overflow-hidden">
      <div className="bg-[linear-gradient(135deg,rgba(239,246,255,1),rgba(248,250,252,1),rgba(236,253,245,1))] border-b border-slate-200 px-6 py-5">
        <div className="flex flex-col gap-4 lg:flex-row lg:items-start lg:justify-between">
          <div className="space-y-2">
            <div className="inline-flex items-center gap-2 rounded-full border border-white/70 bg-white/70 px-3 py-1 text-xs font-semibold uppercase tracking-[0.18em] text-slate-600">
              Portfolio Pulse
              <ArrowUpRight size={14} />
            </div>
            <div>
              <h3 className="text-xl font-semibold text-slate-900">
                PM summary
              </h3>
              <p className="mt-1 max-w-2xl text-sm leading-6 text-slate-600">
                {headline}
              </p>
            </div>
          </div>

          <div className="flex items-center gap-3">
            <span className="rounded-full bg-white/80 px-3 py-1 text-xs font-medium text-slate-600 ring-1 ring-slate-200">
              {source === "live"
                ? "Live dashboard data"
                : "Preview until analytics bridge is connected"}
            </span>
            <button
              type="button"
              onClick={onRefresh}
              className="inline-flex items-center gap-2 rounded-lg border border-slate-200 bg-white px-3 py-2 text-sm font-medium text-slate-700 transition-colors hover:bg-slate-50"
            >
              {isLoading ? (
                <Loader2 size={16} className="animate-spin" />
              ) : (
                <Clock3 size={16} />
              )}
              Refresh
            </button>
          </div>
        </div>
      </div>

      <div className="grid gap-4 p-6 md:grid-cols-2 xl:grid-cols-4">
        {cards.map((card) => {
          const Icon = card.icon;
          return (
            <div
              key={card.label}
              className="rounded-2xl border border-slate-200 bg-slate-50/70 p-4"
            >
              <div className="flex items-start justify-between gap-3">
                <div>
                  <p className="text-sm font-medium text-slate-600">
                    {card.label}
                  </p>
                  <p className="mt-2 text-2xl font-semibold text-slate-900">
                    {card.value}
                  </p>
                </div>
                <div className={`rounded-xl border p-2 ${card.tone}`}>
                  <Icon size={18} />
                </div>
              </div>
              <p className="mt-3 text-sm leading-5 text-slate-500">
                {card.detail}
              </p>
            </div>
          );
        })}
      </div>

      <div className="grid gap-4 border-t border-slate-200 bg-slate-50/60 px-6 py-4 md:grid-cols-2">
        <div>
          <p className="text-xs font-semibold uppercase tracking-[0.18em] text-slate-500">
            Recommended next move
          </p>
          <p className="mt-2 text-sm leading-6 text-slate-700">
            {summary.activeProperties > 0
              ? "Use CSV import to add the next tranche of properties, then compare generation volume and reliability week over week."
              : "Download the CSV template for a quick bulk start, or create a single flagship property to validate your workflow."}
          </p>
        </div>
        <div>
          <p className="text-xs font-semibold uppercase tracking-[0.18em] text-slate-500">
            Speed signal
          </p>
          <p className="mt-2 text-sm leading-6 text-slate-700">
            {summary.averageLatencyMs === null
              ? "Generation timing will appear after the first successful run."
              : `${formatLatency(summary.averageLatencyMs)} across recorded generations.`}
          </p>
        </div>
      </div>
    </section>
  );
}
