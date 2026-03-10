import { useState, useEffect, useCallback } from "react";
import { useParams } from "react-router-dom";
import { Sparkles, Loader2 } from "lucide-react";
import toast from "react-hot-toast";
import { StyleSelector } from "@/components/generate/StyleSelector";
import { ToneSelector } from "@/components/generate/ToneSelector";
import { LengthSelector } from "@/components/generate/LengthSelector";
import { KeywordInput } from "@/components/generate/KeywordInput";
import { StreamingOutput } from "@/components/generate/StreamingOutput";
import { GenerationHistory } from "@/components/generate/GenerationHistory";
import { BrandVoiceSelector } from "@/components/brand/BrandVoiceSelector";
import { useStreamingGeneration } from "@/hooks/useStreamingGeneration";
import { useGenerationStore } from "@/stores/generationStore";
import { useSettingsStore } from "@/stores/settingsStore";
import type {
  ListingStyle,
  ListingTone,
  ListingLength,
  Listing,
  BrandVoice,
} from "@/lib/types";
import { listBrandVoices } from "@/lib/tauri";

export function GenerateListing() {
  const { id: propertyId } = useParams<{ id: string }>();
  const { generate, isGenerating, streamedText, error } =
    useStreamingGeneration();
  const {
    generations,
    isLoadingHistory,
    loadGenerations,
    toggleFavorite,
    deleteGeneration,
  } = useGenerationStore();
  const resetGeneration = useGenerationStore((s) => s.resetGeneration);
  const apiKey = useSettingsStore((s) => s.apiKey);
  const defaultStyle = useSettingsStore((s) => s.defaultStyle);
  const defaultTone = useSettingsStore((s) => s.defaultTone);
  const defaultLength = useSettingsStore((s) => s.defaultLength);

  const [style, setStyle] = useState<ListingStyle>(defaultStyle);
  const [tone, setTone] = useState<ListingTone>(defaultTone);
  const [length, setLength] = useState<ListingLength>(defaultLength);
  const [seoKeywords, setSeoKeywords] = useState<string[]>([]);
  const [brandVoiceId, setBrandVoiceId] = useState<string | null>(null);
  const [brandVoices, setBrandVoices] = useState<BrandVoice[]>([]);

  useEffect(() => {
    if (propertyId) {
      void loadGenerations(propertyId);
    }
  }, [propertyId, loadGenerations]);

  useEffect(() => {
    listBrandVoices()
      .then(setBrandVoices)
      .catch(() => {
        // Non-critical
      });
  }, []);

  const handleGenerate = useCallback(async () => {
    if (!propertyId) return;
    if (!apiKey) {
      toast.error("Add your API key in Settings first.");
      return;
    }

    await generate({
      propertyId,
      style,
      tone,
      length,
      seoKeywords,
      brandVoiceId,
    });
  }, [
    propertyId,
    apiKey,
    style,
    tone,
    length,
    seoKeywords,
    brandVoiceId,
    generate,
  ]);

  const handleSelectHistory = useCallback(
    (listing: Listing) => {
      resetGeneration();
      useGenerationStore.getState().finishGeneration(listing.content);
    },
    [resetGeneration],
  );

  return (
    <div className="flex gap-6">
      {/* Left panel: Controls */}
      <div className="w-80 shrink-0 space-y-5">
        <StyleSelector
          value={style}
          onChange={setStyle}
          disabled={isGenerating}
        />
        <ToneSelector value={tone} onChange={setTone} disabled={isGenerating} />
        <LengthSelector
          value={length}
          onChange={setLength}
          disabled={isGenerating}
        />
        <KeywordInput
          value={seoKeywords}
          onChange={setSeoKeywords}
          disabled={isGenerating}
        />
        <BrandVoiceSelector
          voices={brandVoices}
          value={brandVoiceId}
          onChange={setBrandVoiceId}
          disabled={isGenerating}
        />

        <button
          onClick={handleGenerate}
          disabled={isGenerating || !apiKey}
          className="w-full flex items-center justify-center gap-2 px-4 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-medium text-sm"
        >
          {isGenerating ? (
            <>
              <Loader2 size={16} className="animate-spin" />
              Generating...
            </>
          ) : (
            <>
              <Sparkles size={16} />
              Generate Listing
            </>
          )}
        </button>

        {!apiKey && (
          <p className="text-xs text-amber-600">
            API key required. Go to Settings to add your Anthropic key.
          </p>
        )}
      </div>

      {/* Right panel: Output + History */}
      <div className="flex-1 space-y-6">
        <StreamingOutput
          text={streamedText}
          isGenerating={isGenerating}
          error={error}
          onRegenerate={handleGenerate}
        />

        <GenerationHistory
          generations={generations}
          isLoading={isLoadingHistory}
          onSelect={handleSelectHistory}
          onToggleFavorite={(id) => {
            void toggleFavorite(id);
          }}
          onDelete={(id) => {
            void deleteGeneration(id);
          }}
        />
      </div>
    </div>
  );
}
