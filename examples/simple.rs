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
