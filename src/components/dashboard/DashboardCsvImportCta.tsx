import {
  CheckCircle2,
  Download,
  FileSpreadsheet,
  Loader2,
  Upload,
} from "lucide-react";

interface DashboardCsvImportCtaProps {
  selectedFileName: string | null;
  isImporting: boolean;
  onPickFile: () => void;
  onDownloadTemplate: () => void;
}

export function DashboardCsvImportCta({
  selectedFileName,
  isImporting,
  onPickFile,
  onDownloadTemplate,
}: DashboardCsvImportCtaProps) {
  return (
    <aside className="rounded-2xl border border-slate-200 bg-white p-6 shadow-sm">
      <div className="inline-flex items-center gap-2 rounded-full bg-emerald-50 px-3 py-1 text-xs font-semibold uppercase tracking-[0.18em] text-emerald-700">
        Bulk onboarding
      </div>

      <div className="mt-4 space-y-3">
        <h3 className="text-xl font-semibold text-slate-900">
          Import properties from CSV
        </h3>
        <p className="text-sm leading-6 text-slate-600">
          Good for launch prep, migration, or weekly portfolio refreshes. Upload
          one spreadsheet to create properties in bulk without re-entering the
          same details by hand.
        </p>
      </div>

      <div className="mt-5 rounded-2xl border border-dashed border-slate-300 bg-slate-50 px-4 py-5">
        <div className="flex items-start gap-3">
          <div className="rounded-xl bg-white p-2 text-slate-700 shadow-sm ring-1 ring-slate-200">
            <FileSpreadsheet size={18} />
          </div>
          <div className="space-y-2">
            <p className="text-sm font-medium text-slate-800">
              Recommended shape
            </p>
            <ul className="space-y-2 text-sm leading-5 text-slate-600">
              <li>Include one row per property with clean address fields.</li>
              <li>Use JSON arrays in multi-value columns like key features.</li>
              <li>Start from the template to avoid column mismatch drift.</li>
            </ul>
          </div>
        </div>
      </div>

      <div className="mt-5 flex flex-col gap-3">
        <button
          type="button"
          onClick={onPickFile}
          disabled={isImporting}
          className="inline-flex w-full items-center justify-center gap-2 rounded-xl bg-blue-600 px-4 py-3 text-sm font-medium text-white transition-colors hover:bg-blue-700 disabled:cursor-not-allowed disabled:bg-blue-400"
        >
          {isImporting ? (
            <Loader2 size={16} className="animate-spin" />
          ) : (
            <Upload size={16} />
          )}
          {isImporting ? "Importing CSV..." : "Choose CSV file"}
        </button>

        <button
          type="button"
          onClick={onDownloadTemplate}
          className="inline-flex w-full items-center justify-center gap-2 rounded-xl border border-slate-200 bg-white px-4 py-3 text-sm font-medium text-slate-700 transition-colors hover:bg-slate-50"
        >
          <Download size={16} />
          Download template
        </button>
      </div>

      <div className="mt-5 rounded-2xl bg-slate-900 px-4 py-4 text-slate-100">
        <div className="flex items-start gap-3">
          <CheckCircle2 size={18} className="mt-0.5 text-emerald-300" />
          <div>
            <p className="text-sm font-medium">
              {selectedFileName ?? "No CSV selected yet"}
            </p>
            <p className="mt-1 text-sm leading-5 text-slate-300">
              {selectedFileName
                ? "The selected file is ready to import into the local property workspace."
                : "Once a file is selected, the dashboard can pass its text content to the CSV import command."}
            </p>
          </div>
        </div>
      </div>
    </aside>
  );
}
