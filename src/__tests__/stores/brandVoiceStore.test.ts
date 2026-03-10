import { describe, it, expect, beforeEach, vi } from "vitest";
import { useBrandVoiceStore } from "@/stores/brandVoiceStore";
import * as api from "@/lib/tauri";
import type { BrandVoice } from "@/lib/types";

vi.mock("@/lib/tauri");

describe("brandVoiceStore", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Reset store state
    useBrandVoiceStore.setState({
      voices: [],
      isLoading: false,
      isCreating: false,
      error: null,
    });
  });

  it("should initialize with default state", () => {
    const store = useBrandVoiceStore.getState();
    expect(store.voices).toEqual([]);
    expect(store.isLoading).toBe(false);
    expect(store.isCreating).toBe(false);
    expect(store.error).toBeNull();
  });

  describe("fetchVoices", () => {
    it("should fetch and set brand voices", async () => {
      const mockVoices: BrandVoice[] = [
        {
          id: "1",
          name: "Luxury Voice",
          description: "For upscale properties",
          extractedStyle: "Sophisticated and elegant",
          sampleCount: 3,
          createdAt: "2024-01-01T00:00:00Z",
        },
        {
          id: "2",
          name: "Budget Voice",
          description: "For affordable homes",
          extractedStyle: "Friendly and approachable",
          sampleCount: 2,
          createdAt: "2024-01-02T00:00:00Z",
        },
      ];

      vi.mocked(api.listBrandVoices).mockResolvedValue(mockVoices);

      const { fetchVoices } = useBrandVoiceStore.getState();
      await fetchVoices();

      const state = useBrandVoiceStore.getState();
      expect(state.voices).toEqual(mockVoices);
      expect(state.isLoading).toBe(false);
      expect(state.error).toBeNull();
      expect(api.listBrandVoices).toHaveBeenCalledTimes(1);
    });

    it("should set isLoading during fetch", async () => {
      let loadingDuringFetch = false;

      vi.mocked(api.listBrandVoices).mockImplementation(async () => {
        loadingDuringFetch = useBrandVoiceStore.getState().isLoading;
        return [];
      });

      const { fetchVoices } = useBrandVoiceStore.getState();
      await fetchVoices();

      expect(loadingDuringFetch).toBe(true);
      expect(useBrandVoiceStore.getState().isLoading).toBe(false);
    });

    it("should handle fetch errors", async () => {
      const error = new Error("Network error");
      vi.mocked(api.listBrandVoices).mockRejectedValue(error);

      const { fetchVoices } = useBrandVoiceStore.getState();
      await fetchVoices();

      const state = useBrandVoiceStore.getState();
      expect(state.isLoading).toBe(false);
      expect(state.error).toBe("Network error");
      expect(state.voices).toEqual([]);
    });

    it("should clear previous error on successful fetch", async () => {
      useBrandVoiceStore.setState({ error: "Previous error" });

      vi.mocked(api.listBrandVoices).mockResolvedValue([]);

      const { fetchVoices } = useBrandVoiceStore.getState();
      await fetchVoices();

      const state = useBrandVoiceStore.getState();
      expect(state.error).toBeNull();
    });
  });

  describe("createVoice", () => {
    it("should create brand voice and refresh list", async () => {
      const newVoice: BrandVoice = {
        id: "3",
        name: "Modern Voice",
        description: "For contemporary homes",
        extractedStyle: "Clean and minimalist",
        sampleCount: 2,
        createdAt: "2024-01-03T00:00:00Z",
      };

      vi.mocked(api.createBrandVoice).mockResolvedValue(newVoice);
      vi.mocked(api.listBrandVoices).mockResolvedValue([newVoice]);

      const { createVoice } = useBrandVoiceStore.getState();
      await createVoice("Modern Voice", "For contemporary homes", [
        "Sample 1",
        "Sample 2",
      ]);

      expect(api.createBrandVoice).toHaveBeenCalledWith(
        "Modern Voice",
        "For contemporary homes",
        ["Sample 1", "Sample 2"],
      );
      expect(api.listBrandVoices).toHaveBeenCalledTimes(1);

      const state = useBrandVoiceStore.getState();
      expect(state.voices).toEqual([newVoice]);
      expect(state.isCreating).toBe(false);
      expect(state.error).toBeNull();
    });

    it("should set isCreating during creation", async () => {
      let creatingDuringFetch = false;

      vi.mocked(api.createBrandVoice).mockImplementation(async () => {
        creatingDuringFetch = useBrandVoiceStore.getState().isCreating;
        return {
          id: "1",
          name: "Test",
          description: null,
          extractedStyle: null,
          sampleCount: 0,
          createdAt: "",
        };
      });
      vi.mocked(api.listBrandVoices).mockResolvedValue([]);

      const { createVoice } = useBrandVoiceStore.getState();
      await createVoice("Test", null, []);

      expect(creatingDuringFetch).toBe(true);
      expect(useBrandVoiceStore.getState().isCreating).toBe(false);
    });

    it("should handle creation errors and re-throw", async () => {
      const error = new Error("Creation failed");
      vi.mocked(api.createBrandVoice).mockRejectedValue(error);

      const { createVoice } = useBrandVoiceStore.getState();

      await expect(createVoice("Test", null, ["Sample"])).rejects.toThrow(
        "Creation failed",
      );

      const state = useBrandVoiceStore.getState();
      expect(state.isCreating).toBe(false);
      expect(state.error).toBe("Creation failed");
    });

    it("should clear previous error on successful creation", async () => {
      useBrandVoiceStore.setState({ error: "Previous error" });

      vi.mocked(api.createBrandVoice).mockResolvedValue({
        id: "1",
        name: "Test",
        description: null,
        extractedStyle: null,
        sampleCount: 0,
        createdAt: "",
      });
      vi.mocked(api.listBrandVoices).mockResolvedValue([]);

      const { createVoice } = useBrandVoiceStore.getState();
      await createVoice("Test", null, []);

      const state = useBrandVoiceStore.getState();
      expect(state.error).toBeNull();
    });

    it("should create voice with null description", async () => {
      vi.mocked(api.createBrandVoice).mockResolvedValue({
        id: "1",
        name: "Test",
        description: null,
        extractedStyle: null,
        sampleCount: 1,
        createdAt: "",
      });
      vi.mocked(api.listBrandVoices).mockResolvedValue([]);

      const { createVoice } = useBrandVoiceStore.getState();
      await createVoice("Test", null, ["Sample"]);

      expect(api.createBrandVoice).toHaveBeenCalledWith("Test", null, [
        "Sample",
      ]);
    });
  });

  describe("deleteVoice", () => {
    it("should delete voice from list", async () => {
      const mockVoices: BrandVoice[] = [
        {
          id: "1",
          name: "Voice 1",
          description: null,
          extractedStyle: null,
          sampleCount: 2,
          createdAt: "2024-01-01T00:00:00Z",
        },
        {
          id: "2",
          name: "Voice 2",
          description: null,
          extractedStyle: null,
          sampleCount: 3,
          createdAt: "2024-01-02T00:00:00Z",
        },
      ];

      useBrandVoiceStore.setState({ voices: mockVoices });

      vi.mocked(api.deleteBrandVoice).mockResolvedValue();

      const { deleteVoice } = useBrandVoiceStore.getState();
      await deleteVoice("1");

      expect(api.deleteBrandVoice).toHaveBeenCalledWith("1");

      const state = useBrandVoiceStore.getState();
      expect(state.voices).toHaveLength(1);
      expect(state.voices[0].id).toBe("2");
    });

    it("should handle deletion errors", async () => {
      const mockVoices: BrandVoice[] = [
        {
          id: "1",
          name: "Voice 1",
          description: null,
          extractedStyle: null,
          sampleCount: 2,
          createdAt: "2024-01-01T00:00:00Z",
        },
      ];

      useBrandVoiceStore.setState({ voices: mockVoices });

      const error = new Error("Deletion failed");
      vi.mocked(api.deleteBrandVoice).mockRejectedValue(error);

      const { deleteVoice } = useBrandVoiceStore.getState();
      await deleteVoice("1");

      const state = useBrandVoiceStore.getState();
      expect(state.error).toBe("Deletion failed");
      expect(state.voices).toHaveLength(1); // Not deleted
    });

    it("should delete multiple voices sequentially", async () => {
      const mockVoices: BrandVoice[] = [
        {
          id: "1",
          name: "Voice 1",
          description: null,
          extractedStyle: null,
          sampleCount: 2,
          createdAt: "2024-01-01T00:00:00Z",
        },
        {
          id: "2",
          name: "Voice 2",
          description: null,
          extractedStyle: null,
          sampleCount: 3,
          createdAt: "2024-01-02T00:00:00Z",
        },
        {
          id: "3",
          name: "Voice 3",
          description: null,
          extractedStyle: null,
          sampleCount: 4,
          createdAt: "2024-01-03T00:00:00Z",
        },
      ];

      useBrandVoiceStore.setState({ voices: mockVoices });

      vi.mocked(api.deleteBrandVoice).mockResolvedValue();

      const { deleteVoice } = useBrandVoiceStore.getState();
      await deleteVoice("1");
      await deleteVoice("3");

      const state = useBrandVoiceStore.getState();
      expect(state.voices).toHaveLength(1);
      expect(state.voices[0].id).toBe("2");
    });
  });

  describe("integration", () => {
    it("should fetch, create, and delete voices", async () => {
      // Initial fetch
      vi.mocked(api.listBrandVoices).mockResolvedValue([]);

      const { fetchVoices, createVoice, deleteVoice } =
        useBrandVoiceStore.getState();
      await fetchVoices();

      let state = useBrandVoiceStore.getState();
      expect(state.voices).toEqual([]);

      // Create voice
      const newVoice: BrandVoice = {
        id: "1",
        name: "New Voice",
        description: "Test",
        extractedStyle: null,
        sampleCount: 1,
        createdAt: "2024-01-01T00:00:00Z",
      };

      vi.mocked(api.createBrandVoice).mockResolvedValue(newVoice);
      vi.mocked(api.listBrandVoices).mockResolvedValue([newVoice]);

      await createVoice("New Voice", "Test", ["Sample"]);

      state = useBrandVoiceStore.getState();
      expect(state.voices).toHaveLength(1);

      // Delete voice
      vi.mocked(api.deleteBrandVoice).mockResolvedValue();

      await deleteVoice("1");

      state = useBrandVoiceStore.getState();
      expect(state.voices).toHaveLength(0);
    });
  });
});
