import type { ReactNode } from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { Settings } from "@/pages/Settings";

const { saveSetting, loadSettings, toast } = vi.hoisted(() => ({
  saveSetting: vi.fn(),
  loadSettings: vi.fn(),
  toast: {
    success: vi.fn(),
    error: vi.fn(),
  },
}));

const storeState = {
  apiKey: "",
  aiModel: "claude-sonnet-4-20250514",
  agentName: "Taylor Agent",
  agentPhone: "555-0100",
  agentEmail: "taylor@example.com",
  brokerageName: "North Star Realty",
  defaultStyle: "luxury",
  defaultTone: "warm",
  defaultLength: "medium",
  isLoaded: true,
  loadSettings,
  saveSetting,
};

vi.mock("react-hot-toast", () => ({
  default: toast,
}));

vi.mock("@/stores/settingsStore", () => ({
  useSettingsStore: () => storeState,
}));

vi.mock("@/components/layout/PageHeader", () => ({
  PageHeader: ({
    title,
    subtitle,
    actions,
  }: {
    title: string;
    subtitle: string;
    actions: ReactNode;
  }) => (
    <div>
      <h1>{title}</h1>
      <p>{subtitle}</p>
      {actions}
    </div>
  ),
}));

describe("Settings page", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("blocks invalid API keys before saving", async () => {
    const user = userEvent.setup();

    render(<Settings />);

    const apiKeyInput = screen.getByPlaceholderText("sk-ant-...");
    await user.type(apiKeyInput, "bad-key");
    await user.click(screen.getByRole("button", { name: "Save Settings" }));

    expect(saveSetting).not.toHaveBeenCalled();
    expect(toast.error).toHaveBeenCalledWith(
      'API key must start with "sk-ant-" or be empty',
    );
  });

  it("saves the full settings payload when the form is valid", async () => {
    const user = userEvent.setup();
    saveSetting.mockResolvedValue(undefined);

    render(<Settings />);

    const apiKeyInput = screen.getByPlaceholderText("sk-ant-...");
    await user.type(apiKeyInput, "sk-ant-local-ready");
    await user.click(screen.getByRole("button", { name: "Save Settings" }));

    await waitFor(() => expect(saveSetting).toHaveBeenCalledTimes(9));
    expect(saveSetting).toHaveBeenCalledWith("api_key", "sk-ant-local-ready");
    expect(toast.success).toHaveBeenCalledWith("Settings saved");
  });
});
