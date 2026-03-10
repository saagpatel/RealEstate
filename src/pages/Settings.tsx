import { useEffect, useState, useCallback } from "react";
import toast from "react-hot-toast";
import {
  Eye,
  EyeOff,
  Save,
  Key,
  User,
  Phone,
  Mail,
  Building2,
  Palette,
  Loader2,
  Sparkles,
} from "lucide-react";
import { PageHeader } from "@/components/layout/PageHeader";
import { useSettingsStore } from "@/stores/settingsStore";
import {
  AI_MODELS,
  LISTING_STYLES,
  LISTING_TONES,
  LISTING_LENGTHS,
  SETTING_KEYS,
} from "@/lib/constants";
import type { ListingStyle, ListingTone, ListingLength } from "@/lib/types";

function isValidApiKey(key: string): boolean {
  return key === "" || key.startsWith("sk-ant-");
}

interface SettingsFormState {
  apiKey: string;
  aiModel: string;
  agentName: string;
  agentPhone: string;
  agentEmail: string;
  brokerageName: string;
  defaultStyle: ListingStyle;
  defaultTone: ListingTone;
  defaultLength: ListingLength;
}

export function Settings() {
  const {
    apiKey,
    aiModel,
    agentName,
    agentPhone,
    agentEmail,
    brokerageName,
    defaultStyle,
    defaultTone,
    defaultLength,
    isLoaded,
    loadSettings,
    saveSetting,
  } = useSettingsStore();

  const [showApiKey, setShowApiKey] = useState(false);
  const [isSaving, setIsSaving] = useState(false);

  // Local form state so edits don't immediately update global state
  const [form, setForm] = useState<SettingsFormState>({
    apiKey: "",
    aiModel: AI_MODELS[0].value,
    agentName: "",
    agentPhone: "",
    agentEmail: "",
    brokerageName: "",
    defaultStyle: "luxury" as ListingStyle,
    defaultTone: "warm" as ListingTone,
    defaultLength: "medium" as ListingLength,
  });

  // Sync form state when store loads
  useEffect(() => {
    if (isLoaded) {
      setForm({
        apiKey,
        aiModel,
        agentName,
        agentPhone,
        agentEmail,
        brokerageName,
        defaultStyle,
        defaultTone,
        defaultLength,
      });
    }
  }, [
    isLoaded,
    apiKey,
    aiModel,
    agentName,
    agentPhone,
    agentEmail,
    brokerageName,
    defaultStyle,
    defaultTone,
    defaultLength,
  ]);

  useEffect(() => {
    if (!isLoaded) {
      loadSettings();
    }
  }, [isLoaded, loadSettings]);

  const updateField = useCallback(
    <K extends keyof SettingsFormState>(
      field: K,
      value: SettingsFormState[K],
    ) => {
      setForm((prev) => ({ ...prev, [field]: value }));
    },
    [],
  );

  const handleSave = async () => {
    if (!isValidApiKey(form.apiKey)) {
      toast.error('API key must start with "sk-ant-" or be empty');
      return;
    }

    setIsSaving(true);
    try {
      await Promise.all([
        saveSetting(SETTING_KEYS.API_KEY, form.apiKey),
        saveSetting(SETTING_KEYS.AI_MODEL, form.aiModel),
        saveSetting(SETTING_KEYS.AGENT_NAME, form.agentName),
        saveSetting(SETTING_KEYS.AGENT_PHONE, form.agentPhone),
        saveSetting(SETTING_KEYS.AGENT_EMAIL, form.agentEmail),
        saveSetting(SETTING_KEYS.BROKERAGE_NAME, form.brokerageName),
        saveSetting(SETTING_KEYS.DEFAULT_STYLE, form.defaultStyle),
        saveSetting(SETTING_KEYS.DEFAULT_TONE, form.defaultTone),
        saveSetting(SETTING_KEYS.DEFAULT_LENGTH, form.defaultLength),
      ]);
      toast.success("Settings saved");
    } catch (err) {
      const message =
        err instanceof Error ? err.message : "Failed to save settings";
      toast.error(message);
    } finally {
      setIsSaving(false);
    }
  };

  if (!isLoaded) {
    return (
      <div className="flex items-center justify-center h-64">
        <Loader2 className="h-8 w-8 animate-spin text-blue-600" />
      </div>
    );
  }

  return (
    <div>
      <PageHeader
        title="Settings"
        subtitle="Configure generation defaults, AI model choice, and agent profile details"
        actions={
          <button
            onClick={handleSave}
            disabled={isSaving}
            className="inline-flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isSaving ? (
              <Loader2 className="h-4 w-4 animate-spin" />
            ) : (
              <Save className="h-4 w-4" />
            )}
            Save Settings
          </button>
        }
      />

      <div className="space-y-6">
        {/* API Key Section */}
        <section className="bg-white rounded-lg border border-gray-200 p-6">
          <div className="flex items-center gap-2 mb-4">
            <Key className="h-5 w-5 text-blue-600" />
            <h3 className="text-lg font-semibold text-gray-900">
              Anthropic API Key
            </h3>
          </div>
          <p className="text-sm text-gray-500 mb-4">
            Required for AI-powered listing generation. Get your key from{" "}
            <span className="font-medium text-gray-700">
              console.anthropic.com
            </span>
            .
          </p>
          <div className="relative max-w-lg">
            <input
              type={showApiKey ? "text" : "password"}
              value={form.apiKey}
              onChange={(e) => updateField("apiKey", e.target.value)}
              placeholder="sk-ant-..."
              className={`w-full rounded-lg border px-4 py-2 pr-10 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 ${
                form.apiKey && !isValidApiKey(form.apiKey)
                  ? "border-red-300 bg-red-50"
                  : "border-gray-300"
              }`}
            />
            <button
              type="button"
              onClick={() => setShowApiKey(!showApiKey)}
              className="absolute right-2 top-1/2 -translate-y-1/2 p-1 text-gray-400 hover:text-gray-600"
            >
              {showApiKey ? (
                <EyeOff className="h-4 w-4" />
              ) : (
                <Eye className="h-4 w-4" />
              )}
            </button>
          </div>
          {form.apiKey && !isValidApiKey(form.apiKey) && (
            <p className="mt-2 text-sm text-red-600">
              API key must start with &quot;sk-ant-&quot;
            </p>
          )}
        </section>

        <section className="bg-white rounded-lg border border-gray-200 p-6">
          <div className="flex items-center gap-2 mb-4">
            <Sparkles className="h-5 w-5 text-blue-600" />
            <h3 className="text-lg font-semibold text-gray-900">AI Model</h3>
          </div>
          <p className="text-sm text-gray-500 mb-4">
            Choose which Claude model powers listing, social, email, and brand
            voice generation.
          </p>
          <div className="max-w-2xl space-y-3">
            {AI_MODELS.map((model) => {
              const isSelected = form.aiModel === model.value;
              return (
                <label
                  key={model.value}
                  className={`flex cursor-pointer items-start gap-3 rounded-xl border p-4 transition-colors ${
                    isSelected
                      ? "border-blue-300 bg-blue-50"
                      : "border-gray-200 hover:border-gray-300 hover:bg-gray-50"
                  }`}
                >
                  <input
                    type="radio"
                    name="ai-model"
                    value={model.value}
                    checked={isSelected}
                    onChange={(e) => updateField("aiModel", e.target.value)}
                    className="mt-1 h-4 w-4 text-blue-600 focus:ring-blue-500"
                  />
                  <div>
                    <div className="text-sm font-medium text-gray-900">
                      {model.label}
                    </div>
                    <div className="text-sm text-gray-500">
                      {model.description}
                    </div>
                    <div className="mt-1 text-xs text-gray-400">
                      {model.value}
                    </div>
                  </div>
                </label>
              );
            })}
          </div>
        </section>

        {/* Agent Profile Section */}
        <section className="bg-white rounded-lg border border-gray-200 p-6">
          <div className="flex items-center gap-2 mb-4">
            <User className="h-5 w-5 text-blue-600" />
            <h3 className="text-lg font-semibold text-gray-900">
              Agent Profile
            </h3>
          </div>
          <p className="text-sm text-gray-500 mb-4">
            Your contact information will be included in generated listings and
            marketing materials.
          </p>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 max-w-2xl">
            <div>
              <label className="mb-1 block text-sm font-medium text-gray-700">
                <span className="inline-flex items-center gap-1">
                  <User className="h-3.5 w-3.5" />
                  Full Name
                </span>
              </label>
              <input
                type="text"
                value={form.agentName}
                onChange={(e) => updateField("agentName", e.target.value)}
                placeholder="Jane Smith"
                className="w-full rounded-lg border border-gray-300 px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <div>
              <label className="mb-1 block text-sm font-medium text-gray-700">
                <span className="inline-flex items-center gap-1">
                  <Phone className="h-3.5 w-3.5" />
                  Phone
                </span>
              </label>
              <input
                type="tel"
                value={form.agentPhone}
                onChange={(e) => updateField("agentPhone", e.target.value)}
                placeholder="(555) 123-4567"
                className="w-full rounded-lg border border-gray-300 px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <div>
              <label className="mb-1 block text-sm font-medium text-gray-700">
                <span className="inline-flex items-center gap-1">
                  <Mail className="h-3.5 w-3.5" />
                  Email
                </span>
              </label>
              <input
                type="email"
                value={form.agentEmail}
                onChange={(e) => updateField("agentEmail", e.target.value)}
                placeholder="jane@realty.com"
                className="w-full rounded-lg border border-gray-300 px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <div>
              <label className="mb-1 block text-sm font-medium text-gray-700">
                <span className="inline-flex items-center gap-1">
                  <Building2 className="h-3.5 w-3.5" />
                  Brokerage
                </span>
              </label>
              <input
                type="text"
                value={form.brokerageName}
                onChange={(e) => updateField("brokerageName", e.target.value)}
                placeholder="Premier Realty Group"
                className="w-full rounded-lg border border-gray-300 px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
          </div>
        </section>

        {/* Defaults Section */}
        <section className="bg-white rounded-lg border border-gray-200 p-6">
          <div className="flex items-center gap-2 mb-4">
            <Palette className="h-5 w-5 text-blue-600" />
            <h3 className="text-lg font-semibold text-gray-900">
              Default Generation Preferences
            </h3>
          </div>
          <p className="text-sm text-gray-500 mb-4">
            Pre-selected options when generating new listings. You can override
            these per listing.
          </p>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 max-w-3xl">
            <div>
              <label className="mb-1 block text-sm font-medium text-gray-700">
                Style
              </label>
              <select
                value={form.defaultStyle}
                onChange={(e) =>
                  updateField("defaultStyle", e.target.value as ListingStyle)
                }
                className="w-full rounded-lg border border-gray-300 px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                {LISTING_STYLES.map((s) => (
                  <option key={s.value} value={s.value}>
                    {s.label}
                  </option>
                ))}
              </select>
              <p className="mt-1 text-xs text-gray-400">
                {
                  LISTING_STYLES.find((s) => s.value === form.defaultStyle)
                    ?.description
                }
              </p>
            </div>
            <div>
              <label className="mb-1 block text-sm font-medium text-gray-700">
                Tone
              </label>
              <select
                value={form.defaultTone}
                onChange={(e) =>
                  updateField("defaultTone", e.target.value as ListingTone)
                }
                className="w-full rounded-lg border border-gray-300 px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                {LISTING_TONES.map((t) => (
                  <option key={t.value} value={t.value}>
                    {t.label}
                  </option>
                ))}
              </select>
              <p className="mt-1 text-xs text-gray-400">
                {
                  LISTING_TONES.find((t) => t.value === form.defaultTone)
                    ?.description
                }
              </p>
            </div>
            <div>
              <label className="mb-1 block text-sm font-medium text-gray-700">
                Length
              </label>
              <select
                value={form.defaultLength}
                onChange={(e) =>
                  updateField("defaultLength", e.target.value as ListingLength)
                }
                className="w-full rounded-lg border border-gray-300 px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                {LISTING_LENGTHS.map((l) => (
                  <option key={l.value} value={l.value}>
                    {l.label}
                  </option>
                ))}
              </select>
              <p className="mt-1 text-xs text-gray-400">
                {
                  LISTING_LENGTHS.find((l) => l.value === form.defaultLength)
                    ?.description
                }
              </p>
            </div>
          </div>
        </section>
      </div>
    </div>
  );
}
