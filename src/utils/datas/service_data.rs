#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceDetailType {
    pub img: String,
    pub title: String,
    pub content: String,
    pub background: String,
}

pub fn service_detail_data() -> Vec<ServiceDetailType> {
    [
        ServiceDetailType {
            img: "service-details/shield-white.png".to_string(),
            title: "satisfaction guarantee".to_string(),
            content: "Your happiness is our utmost priority. If for any reason you are not completely satisfied with your experience, we will go above and beyond to make it right. Your satisfaction is guaranteed.".to_string(),
            background: "/images/imgs/services/IMG-001.png".to_string(),
        },
        ServiceDetailType {
            img: "service-details/global-communication-white.png".to_string(),
            title: "multi language support".to_string(),
            content: "Our platform provides comprehensive multi-language support, enabling users to seamlessly interact and communicate in their preferred languages.".to_string(),
            background: "/images/imgs/services/IMG-002.png".to_string(),
        },
        ServiceDetailType {
            img: "service-details/people-suit-white.png".to_string(),
            title: "professionals staff".to_string(),
            content: "Efficient, skilled, and dedicated, our professional staff exemplifies excellence in delivering exceptional service and achieving optimal outcomes.".to_string(),
            background: "/images/imgs/services/IMG-003.png".to_string(),
        }
    ].to_vec()
}
