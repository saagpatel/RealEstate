import { describe, it, expect, beforeEach, vi } from "vitest";
import { renderHook, act, waitFor } from "@testing-library/react";
import { usePhotos } from "@/hooks/usePhotos";
import * as tauri from "@/lib/tauri";
import type { Photo } from "@/lib/types";

vi.mock("@/lib/tauri");

describe("usePhotos", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should settle with empty state when no photos are returned", async () => {
    vi.mocked(tauri.listPhotos).mockResolvedValue([]);

    const { result } = renderHook(() => usePhotos("prop-1"));

    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    expect(result.current.photos).toEqual([]);
    expect(result.current.isImporting).toBe(false);
    expect(result.current.error).toBeNull();
  });

  it("should load photos on mount", async () => {
    const mockPhotos: Photo[] = [
      {
        id: "1",
        propertyId: "prop-1",
        filename: "front.jpg",
        thumbnailPath: "thumb_front.jpg",
        sortOrder: 0,
        caption: null,
        createdAt: "2024-01-01T00:00:00Z",
      },
      {
        id: "2",
        propertyId: "prop-1",
        filename: "kitchen.jpg",
        thumbnailPath: "thumb_kitchen.jpg",
        sortOrder: 1,
        caption: null,
        createdAt: "2024-01-02T00:00:00Z",
      },
    ];

    vi.mocked(tauri.listPhotos).mockResolvedValue(mockPhotos);

    const { result } = renderHook(() => usePhotos("prop-1"));

    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    expect(result.current.photos).toEqual(mockPhotos);
    expect(tauri.listPhotos).toHaveBeenCalledWith("prop-1");
  });

  it("should not load photos if propertyId is undefined", async () => {
    vi.mocked(tauri.listPhotos).mockResolvedValue([]);

    const { result } = renderHook(() => usePhotos(undefined));

    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    expect(tauri.listPhotos).not.toHaveBeenCalled();
    expect(result.current.photos).toEqual([]);
  });

  it("should handle load errors", async () => {
    vi.mocked(tauri.listPhotos).mockRejectedValue(new Error("Load failed"));

    const { result } = renderHook(() => usePhotos("prop-1"));

    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    expect(result.current.error).toBe("Load failed");
    expect(result.current.photos).toEqual([]);
  });

  it("should import photos", async () => {
    const existingPhotos: Photo[] = [
      {
        id: "1",
        propertyId: "prop-1",
        filename: "existing.jpg",
        thumbnailPath: "thumb_existing.jpg",
        sortOrder: 0,
        caption: null,
        createdAt: "2024-01-01T00:00:00Z",
      },
    ];

    const newPhotos: Photo[] = [
      {
        id: "2",
        propertyId: "prop-1",
        filename: "new.jpg",
        thumbnailPath: "thumb_new.jpg",
        sortOrder: 1,
        caption: null,
        createdAt: "2024-01-02T00:00:00Z",
      },
    ];

    vi.mocked(tauri.listPhotos).mockResolvedValue(existingPhotos);
    vi.mocked(tauri.importPhotos).mockResolvedValue(newPhotos);

    const { result } = renderHook(() => usePhotos("prop-1"));

    await waitFor(() => {
      expect(result.current.photos).toEqual(existingPhotos);
    });

    await act(async () => {
      await result.current.importPhotos();
    });

    expect(tauri.importPhotos).toHaveBeenCalledWith("prop-1");
    expect(result.current.photos).toHaveLength(2);
    expect(result.current.photos[1].id).toBe("2");
    expect(result.current.isImporting).toBe(false);
  });

  it("should handle import errors", async () => {
    vi.mocked(tauri.listPhotos).mockResolvedValue([]);
    vi.mocked(tauri.importPhotos).mockRejectedValue(new Error("Import failed"));

    const { result } = renderHook(() => usePhotos("prop-1"));

    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    await act(async () => {
      await result.current.importPhotos();
    });

    expect(result.current.error).toBe("Import failed");
    expect(result.current.isImporting).toBe(false);
  });

  it("should delete photo", async () => {
    const mockPhotos: Photo[] = [
      {
        id: "1",
        propertyId: "prop-1",
        filename: "photo1.jpg",
        thumbnailPath: "thumb1.jpg",
        sortOrder: 0,
        caption: null,
        createdAt: "2024-01-01T00:00:00Z",
      },
      {
        id: "2",
        propertyId: "prop-1",
        filename: "photo2.jpg",
        thumbnailPath: "thumb2.jpg",
        sortOrder: 1,
        caption: null,
        createdAt: "2024-01-02T00:00:00Z",
      },
    ];

    vi.mocked(tauri.listPhotos).mockResolvedValue(mockPhotos);
    vi.mocked(tauri.deletePhoto).mockResolvedValue();

    const { result } = renderHook(() => usePhotos("prop-1"));

    await waitFor(() => {
      expect(result.current.photos).toHaveLength(2);
    });

    await act(async () => {
      await result.current.deletePhoto("1");
    });

    expect(tauri.deletePhoto).toHaveBeenCalledWith("1");
    expect(result.current.photos).toHaveLength(1);
    expect(result.current.photos[0].id).toBe("2");
  });

  it("should handle delete errors", async () => {
    vi.mocked(tauri.listPhotos).mockResolvedValue([
      {
        id: "1",
        propertyId: "prop-1",
        filename: "photo.jpg",
        thumbnailPath: "thumb.jpg",
        sortOrder: 0,
        caption: null,
        createdAt: "2024-01-01T00:00:00Z",
      },
    ]);
    vi.mocked(tauri.deletePhoto).mockRejectedValue(new Error("Delete failed"));

    const { result } = renderHook(() => usePhotos("prop-1"));

    await waitFor(() => {
      expect(result.current.photos).toHaveLength(1);
    });

    await act(async () => {
      await result.current.deletePhoto("1");
    });

    expect(result.current.error).toBe("Delete failed");
    expect(result.current.photos).toHaveLength(1); // Not deleted
  });

  it("should reorder photos optimistically", async () => {
    const mockPhotos: Photo[] = [
      {
        id: "1",
        propertyId: "prop-1",
        filename: "photo1.jpg",
        thumbnailPath: "thumb1.jpg",
        sortOrder: 0,
        caption: null,
        createdAt: "2024-01-01T00:00:00Z",
      },
      {
        id: "2",
        propertyId: "prop-1",
        filename: "photo2.jpg",
        thumbnailPath: "thumb2.jpg",
        sortOrder: 1,
        caption: null,
        createdAt: "2024-01-02T00:00:00Z",
      },
      {
        id: "3",
        propertyId: "prop-1",
        filename: "photo3.jpg",
        thumbnailPath: "thumb3.jpg",
        sortOrder: 2,
        caption: null,
        createdAt: "2024-01-03T00:00:00Z",
      },
    ];

    vi.mocked(tauri.listPhotos).mockResolvedValue(mockPhotos);
    vi.mocked(tauri.reorderPhotos).mockResolvedValue();

    const { result } = renderHook(() => usePhotos("prop-1"));

    await waitFor(() => {
      expect(result.current.photos).toHaveLength(3);
    });

    // Reorder: [1, 2, 3] → [3, 1, 2]
    await act(async () => {
      await result.current.reorderPhotos(["3", "1", "2"]);
    });

    expect(tauri.reorderPhotos).toHaveBeenCalledWith("prop-1", ["3", "1", "2"]);
    expect(result.current.photos[0].id).toBe("3");
    expect(result.current.photos[1].id).toBe("1");
    expect(result.current.photos[2].id).toBe("2");
  });

  it("should rollback on reorder failure", async () => {
    const mockPhotos: Photo[] = [
      {
        id: "1",
        propertyId: "prop-1",
        filename: "photo1.jpg",
        thumbnailPath: "thumb1.jpg",
        sortOrder: 0,
        caption: null,
        createdAt: "2024-01-01T00:00:00Z",
      },
      {
        id: "2",
        propertyId: "prop-1",
        filename: "photo2.jpg",
        thumbnailPath: "thumb2.jpg",
        sortOrder: 1,
        caption: null,
        createdAt: "2024-01-02T00:00:00Z",
      },
    ];

    vi.mocked(tauri.listPhotos).mockResolvedValue(mockPhotos);
    vi.mocked(tauri.reorderPhotos).mockRejectedValue(
      new Error("Reorder failed"),
    );

    const { result } = renderHook(() => usePhotos("prop-1"));

    await waitFor(() => {
      expect(result.current.photos).toHaveLength(2);
    });

    const originalOrder = result.current.photos.map((p) => p.id);

    await act(async () => {
      await result.current.reorderPhotos(["2", "1"]);
    });

    // Should rollback to original order
    await waitFor(() => {
      expect(result.current.photos.map((p) => p.id)).toEqual(originalOrder);
    });

    expect(tauri.listPhotos).toHaveBeenCalledTimes(2);
    expect(result.current.error).toBeNull();
  });
});
