import { invoke } from "@tauri-apps/api/core";
import type {
  Property,
  CreatePropertyInput,
  Listing,
  BrandVoice,
  Photo,
  AnalyticsSummary,
  CsvImportResult,
  ExportTemplate,
} from "./types";

// Property commands
export const createProperty = (input: CreatePropertyInput) =>
  invoke<Property>("create_property", { input });

export const getProperty = (id: string) =>
  invoke<Property>("get_property", { id });

export const listProperties = () => invoke<Property[]>("list_properties");

export const updateProperty = (id: string, input: CreatePropertyInput) =>
  invoke<Property>("update_property", { id, input });

export const deleteProperty = (id: string) =>
  invoke<void>("delete_property", { id });

// Listing commands
export const listListings = (propertyId: string) =>
  invoke<Listing[]>("list_listings", { propertyId });

export const toggleListingFavorite = (id: string) =>
  invoke<void>("toggle_listing_favorite", { id });

export const deleteListing = (id: string) =>
  invoke<void>("delete_listing", { id });

// Generation commands (streaming via Channel)
export const generateListing = (
  args: {
    propertyId: string;
    style: string;
    tone: string;
    length: string;
    seoKeywords: string[];
    brandVoiceId: string | null;
  },
  onEvent: unknown,
) => invoke<void>("generate_listing", { args, onEvent });

export const generateSocial = (
  args: { propertyId: string; platform: string; brandVoiceId: string | null },
  onEvent: unknown,
) => invoke<void>("generate_social", { args, onEvent });

export const generateEmail = (
  args: {
    propertyId: string;
    templateType: string;
    brandVoiceId: string | null;
  },
  onEvent: unknown,
) => invoke<void>("generate_email", { args, onEvent });

// Photo commands
export const importPhotos = (propertyId: string) =>
  invoke<Photo[]>("import_photos", { propertyId });

export const listPhotos = (propertyId: string) =>
  invoke<Photo[]>("list_photos", { propertyId });

export const deletePhoto = (id: string) => invoke<void>("delete_photo", { id });

export const reorderPhotos = (propertyId: string, photoIds: string[]) =>
  invoke<void>("reorder_photos", { propertyId, photoIds });

// Brand voice commands
export const createBrandVoice = (
  name: string,
  description: string | null,
  sampleListings: string[],
) =>
  invoke<BrandVoice>("create_brand_voice", {
    name,
    description,
    sampleListings,
  });

export const listBrandVoices = () => invoke<BrandVoice[]>("list_brand_voices");

export const deleteBrandVoice = (id: string) =>
  invoke<void>("delete_brand_voice", { id });

// Settings commands
export const getSetting = (key: string) =>
  invoke<string>("get_setting", { key });

export const setSetting = (key: string, value: string) =>
  invoke<void>("set_setting", { key, value });

export const getAnalyticsSummary = () =>
  invoke<AnalyticsSummary>("get_analytics_summary");

export const importPropertiesCsv = (csvData: string) =>
  invoke<CsvImportResult>("import_properties_csv", { csvData });

export const getCsvTemplate = () => invoke<string>("get_csv_template");

// Export commands
export const exportPdf = (
  propertyId: string,
  listingIds: string[],
  template: ExportTemplate,
) => invoke<number[]>("export_pdf", { propertyId, listingIds, template });

export const exportDocx = (
  propertyId: string,
  listingIds: string[],
  template: ExportTemplate,
) => invoke<number[]>("export_docx", { propertyId, listingIds, template });

export const copyToClipboard = (text: string) =>
  invoke<void>("copy_to_clipboard", { text });

// License commands
export interface LicenseStatus {
  isValid: boolean;
  licenseKey: string;
  validatedAt: string;
  error: string | null;
}

export const validateLicenseKey = (licenseKey: string) =>
  invoke<LicenseStatus>("validate_license_key", { licenseKey });

export const checkLicense = () => invoke<LicenseStatus>("check_license");
