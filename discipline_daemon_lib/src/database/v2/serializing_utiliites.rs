pub fn escape_string_for_sqilte_into(string: &String, into: &mut String) {
  into.push('\'');

  for char in string.chars() {
    if char == '\'' {
      into.push_str("''");
    } else {
      into.push(char);
    }
  }

  into.push('\'');
}
