/// Export template styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportTemplate {
    Professional,
    Luxury,
    Minimal,
}

impl ExportTemplate {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "professional" => Some(Self::Professional),
            "luxury" => Some(Self::Luxury),
            "minimal" => Some(Self::Minimal),
            _ => None,
        }
    }
}

/// Template configuration for styling exports
#[allow(dead_code)]
pub struct TemplateConfig {
    pub primary_color: (u8, u8, u8),   // RGB
    pub secondary_color: (u8, u8, u8), // RGB
    pub header_font_size: u8,
    pub body_font_size: u8,
    pub include_photos: bool,
    pub photo_layout: PhotoLayout,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum PhotoLayout {
    Grid,      // Multiple photos in grid
    Featured,  // One large photo at top
    Scattered, // Photos interspersed with text
}

impl ExportTemplate {
    pub fn config(&self) -> TemplateConfig {
        match self {
            ExportTemplate::Professional => TemplateConfig {
                primary_color: (0, 51, 102),      // Navy blue
                secondary_color: (102, 102, 102), // Gray
                header_font_size: 18,
                body_font_size: 11,
                include_photos: true,
                photo_layout: PhotoLayout::Grid,
            },
            ExportTemplate::Luxury => TemplateConfig {
                primary_color: (139, 115, 85), // Gold/Bronze
                secondary_color: (64, 64, 64), // Charcoal
                header_font_size: 22,
                body_font_size: 12,
                include_photos: true,
                photo_layout: PhotoLayout::Featured,
            },
            ExportTemplate::Minimal => TemplateConfig {
                primary_color: (0, 0, 0),         // Black
                secondary_color: (128, 128, 128), // Gray
                header_font_size: 16,
                body_font_size: 10,
                include_photos: false, // Minimal template excludes photos
                photo_layout: PhotoLayout::Grid,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_from_str() {
        assert_eq!(
            ExportTemplate::from_str("professional"),
            Some(ExportTemplate::Professional)
        );
        assert_eq!(
            ExportTemplate::from_str("Luxury"),
            Some(ExportTemplate::Luxury)
        );
        assert_eq!(
            ExportTemplate::from_str("MINIMAL"),
            Some(ExportTemplate::Minimal)
        );
        assert_eq!(ExportTemplate::from_str("invalid"), None);
    }

    #[test]
    fn test_professional_config() {
        let config = ExportTemplate::Professional.config();
        assert_eq!(config.primary_color, (0, 51, 102));
        assert_eq!(config.header_font_size, 18);
        assert!(config.include_photos);
    }

    #[test]
    fn test_luxury_config() {
        let config = ExportTemplate::Luxury.config();
        assert_eq!(config.primary_color, (139, 115, 85));
        assert_eq!(config.header_font_size, 22);
        assert!(config.include_photos);
    }

    #[test]
    fn test_minimal_config() {
        let config = ExportTemplate::Minimal.config();
        assert_eq!(config.primary_color, (0, 0, 0));
        assert_eq!(config.header_font_size, 16);
        assert!(!config.include_photos); // Minimal excludes photos
    }
}
