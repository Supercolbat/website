use markdown::{Options, ParseOptions, Constructs, mdast::Node::Yaml, CompileOptions};
use yaml_rust::YamlLoader;
use std::{
    collections::HashMap,
    error::Error,
    fs
};
use crate::reading_time::{count_words, reading_time_from_words};

#[derive(Clone)]
pub struct Blog {
    /// Cache of rendered articles.
    /// TODO: store rendered articles in a temporary folder (i.e. ~/.cache) and redirect
    ///       /blog/{slug} to read from there
    pub articles: HashMap<String, Article>
}

#[derive(Clone)]
pub struct Article {
    // Metadata
    pub title: String,
    pub description: String,
    pub publish_date: String,

    // Not metadata
    pub content: String,

    // Reading time
    pub words: u32,
    pub minutes: u32,
}

impl Blog {
    /// Creates a new `Blog`
    pub fn new() -> Self {
        Blog { articles: HashMap::new() }
    }

    /// Creates a new `Blog` and caches the articles.
    pub fn default() -> Result<Self, Box<dyn Error>> { 
        let mut blog = Blog::new();
        blog.update_articles()?;
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

    /// Convenience method to get from `self.articles`
    pub fn get(&self, key: String) -> Option<&Article> {
        self.articles.get(key.as_str())
    }
}
