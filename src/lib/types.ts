export interface Property {
  id: string;
  address: string;
  city: string;
  state: string;
  zip: string;
  beds: number;
  baths: number;
  sqft: number;
  price: number; // in cents
  propertyType: PropertyType;
  yearBuilt: number | null;
  lotSize: string | null;
  parking: string | null;
  keyFeatures: string[];
  neighborhood: string | null;
  neighborhoodHighlights: string[];
  schoolDistrict: string | null;
  nearbyAmenities: string[];
  agentNotes: string | null;
  createdAt: string;
  updatedAt: string;
}

export type PropertyType =
  | "single_family"
  | "condo"
  | "townhouse"
  | "multi_family"
  | "land"
  | "commercial";

export interface Photo {
  id: string;
  propertyId: string;
  filename: string;
  originalPath: string;
  thumbnailPath: string;
  sortOrder: number;
  caption: string | null;
  createdAt: string;
}

export interface Listing {
  id: string;
  propertyId: string;
  content: string;
  generationType: GenerationType;
  style: ListingStyle | null;
  tone: ListingTone | null;
  length: ListingLength | null;
  seoKeywords: string[];
  brandVoiceId: string | null;
  tokensUsed: number;
  generationCostCents: number;
  isFavorite: boolean;
  createdAt: string;
}

export type GenerationType =
  | "listing"
  | "social_instagram"
  | "social_facebook"
  | "social_linkedin"
  | "email_buyer"
  | "email_seller"
  | "email_open_house";

export type ListingStyle = "luxury" | "family" | "investment" | "first_time";
export type ListingTone = "professional" | "warm" | "exciting";
export type ListingLength = "short" | "medium" | "long";

export interface AnalyticsSummary {
  totalGenerations: number;
  totalCostCents: number;
  averageLatencyMs: number;
  successRate: number;
}

export interface CsvImportError {
  rowNumber: number;
  address: string;
  error: string;
}

export interface CsvImportResult {
  total: number;
  successful: number;
  failed: number;
  errors: CsvImportError[];
}

export interface BrandVoice {
  id: string;
  name: string;
  description: string | null;
  extractedStyle: string; // JSON string, parse with parseExtractedStyle()
  sourceListings: string; // JSON string array
  sampleCount: number;
  createdAt: string;
  updatedAt: string;
}

export interface ExtractedStyle {
  tone: string;
  vocabulary: string[];
  sentencePatterns: string;
  themes: string[];
  signaturePhrases: string[];
  avoids: string[];
  formatting: string;
}

export interface GenerationOptions {
  style: ListingStyle;
  tone: ListingTone;
  length: ListingLength;
  seoKeywords: string[];
  brandVoiceId: string | null;
}

export interface SocialGenerationOptions {
  platform: SocialPlatform;
  brandVoiceId: string | null;
}

export type SocialPlatform = "instagram" | "facebook" | "linkedin";

export interface EmailGenerationOptions {
  templateType: EmailTemplate;
  brandVoiceId: string | null;
}

export type EmailTemplate = "buyer" | "seller" | "open_house";

export type ExportTemplate = "professional" | "luxury" | "minimal";

export type StreamEvent =
  | { event: "started"; data: { estimatedTokens: number } }
  | { event: "delta"; data: { text: string } }
  | {
      event: "finished";
      data: {
        fullText: string;
        inputTokens: number;
        outputTokens: number;
        costCents: number;
      };
    }
  | { event: "error"; data: { message: string } };

export interface AgentInfo {
  name: string;
  phone: string;
  email: string;
  brokerageName: string;
}

export interface CreatePropertyInput {
  address: string;
  city: string;
  state: string;
  zip: string;
  beds: number;
  baths: number;
  sqft: number;
  price: number;
  propertyType: PropertyType;
  yearBuilt: number | null;
  lotSize: string | null;
  parking: string | null;
  keyFeatures: string[];
  neighborhood: string | null;
  neighborhoodHighlights: string[];
  schoolDistrict: string | null;
  nearbyAmenities: string[];
  agentNotes: string | null;
}
