/// 何か入力が必要なString
pub struct RequiredString(String);

pub struct RequiredError;

impl TryFrom<String> for RequiredString {
    type Error = RequiredError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value != String::default() {
            Ok(RequiredString(value))
        } else {
            Err(RequiredError)
        }
    }
}
