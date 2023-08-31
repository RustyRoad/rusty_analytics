use std::{self, collections::HashMap};

#[derive(Debug)]
pub struct User {
    age: Option<u8>, // age could be optional, as not all platforms/users might provide this information
    location: String,
    gender: Gender,
    profession: String,
    education_level: Education,
}

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Other,
    PreferNotToSay,
}

#[derive(Debug)]
pub enum Education {
    HighSchool,  // Completed high school
    SomeCollege, // Went to college but didn't complete a degree
    College,     // Completed college with a degree
    PostGrad,    // Completed post graduation
    Unknown,     // Education details are not provided
}

#[derive(Debug)]
pub struct Campaign {
    name: String,
    start_date: String, // These could be more sophisticated date types in practice.
    end_date: String,
    total_budget: f64,
    daily_budget: f64,
}

#[derive(Debug)]
pub struct AdGroup {
    campaign: Campaign,
    name: String,
    keywords: Vec<String>,
}

#[derive(Debug)]
pub struct FacebookAd {
    id: String,
    account_id: String,
    adset_id: String,
    campaign_id: String,
    name: String,
    creative: Creative,
    status: String,
    effective_authorization_category: Option<String>,
    effective_instagram_media_id: Option<String>,
    effective_instagram_story_id: Option<String>,
    image_crops: Option<ImageCrops>,
    image_hash: Option<String>,
    image_url: Option<String>,
    instagram_actor_id: Option<String>,
    object_id: Option<String>,
    link_url: Option<String>,
    platform_customizations: Option<PlatformCustomizations>,
    product_set_id: Option<String>,
    title: Option<String>,
    url_tags: Option<String>,
    video_id: Option<String>,
}

#[derive(Debug)]
pub struct Creative {
    id: String,
    url_tags: String,
    object_story_spec: ObjectStorySpec,
    object_type: Option<String>,
    title: Option<String>,
    body: Option<String>,
    image_hash: Option<String>,
    image_url: Option<String>,
    link_og_id: Option<String>,
    name: Option<String>,
    object_story_id: Option<String>,
}

#[derive(Debug)]
pub struct ObjectStorySpec {
    page_id: String,
    link_data: LinkData,
}

#[derive(Debug)]
pub struct LinkData {
    link: Option<String>,
    message: Option<String>,
    image_hash: Option<String>,
}

#[derive(Debug)]
pub struct ImageCrops {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Debug)]
pub struct PlatformCustomizations {
    platform: String,
    changes: Vec<Change>,
}

#[derive(Debug)]
pub struct Change {
    field: String,
    new_value: String,
}

#[derive(Debug)]
pub struct Session {
    duration: f64,
    bounced: bool,
    // other fields
}

#[derive(Debug)]
pub struct Conversion {
    converted: bool,
    // other fields
}

pub fn report_campaign_performance(
    campaign: Campaign,
    sessions: Vec<Session>,
    conversions: Vec<Conversion>,
) {
    // Generate a report given campaign and associated sessions and conversions
}

/// Calculate and return the bounce rate
pub fn bounce_rate(sessions: Vec<Session>) -> f64 {
    let total_sessions = sessions.len();
    let bounced_sessions = sessions.iter().filter(|session| session.bounced).count();
    let bounce_rate = (bounced_sessions as f64) / (total_sessions as f64);
    bounce_rate
}

/// Calculate and return the conversion rate
pub fn conversion_rate(conversions: Vec<Conversion>) -> f64 {
    let total_conversions = conversions.len();
    let converted_sessions = conversions
        .iter()
        .filter(|conversion| conversion.converted)
        .count();
    let conversion_rate = (converted_sessions as f64) / (total_conversions as f64);
    conversion_rate
}

pub fn segment_users_by_age(users: Vec<User>) -> HashMap<String, Vec<User>> {
    // Segment users into groups based on demographic/behavior data
    let mut user_segments: HashMap<String, Vec<User>> = HashMap::new();
    for user in users {
        let segment = match user.age {
            Some(age) => {
                if age < 18 {
                    "Under 18"
                } else if age < 25 {
                    "18-24"
                } else if age < 35 {
                    "25-34"
                } else if age < 45 {
                    "35-44"
                } else if age < 55 {
                    "45-54"
                } else {
                    "55+"
                }
            }
            None => "Unknown",
        };
        let segment_users = user_segments
            .entry(segment.to_string())
            .or_insert(Vec::new());
        segment_users.push(user);
    }
    user_segments
}





fn main() {
    println!("Hello, world!");
}
