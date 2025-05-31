/// Represents a single error context, including the action being performed,
/// associated error messages, and any attached information.
#[derive(Debug, Clone)]
struct ErrorContext {
  /// The action being performed when the error occurred.
  action: String,
  /// A list of error messages associated with this context.
  error_messages: Vec<String>,
  /// Additional key-value information attached to the error context.
  attached_info: Vec<ErrorContextAttachment>,
}

/// Represents a key-value pair attached to an error context for additional information.
#[derive(Debug, Clone)]
struct ErrorContextAttachment {
  /// The name of the attachment.
  name: String,
  /// The value of the attachment.
  value: String,
}

/// Represents a generic error with contextual information and a history of previous contexts.
#[derive(Clone)]
pub struct GenericError {
  /// The current error context, describing the most recent action and associated details.
  current_context: ErrorContext,
  /// A stack of previous error contexts, providing a traceable history of the error.
  contexts: Vec<ErrorContext>,
}

impl GenericError {
  /// Creates a new `GenericError` instance with an initial context.
  ///
  /// # Arguments
  /// * `action` - A description of the action being performed when the error occurred.
  ///
  /// # Returns
  /// * `GenericError` - A new error instance with the specified initial context.
  pub fn new(action: impl Into<String>) -> GenericError {
    GenericError {
      current_context: ErrorContext { 
        action: action.into(),
        attached_info: Vec::new(),
        error_messages: Vec::new(),
      },
      contexts: Vec::new(),
    }
  }

  /// Adds an error message to the current context.
  ///
  /// # Arguments
  /// * `error_message` - The error message to add.
  ///
  /// # Returns
  /// * `Self` - The updated error instance.
  pub fn add_error(mut self, error_message: impl Into<String>) -> Self {
    self.current_context.error_messages.push(error_message.into());
    self
  }

  /// Adds a key-value attachment to the current context.
  ///
  /// # Arguments
  /// * `name` - The name of the attachment.
  /// * `value` - The value of the attachment.
  ///
  /// # Returns
  /// * `Self` - The updated error instance.
  pub fn add_attachment(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
    self.current_context.attached_info.push(ErrorContextAttachment { 
      name: name.into(), 
      value: value.into(),
    });

    self
  }

  /// Changes the current context and pushes the previous context onto the stack.
  ///
  /// # Arguments
  /// * `action` - A description of the new action for the current context.
  ///
  /// # Returns
  /// * `Self` - The updated error instance.
  pub fn change_context(mut self, action: impl Into<String>) -> Self {
    let mut previous_context = ErrorContext {
      action: action.into(),
      attached_info: Vec::new(),
      error_messages: Vec::new(),
    };

    std::mem::swap(&mut self.current_context, &mut previous_context);

    self.contexts.push(previous_context);
    self
  }

  // pub fn prepend_generic_error(mut self, generic_error: GenericError) -> Self {
  //   self.contexts.extend_from_slice(&generic_error.contexts);
  //   self.contexts.push(generic_error.current_context);
  //   self
  // }

}

use std::fmt;

impl fmt::Debug for GenericError {
  /// Provides a detailed debug representation of the error, including all contexts,
  /// error messages, and attachments.
  ///
  /// # Example Output
  /// ```
  /// 🧨 GenericError Trace:
  /// ╭── Context [0]: Initializing database
  /// │  💥 Errors:
  /// │    - Failed to connect to database
  /// │  📎 Attachments:
  /// │    - host: localhost
  /// │    - port: 5432
  /// ╰──────────────────────────────
  /// 🟢 Current Context: Query execution
  ///    💥 Errors:
  ///      - Syntax error in SQL query
  ///    📎 Attachments:
  ///      - query: SELECT * FROM users WHERE id = ?
  /// ```
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(f, "🧨 GenericError Trace:")?;

    for (i, context) in self.contexts.iter().enumerate() {
      writeln!(f, "╭── Context [{}]: {}", i, context.action)?;
      if !context.error_messages.is_empty() {
        writeln!(f, "│  💥 Errors:")?;
        for msg in &context.error_messages {
          writeln!(f, "│    - {}", msg)?;
        }
      }
      if !context.attached_info.is_empty() {
        writeln!(f, "│  📎 Attachments:")?;
        for att in &context.attached_info {
          writeln!(f, "│    - {}: {}", att.name, att.value)?;
        }
      }
      writeln!(f, "╰──────────────────────────────")?;
    }

    // Final (current) context
    writeln!(f, "🟢 Current Context: {}", self.current_context.action)?;
    if !self.current_context.error_messages.is_empty() {
      writeln!(f, "   💥 Errors:")?;
      for msg in &self.current_context.error_messages {
        writeln!(f, "     - {}", msg)?;
      }
    }
    if !self.current_context.attached_info.is_empty() {
      writeln!(f, "   📎 Attachments:")?;
      for att in &self.current_context.attached_info {
        writeln!(f, "     - {}: {}", att.name, att.value)?;
      }
    }

    Ok(())
  }
}