use clap::Error;

pub struct ErrorAccumlator<Error> {
  error: Error,
}

impl<Error> ErrorAccumlator<Error> {
  pub fn new(error: Error) -> Self {
    Self {
      error,
    }
  }

  pub fn then_caused_by<NewError>(self, error: NewError) -> ErrorAccumlator<NewError> {
    ErrorAccumlator {
      error,
    }
  }

  pub fn then_caused_by_fn<Function, NewError>(
    self,
    function: Function,
  ) -> ErrorAccumlator<NewError> 
  where 
    Function: Fn(Error) -> NewError
  {
    ErrorAccumlator {
      error: (function)(self.error),
    }
  }
}