pub struct Options {
    /// Marker to use for bullets of items in unordered lists ('*', '+', or '-', default: '*').
    pub bullet: char,
    // Marker to use in certain cases where the primary bullet doesn’t work
    // ('*', '+', or '-', default: '-' when bullet is '*', '*' otherwise).
    pub bullet_other: char,
    /// Marker to use for bullets of items in ordered lists ('.' or ')', default: '.').
    pub bullet_ordered: char,
    /// Marker to use for emphasis ('*' or '_', default: '*').
    pub emphasis: char,
    // Marker to use for fenced code ('`' or '~', default: '`').
    pub fence: char,
    /// Whether to use fenced code always (bool, default: true). The default is to use fenced code
    /// if there is a language defined, if the code is empty, or if it starts or ends in blank lines.
    pub fences: bool,
    // How to indent the content of list items (default: 'IndentOptions::One').
    pub list_item_indent: IndentOptions,
    /// Marker to use for titles ('"' or "'", default: '"').
    pub quote: char,
    /// Marker to use for thematic breaks ('*', '-', or '_', default: '*').
    pub rule: char,
    // Marker to use for strong ('*' or '_', default: '*').
    pub strong: char,
    // Whether to increment the counter of ordered lists items (bool, default: true).
    pub increment_list_marker: bool,
    /// Whether to add the same number of number signs (#) at the end of an ATX heading as the
    /// opening sequence (bool, default: false).
    pub close_atx: bool,
    /// Whether to always use resource links (bool, default: false). The default is to use autolinks
    /// (<https://example.com>) when possible and resource links ([text](url)) otherwise.
    pub resource_link: bool,
    /// Whether to add spaces between markers in thematic breaks (bool, default: false).
    pub rule_spaces: bool,
    /// Whether to use setext headings when possible (bool, default: false). The default is to always
    /// use ATX headings (# heading) instead of setext headings (heading\n=======). Setext headings
    /// cannot be used for empty headings or headings with a rank of three or more.
    pub setext: bool,
    /// Whether to join definitions without a blank line (bool, default: false).
    pub tight_definitions: bool,
    // Number of markers to use for thematic breaks (u32, default: 3, min: 3).
    pub rule_repetition: u32,
}

#[derive(Copy, Clone)]
pub enum IndentOptions {
    // Depends on the item and its parent list uses 'One' if the item and list are tight and 'Tab'
    // otherwise.
    Mixed,
    // The size of the bullet plus one space.
    One,
    /// Tab stop.
    Tab,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            bullet: '*',
            bullet_other: '-',
            bullet_ordered: '.',
            emphasis: '*',
            fence: '`',
            fences: true,
            increment_list_marker: true,
            rule_repetition: 3,
            list_item_indent: IndentOptions::One,
            quote: '"',
            rule: '*',
            strong: '*',
            close_atx: false,
            rule_spaces: false,
            resource_link: false,
            setext: false,
            tight_definitions: false,
        }
    }
}
