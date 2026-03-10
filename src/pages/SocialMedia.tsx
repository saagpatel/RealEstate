import { useState, useEffect, useCallback } from "react";
import { useParams } from "react-router-dom";
import { Sparkles, Loader2 } from "lucide-react";
import { Channel } from "@tauri-apps/api/core";
import toast from "react-hot-toast";
import { PlatformTabs } from "@/components/social/PlatformTabs";
import { SocialPostCard } from "@/components/social/SocialPostCard";
import { BrandVoiceSelector } from "@/components/brand/BrandVoiceSelector";
import { StreamingOutput } from "@/components/generate/StreamingOutput";
import { generateSocial, listBrandVoices } from "@/lib/tauri";
import { useSettingsStore } from "@/stores/settingsStore";
import type { SocialPlatform, BrandVoice, StreamEvent } from "@/lib/types";

function parsePosts(text: string): string[] {
  if (!text.trim()) return [];
  // Split by ---POST N--- delimiter
  const parts = text.split(/---POST\s+\d+---/i).filter((p) => p.trim());
  return parts.map((p) => p.trim());
}

export function SocialMedia() {
  const { id: propertyId } = useParams<{ id: string }>();
  const apiKey = useSettingsStore((s) => s.apiKey);

  const [platform, setPlatform] = useState<SocialPlatform>("instagram");
  const [brandVoiceId, setBrandVoiceId] = useState<string | null>(null);
  const [brandVoices, setBrandVoices] = useState<BrandVoice[]>([]);

  const [isGenerating, setIsGenerating] = useState(false);
  const [streamedText, setStreamedText] = useState("");
  const [finishedText, setFinishedText] = useState("");
  const [error, setError] = useState<string | null>(null);

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

    setIsGenerating(true);
    setStreamedText("");
    setFinishedText("");
    setError(null);

    const channel = new Channel<StreamEvent>();
    channel.onmessage = (event: StreamEvent) => {
      switch (event.event) {
        case "delta":
          setStreamedText((prev) => prev + event.data.text);
          break;
        case "finished":
          setFinishedText(event.data.fullText);
          setIsGenerating(false);
          break;
        case "error":
          setError(event.data.message);
          setIsGenerating(false);
          break;
        case "started":
          break;
      }
    };

    try {
      await generateSocial(
        {
          propertyId,
          platform,
          brandVoiceId,
        },
        channel,
      );
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      setIsGenerating(false);
    }
  }, [propertyId, apiKey, platform, brandVoiceId]);

  const posts = finishedText ? parsePosts(finishedText) : [];
  const showCards = !isGenerating && posts.length > 0;

  return (
    <div className="space-y-6">
      {/* Controls */}
      <div className="bg-white rounded-lg border border-gray-200">
        <PlatformTabs
          value={platform}
          onChange={setPlatform}
          disabled={isGenerating}
        />
        <div className="p-4 flex items-end gap-4">
          <div className="flex-1 max-w-xs">
            <BrandVoiceSelector
              voices={brandVoices}
              value={brandVoiceId}
              onChange={setBrandVoiceId}
              disabled={isGenerating}
            />
          </div>
          <button
            onClick={handleGenerate}
            disabled={isGenerating || !apiKey}
            className="flex items-center gap-2 px-4 py-2.5 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-medium text-sm"
          >
            {isGenerating ? (
              <>
                <Loader2 size={16} className="animate-spin" />
                Generating...
              </>
            ) : (
              <>
                <Sparkles size={16} />
                Generate Posts
              </>
            )}
          </button>
        </div>
        {!apiKey && (
          <p className="px-4 pb-3 text-xs text-amber-600">
            API key required. Go to Settings to add your Anthropic key.
          </p>
        )}
      </div>

      {/* Streaming output while generating */}
      {isGenerating && (
        <StreamingOutput
          text={streamedText}
          isGenerating={isGenerating}
          error={error}
        />
      )}

      {/* Error state */}
      {error && !isGenerating && (
        <StreamingOutput
          text=""
          isGenerating={false}
          error={error}
          onRegenerate={handleGenerate}
        />
      )}

      {/* Individual post cards when finished */}
      {showCards && (
        <div className="space-y-4">
          {posts.map((post, i) => (
            <SocialPostCard
              key={i}
              postNumber={i + 1}
              content={post}
              platform={platform}
            />
          ))}
        </div>
      )}

      {/* Empty state */}
      {!isGenerating && !finishedText && !error && (
        <div className="rounded-lg border border-gray-200 bg-gray-50 p-8 text-center">
          <p className="text-sm text-gray-500">
            Select a platform and click Generate to create social media posts
            for this property.
          </p>
        </div>
      )}
    </div>
  );
}
