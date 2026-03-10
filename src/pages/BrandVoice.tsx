import { useEffect } from "react";
import { Mic } from "lucide-react";
import toast from "react-hot-toast";
import { PageHeader } from "@/components/layout/PageHeader";
import { BrandVoiceUploader } from "@/components/brand/BrandVoiceUploader";
import { BrandVoicePreview } from "@/components/brand/BrandVoicePreview";
import { useBrandVoiceStore } from "@/stores/brandVoiceStore";

export function BrandVoice() {
  const {
    voices,
    isLoading,
    isCreating,
    fetchVoices,
    createVoice,
    deleteVoice,
  } = useBrandVoiceStore();

  useEffect(() => {
    void fetchVoices();
  }, [fetchVoices]);

  const handleCreate = async (
    name: string,
    description: string | null,
    samples: string[],
  ) => {
    try {
      await createVoice(name, description, samples);
      toast.success("Brand voice profile created!");
    } catch {
      toast.error("Failed to create voice profile. Check your API key.");
    }
  };

  const handleDelete = (id: string) => {
    void deleteVoice(id);
    toast.success("Voice profile deleted");
  };

  return (
    <div>
      <PageHeader
        title="Brand Voice"
        subtitle="Train the AI to match your writing style"
      />

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Left: Create new */}
        <BrandVoiceUploader onSubmit={handleCreate} isCreating={isCreating} />

        {/* Right: Existing voices */}
        <div className="space-y-4">
          <h3 className="font-semibold text-gray-900">Your Voice Profiles</h3>

          {isLoading && (
            <div className="bg-gray-50 rounded-lg border border-gray-200 p-6 text-center">
              <p className="text-sm text-gray-500">Loading...</p>
            </div>
          )}

          {!isLoading && voices.length === 0 && (
            <div className="bg-gray-50 rounded-lg border border-gray-200 p-8 text-center">
              <Mic className="w-8 h-8 text-gray-300 mx-auto mb-2" />
              <p className="text-sm text-gray-500">No voice profiles yet</p>
              <p className="text-xs text-gray-400 mt-1">
                Create one by pasting your past listing descriptions
              </p>
            </div>
          )}

          {voices.map((voice) => (
            <BrandVoicePreview
              key={voice.id}
              voice={voice}
              onDelete={handleDelete}
            />
          ))}
        </div>
      </div>
    </div>
  );
}
