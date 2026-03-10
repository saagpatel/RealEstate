import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { PropertyCard } from "@/components/property/PropertyCard";
import type { Property } from "@/lib/types";

const property: Property = {
  id: "property-1",
  address: "123 Market Street",
  city: "San Francisco",
  state: "CA",
  zip: "94105",
  beds: 3,
  baths: 2.5,
  sqft: 1800,
  price: 95000000,
  propertyType: "single_family",
  yearBuilt: 2016,
  lotSize: null,
  parking: null,
  keyFeatures: ["Pool"],
  neighborhood: null,
  neighborhoodHighlights: [],
  schoolDistrict: null,
  nearbyAmenities: [],
  agentNotes: null,
  createdAt: "2026-03-10T00:00:00Z",
  updatedAt: "2026-03-10T00:00:00Z",
};

describe("PropertyCard", () => {
  it("renders a primary property photo when one is available", () => {
    render(
      <MemoryRouter>
        <PropertyCard
          property={property}
          primaryPhotoPath="/tmp/primary-thumb.jpg"
          onDelete={vi.fn()}
        />
      </MemoryRouter>,
    );

    expect(
      screen.getByRole("img", { name: "123 Market Street primary" }),
    ).toBeInTheDocument();
  });
});
