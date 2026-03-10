import { create } from "zustand";
import { getSetting, setSetting } from "@/lib/tauri";
import {
  AI_MODELS,
  DEFAULT_AI_MODEL,
  LISTING_LENGTHS,
  LISTING_STYLES,
  LISTING_TONES,
  SETTING_KEYS,
} from "@/lib/constants";
import type { ListingStyle, ListingTone, ListingLength } from "@/lib/types";

type SettingKey = (typeof SETTING_KEYS)[keyof typeof SETTING_KEYS];

interface SettingsState {
  apiKey: string;
  aiModel: string;
  agentName: string;
  agentPhone: string;
  agentEmail: string;
  brokerageName: string;
  defaultStyle: ListingStyle;
  defaultTone: ListingTone;
  defaultLength: ListingLength;
  isLoaded: boolean;
  loadSettings: () => Promise<void>;
  saveSetting: (key: SettingKey, value: string) => Promise<void>;
}

const keyToStateField: Record<
  SettingKey,
  keyof Omit<SettingsState, "isLoaded" | "loadSettings" | "saveSetting">
> = {
  [SETTING_KEYS.API_KEY]: "apiKey",
  [SETTING_KEYS.AI_MODEL]: "aiModel",
  [SETTING_KEYS.AGENT_NAME]: "agentName",
  [SETTING_KEYS.AGENT_PHONE]: "agentPhone",
  [SETTING_KEYS.AGENT_EMAIL]: "agentEmail",
  [SETTING_KEYS.BROKERAGE_NAME]: "brokerageName",
  [SETTING_KEYS.DEFAULT_STYLE]: "defaultStyle",
  [SETTING_KEYS.DEFAULT_TONE]: "defaultTone",
  [SETTING_KEYS.DEFAULT_LENGTH]: "defaultLength",
};

const ALL_KEYS = Object.values(SETTING_KEYS);
const styleValues = new Set<string>(LISTING_STYLES.map((style) => style.value));
const toneValues = new Set<string>(LISTING_TONES.map((tone) => tone.value));
const lengthValues = new Set<string>(
  LISTING_LENGTHS.map((length) => length.value),
);
const modelValues = new Set<string>(AI_MODELS.map((model) => model.value));

function normalizeSettingValue(key: SettingKey, value: string): string {
  if (key === SETTING_KEYS.DEFAULT_STYLE) {
    return styleValues.has(value as ListingStyle) ? value : "luxury";
  }

  if (key === SETTING_KEYS.DEFAULT_TONE) {
    return toneValues.has(value as ListingTone) ? value : "warm";
  }

  if (key === SETTING_KEYS.DEFAULT_LENGTH) {
    return lengthValues.has(value as ListingLength) ? value : "medium";
  }

  if (key === SETTING_KEYS.AI_MODEL) {
    return modelValues.has(value) ? value : DEFAULT_AI_MODEL;
  }

  return value;
}

export const useSettingsStore = create<SettingsState>((set) => ({
  apiKey: "",
  aiModel: DEFAULT_AI_MODEL,
  agentName: "",
  agentPhone: "",
  agentEmail: "",
  brokerageName: "",
  defaultStyle: "luxury",
  defaultTone: "warm",
  defaultLength: "medium",
  isLoaded: false,

  loadSettings: async () => {
    const results = await Promise.all(
      ALL_KEYS.map(async (key) => {
        try {
          const value = await getSetting(key);
          return [key, value] as const;
        } catch {
          return [key, ""] as const;
        }
      }),
    );

    const updates: Record<string, string> = {};
    for (const [key, value] of results) {
      const field = keyToStateField[key];
      updates[field] = normalizeSettingValue(key, value);
    }

    set({ ...updates, isLoaded: true });
  },

  saveSetting: async (key: SettingKey, value: string) => {
    await setSetting(key, value);
    const field = keyToStateField[key];
    set({ [field]: value });
  },
}));
