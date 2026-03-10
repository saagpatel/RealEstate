import { CheckCircle2, Circle, Settings, Plus, Upload } from "lucide-react";
import { buildSetupChecklist, countCompletedSteps } from "./setupChecklist";

interface DashboardSetupChecklistProps {
  hasApiKey: boolean;
  hasAgentProfile: boolean;
  propertyCount: number;
  onOpenSettings: () => void;
  onCreateProperty: () => void;
  onImportCsv: () => void;
}

export function DashboardSetupChecklist({
  hasApiKey,
  hasAgentProfile,
  propertyCount,
  onOpenSettings,
  onCreateProperty,
  onImportCsv,
}: DashboardSetupChecklistProps) {
  const steps = buildSetupChecklist({
    hasLicenseAccess: true,
    hasApiKey,
    hasAgentProfile,
    propertyCount,
  });
  const completed = countCompletedSteps(steps);
  const isReady = completed === steps.length;

  return (
    <section className="rounded-2xl border border-slate-200 bg-white p-6">
      <div className="flex flex-col gap-4 lg:flex-row lg:items-start lg:justify-between">
        <div>
          <p className="text-xs font-semibold uppercase tracking-[0.18em] text-slate-500">
            Setup readiness
          </p>
          <h3 className="mt-2 text-xl font-semibold text-slate-900">
            {isReady
              ? "Your workspace is ready to ship copy"
              : "Finish the launch checklist"}
          </h3>
          <p className="mt-2 max-w-2xl text-sm leading-6 text-slate-600">
            {isReady
              ? "You have the core setup pieces in place. The fastest next move is to open a property and generate your first listing package."
              : "These are the remaining setup items that matter most before daily use. Completing them now removes the common sources of friction later."}
          </p>
        </div>
        <div className="rounded-full bg-slate-100 px-3 py-1 text-sm font-medium text-slate-700">
          {completed} of {steps.length} complete
        </div>
      </div>

      <div className="mt-5 grid gap-3 md:grid-cols-2">
        {steps.map((step) => (
          <div
            key={step.id}
            className={`rounded-xl border px-4 py-3 ${
              step.complete
                ? "border-emerald-200 bg-emerald-50/70"
                : "border-slate-200 bg-slate-50/70"
            }`}
          >
            <div className="flex items-start gap-3">
              <div className="pt-0.5">
                {step.complete ? (
                  <CheckCircle2 className="text-emerald-600" size={18} />
                ) : (
                  <Circle className="text-slate-400" size={18} />
                )}
              </div>
              <div>
                <p className="text-sm font-medium text-slate-900">
                  {step.label}
                </p>
                <p className="mt-1 text-sm leading-5 text-slate-600">
                  {step.description}
                </p>
              </div>
            </div>
          </div>
        ))}
      </div>

      <div className="mt-5 flex flex-col gap-3 sm:flex-row">
        <button
          type="button"
          onClick={onOpenSettings}
          className="inline-flex items-center justify-center gap-2 rounded-lg border border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-700 transition-colors hover:bg-slate-50"
        >
          <Settings size={16} />
          Open Settings
        </button>
        <button
          type="button"
          onClick={onCreateProperty}
          className="inline-flex items-center justify-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-700"
        >
          <Plus size={16} />
          Create Property
        </button>
        <button
          type="button"
          onClick={onImportCsv}
          className="inline-flex items-center justify-center gap-2 rounded-lg border border-slate-200 bg-white px-4 py-2 text-sm font-medium text-slate-700 transition-colors hover:bg-slate-50"
        >
          <Upload size={16} />
          Import CSV
        </button>
      </div>
    </section>
  );
}
