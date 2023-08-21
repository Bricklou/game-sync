use validator::ValidationError;

#[must_use]
pub fn validate_required_str(val: &String) -> Result<(), ValidationError> {
    if val.len() == 0 {
        return Err(ValidationError::new("empty_string"));
    }
    Ok(())
}
