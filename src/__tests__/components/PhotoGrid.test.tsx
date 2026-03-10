import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { PhotoGrid } from "@/components/property/PhotoGrid";
import type { Photo } from "@/lib/types";

const samplePhotos: Photo[] = [
  {
    id: "photo-1",
    propertyId: "property-1",
    filename: "front.jpg",
    originalPath: "/tmp/front.jpg",
    thumbnailPath: "/tmp/front-thumb.jpg",
    sortOrder: 0,
    caption: null,
    createdAt: "2026-03-10T00:00:00Z",
  },
  {
    id: "photo-2",
    propertyId: "property-1",
    filename: "kitchen.jpg",
    originalPath: "/tmp/kitchen.jpg",
    thumbnailPath: "/tmp/kitchen-thumb.jpg",
    sortOrder: 1,
    caption: null,
    createdAt: "2026-03-10T00:00:00Z",
  },
];

describe("PhotoGrid", () => {
  it("shows the first photo as primary", () => {
    render(
      <PhotoGrid
        photos={samplePhotos}
        isLoading={false}
        onDelete={vi.fn().mockResolvedValue(undefined)}
        onMakePrimary={vi.fn().mockResolvedValue(undefined)}
        onMove={vi.fn().mockResolvedValue(undefined)}
      />,
    );

    expect(screen.getByText("Primary")).toBeInTheDocument();
    expect(screen.getByText("Make primary")).toBeInTheDocument();
  });

  it("routes move and primary actions through the provided handlers", async () => {
    const user = userEvent.setup();
    const onDelete = vi.fn().mockResolvedValue(undefined);
    const onMakePrimary = vi.fn().mockResolvedValue(undefined);
    const onMove = vi.fn().mockResolvedValue(undefined);

    render(
      <PhotoGrid
        photos={samplePhotos}
        isLoading={false}
        onDelete={onDelete}
        onMakePrimary={onMakePrimary}
        onMove={onMove}
      />,
    );

    await user.click(screen.getByRole("button", { name: "Make primary" }));
    await user.click(
      screen.getAllByRole("button", { name: "Move photo right" })[0],
    );
    await user.click(
      screen.getAllByRole("button", { name: "Delete photo" })[0],
    );

    expect(onMakePrimary).toHaveBeenCalledWith("photo-2");
    expect(onMove).toHaveBeenCalledWith("photo-1", "right");
    expect(onDelete).toHaveBeenCalledWith("photo-1");
  });
});
