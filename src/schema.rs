// @generated automatically by Diesel CLI.

diesel::table! {
    companies (id) {
        id -> Varchar,
        date_added -> Nullable<Date>,
        name -> Varchar,
        website -> Nullable<Varchar>,
        logo_cid -> Nullable<Varchar>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    jobs (id) {
        id -> Varchar,
        date_posted -> Nullable<Date>,
        title -> Varchar,
        description -> Nullable<Text>,
        company_id -> Varchar,
        pay -> Nullable<Varchar>,
        location -> Text,
        remote -> Bool,
        job_type -> Varchar,
    }
}

diesel::joinable!(jobs -> companies (company_id));

diesel::allow_tables_to_appear_in_same_query!(
    companies,
    jobs,
);
