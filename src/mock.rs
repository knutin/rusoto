//! Mock request dispatcher for unit testing services
use super::{DispatchSignedRequest, HttpResponse, HttpDispatchError, SignedRequest};

pub struct MockRequestDispatcher {
	mock_response: HttpResponse,
	request_checker: Option<Box<Fn(&SignedRequest)>>
}

impl MockRequestDispatcher {
	pub fn with_status(status: u16) -> MockRequestDispatcher {
		let mut response = HttpResponse::default();
		response.status = status;
		MockRequestDispatcher { 
			mock_response: response,
			request_checker: None
		}
	}

	pub fn with_body(mut self, body: &str) -> MockRequestDispatcher {
		self.mock_response.body = body.to_owned();
		self
	}

	pub fn with_request_checker<F>(mut self, checker: F) -> MockRequestDispatcher where F: Fn(&SignedRequest) + 'static {
		self.request_checker = Some(Box::new(checker));
		self
	}

}

impl DispatchSignedRequest for MockRequestDispatcher {
	fn dispatch(&self, request: &SignedRequest) -> Result<HttpResponse, HttpDispatchError> {
		if self.request_checker.is_some() {
			self.request_checker.as_ref().unwrap()(request);
		}
		Ok(self.mock_response.clone())
	}
}