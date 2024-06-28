// @generated automatically by Diesel CLI.

diesel::table! {
    companies (id) {
        id -> Int4,
        name -> Varchar,
        website -> Nullable<Varchar>,
        logo_cid -> Nullable<Varchar>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    jobs (id) {
        id -> Int4,
        date_posted -> Date,
        title -> Varchar,
        description -> Nullable<Text>,
        company -> Int4,
        pay -> Nullable<Varchar>,
        job_location -> Text,
        remote -> Bool,
        job_type -> Varchar,
    }
}

diesel::joinable!(jobs -> companies (company));

diesel::allow_tables_to_appear_in_same_query!(
    companies,
    jobs,
);
