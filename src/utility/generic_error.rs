
struct ErrorContext {
  action: String,
  error_messages: Vec<String>,
  attached_info: Vec<ErrorContextAttachment>
}

struct ErrorContextAttachment {
  name: String,
  value: String,
}

pub struct GenericError {
  current_context: ErrorContext,
  contexts: Vec<ErrorContext>
}

impl GenericError {
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

  pub fn add_error(mut self, error_message: impl Into<String>) -> Self {
    self.current_context.error_messages.push(error_message.into());
    self
  }

  pub fn add_attachment(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
    self.current_context.attached_info.push(ErrorContextAttachment { 
      name: name.into(), 
      value: value.into()
    });

    self
  }

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
}

use std::fmt;

impl fmt::Debug for GenericError {
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
