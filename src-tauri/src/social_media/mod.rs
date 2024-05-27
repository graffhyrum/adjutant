pub mod twitter;
pub mod threads;
pub mod tumblr;
pub mod bluesky;
pub mod discord;


pub mod social_media {
    use crate::post::post::Post;
    use crate::social_media::twitter::twitter::TwitterAPI;

    pub trait SocialMediaAPI {
        fn login(&self) -> Result<(), String> {
            // Implement the login logic here
            Ok(())
        }

        fn logout(&self) -> Result<(), String> {
            // Implement the logout logic here
            Ok(())
        }

        fn post(&self, post: &Post) -> Result<(), String>;

        fn delete(&self, post_id: &str) -> Result<(), String> {
            // Implement the delete logic here
            Ok(())
        }
    }

    impl SocialMediaAPI for TwitterAPI {
        fn post(&self, post: &crate::post::Post) -> Result<(), String> {
            // Implement the API call here
            Ok(())
        }
    }

    impl SocialMediaAPI for crate::social_media::threads::ThreadsAPI {
        fn post(&self, post: &crate::post::Post) -> Result<(), String> {
            // Implement the API call here
            Ok(())
        }

    }

    impl SocialMediaAPI for crate::social_media::tumblr::TumblrAPI {
        fn post(&self, post: &crate::post::Post) -> Result<(), String> {
            // Implement the API call here
            Ok(())
        }
    }

    impl SocialMediaAPI for crate::social_media::bluesky::BlueskyAPI {
        fn post(&self, post: &crate::post::Post) -> Result<(), String> {
            // Implement the API call here
            Ok(())
        }
    }

    impl SocialMediaAPI for crate::social_media::discord::DiscordAPI {
        fn post(&self, post: &crate::post::Post) -> Result<(), String> {
            // Implement the API call here
            Ok(())
        }
    }


}