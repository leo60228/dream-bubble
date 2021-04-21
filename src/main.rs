// vi: set sw=4:

use chrono::prelude::*;
use dream_bubble::site_updates::{SiteFile, SiteUpdates};
use rocket::http::uri::Segments;
use rocket::http::ContentType;
use rocket::response::content::{Content, Html, Json};
use rocket::response::Redirect;
use rocket::{get, launch, routes, State};

#[get("/")]
async fn index(updates: State<'_, SiteUpdates>) -> Html<Vec<u8>> {
    let url = updates
        .file_at(SiteFile::Html, Utc.ymd(2020, 9, 8).and_hms(0, 0, 0))
        .unwrap()
        .unwrap();

    Html(
        reqwest::get(url)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap()
            .to_vec(),
    )
}

#[get("/<path..>", rank = 1000)]
async fn static_asset(
    path: Segments<'_>,
    updates: State<'_, SiteUpdates>,
) -> Option<Content<Vec<u8>>> {
    if path.get(0) == Some("database") || path.get(0) == Some("api") {
        return None;
    }

    let segments: Vec<_> = path.into_iter().collect();
    let path = format!("/{}", segments.join("/"));

    let ct = ContentType::from_extension(path.rsplit('.').next().unwrap()).unwrap();

    let url = updates
        .path_at(&path, Utc.ymd(2020, 9, 11).and_hms(17, 0, 0))
        .unwrap()?;

    Some(Content(
        ct,
        reqwest::get(url)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap()
            .to_vec(),
    ))
}

#[get("/events/streamData")]
fn stream_data() -> Redirect {
    Redirect::temporary("https://api-test.sibr.dev/replay/v1/replay?from=2020-09-08T00:00:00Z")
}

#[get("/api/getUser")]
fn user() -> Json<&'static str> {
    Json(
        r#"
{
  "id": "00000000-0000-0000-0000-000000000000",
  "email": "user@example.com",
  "appleId": null,
  "googleId": null,
  "facebookId": null,
  "name": null,
  "password": null,
  "coins": 1000,
  "lastActive": "1970-01-01T00:00:00Z",
  "created": "1970-01-01T00:00:00Z",
  "loginStreak": 0,
  "favoriteTeam": "8d87c468-699a-47a8-b40d-cfb73a5660ad",
  "unlockedShop": true,
  "unlockedElection": true,
  "peanutsEaten": 0,
  "squirrels": 0,
  "idol": "c0732e36-3731-4f1a-abdc-daa9563b6506",
  "snacks": {
    "Max_Bet": 98
  },
  "lightMode": false,
  "packSize": 8,
  "spread": [],
  "coffee": 0,
  "favNumber": 0,
  "snackOrder": [
    "Max_Bet"
  ],
  "trackers": {
    "BEGS": 0,
    "BETS": 0,
    "VOTES_CAST": 0,
    "SNACKS_BOUGHT": 1,
    "SNACK_UPGRADES": 98
  },
  "votes": 1,
  "peanuts": 10
}
    "#,
    )
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .manage(SiteUpdates::fetch().await.unwrap())
        .mount("/", routes![index, static_asset, stream_data, user])
}
