import { useEffect, useState, useCallback } from "react";
import { useParams, useNavigate, NavLink, Outlet } from "react-router-dom";
import {
  FileText,
  Share2,
  Mail,
  ArrowLeft,
  Pencil,
  Bed,
  Bath,
  Ruler,
  MapPin,
  Loader2,
  Trash2,
} from "lucide-react";
import toast from "react-hot-toast";
import { ExportMenu } from "@/components/export/ExportMenu";
import { PageHeader } from "@/components/layout/PageHeader";
import { PropertyForm } from "@/components/property/PropertyForm";
import { PhotoUploader } from "@/components/property/PhotoUploader";
import { PhotoGrid } from "@/components/property/PhotoGrid";
import { usePropertyStore } from "@/stores/propertyStore";
import { usePhotos } from "@/hooks/usePhotos";
import { PROPERTY_TYPES } from "@/lib/constants";
import * as tauri from "@/lib/tauri";
import type { Property, CreatePropertyInput } from "@/lib/types";

function formatPrice(cents: number): string {
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
    maximumFractionDigits: 0,
  }).format(cents / 100);
}

function formatNumber(n: number): string {
  return new Intl.NumberFormat("en-US").format(n);
}

function propertyToInput(p: Property): CreatePropertyInput {
  return {
    address: p.address,
    city: p.city,
    state: p.state,
    zip: p.zip,
    beds: p.beds,
    baths: p.baths,
    sqft: p.sqft,
    price: p.price,
    propertyType: p.propertyType,
    yearBuilt: p.yearBuilt,
    lotSize: p.lotSize,
    parking: p.parking,
    keyFeatures: p.keyFeatures,
    neighborhood: p.neighborhood,
    neighborhoodHighlights: p.neighborhoodHighlights,
    schoolDistrict: p.schoolDistrict,
    nearbyAmenities: p.nearbyAmenities,
    agentNotes: p.agentNotes,
  };
}

export function PropertyDetail() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const deleteProperty = usePropertyStore((s) => s.deleteProperty);
  const fetchProperties = usePropertyStore((s) => s.fetchProperties);

  const {
    photos,
    isLoading: photosLoading,
    isImporting,
    isReordering,
    error: photosError,
    importPhotos: handleImportPhotos,
    deletePhoto: handleDeletePhoto,
    reorderPhotos: handleReorderPhotos,
  } = usePhotos(id);

  const [property, setProperty] = useState<Property | null>(null);
  const [listingIds, setListingIds] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isEditing, setIsEditing] = useState(false);

  const loadListingIds = useCallback(async () => {
    if (!id) return [];

    const listings = await tauri.listListings(id);
    const ids = listings.map((listing) => listing.id);
    setListingIds(ids);
    return ids;
  }, [id]);

  const loadProperty = useCallback(async () => {
    if (!id) return;
    setIsLoading(true);
    try {
      const [p, listings] = await Promise.all([
        tauri.getProperty(id),
        loadListingIds(),
      ]);
      setProperty(p);
      setListingIds(listings);
    } catch (err: unknown) {
      const message =
        err instanceof Error ? err.message : "Failed to load property";
      toast.error(message);
    } finally {
      setIsLoading(false);
    }
  }, [id, loadListingIds]);

  useEffect(() => {
    void loadProperty();
  }, [loadProperty]);

  const handleUpdate = async (input: CreatePropertyInput) => {
    if (!id) return;
    try {
      const updated = await tauri.updateProperty(id, input);
      setProperty(updated);
      setIsEditing(false);
      await fetchProperties();
      toast.success("Property updated");
    } catch (err: unknown) {
      const message =
        err instanceof Error ? err.message : "Failed to update property";
      toast.error(message);
    }
  };

  const handleDelete = async () => {
    if (!id) return;
    if (!window.confirm("Delete this property? This cannot be undone.")) return;
    try {
      await deleteProperty(id);
      toast.success("Property deleted");
      navigate("/");
    } catch (err: unknown) {
      const message =
        err instanceof Error ? err.message : "Failed to delete property";
      toast.error(message);
    }
  };

  const movePhoto = async (photoId: string, direction: "left" | "right") => {
    const index = photos.findIndex((photo) => photo.id === photoId);
    if (index === -1) return;

    const nextIndex = direction === "left" ? index - 1 : index + 1;
    if (nextIndex < 0 || nextIndex >= photos.length) return;

    const reordered = [...photos];
    const [moved] = reordered.splice(index, 1);
    reordered.splice(nextIndex, 0, moved);
    await handleReorderPhotos(reordered.map((photo) => photo.id));
  };

  const makePrimaryPhoto = async (photoId: string) => {
    const selected = photos.find((photo) => photo.id === photoId);
    if (!selected) return;

    const reordered = [
      selected.id,
      ...photos
        .filter((photo) => photo.id !== photoId)
        .map((photo) => photo.id),
    ];
    await handleReorderPhotos(reordered);
  };

  const tabs = [
    {
      to: `/property/${id}/listing`,
      icon: FileText,
      label: "Generate Listing",
    },
    { to: `/property/${id}/social`, icon: Share2, label: "Social Media" },
    { to: `/property/${id}/email`, icon: Mail, label: "Email Campaign" },
  ];

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-20">
        <Loader2 size={24} className="animate-spin text-gray-400" />
      </div>
    );
  }

  if (!property) {
    return (
      <div className="text-center py-20">
        <p className="text-gray-500 mb-4">Property not found.</p>
        <button
          onClick={() => navigate("/")}
          className="text-blue-600 hover:underline text-sm"
        >
          Back to Dashboard
        </button>
      </div>
    );
  }

  const typeLabel =
    PROPERTY_TYPES.find((t) => t.value === property.propertyType)?.label ??
    property.propertyType;

  return (
    <div>
      <PageHeader
        title={property.address}
        subtitle={`${property.city}, ${property.state} ${property.zip}`}
        actions={
          <div className="flex gap-2">
            <ExportMenu
              propertyId={property.id}
              listingIds={listingIds}
              getListingIds={loadListingIds}
            />
            <button
              onClick={() => navigate("/")}
              className="flex items-center gap-2 px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors text-sm font-medium text-gray-700"
            >
              <ArrowLeft size={16} />
              Back
            </button>
            <button
              onClick={() => setIsEditing(!isEditing)}
              className="flex items-center gap-2 px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors text-sm font-medium text-gray-700"
            >
              <Pencil size={16} />
              {isEditing ? "Cancel Edit" : "Edit"}
            </button>
            <button
              onClick={handleDelete}
              className="flex items-center gap-2 px-4 py-2 border border-red-300 text-red-600 rounded-lg hover:bg-red-50 transition-colors text-sm font-medium"
            >
              <Trash2 size={16} />
              Delete
            </button>
          </div>
        }
      />

      {/* Property Summary or Edit Form */}
      <div className="bg-white rounded-lg border border-gray-200 p-6 mb-6">
        {isEditing ? (
          <PropertyForm
            initialValues={propertyToInput(property)}
            onSubmit={handleUpdate}
            submitLabel="Save Changes"
          />
        ) : (
          <div className="space-y-6">
            {/* Top row: price + type + stats */}
            <div className="flex flex-wrap items-center gap-6">
              <span className="text-2xl font-bold text-gray-900">
                {formatPrice(property.price)}
              </span>
              <span className="text-sm font-medium text-gray-500 bg-gray-100 px-3 py-1 rounded">
                {typeLabel}
              </span>
              <div className="flex items-center gap-4 text-sm text-gray-600">
                <span className="flex items-center gap-1">
                  <Bed size={14} /> {property.beds} beds
                </span>
                <span className="flex items-center gap-1">
                  <Bath size={14} /> {property.baths} baths
                </span>
                <span className="flex items-center gap-1">
                  <Ruler size={14} /> {formatNumber(property.sqft)} sqft
                </span>
              </div>
            </div>

            {/* Address */}
            <div className="flex items-start gap-2 text-gray-600">
              <MapPin size={16} className="mt-0.5 shrink-0" />
              <span className="text-sm">
                {property.address}, {property.city}, {property.state}{" "}
                {property.zip}
              </span>
            </div>

            {/* Details grid */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
              {property.yearBuilt && (
                <div>
                  <span className="text-gray-500">Year Built</span>
                  <p className="font-medium">{property.yearBuilt}</p>
                </div>
              )}
              {property.lotSize && (
                <div>
                  <span className="text-gray-500">Lot Size</span>
                  <p className="font-medium">{property.lotSize}</p>
                </div>
              )}
              {property.parking && (
                <div>
                  <span className="text-gray-500">Parking</span>
                  <p className="font-medium">{property.parking}</p>
                </div>
              )}
              {property.schoolDistrict && (
                <div>
                  <span className="text-gray-500">School District</span>
                  <p className="font-medium">{property.schoolDistrict}</p>
                </div>
              )}
              {property.neighborhood && (
                <div>
                  <span className="text-gray-500">Neighborhood</span>
                  <p className="font-medium">{property.neighborhood}</p>
                </div>
              )}
            </div>

            {/* Tags */}
            {property.keyFeatures.length > 0 && (
              <div>
                <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">
                  Key Features
                </span>
                <div className="flex flex-wrap gap-2 mt-2">
                  {property.keyFeatures.map((f, i) => (
                    <span
                      key={i}
                      className="bg-blue-50 text-blue-700 text-sm px-3 py-1 rounded-full"
                    >
                      {f}
                    </span>
                  ))}
                </div>
              </div>
            )}

            {property.neighborhoodHighlights.length > 0 && (
              <div>
                <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">
                  Neighborhood Highlights
                </span>
                <div className="flex flex-wrap gap-2 mt-2">
                  {property.neighborhoodHighlights.map((h, i) => (
                    <span
                      key={i}
                      className="bg-green-50 text-green-700 text-sm px-3 py-1 rounded-full"
                    >
                      {h}
                    </span>
                  ))}
                </div>
              </div>
            )}

            {property.nearbyAmenities.length > 0 && (
              <div>
                <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">
                  Nearby Amenities
                </span>
                <div className="flex flex-wrap gap-2 mt-2">
                  {property.nearbyAmenities.map((a, i) => (
                    <span
                      key={i}
                      className="bg-purple-50 text-purple-700 text-sm px-3 py-1 rounded-full"
                    >
                      {a}
                    </span>
                  ))}
                </div>
              </div>
            )}

            {property.agentNotes && (
              <div>
                <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">
                  Agent Notes
                </span>
                <p className="text-sm text-gray-700 mt-2 whitespace-pre-wrap">
                  {property.agentNotes}
                </p>
              </div>
            )}
          </div>
        )}
      </div>

      {/* Photos Section */}
      <div className="bg-white rounded-lg border border-gray-200 p-6 mb-6">
        <div className="flex items-center justify-between mb-4">
          <div>
            <h2 className="text-sm font-semibold text-gray-900 uppercase tracking-wider">
              Photos
            </h2>
            <p className="mt-1 text-xs text-gray-500">
              Drag-free ordering is enabled here: the first photo becomes the
              primary cover image used in exports.
            </p>
          </div>
          <div className="text-right">
            <span className="text-xs text-gray-500">
              {photos.length} photo{photos.length !== 1 ? "s" : ""}
            </span>
            {isReordering && (
              <p className="mt-1 text-xs text-blue-600">Saving new order...</p>
            )}
          </div>
        </div>
        {photosError && (
          <p className="text-sm text-red-600 mb-3">{photosError}</p>
        )}
        <PhotoGrid
          photos={photos}
          isLoading={photosLoading}
          isReordering={isReordering}
          onDelete={handleDeletePhoto}
          onMakePrimary={makePrimaryPhoto}
          onMove={movePhoto}
        />
        <div className={photos.length > 0 ? "mt-4" : ""}>
          <PhotoUploader
            photoCount={photos.length}
            isImporting={isImporting}
            onImport={handleImportPhotos}
          />
        </div>
      </div>

      {/* Tab navigation */}
      <div className="border-b border-gray-200 mb-6">
        <nav className="flex gap-6">
          {tabs.map((tab) => (
            <NavLink
              key={tab.to}
              to={tab.to}
              className={({ isActive }) =>
                `flex items-center gap-2 pb-3 border-b-2 text-sm font-medium transition-colors ${
                  isActive
                    ? "border-blue-600 text-blue-600"
                    : "border-transparent text-gray-500 hover:text-gray-700"
                }`
              }
            >
              <tab.icon size={16} />
              {tab.label}
            </NavLink>
          ))}
        </nav>
      </div>
      <Outlet />
    </div>
  );
}
