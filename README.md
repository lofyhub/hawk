# Public Watch API

This document provides an overview and documentation for the public Watch API.
![Hawk](https://cms-assets.tutsplus.com/cdn-cgi/image/width=1700/uploads/users/108/posts/31764/final_image/how-to-draw-hawk-1-23.jpg)

## Database Schema

The following tables are part of the database schema:

### `corruption_cases`

This table stores information about corruption cases.

- `id` (Int4): The unique identifier for the corruption case.
- `name` (Varchar, max length 255): The name of the corruption case.
- `politician_id` (Int4): The ID of the politician associated with the corruption case.
- `case_description` (Varchar, max length 500): A description of the corruption case.
- `case_date` (Varchar, max length 255): The date of the corruption case.
- `legal_outcome` (Nullable<Varchar>, max length 255): The legal outcome of the corruption case.
- `created_at` (Nullable<Timestamptz>): The timestamp when the case was created.
- `updated_at` (Nullable<Timestamptz>): The timestamp when the case was last updated.
- `title` (Nullable<Varchar>, max length 255): The title of the corruption case.
- `upvotes` (Nullable<Int4>): The number of upvotes the case has received.
- `downvotes` (Nullable<Int4>): The number of downvotes the case has received.
- `link` (Nullable<Varchar>, max length 255): An optional link to reference the corruption case.

### `politicians`

This table stores information about politicians.

- `politician_id` (Int4): The unique identifier for the politician.
- `name` (Varchar): The name of the politician.
- `photo_url` (Nullable<Varchar>): The URL of the politician's photo.
- `office` (Nullable<Varchar>): The office held by the politician.
- `county` (Varchar): The county where the politician operates.
- `political_party` (Nullable<Varchar>): The political party of the politician.
- `source_website` (Nullable<Varchar>): The source website for information about the politician.
- `created_at` (Nullable<Timestamptz>): The timestamp when the politician was created.
- `updated_at` (Nullable<Timestamptz>): The timestamp when the politician was last updated.

### `user_reviews`

This table stores user reviews for corruption cases.

- `id` (Int4): The unique identifier for the user review.
- `case_id` (Int4): The ID of the corruption case being reviewed.
- `title` (Varchar, max length 255): The title of the review.
- `review_text` (Varchar, max length 500): The text of the review.
- `user_id` (Nullable<Varchar>, max length 255): The ID of the user who wrote the review.
- `created_at` (Nullable<Timestamptz>): The timestamp when the review was created.
- `updated_at` (Nullable<Timestamptz>): The timestamp when the review was last updated.
- `upvotes` (Nullable<Int4>): The number of upvotes the review has received.
- `downvotes` (Nullable<Int4>): The number of downvotes the review has received.
- `link` (Nullable<Varchar>, max length 255): An optional link to reference the review.

## Actix Web Routes

The following routes are provided for interacting with the database:

### Corruption Cases

#### `GET /corruption_cases`

Fetches all corruption cases.

**Response**:
- Status: 200 OK
- Body: JSON array of corruption cases

#### `POST /corruption_cases`

Creates a new corruption case.

**Request**:
- Body: JSON object representing a new corruption case

**Response**:
- Status: 201 Created
- Body: JSON object representing the created corruption case

### Cases Ratings

#### `GET /cases/ratings`

Fetches the most upvoted corruption cases.

**Response**:
- Status: 200 OK
- Body: JSON array of corruption cases sorted by upvotes

### Politicians

#### `GET /politicians/{user_id}`

Fetches a single politician by their user ID.

**Request**:
- Path: `user_id` (integer)

**Response**:
- Status: 200 OK
- Body: JSON object representing the politician
- Status: 404 Not Found if the politician is not found

#### `GET /politicians`

Fetches all politicians.

**Response**:
- Status: 200 OK
- Body: JSON array of politicians

#### `POST /politicians`

Creates a new politician.

**Request**:
- Body: JSON object representing a new politician

**Response**:
- Status: 201 Created
- Body: JSON object representing the created politician

### Health Check

#### `GET /`

Performs a health check.

**Response**:
- Status: 200 OK
- Body: JSON object with health status

### Echo

#### `POST /echo`

Echoes back the request body.

**Request**:
- Body: String

**Response**:
- Status: 200 OK
- Body: The same string sent in the request body

### User Reviews

#### `GET /report/{case_id}`

Fetches user reviews for a specific corruption case by case ID.

**Request**:
- Path: `case_id` (integer)

**Response**:
- Status: 200 OK
- Body: JSON array of user reviews
- Status: 404 Not Found if no reviews are found for the case

#### `POST /report`

Creates a new user review for a corruption case.

**Request**:
- Body: JSON object representing a new user review

**Response**:
- Status: 201 Created
- Body: JSON object representing the created user review
