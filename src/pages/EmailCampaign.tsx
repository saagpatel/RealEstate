import { useState, useEffect, useCallback } from "react";
import { useParams } from "react-router-dom";
import { Sparkles, Loader2 } from "lucide-react";
import toast from "react-hot-toast";
import { Channel } from "@tauri-apps/api/core";
import { TemplateSelector } from "@/components/email/TemplateSelector";
import { EmailPreview } from "@/components/email/EmailPreview";
import { BrandVoiceSelector } from "@/components/brand/BrandVoiceSelector";
import { StreamingOutput } from "@/components/generate/StreamingOutput";
import { useGenerationStore } from "@/stores/generationStore";
import { useSettingsStore } from "@/stores/settingsStore";
import { generateEmail, listBrandVoices } from "@/lib/tauri";
import type { EmailTemplate, BrandVoice, StreamEvent } from "@/lib/types";

export function EmailCampaign() {
  const { id: propertyId } = useParams<{ id: string }>();
  const apiKey = useSettingsStore((s) => s.apiKey);

  const { isGenerating, streamedText, error } = useGenerationStore();
  const startGeneration = useGenerationStore((s) => s.startGeneration);
  const appendDelta = useGenerationStore((s) => s.appendDelta);
  const finishGeneration = useGenerationStore((s) => s.finishGeneration);
  const setError = useGenerationStore((s) => s.setError);
  const resetGeneration = useGenerationStore((s) => s.resetGeneration);

  const [templateType, setTemplateType] = useState<EmailTemplate>("buyer");
  const [brandVoiceId, setBrandVoiceId] = useState<string | null>(null);
  const [brandVoices, setBrandVoices] = useState<BrandVoice[]>([]);
  const [isComplete, setIsComplete] = useState(false);

  useEffect(() => {
    listBrandVoices()
      .then(setBrandVoices)
      .catch(() => {
        // Non-critical
      });
  }, []);

  useEffect(() => {
    // Reset completion state when generation starts
    if (isGenerating) {
      setIsComplete(false);
    }
  }, [isGenerating]);

  const handleGenerate = useCallback(async () => {
    if (!propertyId) return;
    if (!apiKey) {
      toast.error("Add your API key in Settings first.");
      return;
    }

    startGeneration();
    setIsComplete(false);

    const channel = new Channel<StreamEvent>();
    channel.onmessage = (event: StreamEvent) => {
      switch (event.event) {
        case "delta":
          appendDelta(event.data.text);
          break;
        case "finished":
          finishGeneration(event.data.fullText);
          setIsComplete(true);
          break;
        case "error":
          setError(event.data.message);
          break;
        case "started":
          break;
      }
    };

    try {
      await generateEmail(
        {
          propertyId,
          templateType,
          brandVoiceId,
        },
        channel,
      );
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
    }
  }, [
    propertyId,
    apiKey,
    templateType,
    brandVoiceId,
    startGeneration,
    appendDelta,
    finishGeneration,
    setError,
  ]);

  // Reset when switching templates
  useEffect(() => {
    resetGeneration();
    setIsComplete(false);
  }, [templateType, resetGeneration]);

  return (
    <div className="flex gap-6">
      {/* Left panel: Controls */}
      <div className="w-80 shrink-0 space-y-5">
        <TemplateSelector
          value={templateType}
          onChange={setTemplateType}
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
              Generating Email...
            </>
          ) : (
            <>
              <Sparkles size={16} />
              Generate Email
            </>
          )}
        </button>

        {!apiKey && (
          <p className="text-xs text-amber-600">
            API key required. Go to Settings to add your Anthropic key.
          </p>
        )}
      </div>

      {/* Right panel: Output */}
      <div className="flex-1 space-y-6">
        {isComplete && streamedText ? (
          <EmailPreview text={streamedText} />
        ) : (
          <StreamingOutput
            text={streamedText}
            isGenerating={isGenerating}
            error={error}
            onRegenerate={handleGenerate}
          />
        )}
      </div>
    </div>
  );
}
