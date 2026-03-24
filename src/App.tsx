import { useEffect, useState } from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import toast, { Toaster } from "react-hot-toast";
import { Sidebar } from "@/components/layout/Sidebar";
import { LicenseModal } from "@/components/LicenseModal";
import { Dashboard } from "@/pages/Dashboard";
import { NewProperty } from "@/pages/NewProperty";
import { PropertyDetail } from "@/pages/PropertyDetail";
import { GenerateListing } from "@/pages/GenerateListing";
import { SocialMedia } from "@/pages/SocialMedia";
import { EmailCampaign } from "@/pages/EmailCampaign";
import { BrandVoice } from "@/pages/BrandVoice";
import { Settings } from "@/pages/Settings";
import { useSettingsStore } from "@/stores/settingsStore";
import { useKeyboardShortcuts } from "@/hooks/useKeyboardShortcuts";
import { checkLicense } from "@/lib/tauri";

function ApiKeyCheck() {
  const { isLoaded, apiKey, loadSettings } = useSettingsStore();

  useEffect(() => {
    if (!isLoaded) {
      loadSettings();
    }
  }, [isLoaded, loadSettings]);

  useEffect(() => {
    if (isLoaded && !apiKey) {
      toast(
        "Add your Anthropic API key in Settings to start generating listings.",
        {
          icon: "\u{1F511}",
          duration: 6000,
          id: "api-key-missing",
        },
      );
    }
  }, [isLoaded, apiKey]);

  return null;
}

function KeyboardShortcuts() {
  useKeyboardShortcuts();
  return null;
}

export default function App() {
  const [licenseValid, setLicenseValid] = useState<boolean | null>(null);

  useEffect(() => {
    checkLicense()
      .then((status) => {
        setLicenseValid(status.isValid);
        if (status.error && status.isValid) {
          const isDevelopmentBypass =
            status.error.startsWith("Development mode:");
          toast(status.error, {
            icon: isDevelopmentBypass ? "\u{1F6E0}\u{FE0F}" : "\u{1F4F6}",
            duration: 5000,
            id: isDevelopmentBypass ? "license-dev-bypass" : "license-offline",
          });
        }
      })
      .catch(() => {
        // If license check fails entirely, allow access (grace period)
        setLicenseValid(true);
      });
  }, []);

  // Still loading
  if (licenseValid === null) {
    return (
      <div className="flex items-center justify-center h-screen bg-gray-50">
        <div className="text-center">
          <div className="w-8 h-8 border-2 border-blue-600 border-t-transparent rounded-full animate-spin mx-auto mb-3" />
          <p className="text-sm text-gray-500">Loading...</p>
        </div>
      </div>
    );
  }

  // License not valid — show activation modal
  if (!licenseValid) {
    return (
      <>
        <LicenseModal onActivated={() => setLicenseValid(true)} />
        <Toaster position="bottom-right" />
      </>
    );
  }

  return (
    <BrowserRouter>
      <div className="flex h-screen bg-gray-50">
        <Sidebar />
        <main className="flex-1 overflow-y-auto p-6">
          <ApiKeyCheck />
          <KeyboardShortcuts />
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/property/new" element={<NewProperty />} />
            <Route path="/property/:id" element={<PropertyDetail />}>
              <Route path="listing" element={<GenerateListing />} />
              <Route path="social" element={<SocialMedia />} />
              <Route path="email" element={<EmailCampaign />} />
            </Route>
            <Route path="/brand-voice" element={<BrandVoice />} />
            <Route path="/settings" element={<Settings />} />
          </Routes>
        </main>
      </div>
      <Toaster position="bottom-right" />
    </BrowserRouter>
  );
}
