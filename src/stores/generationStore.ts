import { create } from "zustand";
import type { Listing } from "@/lib/types";
import {
  listListings,
  toggleListingFavorite,
  deleteListing,
} from "@/lib/tauri";

interface GenerationState {
  isGenerating: boolean;
  streamedText: string;
  error: string | null;
  generations: Listing[];
  isLoadingHistory: boolean;

  startGeneration: () => void;
  appendDelta: (text: string) => void;
  finishGeneration: (fullText: string) => void;
  setError: (message: string) => void;
  resetGeneration: () => void;

  loadGenerations: (propertyId: string) => Promise<void>;
  toggleFavorite: (id: string) => Promise<void>;
  deleteGeneration: (id: string) => Promise<void>;
}

export const useGenerationStore = create<GenerationState>((set) => ({
  isGenerating: false,
  streamedText: "",
  error: null,
  generations: [],
  isLoadingHistory: false,

  startGeneration: () =>
    set({ isGenerating: true, streamedText: "", error: null }),

  appendDelta: (text: string) =>
    set((state) => ({ streamedText: state.streamedText + text })),

  finishGeneration: (fullText: string) =>
    set({ isGenerating: false, streamedText: fullText }),

  setError: (message: string) => set({ isGenerating: false, error: message }),

  resetGeneration: () =>
    set({ isGenerating: false, streamedText: "", error: null }),

  loadGenerations: async (propertyId: string) => {
    set({ isLoadingHistory: true });
    try {
      const listings = await listListings(propertyId);
      // Filter to only listing-type generations
      set({
        generations: listings.filter((l) => l.generationType === "listing"),
      });
    } catch {
      // Silently fail — history is non-critical
    } finally {
      set({ isLoadingHistory: false });
    }
  },

  toggleFavorite: async (id: string) => {
    await toggleListingFavorite(id);
    set((state) => ({
      generations: state.generations.map((g) =>
        g.id === id ? { ...g, isFavorite: !g.isFavorite } : g,
      ),
    }));
  },

  deleteGeneration: async (id: string) => {
    await deleteListing(id);
    set((state) => ({
      generations: state.generations.filter((g) => g.id !== id),
    }));
  },
}));
