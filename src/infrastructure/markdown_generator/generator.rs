use crate::domain::ai::model::AiSummaryResponse;
use crate::domain::database::model::FeedItemEntity;
use markdown_gen::markdown::{AsMarkdown, Link, Markdown};
use regex::Regex;

pub fn summary_generator(summary: &AiSummaryResponse, feeds: &Vec<FeedItemEntity>) -> String {
    let mut buffer = Vec::new();
    let mut md = Markdown::new(&mut buffer);

    // Header
    md.write(summary.global_title.heading(1)).unwrap();
    md.write(summary.introduction.as_str()).unwrap();

    md.write("Ce résumé a été généré via l'intelligence artificielle.".italic())
        .unwrap();

    // Content
    let re_articles = Regex::new(r"\[\d+\]").unwrap();
    for section in &summary.sections {
        // Replace article number by link
        let new_content =
            re_articles.replace_all(section.content.as_str(), |caps: &regex::Captures| {
                let article_id_str = &caps[1];

                // Convert id in to number and find it in feeds items collections
                let is_match = |f: &&FeedItemEntity| match article_id_str.parse::<i64>() {
                    Ok(id_num) => f.id == id_num,
                    Err(_) => false,
                };

                if let Some(feed) = feeds.iter().find(|f| is_match(f)) {
                    format!("[lien]({})", feed.url.clone().unwrap())
                } else {
                    "*article indisponible*".to_string()
                }
            });

        md.write(section.sub_title.heading(3)).unwrap();
        md.write(new_content.into_owned().as_str()).unwrap();

        section.source_ids.iter().for_each(|id| {
            let link = feeds.iter().find(|f| f.id == *id);
            if let Some(item) = link {
                md.write(Link::new(item.url.clone().unwrap().as_str()).append(item.title.as_str()))
                    .unwrap();
                md.write("\n").unwrap();
            }
        })
    }
    String::from_utf8(buffer).unwrap()
}
