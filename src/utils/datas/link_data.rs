use crate::utils::types::{LinkTagType, LinkTagTypeWithSub};

pub fn link_tag_data() -> Vec<LinkTagType> {
    [
        LinkTagType {
            link: "https://hunt-publishing.pixl8media.com".to_string(),
            title: "hunt publishing".to_string(),
        },
        LinkTagType {
            link: "https://sab-marketing.pixl8media.com".to_string(),
            title: "SAB marketing".to_string(),
        },
        LinkTagType {
            link: "book-review".to_string(),
            title: "loretta sullivan book review".to_string(),
        },
        LinkTagType {
            link: "services".to_string(),
            title: "upload materials".to_string(),
        },
        LinkTagType {
            link: "about-us".to_string(),
            title: "about us".to_string(),
        },
    ]
    .to_vec()
}

pub fn link_tag_with_sub_data() -> Vec<LinkTagTypeWithSub> {
    [LinkTagTypeWithSub {
        link: "screenplay".to_string(),
        title: "screenplay adaptation".to_string(),
        sub: sub_screenplay_adaptation(),
    }]
    .to_vec()
}

pub fn sub_screenplay_adaptation() -> Vec<LinkTagType> {
    [
        LinkTagType {
            link: "screenplay/representation".to_string(),
            title: "representation".to_string(),
        },
        LinkTagType {
            link: "screenplay/query-letter".to_string(),
            title: "query letter".to_string(),
        },
        LinkTagType {
            link: "screenplay/treatment-logline".to_string(),
            title: "treatment & logline".to_string(),
        },
        LinkTagType {
            link: "screenplay/trailer".to_string(),
            title: "trailer".to_string(),
        },
        LinkTagType {
            link: "screenplay/storyboard".to_string(),
            title: "storyboard".to_string(),
        },
        LinkTagType {
            link: "screenplay/producer-pitch".to_string(),
            title: "producer's pitch".to_string(),
        },
    ]
    .to_vec()
}
