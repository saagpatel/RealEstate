UPDATE settings
SET value = 'luxury'
WHERE key = 'default_style' AND value NOT IN ('luxury', 'family', 'investment', 'first_time');

UPDATE settings
SET value = 'warm'
WHERE key = 'default_tone' AND value NOT IN ('professional', 'warm', 'exciting');

UPDATE settings
SET value = 'medium'
WHERE key = 'default_length' AND value NOT IN ('short', 'medium', 'long');

INSERT INTO settings (key, value)
SELECT 'ai_model', 'claude-sonnet-4-20250514'
WHERE NOT EXISTS (SELECT 1 FROM settings WHERE key = 'ai_model');

UPDATE settings
SET value = 'claude-sonnet-4-20250514'
WHERE key = 'ai_model' AND value = 'claude-sonnet-4-5-20250929';
