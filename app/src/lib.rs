use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use chrono::{DateTime, Utc};
use handler::internal_error;
use sqlx::PgPool;

pub mod cli;
pub mod database;
pub mod handler;
pub mod render;
pub mod schema;
pub mod templates;

pub const SITE_BASE_URL: &str = "https://jorianwoltjer.com";

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub hmac_key: [u8; 32],
}

pub fn html_template(template: impl Template) -> Result<impl IntoResponse, StatusCode> {
    Ok(Html(template.render().map_err(internal_error)?))
}

pub fn relative_time(timestamp: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = if now > *timestamp {
        now - timestamp
    } else {
        *timestamp - now
    };

    let (value, unit) = if duration.num_seconds() < 60 {
        (duration.num_seconds(), "second")
    } else if duration.num_minutes() < 60 {
        (duration.num_minutes(), "minute")
    } else if duration.num_hours() < 24 {
        (duration.num_hours(), "hour")
    } else if duration.num_days() < 30 {
        (duration.num_days(), "day")
    } else if duration.num_days() < 365 {
        (duration.num_days() / 30, "month")
    } else {
        (duration.num_days() / 365, "year")
    };
    let plural = if value != 1 { "s" } else { "" };

    if now > *timestamp {
        format!("{} {}{} ago", value, unit, plural)
    } else {
        format!("in {} {}{}", value, unit, plural)
    }
}

pub fn get_domain(url: &str) -> String {
    url.split('/')
        .nth(2)
        .map(|domain| domain.to_string())
        .unwrap_or_default()
}

pub fn breadcrumbs_from_slug(slug: &str) -> Vec<(String, String)> {
    let mut breadcrumbs = Vec::new();
    let mut current_slug = String::new();
    let split = slug.split('/').collect::<Vec<_>>();
    for part in split.iter().take(split.len() - 1) {
        if !current_slug.is_empty() {
            current_slug.push('/');
        }
        current_slug.push_str(part);
        breadcrumbs.push((current_slug.clone(), part.to_string()));
    }
    breadcrumbs
}

pub fn cdata_escape(input: &str) -> String {
    format!("<![CDATA[{}]]>", input.replace("]]>", "]]]]><![CDATA[>"))
}

pub async fn extend_slug(
    slug: &str,
    folder_id: i32,
    state: &AppState,
) -> Result<String, sqlx::Error> {
    sqlx::query!("SELECT slug FROM folders WHERE id = $1", folder_id)
        .fetch_one(&state.db)
        .await
        .map(|parent| format!("{}/{slug}", parent.slug))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_seconds_ago() {
        let timestamp = Utc::now() - Duration::seconds(45);
        assert_eq!(relative_time(&timestamp), "45 seconds ago");
    }

    #[test]
    fn test_in_seconds() {
        let timestamp = Utc::now() + Duration::seconds(30);
        assert_eq!(relative_time(&timestamp), "in 29 seconds");
    }

    #[test]
    fn test_one_minute_ago() {
        let timestamp = Utc::now() - Duration::minutes(1);
        assert_eq!(relative_time(&timestamp), "1 minute ago");
    }

    #[test]
    fn test_hours_ago() {
        let timestamp = Utc::now() - Duration::hours(5);
        assert_eq!(relative_time(&timestamp), "5 hours ago");
    }

    #[test]
    fn test_days_ago() {
        let timestamp = Utc::now() - Duration::days(10);
        assert_eq!(relative_time(&timestamp), "10 days ago");
    }

    #[test]
    fn test_months_ago() {
        let timestamp = Utc::now() - Duration::days(90);
        assert_eq!(relative_time(&timestamp), "3 months ago");
    }

    #[test]
    fn test_years_ago() {
        let timestamp = Utc::now() - Duration::days(730); // 2 years
        assert_eq!(relative_time(&timestamp), "2 years ago");
    }

    #[test]
    fn test_future_day() {
        let timestamp = Utc::now() + Duration::days(1);
        assert_eq!(relative_time(&timestamp), "in 23 hours");
    }

    #[test]
    fn test_domain_extraction() {
        assert_eq!(
            get_domain("https://example.com/path/to/resource"),
            "example.com"
        );
        assert_eq!(
            get_domain("http://subdomain.example.com/another/path"),
            "subdomain.example.com"
        );
        assert_eq!(
            get_domain("https://nopath.example.com"),
            "nopath.example.com"
        );
        assert_eq!(get_domain("not-a-url"), "");
    }
}
