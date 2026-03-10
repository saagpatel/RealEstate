import { useEffect, useRef, useState, type ChangeEvent } from "react";
import { useNavigate } from "react-router-dom";
import { Download, Home, Loader2, Plus } from "lucide-react";
import toast from "react-hot-toast";
import { DashboardAnalyticsSummary } from "@/components/dashboard/DashboardAnalyticsSummary";
import { DashboardCsvImportCta } from "@/components/dashboard/DashboardCsvImportCta";
import { DashboardSetupChecklist } from "@/components/dashboard/DashboardSetupChecklist";
import {
  createFallbackAnalyticsSummary,
  getDashboardAnalyticsSummary,
  getDashboardCsvTemplate,
  importDashboardCsv,
  type DashboardAnalyticsSummary as DashboardAnalyticsSnapshot,
} from "@/components/dashboard/dashboardBridge";
import { PageHeader } from "@/components/layout/PageHeader";
import { PropertyCard } from "@/components/property/PropertyCard";
import { listPhotos } from "@/lib/tauri";
import { usePropertyStore } from "@/stores/propertyStore";
import { useSettingsStore } from "@/stores/settingsStore";

export function Dashboard() {
  const navigate = useNavigate();
  const fileInputRef = useRef<HTMLInputElement | null>(null);
  const { properties, isLoading, fetchProperties, deleteProperty } =
    usePropertyStore();
  const apiKey = useSettingsStore((state) => state.apiKey);
  const agentName = useSettingsStore((state) => state.agentName);
  const agentPhone = useSettingsStore((state) => state.agentPhone);
  const agentEmail = useSettingsStore((state) => state.agentEmail);
  const brokerageName = useSettingsStore((state) => state.brokerageName);
  const [analytics, setAnalytics] = useState<DashboardAnalyticsSnapshot>(
    createFallbackAnalyticsSummary(0),
  );
  const [analyticsSource, setAnalyticsSource] = useState<"live" | "fallback">(
    "fallback",
  );
  const [isAnalyticsLoading, setIsAnalyticsLoading] = useState(true);
  const [selectedCsvFileName, setSelectedCsvFileName] = useState<string | null>(
    null,
  );
  const [isImportingCsv, setIsImportingCsv] = useState(false);
  const [primaryPhotoPaths, setPrimaryPhotoPaths] = useState<
    Record<string, string | null>
  >({});

  useEffect(() => {
    fetchProperties().catch((err: unknown) => {
      const message =
        err instanceof Error ? err.message : "Failed to load properties";
      toast.error(message);
    });
  }, [fetchProperties]);

  useEffect(() => {
    const fallback = createFallbackAnalyticsSummary(properties.length);

    setIsAnalyticsLoading(true);
    getDashboardAnalyticsSummary(fallback)
      .then(({ summary, source }) => {
        setAnalytics(summary);
        setAnalyticsSource(source);
      })
      .catch((err: unknown) => {
        const message =
          err instanceof Error
            ? err.message
            : "Failed to load dashboard analytics";
        setAnalytics(fallback);
        setAnalyticsSource("fallback");
        toast.error(message);
      })
      .finally(() => {
        setIsAnalyticsLoading(false);
      });
  }, [properties.length]);

  useEffect(() => {
    let cancelled = false;

    if (properties.length === 0) {
      setPrimaryPhotoPaths({});
      return;
    }

    Promise.all(
      properties.map(async (property) => {
        try {
          const photos = await listPhotos(property.id);
          return [property.id, photos[0]?.thumbnailPath ?? null] as const;
        } catch {
          return [property.id, null] as const;
        }
      }),
    ).then((entries) => {
      if (cancelled) return;

      setPrimaryPhotoPaths(Object.fromEntries(entries));
    });

    return () => {
      cancelled = true;
    };
  }, [properties]);

  const handleDelete = async (id: string) => {
    if (!window.confirm("Delete this property? This cannot be undone.")) return;
    try {
      await deleteProperty(id);
      toast.success("Property deleted");
    } catch (err: unknown) {
      const message =
        err instanceof Error ? err.message : "Failed to delete property";
      toast.error(message);
    }
  };

  const refreshDashboardSummary = () => {
    const fallback = createFallbackAnalyticsSummary(properties.length);
    setIsAnalyticsLoading(true);

    getDashboardAnalyticsSummary(fallback)
      .then(({ summary, source }) => {
        setAnalytics(summary);
        setAnalyticsSource(source);
      })
      .catch((err: unknown) => {
        const message =
          err instanceof Error
            ? err.message
            : "Failed to refresh dashboard analytics";
        toast.error(message);
        setAnalytics(fallback);
        setAnalyticsSource("fallback");
      })
      .finally(() => {
        setIsAnalyticsLoading(false);
      });
  };

  const handleDownloadTemplate = async () => {
    try {
      const content = await getDashboardCsvTemplate();

      const blob = new Blob([content], { type: "text/csv;charset=utf-8" });
      const url = URL.createObjectURL(blob);
      const link = document.createElement("a");
      link.href = url;
      link.download = "realestate-import-template.csv";
      document.body.appendChild(link);
      link.click();
      link.remove();
      URL.revokeObjectURL(url);
      toast.success("CSV template downloaded");
    } catch (err: unknown) {
      const message =
        err instanceof Error ? err.message : "Failed to download CSV template";
      toast.error(message);
    }
  };

  const handleCsvSelection = async (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    setSelectedCsvFileName(file.name);
    setIsImportingCsv(true);

    try {
      const csvData = await file.text();
      const result = await importDashboardCsv(csvData);

      await fetchProperties();
      refreshDashboardSummary();

      const imported = result.successful;
      const failed = result.failed;
      const total = result.total;
      const summaryBits = [
        imported > 0 ? `${imported} imported` : null,
        failed > 0 ? `${failed} failed` : null,
        total > 0 ? `${total} total rows` : null,
      ].filter(Boolean);

      toast.success(
        summaryBits.length > 0
          ? `CSV import complete: ${summaryBits.join(", ")}`
          : "CSV import submitted",
      );
    } catch (err: unknown) {
      const message =
        err instanceof Error ? err.message : "Failed to import CSV";
      toast.error(message);
    } finally {
      setIsImportingCsv(false);
      event.target.value = "";
    }
  };

  const isEmpty = properties.length === 0 && !isLoading;
  const hasAgentProfile =
    agentName.trim().length > 0 &&
    agentPhone.trim().length > 0 &&
    agentEmail.trim().length > 0 &&
    brokerageName.trim().length > 0;

  return (
    <div className="space-y-6">
      <PageHeader
        title="Dashboard"
        subtitle="Manage your property listings and keep an eye on delivery momentum"
        actions={
          <>
            <button
              type="button"
              onClick={handleDownloadTemplate}
              className="flex items-center gap-2 rounded-lg border border-gray-200 bg-white px-4 py-2 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50"
            >
              <Download size={16} />
              CSV Template
            </button>
            <button
              onClick={() => navigate("/property/new")}
              className="flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-700"
            >
              <Plus size={16} />
              New Property
            </button>
          </>
        }
      />

      <input
        ref={fileInputRef}
        type="file"
        accept=".csv,text/csv"
        className="hidden"
        onChange={(event) => {
          void handleCsvSelection(event);
        }}
      />

      <div className="grid grid-cols-1 gap-6 xl:grid-cols-[minmax(0,2fr)_minmax(340px,1fr)]">
        <DashboardAnalyticsSummary
          summary={analytics}
          isLoading={isAnalyticsLoading}
          source={analyticsSource}
          onRefresh={refreshDashboardSummary}
        />

        <DashboardCsvImportCta
          selectedFileName={selectedCsvFileName}
          isImporting={isImportingCsv}
          onPickFile={() => fileInputRef.current?.click()}
          onDownloadTemplate={handleDownloadTemplate}
        />
      </div>

      <DashboardSetupChecklist
        hasApiKey={apiKey.trim().length > 0}
        hasAgentProfile={hasAgentProfile}
        propertyCount={properties.length}
        onOpenSettings={() => navigate("/settings")}
        onCreateProperty={() => navigate("/property/new")}
        onImportCsv={() => fileInputRef.current?.click()}
      />

      {isLoading && (
        <div className="flex items-center justify-center py-20">
          <Loader2 size={24} className="animate-spin text-gray-400" />
        </div>
      )}

      {isEmpty && (
        <div className="rounded-2xl border border-dashed border-gray-300 bg-white px-6 py-14 text-center">
          <div className="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-gray-100">
            <Home size={28} className="text-gray-400" />
          </div>
          <h3 className="mb-2 text-lg font-medium text-gray-900">
            No properties yet
          </h3>
          <p className="mx-auto mb-5 max-w-xl text-sm leading-6 text-gray-500">
            Create your first property to start generating listing descriptions,
            social media posts, and email campaigns, or use the CSV import lane
            above to bring over a batch of listings in one pass.
          </p>
          <div className="flex flex-col items-center justify-center gap-3 sm:flex-row">
            <button
              onClick={() => navigate("/property/new")}
              className="rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-700"
            >
              Create Your First Listing
            </button>
            <button
              type="button"
              onClick={() => fileInputRef.current?.click()}
              className="rounded-lg border border-gray-200 bg-white px-4 py-2 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50"
            >
              Import From CSV
            </button>
          </div>
        </div>
      )}

      {!isEmpty && !isLoading && (
        <section className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-lg font-semibold text-gray-900">
                Active portfolio
              </h3>
              <p className="mt-1 text-sm text-gray-500">
                Jump into any property to edit details or start generating copy.
              </p>
            </div>
            <span className="rounded-full bg-gray-100 px-3 py-1 text-xs font-medium text-gray-600">
              {properties.length}{" "}
              {properties.length === 1 ? "property" : "properties"}
            </span>
          </div>

          <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
            {properties.map((property) => (
              <PropertyCard
                key={property.id}
                property={property}
                primaryPhotoPath={primaryPhotoPaths[property.id] ?? null}
                onDelete={handleDelete}
              />
            ))}
          </div>
        </section>
      )}
    </div>
  );
}
