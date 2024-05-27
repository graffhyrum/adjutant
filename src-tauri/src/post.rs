pub mod post {
    pub struct Post {
        text: String,
        images: Vec<String>, // URLs or paths to images
        videos: Vec<String>, // URLs or paths to videos
        emojis: Vec<String>, // Emoji characters or shortcodes
        platforms: Vec<String>, // Social media platforms
    }
}