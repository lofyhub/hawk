// @generated automatically by Diesel CLI.

diesel::table! {
    /// Representation of the `corruption_cases` table.
    ///
    /// (Automatically generated by Diesel.)
    corruption_cases (id) {
        /// The `id` column of the `corruption_cases` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `name` column of the `corruption_cases` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        name -> Varchar,
        /// The `politician_id` column of the `corruption_cases` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        politician_id -> Int4,
        /// The `case_description` column of the `corruption_cases` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 1000]
        case_description -> Varchar,
        /// The `case_date` column of the `corruption_cases` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        case_date -> Varchar,
        /// The `legal_outcome` column of the `corruption_cases` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        legal_outcome -> Nullable<Varchar>,
        /// The `created_at` column of the `corruption_cases` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Nullable<Timestamptz>,
        /// The `updated_at` column of the `corruption_cases` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Nullable<Timestamptz>,
        /// The `title` column of the `corruption_cases` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        title -> Nullable<Varchar>,
        /// The `upvotes` column of the `corruption_cases` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        upvotes -> Nullable<Int4>,
        /// The `downvotes` column of the `corruption_cases` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        downvotes -> Nullable<Int4>,
        /// The `link` column of the `corruption_cases` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        link -> Nullable<Varchar>,
    }
}

diesel::table! {
    /// Representation of the `politicians` table.
    ///
    /// (Automatically generated by Diesel.)
    politicians (politician_id) {
        /// The `politician_id` column of the `politicians` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        politician_id -> Int4,
        /// The `name` column of the `politicians` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `photo_url` column of the `politicians` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        photo_url -> Nullable<Varchar>,
        /// The `office` column of the `politicians` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        office -> Nullable<Varchar>,
        /// The `county` column of the `politicians` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        county -> Varchar,
        /// The `political_party` column of the `politicians` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        political_party -> Nullable<Varchar>,
        /// The `source_website` column of the `politicians` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        source_website -> Nullable<Varchar>,
        /// The `created_at` column of the `politicians` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Nullable<Timestamptz>,
        /// The `updated_at` column of the `politicians` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Nullable<Timestamptz>,
        /// The `year_of_birth` column of the `politicians` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        year_of_birth -> Nullable<Int4>,
        /// The `sex` column of the `politicians` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 10]
        sex -> Nullable<Varchar>,
    }
}

diesel::table! {
    /// Representation of the `user_reviews` table.
    ///
    /// (Automatically generated by Diesel.)
    user_reviews (id) {
        /// The `id` column of the `user_reviews` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `case_id` column of the `user_reviews` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        case_id -> Int4,
        /// The `title` column of the `user_reviews` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        title -> Varchar,
        /// The `review_text` column of the `user_reviews` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 1000]
        review_text -> Varchar,
        /// The `user_id` column of the `user_reviews` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        user_id -> Nullable<Varchar>,
        /// The `created_at` column of the `user_reviews` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Nullable<Timestamptz>,
        /// The `updated_at` column of the `user_reviews` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Nullable<Timestamptz>,
        /// The `upvotes` column of the `user_reviews` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        upvotes -> Nullable<Int4>,
        /// The `downvotes` column of the `user_reviews` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        downvotes -> Nullable<Int4>,
        /// The `link` column of the `user_reviews` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        link -> Nullable<Varchar>,
    }
}

diesel::joinable!(corruption_cases -> politicians (politician_id));
diesel::joinable!(user_reviews -> corruption_cases (case_id));

diesel::allow_tables_to_appear_in_same_query!(
    corruption_cases,
    politicians,
    user_reviews,
);
