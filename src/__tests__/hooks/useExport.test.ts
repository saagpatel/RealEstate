import { describe, it, expect, beforeEach, vi } from "vitest";
import { renderHook, act, waitFor } from "@testing-library/react";
import { useExport } from "@/hooks/useExport";
import { save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";
import { exportPdf, exportDocx } from "@/lib/tauri";
import toast from "react-hot-toast";

vi.mock("@tauri-apps/plugin-dialog");
vi.mock("@tauri-apps/plugin-fs");
vi.mock("@/lib/tauri");
vi.mock("react-hot-toast");

describe("useExport", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should initialize with isExporting false", () => {
    const { result } = renderHook(() => useExport());
    expect(result.current.isExporting).toBe(false);
  });

  describe("handleExportPdf", () => {
    it("should export PDF successfully", async () => {
      const mockBytes = new Uint8Array([0x25, 0x50, 0x44, 0x46]); // PDF header
      vi.mocked(exportPdf).mockResolvedValue(Array.from(mockBytes));
      vi.mocked(save).mockResolvedValue("/path/to/file.pdf");
      vi.mocked(writeFile).mockResolvedValue();

      const { result } = renderHook(() => useExport());

      await act(async () => {
        await result.current.handleExportPdf(
          "prop-1",
          ["listing-1"],
          "professional",
        );
      });

      expect(exportPdf).toHaveBeenCalledWith(
        "prop-1",
        ["listing-1"],
        "professional",
      );
      expect(save).toHaveBeenCalledWith({
        defaultPath: "property-listing-professional.pdf",
        filters: [{ name: "PDF", extensions: ["pdf"] }],
      });
      expect(writeFile).toHaveBeenCalledWith(
        "/path/to/file.pdf",
        expect.any(Uint8Array),
      );
      expect(toast.success).toHaveBeenCalledWith(
        "Professional PDF saved successfully",
      );
      expect(result.current.isExporting).toBe(false);
    });

    it("should handle user canceling file dialog", async () => {
      const mockBytes = new Uint8Array([0x25, 0x50, 0x44, 0x46]);
      vi.mocked(exportPdf).mockResolvedValue(Array.from(mockBytes));
      vi.mocked(save).mockResolvedValue(null); // User canceled

      const { result } = renderHook(() => useExport());

      await act(async () => {
        await result.current.handleExportPdf("prop-1", ["listing-1"]);
      });

      expect(writeFile).not.toHaveBeenCalled();
      expect(toast.success).not.toHaveBeenCalled();
      expect(result.current.isExporting).toBe(false);
    });

    it("should handle export errors", async () => {
      vi.mocked(exportPdf).mockRejectedValue(new Error("Export failed"));

      const { result } = renderHook(() => useExport());

      await act(async () => {
        await result.current.handleExportPdf("prop-1", ["listing-1"]);
      });

      expect(toast.error).toHaveBeenCalledWith(
        "PDF export failed: Export failed",
      );
      expect(result.current.isExporting).toBe(false);
    });

    it("should set isExporting during export", async () => {
      let resolveExport: ((value: number[]) => void) | undefined;

      vi.mocked(exportPdf).mockImplementation(
        () =>
          new Promise<number[]>((resolve) => {
            resolveExport = resolve;
          }),
      );
      vi.mocked(save).mockImplementation(async () => "/path/to/file.pdf");
      vi.mocked(writeFile).mockImplementation(async () => {});

      const { result } = renderHook(() => useExport());

      act(() => {
        void result.current.handleExportPdf("prop-1", ["listing-1"]);
      });

      await waitFor(() => {
        expect(result.current.isExporting).toBe(true);
      });

      resolveExport?.([0x25, 0x50, 0x44, 0x46]);

      await waitFor(() => {
        expect(result.current.isExporting).toBe(false);
      });
    });
  });

  describe("handleExportDocx", () => {
    it("should export DOCX successfully", async () => {
      const mockBytes = new Uint8Array([0x50, 0x4b, 0x03, 0x04]); // ZIP header
      vi.mocked(exportDocx).mockResolvedValue(Array.from(mockBytes));
      vi.mocked(save).mockResolvedValue("/path/to/file.docx");
      vi.mocked(writeFile).mockResolvedValue();

      const { result } = renderHook(() => useExport());

      await act(async () => {
        await result.current.handleExportDocx(
          "prop-1",
          ["listing-1"],
          "luxury",
        );
      });

      expect(exportDocx).toHaveBeenCalledWith(
        "prop-1",
        ["listing-1"],
        "luxury",
      );
      expect(save).toHaveBeenCalledWith({
        defaultPath: "property-listing-luxury.docx",
        filters: [{ name: "Word Document", extensions: ["docx"] }],
      });
      expect(writeFile).toHaveBeenCalledWith(
        "/path/to/file.docx",
        expect.any(Uint8Array),
      );
      expect(toast.success).toHaveBeenCalledWith(
        "Luxury DOCX saved successfully",
      );
      expect(result.current.isExporting).toBe(false);
    });

    it("should handle user canceling file dialog", async () => {
      const mockBytes = new Uint8Array([0x50, 0x4b, 0x03, 0x04]);
      vi.mocked(exportDocx).mockResolvedValue(Array.from(mockBytes));
      vi.mocked(save).mockResolvedValue(null); // User canceled

      const { result } = renderHook(() => useExport());

      await act(async () => {
        await result.current.handleExportDocx("prop-1", ["listing-1"]);
      });

      expect(writeFile).not.toHaveBeenCalled();
      expect(toast.success).not.toHaveBeenCalled();
      expect(result.current.isExporting).toBe(false);
    });

    it("should handle export errors", async () => {
      vi.mocked(exportDocx).mockRejectedValue(new Error("Export failed"));

      const { result } = renderHook(() => useExport());

      await act(async () => {
        await result.current.handleExportDocx("prop-1", ["listing-1"]);
      });

      expect(toast.error).toHaveBeenCalledWith(
        "DOCX export failed: Export failed",
      );
      expect(result.current.isExporting).toBe(false);
    });

    it("should export multiple listings", async () => {
      const mockBytes = new Uint8Array([0x50, 0x4b, 0x03, 0x04]);
      vi.mocked(exportDocx).mockResolvedValue(Array.from(mockBytes));
      vi.mocked(save).mockResolvedValue("/path/to/file.docx");
      vi.mocked(writeFile).mockResolvedValue();

      const { result } = renderHook(() => useExport());

      await act(async () => {
        await result.current.handleExportDocx("prop-1", [
          "listing-1",
          "listing-2",
          "listing-3",
        ]);
      });

      expect(exportDocx).toHaveBeenCalledWith(
        "prop-1",
        ["listing-1", "listing-2", "listing-3"],
        "professional",
      );
    });
  });
});
