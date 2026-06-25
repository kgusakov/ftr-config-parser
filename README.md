# Config parser for the [Full-Text RSS site config files](https://github.com/fivefilters/ftr-site-config) format

A simple parser for the [instapaper-inspired](https://blog.instapaper.com/post/730281947) site config format, which can be used for extracting readable content from sites. For details about the format itself, read [this](https://help.fivefilters.org/full-text-rss/site-patterns.html)

```rust
use ftr_config_parser::{parse_config, TestUrl, XPath};

fn main() {
    let input_config = "
body: //article
author: //aside/a[contains(@href, '/authors/')]/span
strip: //article/div[1]
strip: //article/ul[1]
strip: //aside
test_url: https://blog.sentry.io/gdpr-sentry-and-you/
";

    let config = parse_config(input_config).unwrap();

    assert_eq!(config.body(), &[XPath::try_from("//article").unwrap()]);
    assert_eq!(
        config.author(),
        &[XPath::try_from("//aside/a[contains(@href, '/authors/')]/span").unwrap()]
    );
    assert_eq!(
        config.strip(),
        &[
            XPath::try_from("//article/div[1]").unwrap(),
            XPath::try_from("//article/ul[1]").unwrap(),
            XPath::try_from("//aside").unwrap(),
        ]
    );
    assert_eq!(
        config.test_url(),
        &[TestUrl("https://blog.sentry.io/gdpr-sentry-and-you/")]
    );
}
```