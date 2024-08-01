use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use phf::phf_set;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use validator::{Validate, ValidationError};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone, AsChangeset, Insertable)]
#[diesel(table_name=crate::models::schema::politicians)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Politician {
    pub politician_id: i32,
    pub name: String,
    pub office: Option<String>,
    pub county: String,
    pub political_party: Option<String>,
    pub source_website: Option<String>,
    pub photo_url: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Validate, Deserialize, Serialize, Clone, Insertable)]
#[diesel(table_name=crate::models::schema::politicians)]
pub struct NewPolitician {
    #[validate(length(min = 6))]
    pub name: String,
    #[validate(length(min = 4))]
    pub office: Option<String>,
    #[validate(custom(function = "county_validator"))]
    pub county: String,
    #[validate(length(min = 2))]
    pub political_party: Option<String>,
    #[validate(url)]
    pub source_website: Option<String>,
    #[validate(url)]
    pub photo_url: Option<String>,
}

static VALID_COUNTIES: phf::Set<&'static str> = phf_set! {
    "Kilifi", "Mombasa", "Kwale", "Tana River", "Lamu", "Taita Taveta", "Garissa",
    "Wajir", "Mandera", "Marsabit", "Isiolo", "Meru", "Embu", "Tharaka Nithi",
    "Laikipia", "Nyandarua", "Nyeri", "Kirinyaga", "Muranga", "Machakos", "Nakuru",
    "Narok", "Kajiado", "West Pokot", "Samburu", "Trans Nzoia", "Uasin Gishu",
    "Elgeyo Marakwet", "Nandi", "Bomet", "Kericho", "Baringo", "Vihiga", "Bungoma",
    "Kakamega", "Kisumu", "Siaya", "HomaBay", "Migori", "Kisii", "Nyamira",
    "Trans Mara", "Narok South", "Kwale North","Nairobi"
};

fn county_validator(county_str: &str) -> Result<(), ValidationError> {
    if VALID_COUNTIES.contains(county_str) {
        Ok(())
    } else {
        let mut error = ValidationError::new("invalid_county");
        error.message = Some(Cow::Owned(format!("Invalid county: {}", county_str)));
        Err(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_county() {
        assert!(county_validator("Mombasa").is_ok());
        assert!(county_validator("Nairobi").is_ok());
    }

    #[test]
    fn test_invalid_county() {
        assert!(county_validator("InvalidCounty").is_err());
    }
}
