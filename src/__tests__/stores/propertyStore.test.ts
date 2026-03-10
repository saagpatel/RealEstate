import { describe, it, expect, beforeEach, vi } from "vitest";
import { usePropertyStore } from "@/stores/propertyStore";
import * as tauri from "@/lib/tauri";
import type { Property, CreatePropertyInput } from "@/lib/types";

vi.mock("@/lib/tauri");

describe("propertyStore", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Reset store state
    usePropertyStore.setState({
      properties: [],
      selectedPropertyId: null,
      isLoading: false,
    });
  });

  it("should initialize with empty state", () => {
    const store = usePropertyStore.getState();
    expect(store.properties).toEqual([]);
    expect(store.selectedPropertyId).toBeNull();
    expect(store.isLoading).toBe(false);
  });

  describe("fetchProperties", () => {
    it("should fetch and set properties", async () => {
      const mockProperties: Property[] = [
        {
          id: "1",
          address: "123 Main St",
          city: "San Francisco",
          state: "CA",
          zip: "94102",
          beds: 3,
          baths: 2,
          sqft: 2500,
          price: 85000000,
          propertyType: "House",
          keyFeatures: [],
          neighborhoodHighlights: [],
          nearbyAmenities: [],
          createdAt: "2024-01-01T00:00:00Z",
          updatedAt: "2024-01-01T00:00:00Z",
        },
        {
          id: "2",
          address: "456 Oak Ave",
          city: "New York",
          state: "NY",
          zip: "10001",
          beds: 4,
          baths: 3,
          sqft: 3200,
          price: 125000000,
          propertyType: "Condo",
          keyFeatures: [],
          neighborhoodHighlights: [],
          nearbyAmenities: [],
          createdAt: "2024-01-02T00:00:00Z",
          updatedAt: "2024-01-02T00:00:00Z",
        },
      ];

      vi.mocked(tauri.listProperties).mockResolvedValue(mockProperties);

      const { fetchProperties } = usePropertyStore.getState();
      await fetchProperties();

      const state = usePropertyStore.getState();
      expect(state.properties).toEqual(mockProperties);
      expect(state.isLoading).toBe(false);
      expect(tauri.listProperties).toHaveBeenCalledTimes(1);
    });

    it("should set isLoading during fetch", async () => {
      let loadingDuringFetch = false;

      vi.mocked(tauri.listProperties).mockImplementation(async () => {
        loadingDuringFetch = usePropertyStore.getState().isLoading;
        return [];
      });

      const { fetchProperties } = usePropertyStore.getState();
      await fetchProperties();

      expect(loadingDuringFetch).toBe(true);
      expect(usePropertyStore.getState().isLoading).toBe(false);
    });

    it("should reset loading and propagate fetch errors", async () => {
      vi.mocked(tauri.listProperties).mockRejectedValue(
        new Error("Network error"),
      );

      const { fetchProperties } = usePropertyStore.getState();

      await expect(fetchProperties()).rejects.toThrow("Network error");

      const state = usePropertyStore.getState();
      expect(state.isLoading).toBe(false);
      expect(state.properties).toEqual([]);
    });
  });

  describe("createProperty", () => {
    it("should create property and add to list", async () => {
      const input: CreatePropertyInput = {
        address: "789 New St",
        city: "Boston",
        state: "MA",
        zip: "02101",
        beds: 2,
        baths: 1.5,
        sqft: 1800,
        price: 65000000,
        propertyType: "Townhouse",
        keyFeatures: ["Updated kitchen"],
        neighborhoodHighlights: [],
        nearbyAmenities: [],
      };

      const createdProperty: Property = {
        id: "3",
        ...input,
        yearBuilt: null,
        lotSize: null,
        parking: null,
        neighborhood: null,
        schoolDistrict: null,
        agentNotes: null,
        createdAt: "2024-01-03T00:00:00Z",
        updatedAt: "2024-01-03T00:00:00Z",
      };

      vi.mocked(tauri.createProperty).mockResolvedValue(createdProperty);

      const { createProperty } = usePropertyStore.getState();
      const result = await createProperty(input);

      expect(result).toEqual(createdProperty);
      expect(tauri.createProperty).toHaveBeenCalledWith(input);

      const state = usePropertyStore.getState();
      expect(state.properties).toHaveLength(1);
      expect(state.properties[0]).toEqual(createdProperty);
    });

    it("should prepend new property to existing list", async () => {
      const existing: Property = {
        id: "1",
        address: "Existing",
        city: "City",
        state: "ST",
        zip: "12345",
        beds: 1,
        baths: 1,
        sqft: 1000,
        price: 50000000,
        propertyType: "Condo",
        keyFeatures: [],
        neighborhoodHighlights: [],
        nearbyAmenities: [],
        createdAt: "2024-01-01T00:00:00Z",
        updatedAt: "2024-01-01T00:00:00Z",
      };

      usePropertyStore.setState({ properties: [existing] });

      const newProperty: Property = {
        id: "2",
        address: "New",
        city: "City",
        state: "ST",
        zip: "12345",
        beds: 2,
        baths: 2,
        sqft: 2000,
        price: 75000000,
        propertyType: "House",
        keyFeatures: [],
        neighborhoodHighlights: [],
        nearbyAmenities: [],
        createdAt: "2024-01-02T00:00:00Z",
        updatedAt: "2024-01-02T00:00:00Z",
      };

      vi.mocked(tauri.createProperty).mockResolvedValue(newProperty);

      const { createProperty } = usePropertyStore.getState();
      await createProperty({} as CreatePropertyInput);

      const state = usePropertyStore.getState();
      expect(state.properties).toHaveLength(2);
      expect(state.properties[0].id).toBe("2"); // New property first
      expect(state.properties[1].id).toBe("1"); // Existing property second
    });
  });

  describe("deleteProperty", () => {
    it("should delete property from list", async () => {
      const properties: Property[] = [
        {
          id: "1",
          address: "Property 1",
          city: "City",
          state: "ST",
          zip: "12345",
          beds: 1,
          baths: 1,
          sqft: 1000,
          price: 50000000,
          propertyType: "Condo",
          keyFeatures: [],
          neighborhoodHighlights: [],
          nearbyAmenities: [],
          createdAt: "2024-01-01T00:00:00Z",
          updatedAt: "2024-01-01T00:00:00Z",
        },
        {
          id: "2",
          address: "Property 2",
          city: "City",
          state: "ST",
          zip: "12345",
          beds: 2,
          baths: 2,
          sqft: 2000,
          price: 75000000,
          propertyType: "House",
          keyFeatures: [],
          neighborhoodHighlights: [],
          nearbyAmenities: [],
          createdAt: "2024-01-02T00:00:00Z",
          updatedAt: "2024-01-02T00:00:00Z",
        },
      ];

      usePropertyStore.setState({ properties });

      vi.mocked(tauri.deleteProperty).mockResolvedValue();

      const { deleteProperty } = usePropertyStore.getState();
      await deleteProperty("1");

      expect(tauri.deleteProperty).toHaveBeenCalledWith("1");

      const state = usePropertyStore.getState();
      expect(state.properties).toHaveLength(1);
      expect(state.properties[0].id).toBe("2");
    });

    it("should clear selectedPropertyId if deleting selected property", async () => {
      const property: Property = {
        id: "1",
        address: "Property",
        city: "City",
        state: "ST",
        zip: "12345",
        beds: 1,
        baths: 1,
        sqft: 1000,
        price: 50000000,
        propertyType: "Condo",
        keyFeatures: [],
        neighborhoodHighlights: [],
        nearbyAmenities: [],
        createdAt: "2024-01-01T00:00:00Z",
        updatedAt: "2024-01-01T00:00:00Z",
      };

      usePropertyStore.setState({
        properties: [property],
        selectedPropertyId: "1",
      });

      vi.mocked(tauri.deleteProperty).mockResolvedValue();

      const { deleteProperty } = usePropertyStore.getState();
      await deleteProperty("1");

      const state = usePropertyStore.getState();
      expect(state.selectedPropertyId).toBeNull();
    });

    it("should preserve selectedPropertyId if deleting different property", async () => {
      const properties: Property[] = [
        {
          id: "1",
          address: "Property 1",
          city: "City",
          state: "ST",
          zip: "12345",
          beds: 1,
          baths: 1,
          sqft: 1000,
          price: 50000000,
          propertyType: "Condo",
          keyFeatures: [],
          neighborhoodHighlights: [],
          nearbyAmenities: [],
          createdAt: "2024-01-01T00:00:00Z",
          updatedAt: "2024-01-01T00:00:00Z",
        },
        {
          id: "2",
          address: "Property 2",
          city: "City",
          state: "ST",
          zip: "12345",
          beds: 2,
          baths: 2,
          sqft: 2000,
          price: 75000000,
          propertyType: "House",
          keyFeatures: [],
          neighborhoodHighlights: [],
          nearbyAmenities: [],
          createdAt: "2024-01-02T00:00:00Z",
          updatedAt: "2024-01-02T00:00:00Z",
        },
      ];

      usePropertyStore.setState({
        properties,
        selectedPropertyId: "2",
      });

      vi.mocked(tauri.deleteProperty).mockResolvedValue();

      const { deleteProperty } = usePropertyStore.getState();
      await deleteProperty("1");

      const state = usePropertyStore.getState();
      expect(state.selectedPropertyId).toBe("2");
    });
  });

  describe("setSelectedPropertyId", () => {
    it("should set selected property ID", () => {
      const { setSelectedPropertyId } = usePropertyStore.getState();
      setSelectedPropertyId("123");

      const state = usePropertyStore.getState();
      expect(state.selectedPropertyId).toBe("123");
    });

    it("should clear selected property ID", () => {
      usePropertyStore.setState({ selectedPropertyId: "123" });

      const { setSelectedPropertyId } = usePropertyStore.getState();
      setSelectedPropertyId(null);

      const state = usePropertyStore.getState();
      expect(state.selectedPropertyId).toBeNull();
    });
  });
});
