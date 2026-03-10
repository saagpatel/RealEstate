import { describe, it, expect, beforeEach, vi } from "vitest";
import { renderHook, act, waitFor } from "@testing-library/react";
import { useStreamingGeneration } from "@/hooks/useStreamingGeneration";
import { Channel } from "@tauri-apps/api/core";
import { generateListing } from "@/lib/tauri";
import { useGenerationStore } from "@/stores/generationStore";

vi.mock("@/lib/tauri");

describe("useStreamingGeneration", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useGenerationStore.setState({
      isGenerating: false,
      streamedText: "",
      error: null,
      generations: [],
      isLoadingHistory: false,
    });
  });

  it("should initialize with default state", () => {
    const { result } = renderHook(() => useStreamingGeneration());

    expect(result.current.isGenerating).toBe(false);
    expect(result.current.streamedText).toBe("");
    expect(result.current.error).toBeNull();
  });

  it("should start generation and set isGenerating", async () => {
    let channelInstance: any;

    vi.mocked(Channel).mockImplementation(function () {
      channelInstance = {
        onmessage: null,
        id: 0,
        __TAURI_CHANNEL_MARKER__: true,
        toJSON() {
          return `__CHANNEL__:${this.id}`;
        },
      };
      return channelInstance;
    } as any);

    vi.mocked(generateListing).mockImplementation(async (params, channel) => {
      // Simulate started event
      if (channel.onmessage) {
        channel.onmessage({ event: "started", data: { estimatedTokens: 500 } });
      }
    });

    const { result } = renderHook(() => useStreamingGeneration());

    await act(async () => {
      await result.current.generate({
        propertyId: "prop-1",
        style: "professional",
        tone: "warm",
        length: "medium",
        seoKeywords: [],
        brandVoiceId: null,
      });
    });

    expect(generateListing).toHaveBeenCalled();
  });

  it("should handle delta events and append text", async () => {
    let channelInstance: any;

    vi.mocked(Channel).mockImplementation(function () {
      channelInstance = {
        onmessage: null,
        id: 0,
        __TAURI_CHANNEL_MARKER__: true,
        toJSON() {
          return `__CHANNEL__:${this.id}`;
        },
      };
      return channelInstance;
    } as any);

    vi.mocked(generateListing).mockImplementation(async (params, channel) => {
      if (channel.onmessage) {
        channel.onmessage({ event: "delta", data: { text: "This " } });
        channel.onmessage({ event: "delta", data: { text: "is " } });
        channel.onmessage({ event: "delta", data: { text: "a test." } });
      }
    });

    const { result } = renderHook(() => useStreamingGeneration());

    await act(async () => {
      await result.current.generate({
        propertyId: "prop-1",
        style: "professional",
        tone: "warm",
        length: "medium",
        seoKeywords: [],
        brandVoiceId: null,
      });
    });

    await waitFor(() => {
      expect(result.current.streamedText).toBe("This is a test.");
    });
  });

  it("should handle finished event", async () => {
    let channelInstance: any;

    vi.mocked(Channel).mockImplementation(function () {
      channelInstance = {
        onmessage: null,
        id: 0,
        __TAURI_CHANNEL_MARKER__: true,
        toJSON() {
          return `__CHANNEL__:${this.id}`;
        },
      };
      return channelInstance;
    } as any);

    vi.mocked(generateListing).mockImplementation(async (params, channel) => {
      if (channel.onmessage) {
        channel.onmessage({
          event: "finished",
          data: { fullText: "Complete listing text." },
        });
      }
    });

    const { result } = renderHook(() => useStreamingGeneration());

    await act(async () => {
      await result.current.generate({
        propertyId: "prop-1",
        style: "professional",
        tone: "warm",
        length: "medium",
        seoKeywords: [],
        brandVoiceId: null,
      });
    });

    await waitFor(() => {
      expect(result.current.isGenerating).toBe(false);
      expect(result.current.streamedText).toBe("Complete listing text.");
    });
  });

  it("should handle error events", async () => {
    let channelInstance: any;

    vi.mocked(Channel).mockImplementation(function () {
      channelInstance = {
        onmessage: null,
        id: 0,
        __TAURI_CHANNEL_MARKER__: true,
        toJSON() {
          return `__CHANNEL__:${this.id}`;
        },
      };
      return channelInstance;
    } as any);

    vi.mocked(generateListing).mockImplementation(async (params, channel) => {
      if (channel.onmessage) {
        channel.onmessage({
          event: "error",
          data: { message: "API rate limit exceeded" },
        });
      }
    });

    const { result } = renderHook(() => useStreamingGeneration());

    await act(async () => {
      await result.current.generate({
        propertyId: "prop-1",
        style: "professional",
        tone: "warm",
        length: "medium",
        seoKeywords: [],
        brandVoiceId: null,
      });
    });

    await waitFor(() => {
      expect(result.current.isGenerating).toBe(false);
      expect(result.current.error).toBe("API rate limit exceeded");
    });
  });

  it("should handle generateListing errors", async () => {
    vi.mocked(Channel).mockImplementation(function () {
      return {
        onmessage: null,
        id: 0,
        __TAURI_CHANNEL_MARKER__: true,
        toJSON() {
          return `__CHANNEL__:${this.id}`;
        },
      };
    } as any);

    vi.mocked(generateListing).mockRejectedValue(
      new Error("Network connection failed"),
    );

    const { result } = renderHook(() => useStreamingGeneration());

    await act(async () => {
      await result.current.generate({
        propertyId: "prop-1",
        style: "professional",
        tone: "warm",
        length: "medium",
        seoKeywords: [],
        brandVoiceId: null,
      });
    });

    await waitFor(() => {
      expect(result.current.error).toBe("Network connection failed");
    });
  });

  it("should pass all parameters to generateListing", async () => {
    vi.mocked(Channel).mockImplementation(function () {
      return {
        onmessage: null,
        id: 0,
        __TAURI_CHANNEL_MARKER__: true,
        toJSON() {
          return `__CHANNEL__:${this.id}`;
        },
      };
    } as any);

    vi.mocked(generateListing).mockResolvedValue();

    const { result } = renderHook(() => useStreamingGeneration());

    await act(async () => {
      await result.current.generate({
        propertyId: "prop-123",
        style: "luxury",
        tone: "professional",
        length: "long",
        seoKeywords: ["downtown", "modern"],
        brandVoiceId: "voice-456",
      });
    });

    expect(generateListing).toHaveBeenCalledWith(
      expect.objectContaining({
        propertyId: "prop-123",
        style: "luxury",
        tone: "professional",
        length: "long",
        seoKeywords: ["downtown", "modern"],
        brandVoiceId: "voice-456",
      }),
      expect.any(Object),
    );
  });
});
