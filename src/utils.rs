use crate::error::ServerError;

const FILES_PATH_PREFIX: &str = "/files/";
const FILES_PATH_PREFIX_LEN: usize = FILES_PATH_PREFIX.len();

pub fn extract_filename_from_request_path(path: &str) -> Result<&str, ServerError> {
    if !path.starts_with(FILES_PATH_PREFIX) {
        return Err(ServerError::IncorrectPathError);
    }
    Ok(&path[FILES_PATH_PREFIX_LEN..])
}
