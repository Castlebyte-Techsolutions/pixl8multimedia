use crate::utils::types::VideoType;

pub fn video_list_data() -> Vec<VideoType> {
    [
        VideoType {
            id: 1,
            embed: "<iframe src='https://www.youtube.com/embed/sl91lGHz6QA' title='Catalyst v2 by Jeff Naparstek' frameborder='0' allow='accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share' referrerpolicy='strict-origin-when-cross-origin' allowfullscreen ></iframe>".into(),
        },
        VideoType {
            id: 2,
            embed: "<iframe src='https://www.youtube.com/embed/9mIPtyjmZGU' title='Seal With a Kiss: The Rescue of Hita, A Baby Ringed Seal' frameborder='0' allow='accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share' referrerpolicy='strict-origin-when-cross-origin' allowfullscreen></iframe>".into(),
        },
        VideoType {
            id: 3,
            embed: "<iframe src='https://www.youtube.com/embed/dZlsMh3skkg' title='Guilty in Mississippi   Trailer v4' frameborder='0' allow='accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share' referrerpolicy='strict-origin-when-cross-origin' allowfullscreen></iframe>".into(),
        },
        VideoType {
            id: 4,
            embed: "<iframe src='https://www.youtube.com/embed/4PTYTlJ0W1I' title='Always Faithful by Gregory Marcel' frameborder='0' allow='accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share' referrerpolicy='strict-origin-when-cross-origin' allowfullscreen></iframe>".into(),
        },
        VideoType {
            id: 5,
            embed: "<iframe src='https://www.youtube.com/embed/Mr6FkF9K-Ng' title='Book Trailer Dream Bunny   CJ Garrett' frameborder='0' allow='accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share' referrerpolicy='strict-origin-when-cross-origin' allowfullscreen></iframe>".into()
        },
        VideoType {
            id: 6,
            embed: "<iframe src='https://www.youtube.com/embed/iuyPru2RTN4' title='BLOOD MANOR by Sterling Daniels' frameborder='0' allow='accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share' referrerpolicy='strict-origin-when-cross-origin' allowfullscreen></iframe>".into()
        },
        VideoType {
            id: 7,
            embed: "<iframe src='https://www.youtube.com/embed/8ZwvgaDP-h4' title='Book Trailer The Brain Drain Joe Putnam' frameborder='0' allow='accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share' referrerpolicy='strict-origin-when-cross-origin' allowfullscreen></iframe>".into()
        }
    ].to_vec()
}
