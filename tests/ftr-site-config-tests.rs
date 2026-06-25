use ftr_config_parser::{
    FindReplaceString, FindString, HttpHeader, IdOrClass, TestUrl, XPath, YesNo, parse_config,
};

#[test]
#[allow(clippy::too_many_lines)]
fn golem_de() {
    let input = include_str!("fixtures/golem.de.txt");
    let config = parse_config(input).unwrap();

    assert_eq!(
        config.title(),
        &[XPath::try_from(
            r#"substring-before( //meta[@property="og:title"]/@content , ' - Golem.de' )"#,
        )
        .unwrap()]
    );
    assert_eq!(
        config.body(),
        &[
            XPath::try_from("//main/article").unwrap(),
            XPath::try_from("//article").unwrap(),
        ]
    );
    assert_eq!(
        config.date(),
        &[XPath::try_from("//time/@datetime").unwrap()]
    );
    assert_eq!(
        config.author(),
        &[XPath::try_from("//a[@rel='author']").unwrap()]
    );
    assert_eq!(
        config.strip(),
        &[
            XPath::try_from("//div[contains(@class, 'authors--withsource')]").unwrap(),
            XPath::try_from("//div[@class='toc']").unwrap(),
            XPath::try_from("//li[not(.//text()[normalize-space()])][not(@class)]").unwrap(),
            XPath::try_from("//div[contains(@class, 'go-teaser-block')]").unwrap(),
            XPath::try_from("//ul[contains(@class, 'go-alink-list')]").unwrap(),
            XPath::try_from(
                "//hr[contains(@class, 'go-hr')]/following-sibling::div[contains(@class, 'go-grid')]",
            )
            .unwrap(),
            XPath::try_from("//div[contains(@class, 'go-article-header__meta')]").unwrap(),
            XPath::try_from("//div[contains(@class, 'go-article-header__button-bar')]").unwrap(),
            XPath::try_from("//article[contains(@class, 'go-teaser--variant-affiliate')]").unwrap(),
            XPath::try_from("//div[contains(@class, 'go-gallery__actions')]").unwrap(),
            XPath::try_from("//nav[contains(@class, 'go-article__pagination')]").unwrap(),
            XPath::try_from(
                "//div[contains(@class, 'go-article-header__series')][.//a[contains(@class, 'go-label')]]",
            )
            .unwrap(),
            XPath::try_from("//details[contains(@class, 'go-article__index')]").unwrap(),
            XPath::try_from("//img[@src='']").unwrap(),
            XPath::try_from("//div[contains(@style,'margin')]").unwrap(),
            XPath::try_from("//figure[contains(@id,'gvideo')]").unwrap(),
            XPath::try_from(
                "//figure/figcaption[contains(text(), 'Bitte aktivieren Sie Javascript')]",
            )
            .unwrap(),
            XPath::try_from("//svg[contains(@class, 'go-external-link__icon')]").unwrap(),
            XPath::try_from(
                "//span[@class='go-vh' and normalize-space(text())='(öffnet im neuen Fenster)']",
            )
            .unwrap(),
        ]
    );
    assert_eq!(
        config.strip_id_or_class(),
        &[
            IdOrClass::try_from("go-heading--h1").unwrap(),
            IdOrClass::try_from("iqadtile4").unwrap(),
            IdOrClass::try_from("gbox_affiliate").unwrap(),
            IdOrClass::try_from("seminars").unwrap(),
            IdOrClass::try_from("supplementary").unwrap(),
            IdOrClass::try_from("list-jtoc").unwrap(),
            IdOrClass::try_from("table-jtoc").unwrap(),
            IdOrClass::try_from("implied").unwrap(),
            IdOrClass::try_from("social-tools").unwrap(),
            IdOrClass::try_from("comments").unwrap(),
            IdOrClass::try_from("footer").unwrap(),
            IdOrClass::try_from("job-market").unwrap(),
            IdOrClass::try_from("tags").unwrap(),
            IdOrClass::try_from("topictags").unwrap(),
            IdOrClass::try_from("go-button-bar").unwrap(),
        ]
    );
    assert_eq!(config.strip_image_src(), &[]);
    assert_eq!(config.prune(), YesNo::No);
    assert_eq!(config.tidy(), YesNo::No);
    assert_eq!(config.autodetect_on_failure(), YesNo::Yes);
    assert_eq!(config.single_page_link(), None);
    assert_eq!(config.single_page_link_in_feed(), None);
    assert_eq!(
        config.next_page_link(),
        Some(XPath::try_from("//li[contains(@class, 'go-pagination__item--next')]/a").unwrap())
    );
    assert_eq!(
        config.replace_string(),
        &[
            FindReplaceString {
                find: FindString::try_from("<h1").unwrap(),
                replace: "<h2",
            },
            FindReplaceString {
                find: FindString::try_from("</h1>").unwrap(),
                replace: "</h2>",
            },
        ]
    );
    assert_eq!(
        config.http_header(),
        &[
            HttpHeader {
                name: "Cookie",
                value: "golem_consent20=cmp|250101",
            },
            HttpHeader {
                name: "user-agent",
                value: "Googlebot",
            },
            HttpHeader {
                name: "Cookie",
                value: "golem_multipage=single",
            },
        ]
    );
    assert_eq!(
        config.test_url(),
        &[
            TestUrl(
                "https://www.golem.de/news/arbeitsplatz-unter-druck-was-haelt-dich-noch-im-job-2509-200011.html",
            ),
            TestUrl(
                "https://www.golem.de/news/onlineshopping-auf-pump-boomt-erstmals-mehr-als-zehn-millionen-neue-ratenkredite-2509-199696.html",
            ),
            TestUrl(
                "http://www.golem.de/news/intel-core-i7-5960x-im-test-die-pc-revolution-beginnt-mit-octacore-und-ddr4-1408-108893.html",
            ),
            TestUrl(
                "http://www.golem.de/news/test-infamous-first-light-neonbunter-actionspass-1408-108914.html",
            ),
            TestUrl(
                "https://www.golem.de/news/ressourcenschonend-programmieren-so-wurden-spiele-fuer-den-commodore-64-und-atari-entwickelt-2307-175508.html",
            ),
        ]
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn lwn_net() {
    let input = include_str!("fixtures/lwn.net.txt");
    let config = parse_config(input).unwrap();

    assert_eq!(config.title(), &[XPath::try_from("//h1").unwrap()]);
    assert_eq!(
        config.body(),
        &[XPath::try_from("//div[@class='ArticleText']").unwrap()]
    );
    assert_eq!(
        config.date(),
        &[
            XPath::try_from("//div[@class='FeatureByline']/text()[preceding-sibling::br]").unwrap(),
            XPath::try_from("//div[@class='GAByline']/p[1]").unwrap(),
        ]
    );
    assert_eq!(
        config.author(),
        &[
            XPath::try_from("//div[@class='FeatureByline']/strong").unwrap(),
            XPath::try_from("substring-after(//div[@class='GAByline']/p[2], 'by ')").unwrap(),
        ]
    );
    assert_eq!(
        config.strip(),
        &[
            XPath::try_from("//div[@class='FeatureByline']").unwrap(),
            XPath::try_from("//div[@class='GAByline']").unwrap(),
            XPath::try_from("//div[@class='ftrss-strip']").unwrap(),
            XPath::try_from("//table[@class='Form']").unwrap(),
        ]
    );
    assert_eq!(config.strip_id_or_class(), &[]);
    assert_eq!(config.strip_image_src(), &[]);
    assert_eq!(config.prune(), YesNo::No);
    assert_eq!(config.tidy(), YesNo::Yes);
    assert_eq!(config.autodetect_on_failure(), YesNo::Yes);
    // last occurrence of single_page_link wins
    assert_eq!(
        config.single_page_link(),
        Some(
            XPath::try_from(
                "concat(//div[@class='ArticleText']//a[contains(text(), 'Read more')]/@href, 'bigpage')",
            )
            .unwrap()
        )
    );
    assert_eq!(config.single_page_link_in_feed(), None);
    assert_eq!(config.next_page_link(), None);
    assert_eq!(
        config.replace_string(),
        &[
            FindReplaceString {
                find: FindString::try_from(r#"<p class="Cat1HL">"#).unwrap(),
                replace: "<h1>",
            },
            FindReplaceString {
                find: FindString::try_from(r#"<h2 class="SummaryHL">"#).unwrap(),
                replace: "<h3>",
            },
            FindReplaceString {
                find: FindString::try_from(r#"<p class="Cat2HL">"#).unwrap(),
                replace: "<h2>",
            },
            FindReplaceString {
                find: FindString::try_from(r#"<hr width="60%" align="left">"#).unwrap(),
                replace: r#"<div class="ftrss-strip">"#,
            },
            FindReplaceString {
                find: FindString::try_from("to post comments)").unwrap(),
                replace: "</div>",
            },
        ]
    );
    assert_eq!(config.http_header(), &[]);
    assert_eq!(
        config.test_url(),
        &[
            TestUrl("http://lwn.net/Articles/668318/"),
            TestUrl("http://lwn.net/Articles/668695/"),
            TestUrl("http://lwn.net/Articles/669114/"),
            TestUrl("http://lwn.net/Articles/670209/"),
            TestUrl("http://lwn.net/Articles/670209/rss"),
            TestUrl("http://lwn.net/Articles/668318/rss"),
            TestUrl("http://lwn.net/Articles/670062/"),
        ]
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn next_ink() {
    let input = include_str!("fixtures/next.ink.txt");
    let config = parse_config(input).unwrap();

    assert_eq!(
        config.title(),
        &[XPath::try_from("//h1[@id='single-article-title']").unwrap()]
    );
    assert_eq!(
        config.body(),
        &[
            XPath::try_from("//div[@id='next-single-post']").unwrap(),
            XPath::try_from("//article[1]").unwrap(),
            XPath::try_from("//div[@id='article-single']").unwrap(),
        ]
    );
    assert_eq!(config.date(), &[]);
    assert_eq!(
        config.author(),
        &[XPath::try_from(
            "normalize-space(//p[contains(@class, 'next-list-author')]//a[@class='next-post-author'])",
        )
        .unwrap()]
    );
    assert_eq!(
        config.strip(),
        &[
            XPath::try_from("//div[contains(@class, 'thumbnail-mobile')]").unwrap(),
            XPath::try_from("//div[contains(@class, 'article-header')]/h1").unwrap(),
            XPath::try_from("//div[contains(@class, 'article-header')]/h2").unwrap(),
        ]
    );
    assert_eq!(
        config.strip_id_or_class(),
        &[
            IdOrClass::try_from("article-author").unwrap(),
            IdOrClass::try_from("article-info").unwrap(),
            IdOrClass::try_from("share-bottom").unwrap(),
            IdOrClass::try_from("reading-time-post").unwrap(),
            IdOrClass::try_from("author-info").unwrap(),
            IdOrClass::try_from("other-article").unwrap(),
            IdOrClass::try_from("aside").unwrap(),
            IdOrClass::try_from("comment-widget").unwrap(),
            IdOrClass::try_from("share-mobile").unwrap(),
            IdOrClass::try_from("paywall").unwrap(),
            IdOrClass::try_from("list-link-internal").unwrap(),
            IdOrClass::try_from("share-button").unwrap(),
            IdOrClass::try_from("public_categories").unwrap(),
            IdOrClass::try_from("gift-button").unwrap(),
            IdOrClass::try_from("go-to-comments-button").unwrap(),
            IdOrClass::try_from("dropdown-button").unwrap(),
            IdOrClass::try_from("dropdown-content-signalement").unwrap(),
            IdOrClass::try_from("article-option").unwrap(),
            IdOrClass::try_from("wp-block-video").unwrap(),
            IdOrClass::try_from("article-info-left").unwrap(),
            IdOrClass::try_from("article-info-right").unwrap(),
        ]
    );
    assert_eq!(config.strip_image_src(), &[]);
    assert_eq!(config.prune(), YesNo::No);
    assert_eq!(config.tidy(), YesNo::No);
    assert_eq!(config.autodetect_on_failure(), YesNo::Yes);
    assert_eq!(config.single_page_link(), None);
    assert_eq!(config.single_page_link_in_feed(), None);
    assert_eq!(config.next_page_link(), None);
    // find_string is unknown and ignored; replace_string without a param gets find=""
    assert_eq!(
        config.replace_string(),
        &[FindReplaceString {
            find: FindString::try_from(r#"class="wp-block-heading""#).unwrap(),
            replace: r#"class="wb_foo""#,
        }]
    );
    assert_eq!(config.http_header(), &[]);
    assert_eq!(
        config.test_url(),
        &[
            TestUrl(
                "https://next.ink/120832/le-reseau-interministeriel-de-letat-rie-fete-ses-10-ans-et-se-modernise/",
            ),
            TestUrl(
                "https://next.ink/127657/edito-limportance-de-bien-citer-et-verifier-ses-sources/",
            ),
            TestUrl("https://next.ink/136362/les-ecrans-du-temps-perdu-pour-les-enfants/"),
            TestUrl(
                "https://next.ink/143136/planete-9-son-absence-serait-statistiquement-impossible/",
            ),
        ]
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn mediapart_fr() {
    let input = include_str!("fixtures/mediapart.fr.txt");
    let config = parse_config(input).unwrap();

    // no space between key and colon: "title://h1[@class="title"]"
    assert_eq!(
        config.title(),
        &[XPath::try_from(r#"//h1[@class="title"]"#).unwrap()]
    );
    assert_eq!(config.body(), &[XPath::try_from("//main[1]").unwrap()]);
    assert_eq!(
        config.date(),
        &[XPath::try_from(
            "//div[contains(concat(' ',normalize-space(@class),' '),' author ')]//time/@datetime",
        )
        .unwrap()]
    );
    assert_eq!(
        config.author(),
        &[XPath::try_from(
            "//div[contains(concat(' ',normalize-space(@class),' '),' author ')]//a[@class='journalist']",
        )
        .unwrap()]
    );
    assert_eq!(
        config.strip(),
        &[
            XPath::try_from("//button").unwrap(),
            XPath::try_from(r#"//article[contains(@class, "collection-card")]"#).unwrap(),
            XPath::try_from(r#"//h2[contains(@class, "subheading-bullet-point")]"#).unwrap(),
            XPath::try_from(r#"//figure[@id="lecteur-audio"]"#).unwrap(),
        ]
    );
    assert_eq!(
        config.strip_id_or_class(),
        &[
            IdOrClass::try_from("news__body__right").unwrap(),
            IdOrClass::try_from("news__heading__top__kicker").unwrap(),
            IdOrClass::try_from("page-title").unwrap(),
            IdOrClass::try_from("news__heading__center").unwrap(),
            IdOrClass::try_from("splitter").unwrap(),
            IdOrClass::try_from("engagement-bar-wrapper").unwrap(),
            IdOrClass::try_from("read-also").unwrap(),
            IdOrClass::try_from("newsletter-form").unwrap(),
            IdOrClass::try_from("paywall-login").unwrap(),
            IdOrClass::try_from("paywall-message").unwrap(),
            IdOrClass::try_from("paywall_no_variance").unwrap(),
            // leading whitespace in source "strip_id_or_class:  screen-reader-only" is handled by split_ascii_whitespace
            IdOrClass::try_from("screen-reader-only").unwrap(),
        ]
    );
    assert_eq!(config.strip_image_src(), &[]);
    assert_eq!(config.prune(), YesNo::No);
    assert_eq!(config.tidy(), YesNo::No);
    assert_eq!(config.autodetect_on_failure(), YesNo::Yes);
    assert_eq!(
        config.single_page_link(),
        Some(XPath::try_from(r#"//link[@rel="canonical"]"#).unwrap())
    );
    assert_eq!(config.single_page_link_in_feed(), None);
    assert_eq!(config.next_page_link(), None);
    assert_eq!(
        config.replace_string(),
        &[
            // `find_string` before this is an unknown key and is ignored
            FindReplaceString {
                find: FindString::try_from(r#"<p class="news__heading__top__intro"#).unwrap(),
                replace: r#"<strong class="news__heading__top__intro"#,
            },
            // `find_string` further down is also unknown; `replace_string` with no param gets find=""
            FindReplaceString {
                find: FindString::try_from(r#"class="container"#).unwrap(),
                replace: r#"class="foo_cntr"#,
            },
        ]
    );
    assert_eq!(config.http_header(), &[]);
    assert_eq!(
        config.test_url(),
        &[TestUrl(
            "https://www.mediapart.fr/journal/france/170116/le-site-slatefr-est-passe-entre-les-mains-du-cac-40",
        )]
    );
}
