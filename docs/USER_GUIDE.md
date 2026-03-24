# User Guide

## What the app is for

RealEstate Listing Optimizer gives agents one desktop workspace for:

- storing property details locally
- importing and organizing property photos
- generating listing descriptions, social posts, and email campaigns
- creating reusable brand voices
- importing properties from CSV
- exporting polished PDF and DOCX packages

## First launch checklist

When you first open the app, complete these steps in order:

1. Activate your license on release builds. Local debug builds allow
   development access without a production license key.
2. Open **Settings** and add your Anthropic API key.
3. Complete your agent profile details.
4. Create your first property or import one from CSV.

The dashboard now includes a setup-readiness panel so you can see which of
those steps are still incomplete.

## Core workflow

### 1. Create or import a property

You can either:

- click **New Property** from the dashboard, or
- download the CSV template and import multiple properties at once

The CSV import flow expects multi-value fields such as amenities and key
features to be represented as JSON arrays.

For the exact CSV contract, use [CSV Import Reference](./CSV_IMPORT_REFERENCE.md).

### 2. Add and organize photos

Open a property and use the photo section to:

- import images from disk
- delete unwanted images
- move photos left or right
- promote any photo to **Primary**

The first photo in the order is treated as the primary cover image used by
exports.

### 3. Generate marketing content

Inside a property workspace, use the three content tabs:

- **Generate Listing**
- **Social Media**
- **Email Campaign**

Each flow supports Anthropic-powered generation and can use a saved brand voice
when one is available.

## Brand voice

Brand voices let you reuse a consistent style across generated materials.

To create one:

1. Open **Brand Voice**
2. Upload or paste at least two sample listings
3. Save the voice for reuse in listing, social, and email generation

## Exporting content

From the property workspace, use **Export** to create a package from the
generated content attached to that property.

Available templates:

- **Professional**: balanced presentation with photos included
- **Luxury**: larger headings with a more premium presentation
- **Minimal**: text-first export that excludes photo sections

Available formats:

- PDF
- DOCX

## Settings

Settings currently controls:

- Anthropic API key
- AI model preference
- default listing style, tone, and length
- agent name, phone, email, and brokerage name

If generation is disabled, the first place to check is whether the API key is
present and valid.

## Data storage

The application stores its working data locally using SQLite plus local file
storage for imported photos and thumbnails.

That means:

- your working property data is local to the machine unless you manually export it
- photo imports are copied into app-managed storage
- deleting the app does not automatically imply a managed cloud backup exists
