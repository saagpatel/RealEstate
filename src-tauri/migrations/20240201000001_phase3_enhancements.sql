-- Phase 3 enhancements: AI model selection and analytics

-- Add AI model preference setting
INSERT INTO settings (key, value) VALUES ('ai_model', 'claude-sonnet-4-20250514');

-- Analytics tables for tracking usage and performance
CREATE TABLE generation_analytics (
    id TEXT PRIMARY KEY,
    property_id TEXT REFERENCES properties(id) ON DELETE CASCADE,
    generation_type TEXT NOT NULL,
    model_used TEXT NOT NULL,
    input_tokens INTEGER NOT NULL,
    output_tokens INTEGER NOT NULL,
    cost_cents INTEGER NOT NULL,
    latency_ms INTEGER NOT NULL,
    success INTEGER NOT NULL DEFAULT 1,
    error_message TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_analytics_property ON generation_analytics(property_id, created_at DESC);
CREATE INDEX idx_analytics_created ON generation_analytics(created_at DESC);

CREATE TABLE export_analytics (
    id TEXT PRIMARY KEY,
    property_id TEXT REFERENCES properties(id) ON DELETE CASCADE,
    export_format TEXT NOT NULL CHECK(export_format IN ('pdf', 'docx')),
    listing_count INTEGER NOT NULL,
    photo_count INTEGER NOT NULL,
    file_size_bytes INTEGER NOT NULL,
    generation_time_ms INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_export_analytics_property ON export_analytics(property_id, created_at DESC);
CREATE INDEX idx_export_analytics_created ON export_analytics(created_at DESC);
