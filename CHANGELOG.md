# ContactList-rust

## 0.1.0 - [UNRELEASED] - Rust Update
Rust rewrite of ContactList adding Bootstrap theming and the switch to CouchDB from SQL databases such as SQLite3.

### Additions
- Bootstrap theming
- Custom tablesorter theme
- Edit button on view pages
- More social media platforms added to the base configuration
- WYSIWYG editor for text
- Optional "notes" per field

### Removed
- Theme switcher, for now

### Changes
- config/config.json layout changed to make deserialization by Serde easier
- ContactList now uses CouchDB instead of SQLite3 for data storage

### Fixes
- Date ordering on tables