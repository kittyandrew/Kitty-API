use rocket::request::{Outcome, Request, FromRequest};
use rocket::http::Status;
use serde::{Serialize};


#[derive(Debug, Serialize, Clone)]
pub struct PageSize(pub usize);


#[derive(Debug)]
pub enum HeaderError {
    PageSizeBad,
}


#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for PageSize {
    type Error = HeaderError;

    async fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("X-PAGE-SIZE") {
            Some(v) => match v.parse::<usize>() {
                Ok(page_size) => match page_size {
                    // Forbid 0 page size
                    n if n > 0 => Outcome::Success(PageSize(n)),
                    _ => Outcome::Failure((Status::PreconditionFailed, HeaderError::PageSizeBad)),
                },
                Err(_) => Outcome::Failure((Status::PreconditionFailed, HeaderError::PageSizeBad)),
            },
            // Fallback to default of 5
            None => Outcome::Success(PageSize(5)),
        }
    }
}

