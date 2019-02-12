table! {
    albums (id) {
        id -> Text,
        name -> Text,
        parent_album_id -> Nullable<Text>,
    }
}

table! {
    photos (id) {
        id -> Text,
        name -> Text,
        album_id -> Text,
        creation_date -> Nullable<Timestamp>,
        flash -> Nullable<Text>,
        aperture -> Nullable<Text>,
        exposure_time -> Nullable<Text>,
        focal_length -> Nullable<Text>,
        focal_length_in_35mm -> Nullable<Text>,
        camera -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    albums,
    photos,
);