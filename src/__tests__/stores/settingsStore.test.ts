import { describe, it, expect, beforeEach, vi } from "vitest";
import { useSettingsStore } from "@/stores/settingsStore";
import { getSetting, setSetting } from "@/lib/tauri";
import { SETTING_KEYS } from "@/lib/constants";

vi.mock("@/lib/tauri");

describe("settingsStore", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Reset store state
    useSettingsStore.setState({
      apiKey: "",
      aiModel: "claude-sonnet-4-20250514",
      agentName: "",
      agentPhone: "",
      agentEmail: "",
      brokerageName: "",
      defaultStyle: "luxury",
      defaultTone: "warm",
      defaultLength: "medium",
      isLoaded: false,
    });
  });

  it("should initialize with default values", () => {
    const store = useSettingsStore.getState();
    expect(store.apiKey).toBe("");
    expect(store.aiModel).toBe("claude-sonnet-4-20250514");
    expect(store.agentName).toBe("");
    expect(store.agentPhone).toBe("");
    expect(store.agentEmail).toBe("");
    expect(store.brokerageName).toBe("");
    expect(store.defaultStyle).toBe("luxury");
    expect(store.defaultTone).toBe("warm");
    expect(store.defaultLength).toBe("medium");
    expect(store.isLoaded).toBe(false);
  });

  describe("loadSettings", () => {
    it("should load all settings from backend", async () => {
      const mockSettings = {
        [SETTING_KEYS.API_KEY]: "sk-ant-test-123",
        [SETTING_KEYS.AI_MODEL]: "claude-3-5-haiku-latest",
        [SETTING_KEYS.AGENT_NAME]: "John Doe",
        [SETTING_KEYS.AGENT_PHONE]: "555-1234",
        [SETTING_KEYS.AGENT_EMAIL]: "john@example.com",
        [SETTING_KEYS.BROKERAGE_NAME]: "Realty Co",
        [SETTING_KEYS.DEFAULT_STYLE]: "family",
        [SETTING_KEYS.DEFAULT_TONE]: "warm",
        [SETTING_KEYS.DEFAULT_LENGTH]: "short",
      };

      vi.mocked(getSetting).mockImplementation(async (key) => {
        return mockSettings[key as keyof typeof mockSettings] || "";
      });

      const { loadSettings } = useSettingsStore.getState();
      await loadSettings();

      const state = useSettingsStore.getState();
      expect(state.apiKey).toBe("sk-ant-test-123");
      expect(state.aiModel).toBe("claude-3-5-haiku-latest");
      expect(state.agentName).toBe("John Doe");
      expect(state.agentPhone).toBe("555-1234");
      expect(state.agentEmail).toBe("john@example.com");
      expect(state.brokerageName).toBe("Realty Co");
      expect(state.defaultStyle).toBe("family");
      expect(state.defaultTone).toBe("warm");
      expect(state.defaultLength).toBe("short");
      expect(state.isLoaded).toBe(true);
    });

    it("should handle missing settings gracefully", async () => {
      vi.mocked(getSetting).mockRejectedValue(new Error("Setting not found"));

      const { loadSettings } = useSettingsStore.getState();
      await loadSettings();

      const state = useSettingsStore.getState();
      expect(state.apiKey).toBe("");
      expect(state.aiModel).toBe("claude-sonnet-4-20250514");
      expect(state.agentName).toBe("");
      expect(state.isLoaded).toBe(true);
    });

    it("should load partial settings when some fail", async () => {
      vi.mocked(getSetting).mockImplementation(async (key) => {
        if (key === SETTING_KEYS.API_KEY) {
          return "sk-ant-partial-key";
        }
        if (key === SETTING_KEYS.AGENT_NAME) {
          return "Jane Smith";
        }
        throw new Error("Setting not found");
      });

      const { loadSettings } = useSettingsStore.getState();
      await loadSettings();

      const state = useSettingsStore.getState();
      expect(state.apiKey).toBe("sk-ant-partial-key");
      expect(state.agentName).toBe("Jane Smith");
      expect(state.agentPhone).toBe(""); // Failed, so empty
      expect(state.isLoaded).toBe(true);
    });
  });

  describe("saveSetting", () => {
    it("should save API key setting", async () => {
      vi.mocked(setSetting).mockResolvedValue();

      const { saveSetting } = useSettingsStore.getState();
      await saveSetting(SETTING_KEYS.API_KEY, "sk-ant-new-key");

      expect(setSetting).toHaveBeenCalledWith(
        SETTING_KEYS.API_KEY,
        "sk-ant-new-key",
      );

      const state = useSettingsStore.getState();
      expect(state.apiKey).toBe("sk-ant-new-key");
    });

    it("should save AI model setting", async () => {
      vi.mocked(setSetting).mockResolvedValue();

      const { saveSetting } = useSettingsStore.getState();
      await saveSetting(SETTING_KEYS.AI_MODEL, "claude-3-5-haiku-latest");

      const state = useSettingsStore.getState();
      expect(state.aiModel).toBe("claude-3-5-haiku-latest");
    });

    it("should save agent name setting", async () => {
      vi.mocked(setSetting).mockResolvedValue();

      const { saveSetting } = useSettingsStore.getState();
      await saveSetting(SETTING_KEYS.AGENT_NAME, "Alice Johnson");

      expect(setSetting).toHaveBeenCalledWith(
        SETTING_KEYS.AGENT_NAME,
        "Alice Johnson",
      );

      const state = useSettingsStore.getState();
      expect(state.agentName).toBe("Alice Johnson");
    });

    it("should save agent phone setting", async () => {
      vi.mocked(setSetting).mockResolvedValue();

      const { saveSetting } = useSettingsStore.getState();
      await saveSetting(SETTING_KEYS.AGENT_PHONE, "555-9876");

      const state = useSettingsStore.getState();
      expect(state.agentPhone).toBe("555-9876");
    });

    it("should save agent email setting", async () => {
      vi.mocked(setSetting).mockResolvedValue();

      const { saveSetting } = useSettingsStore.getState();
      await saveSetting(SETTING_KEYS.AGENT_EMAIL, "alice@realty.com");

      const state = useSettingsStore.getState();
      expect(state.agentEmail).toBe("alice@realty.com");
    });

    it("should save brokerage name setting", async () => {
      vi.mocked(setSetting).mockResolvedValue();

      const { saveSetting } = useSettingsStore.getState();
      await saveSetting(SETTING_KEYS.BROKERAGE_NAME, "Elite Realty");

      const state = useSettingsStore.getState();
      expect(state.brokerageName).toBe("Elite Realty");
    });

    it("should save default style setting", async () => {
      vi.mocked(setSetting).mockResolvedValue();

      const { saveSetting } = useSettingsStore.getState();
      await saveSetting(SETTING_KEYS.DEFAULT_STYLE, "casual");

      const state = useSettingsStore.getState();
      expect(state.defaultStyle).toBe("casual");
    });

    it("should save default tone setting", async () => {
      vi.mocked(setSetting).mockResolvedValue();

      const { saveSetting } = useSettingsStore.getState();
      await saveSetting(SETTING_KEYS.DEFAULT_TONE, "professional");

      const state = useSettingsStore.getState();
      expect(state.defaultTone).toBe("professional");
    });

    it("should save default length setting", async () => {
      vi.mocked(setSetting).mockResolvedValue();

      const { saveSetting } = useSettingsStore.getState();
      await saveSetting(SETTING_KEYS.DEFAULT_LENGTH, "long");

      const state = useSettingsStore.getState();
      expect(state.defaultLength).toBe("long");
    });

    it("should save multiple settings sequentially", async () => {
      vi.mocked(setSetting).mockResolvedValue();

      const { saveSetting } = useSettingsStore.getState();
      await saveSetting(SETTING_KEYS.API_KEY, "sk-ant-multi-1");
      await saveSetting(SETTING_KEYS.AGENT_NAME, "Multi Test");
      await saveSetting(SETTING_KEYS.DEFAULT_STYLE, "luxury");

      const state = useSettingsStore.getState();
      expect(state.apiKey).toBe("sk-ant-multi-1");
      expect(state.agentName).toBe("Multi Test");
      expect(state.defaultStyle).toBe("luxury");
    });
  });

  describe("integration", () => {
    it("should load and then update settings", async () => {
      // Initial load
      vi.mocked(getSetting).mockImplementation(async (key) => {
        if (key === SETTING_KEYS.API_KEY) return "sk-ant-initial";
        if (key === SETTING_KEYS.AGENT_NAME) return "Initial Name";
        return "";
      });

      const { loadSettings, saveSetting } = useSettingsStore.getState();
      await loadSettings();

      let state = useSettingsStore.getState();
      expect(state.apiKey).toBe("sk-ant-initial");
      expect(state.agentName).toBe("Initial Name");

      // Update setting
      vi.mocked(setSetting).mockResolvedValue();
      await saveSetting(SETTING_KEYS.API_KEY, "sk-ant-updated");

      state = useSettingsStore.getState();
      expect(state.apiKey).toBe("sk-ant-updated");
      expect(state.agentName).toBe("Initial Name"); // Unchanged
    });
  });
});
