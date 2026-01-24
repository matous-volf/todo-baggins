### Fluent grammar examples for dioxus-i18n.

## These examples demonstrate Fluent file grammar and how dioxus-i18n can be
## used to access these translations.

## Examples derived from: https://projectfluent.org/fluent/guide/index.html

# Simple message
simple-message = This is a simple message.

# $name (String) - The name you want to display.
message-with-variable = This is a message with a variable: { $name }.

# Reference to a term.
-a-term = This is a common term used by many messages.
message-referencing-a-term = This is a message with a reference: { -a-term }.

# Use of special characters.
message-with-special-character = This message contain opening curly brace {"{"} and a closing curly brace {"}"}.

# Message with blanks.
blank-is-removed =     This message starts with no blanks.
blank-is-preserved = {"    "}This message starts with 4 spaces (note HTML contracts them).

# Message with attributes.
message-with-attributes = Predefined value
    .placeholder = email@example.com
    .aria-label = Login input value
    .title = Type your login email

# Message with quotes.
literal-quote-cryptic = Text in {"\""}double quotes{"\""}.
literal-quote-preferred = Text in "double quotes".

# Message with Unicode characters.
unicode-cryptic = {"\u2605"} {"\u2606"} {"\u2728"} {"\u262F"} {"\u263A"}
unicode-preferred = ★ ☆ ✨ ☯ ☺

# Message with a placeable.
single-line = Text can be written in a single line.

multi-line = Text can also span multiple lines
    as long as each new line is indented
    by at least one space.

block-line =
    Sometimes it's more readable to format
    multiline text as a "block", which means
    starting it on a new line. All lines must
    be indented by at least one space.

# Message using functions.
#
# Note: Builtin functions are currently unsupported: See Fluent issue https://github.com/projectfluent/fluent-rs/issues/181
# The Bundle::add_builtins() function is not published at the time of writing this example.
#
# Using a builtin currently results in an error.
#
# $duration (Number) - The duration in seconds.
time-elapsed-no-function = Time elapsed: { $duration }s.
time-elapsed-function = Currently unsupported: error raised: { NUMBER($duration) }.

# Message reference.
referenced-message = Referenced message
message-referencing-another-message = Message referencing another message: { referenced-message }.

# Message selection plurals.
message-selection-plurals =
    { $value ->
       *[one] Value is one: { $value }.
        [other] Value is more than one: { $value }.
    }

# Message selection numeric.
# Argument must be numeric.
message-selection-numeric =
    { NUMERIC($value) ->
        [0.0]   Zero: { $value }.
       *[0.5]   A half: { $value }.
        [other] Other ($value)
    }

# Message selection number.
#
# Note: Builtin functions are currently unsupported: See Fluent issue https://github.com/projectfluent/fluent-rs/issues/181
# The Bundle::add_builtins() function is not published at the time of writing this example.
#
# Using the NUMBER builtin always results in a default behaviour.
#
message-selection-number = { NUMBER($pos, type: "ordinal") ->
   [1] First!
   [one] {$pos}st
   [two] {$pos}nd
   [few] {$pos}rd
  *[other] {$pos}th
}


# Variables in references.
-term-using-variable = https://{ $host }
message-using-term-with-variable = For example: { -term-using-variable(host: "example.com") }.

-term-using-variable-2 =
    { $case ->
       *[nominative] Firefox
        [locative] Firefoksie
    }
message-using-term-with-variable-2-1 = Informacje o { -term-using-variable-2(case: "locative") }.
message-using-term-with-variable-2-2 = About { -term-using-variable-2(case: "nominative") }.
message-using-term-with-variable-2-default = About { -term-using-variable-2(case: "") }.
message-using-term-with-variable-2-not-provided = About { -term-using-variable-2 }.

-brand-name =
    { $case ->
       *[nominative] Firefox
        [locative] Firefoksie
    }

string-literal = { "string literal" }
number-literal-1 = { 1 }
number-literal-2 = { -123 }
number-literal-3 = { 3.14 }
inline-expression-placeable-1 = { { "string literal" } }
inline-expression-placeable-2 = { { 123 } }
