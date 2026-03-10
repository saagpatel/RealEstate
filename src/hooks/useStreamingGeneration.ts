import { useCallback } from "react";
import { Channel } from "@tauri-apps/api/core";
import { generateListing } from "@/lib/tauri";
import { useGenerationStore } from "@/stores/generationStore";
import type {
  StreamEvent,
  ListingStyle,
  ListingTone,
  ListingLength,
} from "@/lib/types";

interface GenerateListingParams {
  propertyId: string;
  style: ListingStyle;
  tone: ListingTone;
  length: ListingLength;
  seoKeywords: string[];
  brandVoiceId: string | null;
}

export function useStreamingGeneration() {
  const { isGenerating, streamedText, error } = useGenerationStore();
  const startGeneration = useGenerationStore((s) => s.startGeneration);
  const appendDelta = useGenerationStore((s) => s.appendDelta);
  const finishGeneration = useGenerationStore((s) => s.finishGeneration);
  const setError = useGenerationStore((s) => s.setError);
  const loadGenerations = useGenerationStore((s) => s.loadGenerations);

  const generate = useCallback(
    async (params: GenerateListingParams) => {
      startGeneration();

      const channel = new Channel<StreamEvent>();
      channel.onmessage = (event: StreamEvent) => {
        switch (event.event) {
          case "delta":
            appendDelta(event.data.text);
            break;
          case "finished":
            finishGeneration(event.data.fullText);
            // Reload history to include the new generation
            void loadGenerations(params.propertyId);
            break;
          case "error":
            setError(event.data.message);
            break;
          case "started":
            // Could use estimatedTokens for progress, but not needed for MVP
            break;
        }
      };

      try {
        await generateListing(
          {
            propertyId: params.propertyId,
            style: params.style,
            tone: params.tone,
            length: params.length,
            seoKeywords: params.seoKeywords,
            brandVoiceId: params.brandVoiceId,
          },
          channel,
        );
      } catch (err: unknown) {
        const message = err instanceof Error ? err.message : String(err);
        setError(message);
      }
    },
    [startGeneration, appendDelta, finishGeneration, setError, loadGenerations],
  );

  return { generate, isGenerating, streamedText, error };
}
