import { render, screen } from "@testing-library/react";

const { checkLicense, loadSettings, toast } = vi.hoisted(() => ({
  checkLicense: vi.fn(),
  loadSettings: vi.fn(),
  toast: Object.assign(vi.fn(), {
    success: vi.fn(),
    error: vi.fn(),
  }),
}));

vi.mock("react-hot-toast", () => ({
  default: toast,
  Toaster: () => null,
}));

vi.mock("@/lib/tauri", () => ({
  checkLicense,
}));

vi.mock("@/stores/settingsStore", () => ({
  useSettingsStore: () => ({
    isLoaded: true,
    apiKey: "sk-ant-local-test",
    loadSettings,
  }),
}));

vi.mock("@/hooks/useKeyboardShortcuts", () => ({
  useKeyboardShortcuts: vi.fn(),
}));

vi.mock("@/components/layout/Sidebar", () => ({
  Sidebar: () => <div data-testid="sidebar">sidebar</div>,
}));

vi.mock("@/components/LicenseModal", () => ({
  LicenseModal: () => <div>license-modal</div>,
}));

vi.mock("@/pages/Dashboard", () => ({
  Dashboard: () => <div>dashboard-page</div>,
}));

vi.mock("@/pages/NewProperty", () => ({
  NewProperty: () => <div>new-property-page</div>,
}));

vi.mock("@/pages/PropertyDetail", () => ({
  PropertyDetail: () => <div>property-detail-page</div>,
}));

vi.mock("@/pages/GenerateListing", () => ({
  GenerateListing: () => <div>generate-listing-page</div>,
}));

vi.mock("@/pages/SocialMedia", () => ({
  SocialMedia: () => <div>social-media-page</div>,
}));

vi.mock("@/pages/EmailCampaign", () => ({
  EmailCampaign: () => <div>email-campaign-page</div>,
}));

vi.mock("@/pages/BrandVoice", () => ({
  BrandVoice: () => <div>brand-voice-page</div>,
}));

vi.mock("@/pages/Settings", () => ({
  Settings: () => <div>settings-page</div>,
}));

import App from "@/App";

describe("App readiness", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("allows local debug access when the backend returns a development bypass status", async () => {
    checkLicense.mockResolvedValue({
      isValid: true,
      licenseKey: "",
      validatedAt: "2026-03-14T00:00:00Z",
      error:
        "Development mode: license checks are bypassed locally. Release builds still require activation.",
    });

    render(<App />);

    expect(await screen.findByText("dashboard-page")).toBeInTheDocument();
    expect(screen.queryByText("license-modal")).not.toBeInTheDocument();
    expect(toast).toHaveBeenCalledWith(
      expect.stringContaining(
        "Development mode: license checks are bypassed locally.",
      ),
      expect.objectContaining({ id: "license-dev-bypass" }),
    );
  });

  it("shows the license modal when release-style access is denied", async () => {
    checkLicense.mockResolvedValue({
      isValid: false,
      licenseKey: "",
      validatedAt: "",
      error: "No license key found",
    });

    render(<App />);

    expect(await screen.findByText("license-modal")).toBeInTheDocument();
    expect(screen.queryByText("dashboard-page")).not.toBeInTheDocument();
  });
});
