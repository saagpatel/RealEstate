import { create } from "zustand";
import type { BrandVoice } from "../lib/types";
import * as api from "../lib/tauri";

interface BrandVoiceState {
  voices: BrandVoice[];
  isLoading: boolean;
  isCreating: boolean;
  error: string | null;
  fetchVoices: () => Promise<void>;
  createVoice: (
    name: string,
    description: string | null,
    sampleListings: string[],
  ) => Promise<void>;
  deleteVoice: (id: string) => Promise<void>;
}

export const useBrandVoiceStore = create<BrandVoiceState>((set) => ({
  voices: [],
  isLoading: false,
  isCreating: false,
  error: null,

  fetchVoices: async () => {
    set({ isLoading: true, error: null });
    try {
      const voices = await api.listBrandVoices();
      set({ voices, isLoading: false });
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      set({ error: message, isLoading: false });
    }
  },

  createVoice: async (name, description, sampleListings) => {
    set({ isCreating: true, error: null });
    try {
      await api.createBrandVoice(name, description, sampleListings);
      // Refresh the list
      const voices = await api.listBrandVoices();
      set({ voices, isCreating: false });
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      set({ error: message, isCreating: false });
      throw err; // Re-throw so the component can handle it
    }
  },

  deleteVoice: async (id) => {
    try {
      await api.deleteBrandVoice(id);
      set((state) => ({
        voices: state.voices.filter((v) => v.id !== id),
      }));
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      set({ error: message });
    }
  },
}));
