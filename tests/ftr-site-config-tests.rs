use ftr_config_parser::{
    Config, FindReplaceString, FindString, HttpHeader, IdOrClass, TestUrl, XPath, YesNo,
    parse_config,
};

#[test]
#[allow(clippy::too_many_lines)]
fn golem_de() {
    let input = include_str!("fixtures/golem.de.txt");
    assert_eq!(
        parse_config(input).unwrap(),
        Config {
            title: vec![XPath(
                r#"substring-before( //meta[@property="og:title"]/@content , ' - Golem.de' )"#,
            )],
            body: vec![XPath("//main/article"), XPath("//article")],
            date: vec![XPath("//time/@datetime")],
            author: vec![XPath("//a[@rel='author']")],
            strip: vec![
                XPath("//div[contains(@class, 'authors--withsource')]"),
                XPath("//div[@class='toc']"),
                XPath("//li[not(.//text()[normalize-space()])][not(@class)]"),
                XPath("//div[contains(@class, 'go-teaser-block')]"),
                XPath("//ul[contains(@class, 'go-alink-list')]"),
                XPath(
                    "//hr[contains(@class, 'go-hr')]/following-sibling::div[contains(@class, 'go-grid')]"
                ),
                XPath("//div[contains(@class, 'go-article-header__meta')]"),
                XPath("//div[contains(@class, 'go-article-header__button-bar')]"),
                XPath("//article[contains(@class, 'go-teaser--variant-affiliate')]"),
                XPath("//div[contains(@class, 'go-gallery__actions')]"),
                XPath("//nav[contains(@class, 'go-article__pagination')]"),
                XPath(
                    "//div[contains(@class, 'go-article-header__series')][.//a[contains(@class, 'go-label')]]"
                ),
                XPath("//details[contains(@class, 'go-article__index')]"),
                XPath("//img[@src='']"),
                XPath("//div[contains(@style,'margin')]"),
                XPath("//figure[contains(@id,'gvideo')]"),
                XPath("//figure/figcaption[contains(text(), 'Bitte aktivieren Sie Javascript')]"),
                XPath("//svg[contains(@class, 'go-external-link__icon')]"),
                XPath(
                    "//span[@class='go-vh' and normalize-space(text())='(öffnet im neuen Fenster)']"
                ),
            ],
            strip_id_or_class: vec![
                IdOrClass("go-heading--h1"),
                IdOrClass("iqadtile4"),
                IdOrClass("gbox_affiliate"),
                IdOrClass("seminars"),
                IdOrClass("supplementary"),
                IdOrClass("list-jtoc"),
                IdOrClass("table-jtoc"),
                IdOrClass("implied"),
                IdOrClass("social-tools"),
                IdOrClass("comments"),
                IdOrClass("footer"),
                IdOrClass("job-market"),
                IdOrClass("tags"),
                IdOrClass("topictags"),
                IdOrClass("go-button-bar"),
            ],
            strip_image_src: vec![],
            prune: YesNo::No,
            tidy: YesNo::No,
            autodetect_on_failure: YesNo::Yes,
            single_page_link: None,
            single_page_link_in_feed: None,
            next_page_link: Some(XPath(
                "//li[contains(@class, 'go-pagination__item--next')]/a",
            )),
            replace_string: vec![
                FindReplaceString {
                    find: FindString::try_from("<h1").unwrap(),
                    replace: "<h2"
                },
                FindReplaceString {
                    find: FindString::try_from("</h1>").unwrap(),
                    replace: "</h2>"
                },
            ],
            http_header: vec![
                HttpHeader {
                    name: "Cookie",
                    value: "golem_consent20=cmp|250101"
                },
                HttpHeader {
                    name: "user-agent",
                    value: "Googlebot"
                },
                HttpHeader {
                    name: "Cookie",
                    value: "golem_multipage=single"
                },
            ],
            test_url: vec![
                TestUrl(
                    "https://www.golem.de/news/arbeitsplatz-unter-druck-was-haelt-dich-noch-im-job-2509-200011.html"
                ),
                TestUrl(
                    "https://www.golem.de/news/onlineshopping-auf-pump-boomt-erstmals-mehr-als-zehn-millionen-neue-ratenkredite-2509-199696.html"
                ),
                TestUrl(
                    "http://www.golem.de/news/intel-core-i7-5960x-im-test-die-pc-revolution-beginnt-mit-octacore-und-ddr4-1408-108893.html"
                ),
                TestUrl(
                    "http://www.golem.de/news/test-infamous-first-light-neonbunter-actionspass-1408-108914.html"
                ),
                TestUrl(
                    "https://www.golem.de/news/ressourcenschonend-programmieren-so-wurden-spiele-fuer-den-commodore-64-und-atari-entwickelt-2307-175508.html"
                ),
            ],
        }
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn lwn_net() {
    let input = include_str!("fixtures/lwn.net.txt");
    assert_eq!(
        parse_config(input).unwrap(),
        Config {
            title: vec![XPath("//h1")],
            body: vec![XPath("//div[@class='ArticleText']")],
            date: vec![
                XPath("//div[@class='FeatureByline']/text()[preceding-sibling::br]"),
                XPath("//div[@class='GAByline']/p[1]"),
            ],
            author: vec![
                XPath("//div[@class='FeatureByline']/strong"),
                XPath("substring-after(//div[@class='GAByline']/p[2], 'by ')"),
            ],
            strip: vec![
                XPath("//div[@class='FeatureByline']"),
                XPath("//div[@class='GAByline']"),
                XPath("//div[@class='ftrss-strip']"),
                XPath("//table[@class='Form']"),
            ],
            strip_id_or_class: vec![],
            strip_image_src: vec![],
            prune: YesNo::No,
            tidy: YesNo::Yes,
            autodetect_on_failure: YesNo::Yes,
            // last occurrence of single_page_link wins
            single_page_link: Some(XPath(
                "concat(//div[@class='ArticleText']//a[contains(text(), 'Read more')]/@href, 'bigpage')",
            )),
            single_page_link_in_feed: None,
            next_page_link: None,
            replace_string: vec![
                FindReplaceString {
                    find: FindString::try_from(r#"<p class="Cat1HL">"#).unwrap(),
                    replace: "<h1>"
                },
                FindReplaceString {
                    find: FindString::try_from(r#"<h2 class="SummaryHL">"#).unwrap(),
                    replace: "<h3>"
                },
                FindReplaceString {
                    find: FindString::try_from(r#"<p class="Cat2HL">"#).unwrap(),
                    replace: "<h2>"
                },
                FindReplaceString {
                    find: FindString::try_from(r#"<hr width="60%" align="left">"#).unwrap(),
                    replace: r#"<div class="ftrss-strip">"#,
                },
                FindReplaceString {
                    find: FindString::try_from("to post comments)").unwrap(),
                    replace: "</div>"
                },
            ],
            http_header: vec![],
            test_url: vec![
                TestUrl("http://lwn.net/Articles/668318/"),
                TestUrl("http://lwn.net/Articles/668695/"),
                TestUrl("http://lwn.net/Articles/669114/"),
                TestUrl("http://lwn.net/Articles/670209/"),
                TestUrl("http://lwn.net/Articles/670209/rss"),
                TestUrl("http://lwn.net/Articles/668318/rss"),
                TestUrl("http://lwn.net/Articles/670062/"),
            ],
        }
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn next_ink() {
    let input = include_str!("fixtures/next.ink.txt");
    assert_eq!(
        parse_config(input).unwrap(),
        Config {
            title: vec![XPath("//h1[@id='single-article-title']")],
            body: vec![
                XPath("//div[@id='next-single-post']"),
                XPath("//article[1]"),
                XPath("//div[@id='article-single']"),
            ],
            date: vec![],
            author: vec![XPath(
                "normalize-space(//p[contains(@class, 'next-list-author')]//a[@class='next-post-author'])",
            )],
            strip: vec![
                XPath("//div[contains(@class, 'thumbnail-mobile')]"),
                XPath("//div[contains(@class, 'article-header')]/h1"),
                XPath("//div[contains(@class, 'article-header')]/h2"),
            ],
            strip_id_or_class: vec![
                IdOrClass("article-author"),
                IdOrClass("article-info"),
                IdOrClass("share-bottom"),
                IdOrClass("reading-time-post"),
                IdOrClass("author-info"),
                IdOrClass("other-article"),
                IdOrClass("aside"),
                IdOrClass("comment-widget"),
                IdOrClass("share-mobile"),
                IdOrClass("paywall"),
                IdOrClass("list-link-internal"),
                IdOrClass("share-button"),
                IdOrClass("public_categories"),
                IdOrClass("gift-button"),
                IdOrClass("go-to-comments-button"),
                IdOrClass("dropdown-button"),
                IdOrClass("dropdown-content-signalement"),
                IdOrClass("article-option"),
                IdOrClass("wp-block-video"),
                IdOrClass("article-info-left"),
                IdOrClass("article-info-right"),
            ],
            strip_image_src: vec![],
            prune: YesNo::No,
            tidy: YesNo::No,
            autodetect_on_failure: YesNo::Yes,
            single_page_link: None,
            single_page_link_in_feed: None,
            next_page_link: None,
            // find_string is unknown and ignored; replace_string without a param gets find=""
            replace_string: vec![FindReplaceString {
                find: FindString::try_from(r#"class="wp-block-heading""#).unwrap(),
                replace: r#"class="wb_foo""#,
            }],
            http_header: vec![],
            test_url: vec![
                TestUrl(
                    "https://next.ink/120832/le-reseau-interministeriel-de-letat-rie-fete-ses-10-ans-et-se-modernise/"
                ),
                TestUrl(
                    "https://next.ink/127657/edito-limportance-de-bien-citer-et-verifier-ses-sources/"
                ),
                TestUrl("https://next.ink/136362/les-ecrans-du-temps-perdu-pour-les-enfants/"),
                TestUrl(
                    "https://next.ink/143136/planete-9-son-absence-serait-statistiquement-impossible/"
                ),
            ],
        }
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn mediapart_fr() {
    let input = include_str!("fixtures/mediapart.fr.txt");
    assert_eq!(
        parse_config(input).unwrap(),
        Config {
            // no space between key and colon: "title://h1[@class="title"]"
            title: vec![XPath(r#"//h1[@class="title"]"#)],
            body: vec![XPath("//main[1]")],
            date: vec![XPath(
                "//div[contains(concat(' ',normalize-space(@class),' '),' author ')]//time/@datetime",
            )],
            author: vec![XPath(
                "//div[contains(concat(' ',normalize-space(@class),' '),' author ')]//a[@class='journalist']",
            )],
            strip: vec![
                XPath("//button"),
                XPath(r#"//article[contains(@class, "collection-card")]"#),
                XPath(r#"//h2[contains(@class, "subheading-bullet-point")]"#),
                XPath(r#"//figure[@id="lecteur-audio"]"#),
            ],
            strip_id_or_class: vec![
                IdOrClass("news__body__right"),
                IdOrClass("news__heading__top__kicker"),
                IdOrClass("page-title"),
                IdOrClass("news__heading__center"),
                IdOrClass("splitter"),
                IdOrClass("engagement-bar-wrapper"),
                IdOrClass("read-also"),
                IdOrClass("newsletter-form"),
                IdOrClass("paywall-login"),
                IdOrClass("paywall-message"),
                IdOrClass("paywall_no_variance"),
                // leading whitespace in source "strip_id_or_class:  screen-reader-only" is handled by split_ascii_whitespace
                IdOrClass("screen-reader-only"),
            ],
            strip_image_src: vec![],
            prune: YesNo::No,
            tidy: YesNo::No,
            autodetect_on_failure: YesNo::Yes,
            single_page_link: Some(XPath(r#"//link[@rel="canonical"]"#)),
            single_page_link_in_feed: None,
            next_page_link: None,
            replace_string: vec![
                // `find_string` before this is an unknown key and is ignored
                FindReplaceString {
                    find: FindString::try_from(r#"<p class="news__heading__top__intro"#).unwrap(),
                    replace: r#"<strong class="news__heading__top__intro"#,
                },
                // `find_string` further down is also unknown; `replace_string` with no param gets find=""
                FindReplaceString {
                    find: FindString::try_from(r#"class="container"#).unwrap(),
                    replace: r#"class="foo_cntr"#
                },
            ],
            http_header: vec![],
            test_url: vec![TestUrl(
                "https://www.mediapart.fr/journal/france/170116/le-site-slatefr-est-passe-entre-les-mains-du-cac-40",
            )],
        }
    );
}
