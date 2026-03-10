use crate::db::properties::Property;

pub struct AgentInfo {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub brokerage: String,
}

pub struct GenerationOptions {
    pub style: String,
    pub tone: String,
    pub length: String,
    pub seo_keywords: Vec<String>,
}

/// Build the property analysis prompt (Stage 1 - non-streaming)
pub fn build_analysis_prompt(property: &Property) -> (String, String) {
    let system = r#"You are a real estate market analyst. Given property details, produce a JSON analysis with exactly these fields:
- "selling_points": array of 5 strings, each a specific compelling feature (not generic)
- "target_buyer": string describing the ideal buyer persona in 1-2 sentences
- "neighborhood_appeal": string describing what makes the location desirable in 1-2 sentences
- "comparable_positioning": string describing how to position this property vs typical listings in the area
- "emotional_hooks": array of 3 strings, each an emotional angle for marketing

Respond with ONLY valid JSON. No markdown, no explanation."#
        .to_string();

    let key_features: Vec<String> =
        serde_json::from_str(&property.key_features).unwrap_or_default();
    let highlights: Vec<String> =
        serde_json::from_str(&property.neighborhood_highlights).unwrap_or_default();
    let amenities: Vec<String> =
        serde_json::from_str(&property.nearby_amenities).unwrap_or_default();

    let price_formatted = format_price(property.price);

    let user = format!(
        "Property: {}, {}, {} {}\nType: {} | Built: {}\n{} bed / {} bath / {} sqft\nPrice: ${}\nLot: {} | Parking: {}\nKey Features: {}\nNeighborhood: {}\nHighlights: {}\nSchools: {}\nNearby: {}",
        property.address,
        property.city,
        property.state,
        property.zip,
        property.property_type.replace('_', " "),
        property.year_built.map_or("N/A".to_string(), |y| y.to_string()),
        property.beds,
        property.baths,
        property.sqft,
        price_formatted,
        property.lot_size.as_deref().unwrap_or("N/A"),
        property.parking.as_deref().unwrap_or("N/A"),
        key_features.join(", "),
        property.neighborhood.as_deref().unwrap_or("N/A"),
        highlights.join(", "),
        property.school_district.as_deref().unwrap_or("N/A"),
        amenities.join(", "),
    );

    (system, user)
}

/// Build the listing generation prompt (Stage 2 - streaming)
pub fn build_listing_prompt(
    property: &Property,
    analysis_json: &str,
    options: &GenerationOptions,
    brand_voice_block: Option<&str>,
    agent_info: &AgentInfo,
) -> (String, String) {
    let style_instructions = get_style_instructions(&options.style);
    let tone_instructions = get_tone_instructions(&options.tone);
    let length_instructions = get_length_instructions(&options.length);

    let brand_block = brand_voice_block.unwrap_or("");
    let seo_keywords = if options.seo_keywords.is_empty() {
        "none specified".to_string()
    } else {
        options.seo_keywords.join(", ")
    };

    let agent_cta = if !agent_info.name.is_empty() {
        format!(
            "Agent: {} | {} | {} | {}",
            agent_info.name, agent_info.phone, agent_info.email, agent_info.brokerage
        )
    } else {
        String::new()
    };

    let system = format!(
        r#"You are an expert real estate copywriter who writes listing descriptions that drive showing requests. Your descriptions are specific, vivid, and never generic.

STYLE: {style_instructions}
TONE: {tone_instructions}
LENGTH: {length_instructions}

{brand_block}

RULES:
- NEVER fabricate features not provided in the property data
- NEVER use these overused phrases: "welcome home", "must see", "won't last long", "priced to sell", "hidden gem"
- NEVER reference protected classes (race, religion, national origin, familial status, disability, sex)
- DO use specific details from the property data — exact features, neighborhood names, school names
- DO front-load the most compelling information in the first sentence
- DO naturally incorporate these SEO keywords where they fit: {seo_keywords}
- DO end with a clear call to action including agent contact info if provided
- Write in second person ("you'll love") or third person ("this home features"), not first person

OUTPUT: Write ONLY the listing description. No titles, no labels, no markdown formatting."#
    );

    let key_features: Vec<String> =
        serde_json::from_str(&property.key_features).unwrap_or_default();
    let highlights: Vec<String> =
        serde_json::from_str(&property.neighborhood_highlights).unwrap_or_default();
    let amenities: Vec<String> =
        serde_json::from_str(&property.nearby_amenities).unwrap_or_default();
    let price_formatted = format_price(property.price);

    let user = format!(
        r#"PROPERTY DATA:
Address: {}, {}, {} {}
Type: {} | Built: {} | {} bed / {} bath / {} sqft
Price: ${}
Lot: {} | Parking: {}
Key Features: {}
Neighborhood: {}
Highlights: {}
Schools: {}
Nearby: {}
{}

MARKET ANALYSIS:
{}

Write the listing description now."#,
        property.address,
        property.city,
        property.state,
        property.zip,
        property.property_type.replace('_', " "),
        property
            .year_built
            .map_or("N/A".to_string(), |y| y.to_string()),
        property.beds,
        property.baths,
        property.sqft,
        price_formatted,
        property.lot_size.as_deref().unwrap_or("N/A"),
        property.parking.as_deref().unwrap_or("N/A"),
        key_features.join(", "),
        property.neighborhood.as_deref().unwrap_or("N/A"),
        highlights.join(", "),
        property.school_district.as_deref().unwrap_or("N/A"),
        amenities.join(", "),
        agent_cta,
        analysis_json,
    );

    (system, user)
}

/// Build social media generation prompt
pub fn build_social_prompt(
    property: &Property,
    analysis_json: &str,
    platform: &str,
    brand_voice_block: Option<&str>,
    agent_info: &AgentInfo,
) -> (String, String) {
    let platform_instructions = get_platform_instructions(platform);
    let brand_block = brand_voice_block.unwrap_or("");

    let agent_cta = if !agent_info.name.is_empty() {
        format!(
            "{}, {} | {} | {}",
            agent_info.name, agent_info.phone, agent_info.email, agent_info.brokerage
        )
    } else {
        String::new()
    };

    let system = format!(
        r#"You are a social media marketing expert specializing in real estate. Create {platform}-optimized posts for this property listing.

{platform_instructions}

{brand_block}

RULES:
- NEVER fabricate features not in the property data
- Each post should take a different angle (lifestyle, features, neighborhood, investment value, urgency)
- Include a clear CTA with agent contact info if provided: {agent_cta}
- Write the exact number of posts requested

OUTPUT FORMAT:
---POST 1---
{{post content including hashtags}}
---POST 2---
{{post content including hashtags}}
---POST 3---
{{post content including hashtags}}"#
    );

    let key_features: Vec<String> =
        serde_json::from_str(&property.key_features).unwrap_or_default();
    let price_formatted = format_price(property.price);

    let user = format!(
        "Property: {}, {}, {} {}\n{} bed / {} bath / {} sqft | ${}\nFeatures: {}\nNeighborhood: {}\n\nAnalysis: {}",
        property.address, property.city, property.state, property.zip,
        property.beds, property.baths, property.sqft, price_formatted,
        key_features.join(", "),
        property.neighborhood.as_deref().unwrap_or("N/A"),
        analysis_json,
    );

    (system, user)
}

/// Build email generation prompt
pub fn build_email_prompt(
    property: &Property,
    analysis_json: &str,
    template_type: &str,
    brand_voice_block: Option<&str>,
    agent_info: &AgentInfo,
) -> (String, String) {
    let template_instructions = get_template_instructions(template_type);
    let brand_block = brand_voice_block.unwrap_or("");

    let system = format!(
        r#"You are an email marketing specialist for real estate. Write a {template_type} email.

{template_instructions}

{brand_block}

RULES:
- Subject line: 6-10 words, creates curiosity or urgency, no ALL CAPS, no spam trigger words
- Preview text: 40-90 characters, complements subject line (shown in inbox preview)
- Body: Personal, actionable, scannable (short paragraphs, bold key details)
- Include property details naturally, don't dump raw data
- End with one clear CTA (not multiple competing CTAs)
- Sign off with agent name: {}, phone: {}, email: {}

OUTPUT FORMAT:
SUBJECT: {{subject line}}
PREVIEW: {{preview text}}
---
{{email body}}"#,
        agent_info.name, agent_info.phone, agent_info.email
    );

    let key_features: Vec<String> =
        serde_json::from_str(&property.key_features).unwrap_or_default();
    let price_formatted = format_price(property.price);

    let user =
        format!(
        "Property: {}, {}, {} {}\n{} bed / {} bath / {} sqft | ${}\nFeatures: {}\n\nAnalysis: {}",
        property.address, property.city, property.state, property.zip,
        property.beds, property.baths, property.sqft, price_formatted,
        key_features.join(", "),
        analysis_json,
    );

    (system, user)
}

/// Build brand voice extraction prompt
pub fn build_voice_extraction_prompt(sample_listings: &[String]) -> (String, String) {
    let system = r#"You are a linguistic analyst specializing in writing style extraction. Analyze the following real estate listing descriptions written by the same agent and extract their unique writing voice.

OUTPUT: Respond with ONLY valid JSON matching this exact structure:
{
  "tone": "1-2 sentence description of overall tone",
  "vocabulary": ["array", "of", "10-15", "distinctive", "words/phrases"],
  "sentence_patterns": "Description of typical sentence structure, length, use of questions/exclamations",
  "themes": ["array", "of", "3-5", "recurring", "themes"],
  "signature_phrases": ["exact", "phrases", "they", "reuse"],
  "avoids": ["words", "or", "patterns", "they", "never", "use"],
  "formatting": "Description of how they structure descriptions"
}"#
        .to_string();

    let mut user =
        String::from("Analyze these listing descriptions by the same real estate agent:\n\n");
    for (i, listing) in sample_listings.iter().enumerate() {
        user.push_str(&format!("LISTING {}:\n{}\n\n", i + 1, listing));
    }

    (system, user)
}

/// Build the brand voice injection block for generation prompts
pub fn build_voice_block(extracted_style_json: &str) -> Option<String> {
    let style: serde_json::Value = serde_json::from_str(extracted_style_json).ok()?;

    Some(format!(
        r#"BRAND VOICE: Match this writing style:
- Tone: {}
- Vocabulary preferences: {}
- Sentence patterns: {}
- Signature themes: {}
- Example phrases to emulate: {}
Maintain this voice while following all other instructions."#,
        style["tone"].as_str().unwrap_or(""),
        style["vocabulary"]
            .as_array()
            .map(|v| v
                .iter()
                .filter_map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", "))
            .unwrap_or_default(),
        style["sentence_patterns"].as_str().unwrap_or(""),
        style["themes"]
            .as_array()
            .map(|v| v
                .iter()
                .filter_map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", "))
            .unwrap_or_default(),
        style["signature_phrases"]
            .as_array()
            .map(|v| v
                .iter()
                .filter_map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", "))
            .unwrap_or_default(),
    ))
}

fn get_style_instructions(style: &str) -> &str {
    match style {
        "luxury" => "Write for affluent buyers who value exclusivity, craftsmanship, and lifestyle. Use sophisticated language: \"bespoke\", \"curated\", \"impeccable\", \"artisan\". Emphasize unique/custom elements, premium materials, and the lifestyle the home enables. Longer sentences, aspirational imagery.",
        "family" => "Write for families prioritizing space, safety, schools, and community. Emphasize practical features: storage, yard space, bedroom count, school proximity, family-friendly neighborhood. Use warm, inviting language: \"spacious\", \"sun-filled\", \"gathering\", \"grow\". Focus on daily life and making memories.",
        "investment" => "Write for investors focused on ROI, rental potential, and appreciation. Lead with numbers and market position. Emphasize: rental income potential, neighborhood growth trajectory, low maintenance, strong tenant demand, proximity to employment centers. Practical, data-driven language.",
        "first_time" => "Write for first-time buyers who are excited but cautious about affordability and maintenance. Emphasize move-in readiness, value, and low-effort living. Use encouraging language: \"ideal starter\", \"turnkey\", \"manageable\", \"well-maintained\". Address common concerns: condition, costs, neighborhood safety.",
        _ => "Write a professional, well-crafted listing description.",
    }
}

fn get_tone_instructions(tone: &str) -> &str {
    match tone {
        "professional" => "Authoritative and polished. Clean sentence structure, industry terminology used naturally, confident assertions. No exclamation marks. Measured enthusiasm.",
        "warm" => "Conversational and inviting. Help readers picture themselves living there. Use sensory details (morning light, quiet streets, the sound of...). Occasional questions to engage. Balanced enthusiasm.",
        "exciting" => "High energy, action-oriented. Dynamic verbs, vivid imagery, enthusiastic punctuation (sparingly). Create urgency through desirability, not pressure tactics. Paint an aspirational picture.",
        _ => "Professional and engaging tone.",
    }
}

fn get_length_instructions(length: &str) -> &str {
    match length {
        "short" => "100-150 words. One punchy paragraph. Best for: MLS systems with character limits (~500-750 chars), quick social media repurposing.",
        "medium" => "200-300 words. 2-3 paragraphs. Best for: standard MLS listings, Zillow/Realtor.com descriptions.",
        "long" => "400-500 words. 4-5 paragraphs with a narrative arc (hook -> features -> lifestyle -> neighborhood -> CTA). Best for: luxury listings, agent websites, marketing packages.",
        _ => "200-300 words. 2-3 paragraphs.",
    }
}

fn get_platform_instructions(platform: &str) -> &str {
    match platform {
        "instagram" => "Write 3 Instagram captions. CRITICAL: First 125 characters must be the hook (visible before \"...more\" cutoff). Total caption under 2,200 characters. Include 5-8 relevant hashtags at the end. Mix broad (#realestate #homesforsale) with specific ones. Use line breaks for readability. Emojis are acceptable but don't overdo it.",
        "facebook" => "Write 3 Facebook posts. No character limit but keep to 100-200 words for engagement. Use a conversational tone. Include a strong CTA (\"Comment below\", \"Send me a DM\", \"Link in comments\"). One post should be a question format to drive engagement. No hashtags (they don't help on Facebook).",
        "linkedin" => "Write 3 LinkedIn posts. Professional tone, 150-250 words. Position the agent as a market expert, not just listing a property. Include 3-5 relevant hashtags. One post should include a market insight or educational angle. Focus on investment value and market positioning.",
        _ => "Write 3 social media posts optimized for engagement.",
    }
}

fn get_template_instructions(template: &str) -> &str {
    match template {
        "buyer" => "This email goes to a potential buyer whose search criteria match this property. Open with why this property fits their needs. Highlight 3-4 key features that match typical buyer criteria. Create soft urgency (\"just listed\", \"early access\"). CTA: \"Schedule a private showing\" or \"Reply to learn more\".",
        "seller" => "This email goes to a potential seller in the same neighborhood. Use this listing as social proof of market activity. Reference the neighborhood by name. Highlight the sale price / market conditions. CTA: \"Curious what your home is worth?\" or \"Free comparative market analysis\".",
        "open_house" => "This email invites potential buyers to an open house. Include: date, time, address, 3 property highlights, what refreshments/experience to expect. Create excitement. CTA: \"RSVP\" or \"Add to calendar\". Make the open house sound like an event, not a chore.",
        _ => "Write a professional real estate email.",
    }
}

fn format_price(price_cents: i64) -> String {
    let dollars = price_cents / 100;
    let mut s = dollars.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.drain(..).collect();
    for (i, c) in chars.iter().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.insert(0, ',');
        }
        result.insert(0, *c);
    }
    result
}

/// Get max_tokens for a generation type and length
pub fn max_tokens_for_listing(length: &str) -> u32 {
    match length {
        "short" => 1024,
        "medium" => 2048,
        "long" => 4096,
        _ => 2048,
    }
}

pub const MAX_TOKENS_ANALYSIS: u32 = 1024;
pub const MAX_TOKENS_SOCIAL: u32 = 2048;
pub const MAX_TOKENS_EMAIL: u32 = 2048;
pub const MAX_TOKENS_BRAND_VOICE: u32 = 2048;

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_property() -> Property {
        Property {
            id: "test-id".to_string(),
            address: "123 Oak Street".to_string(),
            city: "San Francisco".to_string(),
            state: "CA".to_string(),
            zip: "94105".to_string(),
            beds: 3,
            baths: 2.5,
            sqft: 1800,
            price: 95000000,
            property_type: "single_family".to_string(),
            year_built: Some(2015),
            lot_size: Some("0.25 acres".to_string()),
            parking: Some("2-car garage".to_string()),
            key_features: r#"["hardwood floors","chef's kitchen","pool"]"#.to_string(),
            neighborhood: Some("Mission Bay".to_string()),
            neighborhood_highlights: r#"["Walk Score 95"]"#.to_string(),
            school_district: Some("SFUSD".to_string()),
            nearby_amenities: r#"["Whole Foods 0.3mi"]"#.to_string(),
            agent_notes: None,
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        }
    }

    #[test]
    fn test_build_analysis_prompt() {
        let property = sample_property();
        let (system, user) = build_analysis_prompt(&property);

        assert!(system.contains("real estate market analyst"));
        assert!(system.contains("selling_points"));
        assert!(user.contains("123 Oak Street"));
        assert!(user.contains("San Francisco"));
        assert!(user.contains("3 bed"));
        assert!(user.contains("950,000"));
    }

    #[test]
    fn test_build_listing_prompt_all_styles() {
        let property = sample_property();
        let analysis = r#"{"selling_points":["test"]}"#;
        let agent = AgentInfo {
            name: "Jane Smith".to_string(),
            phone: "555-1234".to_string(),
            email: "jane@example.com".to_string(),
            brokerage: "RE/MAX".to_string(),
        };

        for style in &["luxury", "family", "investment", "first_time"] {
            let options = GenerationOptions {
                style: style.to_string(),
                tone: "warm".to_string(),
                length: "medium".to_string(),
                seo_keywords: vec!["waterfront".to_string()],
            };
            let (system, user) = build_listing_prompt(&property, analysis, &options, None, &agent);
            assert!(system.contains("expert real estate copywriter"));
            assert!(system.contains("waterfront"));
            assert!(user.contains("123 Oak Street"));
        }
    }

    #[test]
    fn test_build_social_prompt() {
        let property = sample_property();
        let agent = AgentInfo {
            name: "Jane".to_string(),
            phone: "555".to_string(),
            email: "j@e.com".to_string(),
            brokerage: "RE".to_string(),
        };

        let (system, _) = build_social_prompt(&property, "{}", "instagram", None, &agent);
        assert!(system.contains("Instagram"));
        assert!(system.contains("hashtags"));

        let (system, _) = build_social_prompt(&property, "{}", "facebook", None, &agent);
        assert!(system.contains("Facebook"));

        let (system, _) = build_social_prompt(&property, "{}", "linkedin", None, &agent);
        assert!(system.contains("LinkedIn"));
    }

    #[test]
    fn test_build_email_prompt() {
        let property = sample_property();
        let agent = AgentInfo {
            name: "Jane".to_string(),
            phone: "555".to_string(),
            email: "j@e.com".to_string(),
            brokerage: "RE".to_string(),
        };

        let (system, _) = build_email_prompt(&property, "{}", "buyer", None, &agent);
        assert!(system.contains("SUBJECT:"));

        let (system, _) = build_email_prompt(&property, "{}", "seller", None, &agent);
        assert!(system.contains("email"));
    }

    #[test]
    fn test_format_price() {
        assert_eq!(format_price(95000000), "950,000");
        assert_eq!(format_price(125000000), "1,250,000");
        assert_eq!(format_price(50000), "500");
    }

    #[test]
    fn test_build_voice_block() {
        let style = r#"{"tone":"Warm and conversational","vocabulary":["stunning","impeccable"],"sentence_patterns":"Short punchy sentences","themes":["luxury","lifestyle"],"signature_phrases":["dream home"]}"#;
        let block = build_voice_block(style).unwrap();
        assert!(block.contains("Warm and conversational"));
        assert!(block.contains("stunning"));
    }

    #[test]
    fn test_max_tokens() {
        assert_eq!(max_tokens_for_listing("short"), 1024);
        assert_eq!(max_tokens_for_listing("medium"), 2048);
        assert_eq!(max_tokens_for_listing("long"), 4096);
    }
}
