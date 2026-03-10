import { describe, it, expect, beforeEach, vi } from "vitest";
import { useGenerationStore } from "@/stores/generationStore";
import {
  listListings,
  toggleListingFavorite,
  deleteListing,
} from "@/lib/tauri";
import type { Listing } from "@/lib/types";

vi.mock("@/lib/tauri");

describe("generationStore", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Reset store state
    useGenerationStore.setState({
      isGenerating: false,
      streamedText: "",
      error: null,
      generations: [],
      isLoadingHistory: false,
    });
  });

  it("should initialize with default state", () => {
    const store = useGenerationStore.getState();
    expect(store.isGenerating).toBe(false);
    expect(store.streamedText).toBe("");
    expect(store.error).toBeNull();
    expect(store.generations).toEqual([]);
    expect(store.isLoadingHistory).toBe(false);
  });

  describe("startGeneration", () => {
    it("should set isGenerating to true and reset state", () => {
      useGenerationStore.setState({
        streamedText: "old text",
        error: "old error",
      });

      const { startGeneration } = useGenerationStore.getState();
      startGeneration();

      const state = useGenerationStore.getState();
      expect(state.isGenerating).toBe(true);
      expect(state.streamedText).toBe("");
      expect(state.error).toBeNull();
    });
  });

  describe("appendDelta", () => {
    it("should append text to streamedText", () => {
      const { appendDelta } = useGenerationStore.getState();
      appendDelta("This ");
      appendDelta("is ");
      appendDelta("a ");
      appendDelta("test.");

      const state = useGenerationStore.getState();
      expect(state.streamedText).toBe("This is a test.");
    });

    it("should work with empty initial state", () => {
      const { appendDelta } = useGenerationStore.getState();
      appendDelta("Hello");

      const state = useGenerationStore.getState();
      expect(state.streamedText).toBe("Hello");
    });
  });

  describe("finishGeneration", () => {
    it("should set final text and stop generating", () => {
      useGenerationStore.setState({
        isGenerating: true,
        streamedText: "partial text",
      });

      const { finishGeneration } = useGenerationStore.getState();
      finishGeneration("Complete text here.");

      const state = useGenerationStore.getState();
      expect(state.isGenerating).toBe(false);
      expect(state.streamedText).toBe("Complete text here.");
    });
  });

  describe("setError", () => {
    it("should set error and stop generating", () => {
      useGenerationStore.setState({ isGenerating: true });

      const { setError } = useGenerationStore.getState();
      setError("API rate limit exceeded");

      const state = useGenerationStore.getState();
      expect(state.isGenerating).toBe(false);
      expect(state.error).toBe("API rate limit exceeded");
    });
  });

  describe("resetGeneration", () => {
    it("should reset all generation state", () => {
      useGenerationStore.setState({
        isGenerating: true,
        streamedText: "some text",
        error: "some error",
      });

      const { resetGeneration } = useGenerationStore.getState();
      resetGeneration();

      const state = useGenerationStore.getState();
      expect(state.isGenerating).toBe(false);
      expect(state.streamedText).toBe("");
      expect(state.error).toBeNull();
    });
  });

  describe("loadGenerations", () => {
    it("should load listing-type generations for property", async () => {
      const mockListings: Listing[] = [
        {
          id: "1",
          propertyId: "prop-1",
          content: "Listing 1",
          generationType: "listing",
          style: "professional",
          tone: "warm",
          length: "medium",
          seoKeywords: [],
          tokensUsed: 450,
          generationCostCents: 5,
          isFavorite: false,
          createdAt: "2024-01-01T00:00:00Z",
        },
        {
          id: "2",
          propertyId: "prop-1",
          content: "Social post",
          generationType: "social",
          style: "casual",
          tone: "friendly",
          length: "short",
          seoKeywords: [],
          tokensUsed: 200,
          generationCostCents: 2,
          isFavorite: false,
          createdAt: "2024-01-02T00:00:00Z",
        },
        {
          id: "3",
          propertyId: "prop-1",
          content: "Listing 2",
          generationType: "listing",
          style: "luxury",
          tone: "professional",
          length: "long",
          seoKeywords: [],
          tokensUsed: 600,
          generationCostCents: 8,
          isFavorite: true,
          createdAt: "2024-01-03T00:00:00Z",
        },
      ];

      vi.mocked(listListings).mockResolvedValue(mockListings);

      const { loadGenerations } = useGenerationStore.getState();
      await loadGenerations("prop-1");

      expect(listListings).toHaveBeenCalledWith("prop-1");

      const state = useGenerationStore.getState();
      expect(state.isLoadingHistory).toBe(false);
      expect(state.generations).toHaveLength(2); // Only listing-type
      expect(state.generations[0].id).toBe("1");
      expect(state.generations[1].id).toBe("3");
    });

    it("should set isLoadingHistory during load", async () => {
      let loadingDuringFetch = false;

      vi.mocked(listListings).mockImplementation(async () => {
        loadingDuringFetch = useGenerationStore.getState().isLoadingHistory;
        return [];
      });

      const { loadGenerations } = useGenerationStore.getState();
      await loadGenerations("prop-1");

      expect(loadingDuringFetch).toBe(true);
      expect(useGenerationStore.getState().isLoadingHistory).toBe(false);
    });

    it("should handle load errors gracefully", async () => {
      vi.mocked(listListings).mockRejectedValue(new Error("Network error"));

      const { loadGenerations } = useGenerationStore.getState();
      await loadGenerations("prop-1");

      const state = useGenerationStore.getState();
      expect(state.isLoadingHistory).toBe(false);
      expect(state.generations).toEqual([]);
    });
  });

  describe("toggleFavorite", () => {
    it("should toggle favorite status", async () => {
      const mockGenerations: Listing[] = [
        {
          id: "1",
          propertyId: "prop-1",
          content: "Listing",
          generationType: "listing",
          style: "professional",
          tone: "warm",
          length: "medium",
          seoKeywords: [],
          tokensUsed: 450,
          generationCostCents: 5,
          isFavorite: false,
          createdAt: "2024-01-01T00:00:00Z",
        },
      ];

      useGenerationStore.setState({ generations: mockGenerations });

      vi.mocked(toggleListingFavorite).mockResolvedValue();

      const { toggleFavorite } = useGenerationStore.getState();
      await toggleFavorite("1");

      expect(toggleListingFavorite).toHaveBeenCalledWith("1");

      const state = useGenerationStore.getState();
      expect(state.generations[0].isFavorite).toBe(true);
    });

    it("should toggle favorite back to false", async () => {
      const mockGenerations: Listing[] = [
        {
          id: "1",
          propertyId: "prop-1",
          content: "Listing",
          generationType: "listing",
          style: "professional",
          tone: "warm",
          length: "medium",
          seoKeywords: [],
          tokensUsed: 450,
          generationCostCents: 5,
          isFavorite: true,
          createdAt: "2024-01-01T00:00:00Z",
        },
      ];

      useGenerationStore.setState({ generations: mockGenerations });

      vi.mocked(toggleListingFavorite).mockResolvedValue();

      const { toggleFavorite } = useGenerationStore.getState();
      await toggleFavorite("1");

      const state = useGenerationStore.getState();
      expect(state.generations[0].isFavorite).toBe(false);
    });

    it("should only toggle the specified generation", async () => {
      const mockGenerations: Listing[] = [
        {
          id: "1",
          propertyId: "prop-1",
          content: "Listing 1",
          generationType: "listing",
          style: "professional",
          tone: "warm",
          length: "medium",
          seoKeywords: [],
          tokensUsed: 450,
          generationCostCents: 5,
          isFavorite: false,
          createdAt: "2024-01-01T00:00:00Z",
        },
        {
          id: "2",
          propertyId: "prop-1",
          content: "Listing 2",
          generationType: "listing",
          style: "luxury",
          tone: "professional",
          length: "long",
          seoKeywords: [],
          tokensUsed: 600,
          generationCostCents: 8,
          isFavorite: true,
          createdAt: "2024-01-02T00:00:00Z",
        },
      ];

      useGenerationStore.setState({ generations: mockGenerations });

      vi.mocked(toggleListingFavorite).mockResolvedValue();

      const { toggleFavorite } = useGenerationStore.getState();
      await toggleFavorite("1");

      const state = useGenerationStore.getState();
      expect(state.generations[0].isFavorite).toBe(true); // Toggled
      expect(state.generations[1].isFavorite).toBe(true); // Unchanged
    });
  });

  describe("deleteGeneration", () => {
    it("should delete generation from list", async () => {
      const mockGenerations: Listing[] = [
        {
          id: "1",
          propertyId: "prop-1",
          content: "Listing 1",
          generationType: "listing",
          style: "professional",
          tone: "warm",
          length: "medium",
          seoKeywords: [],
          tokensUsed: 450,
          generationCostCents: 5,
          isFavorite: false,
          createdAt: "2024-01-01T00:00:00Z",
        },
        {
          id: "2",
          propertyId: "prop-1",
          content: "Listing 2",
          generationType: "listing",
          style: "luxury",
          tone: "professional",
          length: "long",
          seoKeywords: [],
          tokensUsed: 600,
          generationCostCents: 8,
          isFavorite: true,
          createdAt: "2024-01-02T00:00:00Z",
        },
      ];

      useGenerationStore.setState({ generations: mockGenerations });

      vi.mocked(deleteListing).mockResolvedValue();

      const { deleteGeneration } = useGenerationStore.getState();
      await deleteGeneration("1");

      expect(deleteListing).toHaveBeenCalledWith("1");

      const state = useGenerationStore.getState();
      expect(state.generations).toHaveLength(1);
      expect(state.generations[0].id).toBe("2");
    });
  });

  describe("generation flow", () => {
    it("should handle complete generation flow", () => {
      const { startGeneration, appendDelta, finishGeneration } =
        useGenerationStore.getState();

      // Start
      startGeneration();
      let state = useGenerationStore.getState();
      expect(state.isGenerating).toBe(true);
      expect(state.streamedText).toBe("");

      // Stream chunks
      appendDelta("This is ");
      appendDelta("a beautiful ");
      appendDelta("property.");

      state = useGenerationStore.getState();
      expect(state.streamedText).toBe("This is a beautiful property.");

      // Finish
      finishGeneration("This is a beautiful property.");

      state = useGenerationStore.getState();
      expect(state.isGenerating).toBe(false);
      expect(state.streamedText).toBe("This is a beautiful property.");
    });

    it("should handle error during generation", () => {
      const { startGeneration, appendDelta, setError } =
        useGenerationStore.getState();

      startGeneration();
      appendDelta("Partial text...");

      setError("API rate limit exceeded");

      const state = useGenerationStore.getState();
      expect(state.isGenerating).toBe(false);
      expect(state.error).toBe("API rate limit exceeded");
    });
  });
});
