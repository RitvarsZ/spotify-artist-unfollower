use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth, model::{FullArtist}};

#[tokio::main]
async fn main() {
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("user-follow-read", "user-follow-modify")).unwrap();
    let mut spotify = AuthCodeSpotify::new(creds, oauth);
    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    let limit = Some(50);
    let mut offset = None;
    let mut next_string: String;

    loop {
        let artists = spotify.current_user_followed_artists(offset, limit).await.unwrap();
        let ids = artists.items.iter().map(|artist| &artist.id);
        
        spotify.user_unfollow_artists(ids).await.unwrap();
        print_names(artists.items);

        if artists.next.is_none() {
            println!("Done!");
            break;
        }

        next_string = artists.cursors.unwrap().after.unwrap().clone();
        offset = Some(next_string.as_str());
    }
}

fn print_names(items: Vec<FullArtist>) {
    for item in items {
        println!("Unfollowed {}", item.name);
    }
}
