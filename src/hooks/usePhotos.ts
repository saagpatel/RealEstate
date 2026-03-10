import { useState, useCallback, useEffect } from "react";
import type { Photo } from "@/lib/types";
import * as tauri from "@/lib/tauri";

interface UsePhotosReturn {
  photos: Photo[];
  isLoading: boolean;
  isImporting: boolean;
  isReordering: boolean;
  error: string | null;
  loadPhotos: () => Promise<void>;
  importPhotos: () => Promise<void>;
  deletePhoto: (id: string) => Promise<void>;
  reorderPhotos: (photoIds: string[]) => Promise<void>;
}

export function usePhotos(propertyId: string | undefined): UsePhotosReturn {
  const [photos, setPhotos] = useState<Photo[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [isImporting, setIsImporting] = useState(false);
  const [isReordering, setIsReordering] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadPhotos = useCallback(async () => {
    if (!propertyId) return;
    setIsLoading(true);
    setError(null);
    try {
      const result = await tauri.listPhotos(propertyId);
      setPhotos(result);
    } catch (err: unknown) {
      const message =
        err instanceof Error ? err.message : "Failed to load photos";
      setError(message);
    } finally {
      setIsLoading(false);
    }
  }, [propertyId]);

  const importPhotos = useCallback(async () => {
    if (!propertyId) return;
    setIsImporting(true);
    setError(null);
    try {
      const newPhotos = await tauri.importPhotos(propertyId);
      if (newPhotos.length > 0) {
        setPhotos((prev) => [...prev, ...newPhotos]);
      }
    } catch (err: unknown) {
      const message =
        err instanceof Error ? err.message : "Failed to import photos";
      setError(message);
    } finally {
      setIsImporting(false);
    }
  }, [propertyId]);

  const deletePhoto = useCallback(async (id: string) => {
    setError(null);
    try {
      await tauri.deletePhoto(id);
      setPhotos((prev) => prev.filter((p) => p.id !== id));
    } catch (err: unknown) {
      const message =
        err instanceof Error ? err.message : "Failed to delete photo";
      setError(message);
    }
  }, []);

  const reorderPhotos = useCallback(
    async (photoIds: string[]) => {
      if (!propertyId) return;
      setError(null);
      setIsReordering(true);

      // Optimistic update
      const reordered = photoIds
        .map((id) => photos.find((p) => p.id === id))
        .filter((p): p is Photo => p !== undefined)
        .map((p, i) => ({ ...p, sortOrder: i }));
      setPhotos(reordered);

      try {
        await tauri.reorderPhotos(propertyId, photoIds);
      } catch (err: unknown) {
        const message =
          err instanceof Error ? err.message : "Failed to reorder photos";
        setError(message);
        // Reload on failure to restore correct order
        await loadPhotos();
      } finally {
        setIsReordering(false);
      }
    },
    [propertyId, photos, loadPhotos],
  );

  useEffect(() => {
    void loadPhotos();
  }, [loadPhotos]);

  return {
    photos,
    isLoading,
    isImporting,
    isReordering,
    error,
    loadPhotos,
    importPhotos,
    deletePhoto,
    reorderPhotos,
  };
}
