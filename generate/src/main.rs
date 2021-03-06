use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct Feed {
    pub country_iso: String,
    pub license: Option<String>,
    pub attribution: String,
    pub feed_url: url::Url,
    pub info_url: url::Url,
    pub comments: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FormattedFeed {
    pub country_flag: String,
    pub country_name: String,
    pub country_iso: String,
    pub license: String,
    pub attribution: String,
    pub feed_url: url::Url,
    pub info_url: url::Url,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Feeds<F> {
    pub gtfs_feeds: Vec<F>,
    pub netex_feeds: Vec<F>,
}

fn format_feeds(feeds: &Vec<Feed>) -> Vec<FormattedFeed> {
    return feeds
        .iter()
        .map(|feed| FormattedFeed {
            country_flag: country_emoji::code_to_flag(&feed.country_iso.to_uppercase())
                .unwrap()
                .to_string(),
            country_name: country_emoji::code_to_name(&feed.country_iso.to_lowercase())
                .unwrap()
                .to_string(),
            country_iso: feed.country_iso.clone(),
            license: feed.license.clone().unwrap_or("<i>Unknown</i>".to_owned()),
            attribution: feed.attribution.clone(),
            feed_url: feed.feed_url.clone(),
            info_url: feed.info_url.clone(),
            comments: feed.comments.clone().unwrap_or("–".to_owned()),
        })
        .collect();
}

fn main() {
    let feeds_toml = fs::read_to_string("../feeds.toml").unwrap();
    let feed_container: Feeds<Feed> = toml::from_str(&feeds_toml).unwrap();

    let nginx_conf_template_raw = fs::read_to_string("./templates/nginx.conf.mustache").unwrap();
    let html_template_raw = fs::read_to_string("./templates/index.html.mustache").unwrap();
    let map_template_raw = fs::read_to_string("./templates/map.svg.mustache").unwrap();

    let nginx_conf_template = mustache::compile_str(&nginx_conf_template_raw).unwrap();
    let html_template = mustache::compile_str(&html_template_raw).unwrap();
    let map_template = mustache::compile_str(&map_template_raw).unwrap();

    fs::create_dir_all("./output").unwrap();

    let mut formatted_gtfs_feeds = format_feeds(&feed_container.gtfs_feeds);
    let mut formatted_netex_feeds = format_feeds(&feed_container.netex_feeds);

    formatted_gtfs_feeds.sort_by(|a, b| (&a).country_name.cmp(&b.country_name));
    formatted_netex_feeds.sort_by(|a, b| (&a).country_name.cmp(&b.country_name));

    let gtfs_count_before_dedup = formatted_gtfs_feeds.len();
    let netex_count_before_dedup = formatted_netex_feeds.len();

    formatted_gtfs_feeds.dedup_by(|a, b| a.country_name == b.country_name);
    formatted_netex_feeds.dedup_by(|a, b| a.country_name == b.country_name);

    if gtfs_count_before_dedup != formatted_gtfs_feeds.len() {
        panic!("multiple gtfs feeds found for the same country")
    }
    if netex_count_before_dedup != formatted_netex_feeds.len() {
        panic!("multiple netex feeds found for the same country")
    }

    let mut nginx_conf = fs::File::create("./output/nginx.conf").unwrap();
    nginx_conf_template
        .render(
            &mut nginx_conf,
            &Feeds::<FormattedFeed> {
                gtfs_feeds: (&formatted_gtfs_feeds).clone(),
                netex_feeds: (&formatted_netex_feeds).clone(),
            },
        )
        .unwrap();

    let mut html = fs::File::create("./output/index.html").unwrap();
    html_template
        .render(
            &mut html,
            &Feeds::<FormattedFeed> {
                gtfs_feeds: (&formatted_gtfs_feeds).clone(),
                netex_feeds: (&formatted_netex_feeds).clone(),
            },
        )
        .unwrap();

    let gtfs_countries: HashMap<String, bool> = (&formatted_gtfs_feeds)
        .iter()
        .map(|feed| ((&feed).country_iso.to_lowercase().clone(), true))
        .collect();
    let mut gtfs_map = fs::File::create("./output/gtfs.map.svg").unwrap();
    map_template.render(&mut gtfs_map, &gtfs_countries).unwrap();

    let netex_countries: HashMap<String, bool> = (&formatted_netex_feeds)
        .iter()
        .map(|feed| ((&feed).country_iso.to_lowercase().clone(), true))
        .collect();
    let mut netex_map = fs::File::create("./output/netex.map.svg").unwrap();
    map_template
        .render(&mut netex_map, &netex_countries)
        .unwrap();
}
