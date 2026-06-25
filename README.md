# Config parser for the [Full-Text RSS site config files](https://github.com/fivefilters/ftr-site-config) format

A simple parser for the [instapaper-inspired](https://blog.instapaper.com/post/730281947) site config format, which can be used for extracting readable content from sites. For details about the format itself, read [this](https://help.fivefilters.org/full-text-rss/site-patterns.html)

```rust
use ftr_config_parser::{TestUrl, XPath, parse_config};

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

    let body: Vec<&str> = config.body().iter().map(XPath::as_str).collect();
    assert_eq!(body, ["//article"]);

    let author: Vec<&str> = config.author().iter().map(XPath::as_str).collect();
    assert_eq!(author, ["//aside/a[contains(@href, '/authors/')]/span"]);

    let strip: Vec<&str> = config.strip().iter().map(XPath::as_str).collect();
    assert_eq!(strip, ["//article/div[1]", "//article/ul[1]", "//aside"]);

    let urls: Vec<&str> = config.test_url().iter().map(TestUrl::as_str).collect();
    assert_eq!(urls, ["https://blog.sentry.io/gdpr-sentry-and-you/"]);
}
```

## Check the real ftr-site-config corpus
```bach
just setup
just check-corpus
```