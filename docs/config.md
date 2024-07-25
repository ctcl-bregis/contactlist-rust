Sections in this document is structured like the configuration file. 

# config.json
config.json is a configuration file used for application configuration that does not require changes to the database layout. The file can be safely changed in production environments.

Currently, config.json covers:
- What data is shown on HTML tables
- Links in the navigation bar
- Binding IP address and port

# connection.json
This configuration file covers just database connection and credentials.

Fields:
- cdburl - String - Database server URL
- cdbuser - String - Database username
- cdbpass - String - Database user password
- cdbdb - String - Database name

# schema.json
ContactList makes use of Apache CouchDB as its database. If the configuration file is loaded successfully and the database has no documents, the entire configuration file is added as a document for the ContactList server to compare any differences in the configuration used to set up the database.

Warning: Currently, changes to schema.json may require the database to set up again

## schema
This is configuration for how data is stored in the database.

"schema" is a dictionary of various configuration options.

### fields
Fields is a dictionary of data fields

The ID, key of each field must be an alphanumeric four-character string (characters: abcdefghijklmnopqrstuvwxyz0123456789). Letter case is ignored and it is recommended that lowercase letters are used anyway. Currently this allows for 58,905 unique fields. For most cases, that is plenty. If there is the need for more than 58,905 fields, changes to the code can be done.

The use of four-letter IDs is simply for consistency so everything appears the same length in editors.

#### title
"title" is the title of the field shown in form pages and tables.

#### type
"type" defines the type of the field. Specific types may add more fields.

##### dropdown
The "dropdown" type is as describes.

Specific fields:
- dd - String - The list of dropdown values to use from "dropdown" dictionary

##### string
The "string" type provides an input for a single line of text

Specific fields:
- max - 16-bit Integer (u16) - Maximum length of text data

##### textarea
The "textarea" type is similar to "string" but for multi-line text.
 
Specific fields:
- max - 16-bit Integer (u16) - Maximum length of text data

##### multiddstring
The "multiddstring" type adds a defined amount of fields that are a combination of "string" and "dropdown"

Specific fields
- max - 16-bit Integer (u16) - Maximum amount of field combinations
- dd - String - Dropdown file used for all dropdown fields

No "title" field as "ddtitle" and "sttitle" takes place of it:
- "ddtitle" - String - Title used for all dropdown fields
- "sttitle" - String - Title used for all string fields

### cats
The "cats" dictionary defines the displayed title for the categories ("cat") defined for each field

### dropdown
"dropdown" is a dictionary of possible dropdown values that can be referenced by a *field*

The build script will raise an error if a field references an invalid *dropdown* list. 

Dropdown ID names follow the same four-character rule of field ID names.

