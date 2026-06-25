use ftr_config_parser::{IdOrClass, TestUrl, XPath, YesNo, parse_config};

#[test]
#[allow(clippy::too_many_lines)]
fn golem_de() {
    let input = include_str!("fixtures/golem.de.txt");
    let config = parse_config(input).unwrap();

    let title: Vec<&str> = config.title().iter().map(XPath::as_str).collect();
    assert_eq!(
        title,
        [r#"substring-before( //meta[@property="og:title"]/@content , ' - Golem.de' )"#]
    );
    let body: Vec<&str> = config.body().iter().map(XPath::as_str).collect();
    assert_eq!(body, ["//main/article", "//article"]);
    let date: Vec<&str> = config.date().iter().map(XPath::as_str).collect();
    assert_eq!(date, ["//time/@datetime"]);
    let author: Vec<&str> = config.author().iter().map(XPath::as_str).collect();
    assert_eq!(author, ["//a[@rel='author']"]);
    let strip: Vec<&str> = config.strip().iter().map(XPath::as_str).collect();
    assert_eq!(
        strip,
        [
            "//div[contains(@class, 'authors--withsource')]",
            "//div[@class='toc']",
            "//li[not(.//text()[normalize-space()])][not(@class)]",
            "//div[contains(@class, 'go-teaser-block')]",
            "//ul[contains(@class, 'go-alink-list')]",
            "//hr[contains(@class, 'go-hr')]/following-sibling::div[contains(@class, 'go-grid')]",
            "//div[contains(@class, 'go-article-header__meta')]",
            "//div[contains(@class, 'go-article-header__button-bar')]",
            "//article[contains(@class, 'go-teaser--variant-affiliate')]",
            "//div[contains(@class, 'go-gallery__actions')]",
            "//nav[contains(@class, 'go-article__pagination')]",
            "//div[contains(@class, 'go-article-header__series')][.//a[contains(@class, 'go-label')]]",
            "//details[contains(@class, 'go-article__index')]",
            "//img[@src='']",
            "//div[contains(@style,'margin')]",
            "//figure[contains(@id,'gvideo')]",
            "//figure/figcaption[contains(text(), 'Bitte aktivieren Sie Javascript')]",
            "//svg[contains(@class, 'go-external-link__icon')]",
            "//span[@class='go-vh' and normalize-space(text())='(öffnet im neuen Fenster)']",
        ]
    );
    let strip_id_or_class: Vec<&str> = config
        .strip_id_or_class()
        .iter()
        .map(IdOrClass::as_str)
        .collect();
    assert_eq!(
        strip_id_or_class,
        [
            "go-heading--h1",
            "iqadtile4",
            "gbox_affiliate",
            "seminars",
            "supplementary",
            "list-jtoc",
            "table-jtoc",
            "implied",
            "social-tools",
            "comments",
            "footer",
            "job-market",
            "tags",
            "topictags",
            "go-button-bar",
        ]
    );
    assert_eq!(config.strip_image_src(), &[]);
    assert_eq!(config.prune(), YesNo::No);
    assert_eq!(config.tidy(), YesNo::No);
    assert_eq!(config.autodetect_on_failure(), YesNo::Yes);
    assert_eq!(config.single_page_link(), None);
    assert_eq!(config.single_page_link_in_feed(), None);
    assert_eq!(
        config.next_page_link().map(|x| x.as_str()),
        Some("//li[contains(@class, 'go-pagination__item--next')]/a")
    );
    let replacements: Vec<(&str, &str)> = config
        .replace_string()
        .iter()
        .map(|r| (r.find().as_str(), r.replace()))
        .collect();
    assert_eq!(replacements, [("<h1", "<h2"), ("</h1>", "</h2>")]);
    let headers: Vec<(&str, &str)> = config
        .http_header()
        .iter()
        .map(|h| (h.name(), h.value()))
        .collect();
    assert_eq!(
        headers,
        [
            ("Cookie", "golem_consent20=cmp|250101"),
            ("user-agent", "Googlebot"),
            ("Cookie", "golem_multipage=single"),
        ]
    );
    let urls: Vec<&str> = config.test_url().iter().map(TestUrl::as_str).collect();
    assert_eq!(
        urls,
        [
            "https://www.golem.de/news/arbeitsplatz-unter-druck-was-haelt-dich-noch-im-job-2509-200011.html",
            "https://www.golem.de/news/onlineshopping-auf-pump-boomt-erstmals-mehr-als-zehn-millionen-neue-ratenkredite-2509-199696.html",
            "http://www.golem.de/news/intel-core-i7-5960x-im-test-die-pc-revolution-beginnt-mit-octacore-und-ddr4-1408-108893.html",
            "http://www.golem.de/news/test-infamous-first-light-neonbunter-actionspass-1408-108914.html",
            "https://www.golem.de/news/ressourcenschonend-programmieren-so-wurden-spiele-fuer-den-commodore-64-und-atari-entwickelt-2307-175508.html",
        ]
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn lwn_net() {
    let input = include_str!("fixtures/lwn.net.txt");
    let config = parse_config(input).unwrap();

    let title: Vec<&str> = config.title().iter().map(XPath::as_str).collect();
    assert_eq!(title, ["//h1"]);
    let body: Vec<&str> = config.body().iter().map(XPath::as_str).collect();
    assert_eq!(body, ["//div[@class='ArticleText']"]);
    let date: Vec<&str> = config.date().iter().map(XPath::as_str).collect();
    assert_eq!(
        date,
        [
            "//div[@class='FeatureByline']/text()[preceding-sibling::br]",
            "//div[@class='GAByline']/p[1]",
        ]
    );
    let author: Vec<&str> = config.author().iter().map(XPath::as_str).collect();
    assert_eq!(
        author,
        [
            "//div[@class='FeatureByline']/strong",
            "substring-after(//div[@class='GAByline']/p[2], 'by ')",
        ]
    );
    let strip: Vec<&str> = config.strip().iter().map(XPath::as_str).collect();
    assert_eq!(
        strip,
        [
            "//div[@class='FeatureByline']",
            "//div[@class='GAByline']",
            "//div[@class='ftrss-strip']",
            "//table[@class='Form']",
        ]
    );
    assert_eq!(config.strip_id_or_class(), &[]);
    assert_eq!(config.strip_image_src(), &[]);
    assert_eq!(config.prune(), YesNo::No);
    assert_eq!(config.tidy(), YesNo::Yes);
    assert_eq!(config.autodetect_on_failure(), YesNo::Yes);
    assert_eq!(
        config.single_page_link().map(|x| x.as_str()),
        Some(
            "concat(//div[@class='ArticleText']//a[contains(text(), 'Read more')]/@href, 'bigpage')"
        )
    );
    assert_eq!(config.single_page_link_in_feed(), None);
    assert_eq!(config.next_page_link(), None);
    let replacements: Vec<(&str, &str)> = config
        .replace_string()
        .iter()
        .map(|r| (r.find().as_str(), r.replace()))
        .collect();
    assert_eq!(
        replacements,
        [
            (r#"<p class="Cat1HL">"#, "<h1>"),
            (r#"<h2 class="SummaryHL">"#, "<h3>"),
            (r#"<p class="Cat2HL">"#, "<h2>"),
            (
                r#"<hr width="60%" align="left">"#,
                r#"<div class="ftrss-strip">"#,
            ),
            ("to post comments)", "</div>"),
        ]
    );
    assert_eq!(config.http_header(), &[]);
    let urls: Vec<&str> = config.test_url().iter().map(TestUrl::as_str).collect();
    assert_eq!(
        urls,
        [
            "http://lwn.net/Articles/668318/",
            "http://lwn.net/Articles/668695/",
            "http://lwn.net/Articles/669114/",
            "http://lwn.net/Articles/670209/",
            "http://lwn.net/Articles/670209/rss",
            "http://lwn.net/Articles/668318/rss",
            "http://lwn.net/Articles/670062/",
        ]
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn next_ink() {
    let input = include_str!("fixtures/next.ink.txt");
    let config = parse_config(input).unwrap();

    let title: Vec<&str> = config.title().iter().map(XPath::as_str).collect();
    assert_eq!(title, ["//h1[@id='single-article-title']"]);
    let body: Vec<&str> = config.body().iter().map(XPath::as_str).collect();
    assert_eq!(
        body,
        [
            "//div[@id='next-single-post']",
            "//article[1]",
            "//div[@id='article-single']",
        ]
    );
    assert_eq!(config.date(), &[]);
    let author: Vec<&str> = config.author().iter().map(XPath::as_str).collect();
    assert_eq!(
        author,
        [
            "normalize-space(//p[contains(@class, 'next-list-author')]//a[@class='next-post-author'])"
        ]
    );
    let strip: Vec<&str> = config.strip().iter().map(XPath::as_str).collect();
    assert_eq!(
        strip,
        [
            "//div[contains(@class, 'thumbnail-mobile')]",
            "//div[contains(@class, 'article-header')]/h1",
            "//div[contains(@class, 'article-header')]/h2",
        ]
    );
    let strip_id_or_class: Vec<&str> = config
        .strip_id_or_class()
        .iter()
        .map(IdOrClass::as_str)
        .collect();
    assert_eq!(
        strip_id_or_class,
        [
            "article-author",
            "article-info",
            "share-bottom",
            "reading-time-post",
            "author-info",
            "other-article",
            "aside",
            "comment-widget",
            "share-mobile",
            "paywall",
            "list-link-internal",
            "share-button",
            "public_categories",
            "gift-button",
            "go-to-comments-button",
            "dropdown-button",
            "dropdown-content-signalement",
            "article-option",
            "wp-block-video",
            "article-info-left",
            "article-info-right",
        ]
    );
    assert_eq!(config.strip_image_src(), &[]);
    assert_eq!(config.prune(), YesNo::No);
    assert_eq!(config.tidy(), YesNo::No);
    assert_eq!(config.autodetect_on_failure(), YesNo::Yes);
    assert_eq!(config.single_page_link(), None);
    assert_eq!(config.single_page_link_in_feed(), None);
    assert_eq!(config.next_page_link(), None);
    let replacements: Vec<(&str, &str)> = config
        .replace_string()
        .iter()
        .map(|r| (r.find().as_str(), r.replace()))
        .collect();
    assert_eq!(
        replacements,
        [(r#"class="wp-block-heading""#, r#"class="wb_foo""#)]
    );
    assert_eq!(config.http_header(), &[]);
    let urls: Vec<&str> = config.test_url().iter().map(TestUrl::as_str).collect();
    assert_eq!(
        urls,
        [
            "https://next.ink/120832/le-reseau-interministeriel-de-letat-rie-fete-ses-10-ans-et-se-modernise/",
            "https://next.ink/127657/edito-limportance-de-bien-citer-et-verifier-ses-sources/",
            "https://next.ink/136362/les-ecrans-du-temps-perdu-pour-les-enfants/",
            "https://next.ink/143136/planete-9-son-absence-serait-statistiquement-impossible/",
        ]
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn mediapart_fr() {
    let input = include_str!("fixtures/mediapart.fr.txt");
    let config = parse_config(input).unwrap();

    let title: Vec<&str> = config.title().iter().map(XPath::as_str).collect();
    assert_eq!(title, [r#"//h1[@class="title"]"#]);
    let body: Vec<&str> = config.body().iter().map(XPath::as_str).collect();
    assert_eq!(body, ["//main[1]"]);
    let date: Vec<&str> = config.date().iter().map(XPath::as_str).collect();
    assert_eq!(
        date,
        ["//div[contains(concat(' ',normalize-space(@class),' '),' author ')]//time/@datetime"]
    );
    let author: Vec<&str> = config.author().iter().map(XPath::as_str).collect();
    assert_eq!(
        author,
        [
            "//div[contains(concat(' ',normalize-space(@class),' '),' author ')]//a[@class='journalist']"
        ]
    );
    let strip: Vec<&str> = config.strip().iter().map(XPath::as_str).collect();
    assert_eq!(
        strip,
        [
            "//button",
            r#"//article[contains(@class, "collection-card")]"#,
            r#"//h2[contains(@class, "subheading-bullet-point")]"#,
            r#"//figure[@id="lecteur-audio"]"#,
        ]
    );
    let strip_id_or_class: Vec<&str> = config
        .strip_id_or_class()
        .iter()
        .map(IdOrClass::as_str)
        .collect();
    assert_eq!(
        strip_id_or_class,
        [
            "news__body__right",
            "news__heading__top__kicker",
            "page-title",
            "news__heading__center",
            "splitter",
            "engagement-bar-wrapper",
            "read-also",
            "newsletter-form",
            "paywall-login",
            "paywall-message",
            "paywall_no_variance",
            "screen-reader-only",
        ]
    );
    assert_eq!(config.strip_image_src(), &[]);
    assert_eq!(config.prune(), YesNo::No);
    assert_eq!(config.tidy(), YesNo::No);
    assert_eq!(config.autodetect_on_failure(), YesNo::Yes);
    assert_eq!(
        config.single_page_link().map(|x| x.as_str()),
        Some(r#"//link[@rel="canonical"]"#)
    );
    assert_eq!(config.single_page_link_in_feed(), None);
    assert_eq!(config.next_page_link(), None);
    let replacements: Vec<(&str, &str)> = config
        .replace_string()
        .iter()
        .map(|r| (r.find().as_str(), r.replace()))
        .collect();
    assert_eq!(
        replacements,
        [
            (
                r#"<p class="news__heading__top__intro"#,
                r#"<strong class="news__heading__top__intro"#,
            ),
            (r#"class="container"#, r#"class="foo_cntr"#),
        ]
    );
    assert_eq!(config.http_header(), &[]);
    let urls: Vec<&str> = config.test_url().iter().map(TestUrl::as_str).collect();
    assert_eq!(
        urls,
        [
            "https://www.mediapart.fr/journal/france/170116/le-site-slatefr-est-passe-entre-les-mains-du-cac-40"
        ]
    );
}
