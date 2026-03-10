import { useState, useRef, useEffect } from "react";
import { Download, FileText, File, Loader2, ChevronDown } from "lucide-react";
import toast from "react-hot-toast";
import { useExport } from "../../hooks/useExport";
import { EXPORT_TEMPLATES } from "@/lib/constants";
import type { ExportTemplate } from "@/lib/types";

interface ExportMenuProps {
  propertyId: string;
  listingIds: string[];
  disabled?: boolean;
  getListingIds?: () => Promise<string[]>;
}

export function ExportMenu({
  propertyId,
  listingIds,
  disabled = false,
  getListingIds,
}: ExportMenuProps) {
  const [open, setOpen] = useState(false);
  const [template, setTemplate] = useState<ExportTemplate>("professional");
  const menuRef = useRef<HTMLDivElement>(null);
  const { handleExportPdf, handleExportDocx, isExporting } = useExport();

  useEffect(() => {
    function handleClickOutside(e: MouseEvent) {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        setOpen(false);
      }
    }
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const onExportPdf = async () => {
    setOpen(false);
    const exportListingIds = getListingIds ? await getListingIds() : listingIds;
    if (exportListingIds.length === 0) {
      toast.error(
        "Generate at least one listing package item before exporting.",
      );
      return;
    }
    await handleExportPdf(propertyId, exportListingIds, template);
  };

  const onExportDocx = async () => {
    setOpen(false);
    const exportListingIds = getListingIds ? await getListingIds() : listingIds;
    if (exportListingIds.length === 0) {
      toast.error(
        "Generate at least one listing package item before exporting.",
      );
      return;
    }
    await handleExportDocx(propertyId, exportListingIds, template);
  };

  const isDisabled =
    disabled || (listingIds.length === 0 && !getListingIds) || isExporting;

  return (
    <div className="relative" ref={menuRef}>
      <button
        onClick={() => setOpen(!open)}
        disabled={isDisabled}
        className="inline-flex items-center gap-1.5 px-3 py-1.5 text-sm rounded-md bg-gray-100 text-gray-700 hover:bg-gray-200 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
      >
        {isExporting ? (
          <Loader2 className="w-4 h-4 animate-spin" />
        ) : (
          <Download className="w-4 h-4" />
        )}
        Export
        <ChevronDown className="w-3 h-3" />
      </button>

      {open && (
        <div className="absolute right-0 mt-1 w-72 bg-white border border-gray-200 rounded-lg shadow-lg z-10">
          <div className="border-b border-gray-100 px-3 py-3">
            <p className="text-xs font-semibold uppercase tracking-[0.16em] text-gray-500">
              Export template
            </p>
            <div className="mt-2 space-y-2">
              {EXPORT_TEMPLATES.map((option) => (
                <label
                  key={option.value}
                  className="flex cursor-pointer items-start gap-3 rounded-lg border border-gray-100 px-3 py-2 hover:border-gray-200 hover:bg-gray-50"
                >
                  <input
                    type="radio"
                    name={`export-template-${propertyId}`}
                    checked={template === option.value}
                    onChange={() => setTemplate(option.value)}
                    className="mt-1"
                  />
                  <div>
                    <p className="text-sm font-medium text-gray-800">
                      {option.label}
                    </p>
                    <p className="text-xs leading-5 text-gray-500">
                      {option.description}
                    </p>
                  </div>
                </label>
              ))}
            </div>
          </div>
          <button
            onClick={onExportPdf}
            className="flex items-center gap-2 w-full px-3 py-2 text-sm text-gray-700 hover:bg-gray-50"
          >
            <FileText className="w-4 h-4 text-red-500" />
            Download PDF
          </button>
          <button
            onClick={onExportDocx}
            className="flex items-center gap-2 w-full px-3 py-2 text-sm text-gray-700 hover:bg-gray-50 rounded-b-lg"
          >
            <File className="w-4 h-4 text-blue-500" />
            Download DOCX
          </button>
        </div>
      )}
    </div>
  );
}
