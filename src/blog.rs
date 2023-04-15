use markdown::{
    Options, Constructs,
    ParseOptions, CompileOptions,
    mdast::Node::Yaml,
};
use yaml_rust::YamlLoader;
use quick_xml::{
    events::BytesText,
    writer::Writer,
};
use std::{
    collections::HashMap,
    error::Error,
    io::Cursor,
    fs,
};
use crate::reading_time::{count_words, reading_time_from_words};

#[derive(Clone)]
/// TODO: store rendered articles in a temporary folder (i.e. ~/.cache) and redirect
///       /blog/{slug} to read from there
pub struct Blog {
    /// Cache of rendered articles.
    pub articles: HashMap<String, Article>,

    /// RSS feed based on cached articles
    pub rss: String
}

#[derive(Clone)]
pub struct Article {
    /// Title of the article
    pub title: String,

    /// Description about the content
    pub description: String,

    /// Date the article was published
    pub publish_date: String,

    /// Overall topic of the article
    pub category: String,

    /// The article
    pub content: String,

    /// Number of words in the article
    pub words: u32,

    /// The number of minutes it would take to read the article
    pub minutes: u32,
}

impl Blog {
    /// Creates a new `Blog`
    pub fn new() -> Self {
        Blog { articles: HashMap::new(), rss: String::default() }
    }

    /// Creates a new `Blog` and caches the articles.
    pub fn default() -> Result<Self, Box<dyn Error>> { 
        let mut blog = Blog::new();
        blog.update_articles()?;
        blog.update_rss()?;
        Ok(blog)
    }

    /// Caches all articles in the `./articles` folder
    pub fn update_articles(&mut self) -> Result<(), Box<dyn Error>> {
        for entry in fs::read_dir("./articles")? {
            self.update_article(&entry?)?;
        }

        Ok(())
    }

    /// Caches a single article given a DirEntry
    pub fn update_article(&mut self, dir: &fs::DirEntry) -> Result<(), Box<dyn Error>> {
        let parse_options = ParseOptions {
            constructs: Constructs {
                frontmatter: true,
                ..Constructs::gfm()
            },
            ..ParseOptions::default()
        };
        let options = Options {
            parse: ParseOptions {
                constructs: Constructs {
                    frontmatter: true,
                    ..Constructs::gfm()
                },
                ..ParseOptions::default()
            },
            compile: CompileOptions {
                allow_dangerous_html: true,
                ..CompileOptions::default()
            }
        };

        // Read file contents
        let content = fs::read_to_string(dir.path())?;

        // Parse markdown to HTML (and AST)
        let mdast = markdown::to_mdast(content.as_str(), &parse_options).unwrap();
        let md = markdown::to_html_with_options(content.as_str(), &options).unwrap();

        // Ignore if the article is missing a frontmatter
        if let Yaml(frontmatter) = &mdast.children().unwrap()[0] {
            // Parse frontmatter as YAML
            let docs = YamlLoader::load_from_str(frontmatter.value.as_str()).unwrap();
            let doc = &docs[0];

            // Construct Article
            let words = count_words(mdast.to_string());
            let article = Article {
                title: String::from(doc["title"].as_str().unwrap_or("")),
                description: String::from(doc["description"].as_str().unwrap_or("")),
                publish_date: String::from(doc["published_at"].as_str().unwrap_or("")),
                category: String::from(doc["category"].as_str().unwrap_or("")),

                content: md.clone(),

                words,
                minutes: reading_time_from_words(words),
            };

            // Add article to the HashMap
            // The slug is the filename without the `.md`
            // Someone tell me if I'm missing something, because this line just *feels wrong*
            let slug = String::from(
                dir.path()
                   .file_name()
                   .unwrap()
                   .to_string_lossy()
                   .to_string()
                   .trim_end_matches(".md")
            );
            self.articles.remove(slug.as_str());
            self.articles.insert(slug, article);
        }

        Ok(())
    }

    /// Caches an RSS feed based on articles
    pub fn update_rss(&mut self) -> Result<(), Box<dyn Error>> {
        let mut writer = Writer::new(Cursor::new(Vec::default()));

        writer.create_element("rss")
            .with_attribute(("version", "2.0"))
                .write_inner_content(|writer| {
                    // Channel
                    writer.create_element("channel")
                        .write_inner_content(|writer| {
                            // RSS metadata
                            writer.create_element("title")
                                .write_text_content(BytesText::new("Joey Lent"))?;

                            writer.create_element("link")
                                .write_text_content(BytesText::new("https://joeylent.dev/"))?;

                            writer.create_element("description")
                                .write_text_content(BytesText::new("Probably a programming-related blog."))?;

                            writer.create_element("language")
                                .write_text_content(BytesText::new("en-us"))?;

                            for article in &self.articles {
                                let (slug, post) = article;

                                writer.create_element("item")
                                    .write_inner_content(|writer| {
                                        writer.create_element("title")
                                            .write_text_content(BytesText::new(&post.title))?;

                                        writer.create_element("link")
                                            .write_text_content(BytesText::new(&format!("https://joeylent.dev/blog/{}", slug)))?;

                                        writer.create_element("description")
                                            .write_text_content(BytesText::new(&post.description))?;

                                        writer.create_element("author")
                                            .write_text_content(BytesText::new("supercolbat@protonmail.com (Joey Lent)"))?;

                                        for category in (&post.category).split(',') {
                                            writer.create_element("category")
                                                .write_text_content(BytesText::new(category.trim()))?;
                                        }

                                        // TODO: format into proper date structure
                                        // writer.create_element("pubDate")
                                        //     .write_text_content(BytesText::new(&post.publish_date))?;

                                    Ok(())
                                })?;
                        }
                        Ok(())
                    })?;
                Ok(())
            })?;

        self.rss = String::from_utf8(writer.into_inner().into_inner())?;

        Ok(())
    }

    /// Convenience method to get from `self.articles`
    pub fn get(&self, key: String) -> Option<&Article> {
        self.articles.get(key.as_str())
    }
}
