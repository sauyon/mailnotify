use std::fmt;

type MailId = u64;

struct Notification {
	id: MailId,
	subj: String,
	msg: String,
}

struct Service {
	// backend service state
}

#[derive(Debug)]
enum Error {
	// stuff
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "")
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    None
  }
}

enum Action {
	// actions
	Archive(MailId),
	Delete(MailId),
}

impl Service {
	fn get_notification(&mut self) -> Result<Notification, Error> {
		return Ok(Notification{id: 0, subj: String::from(""), msg: String::from("")})
	}

	fn login(&mut self) -> Result<(), Error> {
		// not actually sure how this works, you figure it out ;)
		return Ok(())
	}

	fn exec(action: &Action) -> Result<(), Error> {
		// do the stuff
		return Ok(())
	}
}
