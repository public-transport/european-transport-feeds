use comrak::{
    markdown_to_html, ComrakExtensionOptions, ComrakOptions, ComrakParseOptions,
    ComrakRenderOptions,
};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Feed {
    pub country_iso: String,
    pub feed_url: String,
    pub license: String,
    pub attribution: String,
    pub info_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FeedWithMetadata {
    pub feed: Feed,
    pub country_flag: String,
    pub country_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Feeds<F> {
    pub feeds: Vec<F>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Content {
    pub content: String,
}

fn main() {
    let feeds_ndjson = fs::read_to_string("../feeds.ndjson").unwrap();
    let feeds: Vec<Feed> = feeds_ndjson
        .lines()
        .map(|line| serde_json::from_str(&line).unwrap())
        .collect();

    let nginx_conf_template_raw = fs::read_to_string("./templates/nginx.conf.mustache").unwrap();
    let markdown_template_raw = fs::read_to_string("./templates/index.md.mustache").unwrap();
    let html_template_raw = fs::read_to_string("./templates/index.html.mustache").unwrap();

    let nginx_conf_template = mustache::compile_str(&nginx_conf_template_raw).unwrap();
    let markdown_template = mustache::compile_str(&markdown_template_raw).unwrap();
    let html_template = mustache::compile_str(&html_template_raw).unwrap();

    fs::create_dir_all("./output").unwrap();

    let mut nginx_conf = fs::File::create("./output/nginx.conf").unwrap();
    nginx_conf_template
        .render(
            &mut nginx_conf,
            &Feeds::<Feed> {
                feeds: (&feeds).clone(),
            },
        )
        .unwrap();

    let mut feeds_with_metadata: Vec<FeedWithMetadata> = (&feeds)
        .iter()
        .map(|feed| FeedWithMetadata {
            feed: feed.clone(),
            country_flag: country_emoji::code_to_flag(&feed.country_iso.to_uppercase())
                .unwrap()
                .to_string(),
            country_name: country_emoji::code_to_name(&feed.country_iso.to_lowercase())
                .unwrap()
                .to_string(),
        })
        .collect();
    feeds_with_metadata.sort_by(|a, b| (&a).country_name.cmp(&b.country_name));

    let markdown_text = markdown_template
        .render_to_string(&Feeds::<FeedWithMetadata> {
            feeds: (&feeds_with_metadata).clone(),
        })
        .unwrap();

    let formatting_options = ComrakOptions {
        extension: ComrakExtensionOptions {
            strikethrough: true,
            tagfilter: true,
            table: true,
            autolink: true,
            tasklist: true,
            superscript: false,
            header_ids: None,
            footnotes: false,
            description_lists: false,
            front_matter_delimiter: None,
        },
        parse: ComrakParseOptions {
            smart: false,
            default_info_string: None,
        },
        render: ComrakRenderOptions {
            hardbreaks: false,
            github_pre_lang: true,
            width: 0,
            unsafe_: false,
            escape: false,
        },
    };

    let html_text = markdown_to_html(&markdown_text, &formatting_options);
    let mut html = fs::File::create("./output/index.html").unwrap();
    html_template
        .render(
            &mut html,
            &Content {
                content: (&html_text).clone(),
            },
        )
        .unwrap();
}
