import { useState, useCallback } from "react";
import { save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";
import { exportPdf, exportDocx } from "../lib/tauri";
import type { ExportTemplate } from "../lib/types";
import toast from "react-hot-toast";

export function useExport() {
  const [isExporting, setIsExporting] = useState(false);

  const handleExportPdf = useCallback(
    async (
      propertyId: string,
      listingIds: string[],
      template: ExportTemplate = "professional",
    ) => {
      setIsExporting(true);
      try {
        const bytes = await exportPdf(propertyId, listingIds, template);
        const filePath = await save({
          defaultPath: `property-listing-${template}.pdf`,
          filters: [{ name: "PDF", extensions: ["pdf"] }],
        });
        if (filePath) {
          await writeFile(filePath, new Uint8Array(bytes));
          toast.success(
            `${capitalizeTemplate(template)} PDF saved successfully`,
          );
        }
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        toast.error(`PDF export failed: ${message}`);
      } finally {
        setIsExporting(false);
      }
    },
    [],
  );

  const handleExportDocx = useCallback(
    async (
      propertyId: string,
      listingIds: string[],
      template: ExportTemplate = "professional",
    ) => {
      setIsExporting(true);
      try {
        const bytes = await exportDocx(propertyId, listingIds, template);
        const filePath = await save({
          defaultPath: `property-listing-${template}.docx`,
          filters: [{ name: "Word Document", extensions: ["docx"] }],
        });
        if (filePath) {
          await writeFile(filePath, new Uint8Array(bytes));
          toast.success(
            `${capitalizeTemplate(template)} DOCX saved successfully`,
          );
        }
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        toast.error(`DOCX export failed: ${message}`);
      } finally {
        setIsExporting(false);
      }
    },
    [],
  );

  return { handleExportPdf, handleExportDocx, isExporting };
}

function capitalizeTemplate(template: ExportTemplate) {
  return template.charAt(0).toUpperCase() + template.slice(1);
}
