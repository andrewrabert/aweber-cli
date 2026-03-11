use serde_json::Value;
use std::fmt;

#[derive(Debug)]
pub struct ValidationError {
    pub path: String,
    pub message: String,
    pub detail: Option<String>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error: {} at {}", self.message, self.path)?;
        if let Some(detail) = &self.detail {
            write!(f, "\n  {detail}")?;
        }
        Ok(())
    }
}

/// Validate a body_json value. Returns Ok(()) or a list of all errors found.
pub fn validate_body_json(body_json: &Value) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    validate_node(body_json, "<root>", 0, &mut errors);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// -- Constants ---------------------------------------------------------------

const STRUCTURAL_TYPES: &[&str] = &["Document", "ContentRegion", "ContainerRow", "Container"];

const ELEMENT_GROUP_TYPES: &[&str] = &[
    "Article",
    "ButtonElement",
    "Carousel",
    "Coupon",
    "Divider",
    "Feed",
    "FollowMe",
    "Headline",
    "ImageElement",
    "Logo",
    "Paragraph",
    "Product",
    "Share",
    "Signature",
    "Social",
    "TextElement",
    "Video",
];

const LEAF_ELEMENT_TYPES: &[&str] = &["Text", "Image", "ButtonElement"];

// -- Node shape validation ---------------------------------------------------

fn get_str<'a>(obj: &'a serde_json::Map<String, Value>, key: &str) -> Option<&'a str> {
    obj.get(key).and_then(|v| v.as_str())
}

fn validate_node_shape(
    obj: &serde_json::Map<String, Value>,
    path: &str,
    errors: &mut Vec<ValidationError>,
) {
    for key in &["type", "version", "properties", "children"] {
        if !obj.contains_key(*key) {
            errors.push(ValidationError {
                path: path.to_string(),
                message: format!("missing required field \"{key}\""),
                detail: Some("every node must have: type, name, version, properties, children".into()),
            });
        }
    }
    // name can be false or string -- just check it exists
    if !obj.contains_key("name") {
        errors.push(ValidationError {
            path: path.to_string(),
            message: "missing required field \"name\"".into(),
            detail: Some("must be a string for named children or false for unnamed".into()),
        });
    }

    if let Some(version) = obj.get("version") {
        if version.as_i64() != Some(1) && version.as_u64() != Some(1) {
            errors.push(ValidationError {
                path: path.to_string(),
                message: format!("version must be 1, got {version}"),
                detail: None,
            });
        }
    }

    if let Some(props) = obj.get("properties") {
        if !props.is_object() {
            errors.push(ValidationError {
                path: path.to_string(),
                message: "properties must be an object".into(),
                detail: None,
            });
        }
    }

    if let Some(children) = obj.get("children") {
        if !children.is_array() {
            errors.push(ValidationError {
                path: path.to_string(),
                message: "children must be an array".into(),
                detail: None,
            });
        }
    }
}

// -- Tree traversal ----------------------------------------------------------

fn validate_node(
    node: &Value,
    expected_context: &str,
    level: usize,
    errors: &mut Vec<ValidationError>,
) {
    let path = expected_context;

    let obj = match node.as_object() {
        Some(o) => o,
        None => {
            errors.push(ValidationError {
                path: path.to_string(),
                message: "node must be a JSON object".into(),
                detail: None,
            });
            return;
        }
    };

    validate_node_shape(obj, path, errors);

    let node_type = match get_str(obj, "type") {
        Some(t) => t,
        None => return, // already reported missing type
    };

    // Validate type is known
    let all_known: Vec<&str> = STRUCTURAL_TYPES
        .iter()
        .chain(ELEMENT_GROUP_TYPES.iter())
        .chain(LEAF_ELEMENT_TYPES.iter())
        .copied()
        .collect();
    if !all_known.contains(&node_type) {
        errors.push(ValidationError {
            path: path.to_string(),
            message: format!("unknown component type \"{node_type}\""),
            detail: Some(format!("valid types: {}", all_known.join(", "))),
        });
        return;
    }

    // Level-based nesting validation
    match level {
        0 => validate_document(obj, node_type, path, errors),
        1 => validate_content_region(obj, node_type, path, errors),
        2 => validate_container_row(obj, node_type, path, errors),
        3 => validate_container(obj, node_type, path, errors),
        4 => validate_element_group(obj, node_type, path, errors),
        5 => validate_leaf_element(obj, node_type, path, errors),
        _ => {
            errors.push(ValidationError {
                path: path.to_string(),
                message: format!("tree too deep (level {level})"),
                detail: Some("maximum depth is 6 levels (0=Document through 5=leaf)".into()),
            });
        }
    }
}

// -- Level validators --------------------------------------------------------

fn validate_document(
    obj: &serde_json::Map<String, Value>,
    node_type: &str,
    path: &str,
    errors: &mut Vec<ValidationError>,
) {
    if node_type != "Document" {
        errors.push(ValidationError {
            path: path.to_string(),
            message: format!("root node must be type \"Document\", got \"{node_type}\""),
            detail: None,
        });
        return;
    }
    validate_document_properties(obj, path, errors);
    validate_children_of_type(obj, path, "ContentRegion", 1, errors);
}

fn validate_content_region(
    obj: &serde_json::Map<String, Value>,
    node_type: &str,
    path: &str,
    errors: &mut Vec<ValidationError>,
) {
    if node_type != "ContentRegion" {
        errors.push(ValidationError {
            path: path.to_string(),
            message: format!("expected ContentRegion, got \"{node_type}\""),
            detail: None,
        });
        return;
    }
    validate_children_of_type(obj, path, "ContainerRow", 2, errors);
}

fn validate_container_row(
    obj: &serde_json::Map<String, Value>,
    node_type: &str,
    path: &str,
    errors: &mut Vec<ValidationError>,
) {
    if node_type != "ContainerRow" {
        errors.push(ValidationError {
            path: path.to_string(),
            message: format!("expected ContainerRow, got \"{node_type}\""),
            detail: None,
        });
        return;
    }
    validate_children_of_type(obj, path, "Container", 3, errors);
    validate_container_widths(obj, path, errors);
}

fn validate_container(
    obj: &serde_json::Map<String, Value>,
    node_type: &str,
    path: &str,
    errors: &mut Vec<ValidationError>,
) {
    if node_type != "Container" {
        errors.push(ValidationError {
            path: path.to_string(),
            message: format!("expected Container, got \"{node_type}\""),
            detail: None,
        });
        return;
    }
    validate_container_properties(obj, path, errors);

    let children = match obj.get("children").and_then(|v| v.as_array()) {
        Some(c) => c,
        None => return,
    };

    for (i, child) in children.iter().enumerate() {
        let child_type = child
            .as_object()
            .and_then(|o| get_str(o, "type"))
            .unwrap_or("<unknown>");
        if !ELEMENT_GROUP_TYPES.contains(&child_type) {
            errors.push(ValidationError {
                path: format!("{path} > {child_type}[{i}]"),
                message: format!("invalid ElementGroup type \"{child_type}\" in Container"),
                detail: Some(format!("valid types: {}", ELEMENT_GROUP_TYPES.join(", "))),
            });
        } else {
            let child_path = format!("{path} > {child_type}[{i}]");
            validate_node(child, &child_path, 4, errors);
        }
    }
}

fn validate_element_group(
    obj: &serde_json::Map<String, Value>,
    node_type: &str,
    path: &str,
    errors: &mut Vec<ValidationError>,
) {
    if !ELEMENT_GROUP_TYPES.contains(&node_type) {
        errors.push(ValidationError {
            path: path.to_string(),
            message: format!("expected ElementGroup type, got \"{node_type}\""),
            detail: None,
        });
        return;
    }

    validate_element_group_properties(obj, node_type, path, errors);
    validate_named_children(obj, node_type, path, errors);
}

fn validate_leaf_element(
    obj: &serde_json::Map<String, Value>,
    node_type: &str,
    path: &str,
    errors: &mut Vec<ValidationError>,
) {
    if !LEAF_ELEMENT_TYPES.contains(&node_type) {
        errors.push(ValidationError {
            path: path.to_string(),
            message: format!("expected leaf element (Text, Image, ButtonElement), got \"{node_type}\""),
            detail: None,
        });
        return;
    }

    // Leaf elements must have children: []
    if let Some(children) = obj.get("children").and_then(|v| v.as_array()) {
        if !children.is_empty() {
            errors.push(ValidationError {
                path: path.to_string(),
                message: format!("leaf element \"{node_type}\" must have empty children"),
                detail: None,
            });
        }
    }

    validate_leaf_properties(obj, node_type, path, errors);
}

// -- Children helpers --------------------------------------------------------

fn validate_children_of_type(
    obj: &serde_json::Map<String, Value>,
    path: &str,
    expected_child_type: &str,
    child_level: usize,
    errors: &mut Vec<ValidationError>,
) {
    let children = match obj.get("children").and_then(|v| v.as_array()) {
        Some(c) => c,
        None => return, // already reported
    };

    if children.is_empty() {
        errors.push(ValidationError {
            path: path.to_string(),
            message: format!("must have at least one {expected_child_type} child"),
            detail: None,
        });
        return;
    }

    for (i, child) in children.iter().enumerate() {
        let child_type = child
            .as_object()
            .and_then(|o| get_str(o, "type"))
            .unwrap_or("<unknown>");

        let child_name = child
            .as_object()
            .and_then(|o| o.get("name"))
            .and_then(|v| v.as_str());

        let child_path = if let Some(name) = child_name {
            format!("{path} > {child_type}(\"{name}\")")
        } else {
            format!("{path} > {child_type}[{i}]")
        };

        if child_type != expected_child_type {
            errors.push(ValidationError {
                path: child_path.clone(),
                message: format!(
                    "invalid child type \"{child_type}\"",
                ),
                detail: Some(format!("expected: {expected_child_type}")),
            });
        } else {
            validate_node(child, &child_path, child_level, errors);
        }
    }
}

// -- Named children for ElementGroups ----------------------------------------

struct ExpectedChild {
    child_type: &'static str,
    name: Option<&'static str>, // None means name: false
}

fn expected_children(group_type: &str) -> &'static [ExpectedChild] {
    match group_type {
        "Paragraph" => &[ExpectedChild { child_type: "Text", name: Some("content") }],
        "TextElement" => &[ExpectedChild { child_type: "Text", name: Some("content") }],
        "Headline" => &[ExpectedChild { child_type: "Text", name: Some("content") }],
        "ImageElement" => &[ExpectedChild { child_type: "Image", name: Some("image") }],
        "Logo" => &[ExpectedChild { child_type: "Image", name: Some("image") }],
        "Article" => &[
            ExpectedChild { child_type: "Text", name: Some("title") },
            ExpectedChild { child_type: "Text", name: Some("content") },
            ExpectedChild { child_type: "Text", name: Some("link_text") },
            ExpectedChild { child_type: "Image", name: Some("image") },
        ],
        "Share" => &[ExpectedChild { child_type: "Text", name: Some("content") }],
        "Signature" => &[
            ExpectedChild { child_type: "Text", name: Some("from_text") },
            ExpectedChild { child_type: "Text", name: Some("from_name") },
            ExpectedChild { child_type: "Text", name: Some("from_email") },
            ExpectedChild { child_type: "Text", name: Some("signature") },
            ExpectedChild { child_type: "Image", name: Some("image") },
        ],
        "Video" => &[ExpectedChild { child_type: "Image", name: None }],
        "Product" => &[
            ExpectedChild { child_type: "Text", name: Some("title") },
            ExpectedChild { child_type: "Text", name: Some("content") },
            ExpectedChild { child_type: "Text", name: Some("price") },
            ExpectedChild { child_type: "Image", name: Some("image") },
            ExpectedChild { child_type: "ButtonElement", name: Some("button") },
        ],
        "Coupon" => &[
            ExpectedChild { child_type: "Text", name: Some("title") },
            ExpectedChild { child_type: "Text", name: Some("content") },
            ExpectedChild { child_type: "Text", name: Some("disclaimer") },
            ExpectedChild { child_type: "Text", name: Some("expiration") },
            ExpectedChild { child_type: "Image", name: Some("image") },
        ],
        "FollowMe" => &[ExpectedChild { child_type: "Text", name: Some("content") }],
        // Childless groups
        "ButtonElement" | "Divider" | "Feed" | "Social" => &[],
        // Carousel: special case, handled separately
        "Carousel" => &[],
        _ => &[],
    }
}

fn validate_named_children(
    obj: &serde_json::Map<String, Value>,
    group_type: &str,
    path: &str,
    errors: &mut Vec<ValidationError>,
) {
    let children = match obj.get("children").and_then(|v| v.as_array()) {
        Some(c) => c,
        None => return,
    };

    // Special case: Carousel allows 1+ Image children all named "image"
    if group_type == "Carousel" {
        if children.is_empty() {
            errors.push(ValidationError {
                path: path.to_string(),
                message: "Carousel must have at least one Image child".into(),
                detail: None,
            });
            return;
        }
        for (i, child) in children.iter().enumerate() {
            let child_type = child.as_object().and_then(|o| get_str(o, "type")).unwrap_or("");
            if child_type != "Image" {
                errors.push(ValidationError {
                    path: format!("{path} > [child {i}]"),
                    message: format!("Carousel children must be Image, got \"{child_type}\""),
                    detail: None,
                });
            } else {
                let child_path = format!("{path} > Image(\"image\")[{i}]");
                validate_node(child, &child_path, 5, errors);
            }
        }
        return;
    }

    let expected = expected_children(group_type);

    // Childless groups
    if expected.is_empty() {
        if !children.is_empty() {
            errors.push(ValidationError {
                path: path.to_string(),
                message: format!("{group_type} must have no children"),
                detail: None,
            });
        }
        return;
    }

    // Check expected children are present
    for exp in expected {
        let name_match = |child: &Value| -> bool {
            let child_obj = match child.as_object() {
                Some(o) => o,
                None => return false,
            };
            let ct = get_str(child_obj, "type").unwrap_or("");
            if ct != exp.child_type {
                return false;
            }
            match exp.name {
                Some(expected_name) => {
                    child_obj.get("name").and_then(|v| v.as_str()) == Some(expected_name)
                }
                None => {
                    // name should be false
                    child_obj.get("name") == Some(&Value::Bool(false))
                }
            }
        };

        if let Some(child) = children.iter().find(|c| name_match(c)) {
            let child_path = match exp.name {
                Some(n) => format!("{path} > {}(\"{n}\")", exp.child_type),
                None => format!("{path} > {}", exp.child_type),
            };
            validate_node(child, &child_path, 5, errors);
        } else {
            let expected_desc = match exp.name {
                Some(n) => format!("{}(\"{}\")", exp.child_type, n),
                None => format!("{} (unnamed)", exp.child_type),
            };
            errors.push(ValidationError {
                path: path.to_string(),
                message: format!("missing required child {expected_desc}"),
                detail: Some(format!(
                    "expected children: {}",
                    expected
                        .iter()
                        .map(|e| match e.name {
                            Some(n) => format!("{}(\"{}\")", e.child_type, n),
                            None => format!("{} (unnamed)", e.child_type),
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                )),
            });
        }
    }
}

// -- Property validators -----------------------------------------------------

fn check_enum(
    props: &serde_json::Map<String, Value>,
    key: &str,
    valid: &[&str],
    path: &str,
    errors: &mut Vec<ValidationError>,
) {
    if let Some(val) = props.get(key).and_then(|v| v.as_str()) {
        if !valid.contains(&val) {
            errors.push(ValidationError {
                path: path.to_string(),
                message: format!("enum value \"{val}\" not valid for property \"{key}\""),
                detail: Some(format!("valid values: {}", valid.join(", "))),
            });
        }
    }
}

fn check_hex_id(
    props: &serde_json::Map<String, Value>,
    key: &str,
    path: &str,
    errors: &mut Vec<ValidationError>,
) {
    if let Some(val) = props.get(key).and_then(|v| v.as_str()) {
        if !val.is_empty() && (val.len() != 24 || !val.chars().all(|c| c.is_ascii_hexdigit())) {
            errors.push(ValidationError {
                path: path.to_string(),
                message: format!("\"{key}\" must be a 24-character hex string"),
                detail: Some(format!("got: \"{val}\"")),
            });
        }
    }
}

fn validate_document_properties(
    obj: &serde_json::Map<String, Value>,
    path: &str,
    errors: &mut Vec<ValidationError>,
) {
    let props = match obj.get("properties").and_then(|v| v.as_object()) {
        Some(p) => p,
        None => return,
    };
    check_enum(props, "buttonAlign", &["left", "center", "right"], path, errors);
    check_enum(props, "borderStyle", &["none", "dotted", "dashed", "solid"], path, errors);
    check_hex_id(props, "theme_id", path, errors);
    check_hex_id(props, "template_id", path, errors);
}

fn validate_container_properties(
    obj: &serde_json::Map<String, Value>,
    path: &str,
    errors: &mut Vec<ValidationError>,
) {
    let props = match obj.get("properties").and_then(|v| v.as_object()) {
        Some(p) => p,
        None => return,
    };
    check_enum(props, "valign", &["top", "middle", "bottom"], path, errors);
}

fn validate_container_widths(
    obj: &serde_json::Map<String, Value>,
    path: &str,
    errors: &mut Vec<ValidationError>,
) {
    let children = match obj.get("children").and_then(|v| v.as_array()) {
        Some(c) => c,
        None => return,
    };

    let mut total: f64 = 0.0;
    let mut all_parseable = true;
    for child in children {
        let width = child
            .as_object()
            .and_then(|o| o.get("properties"))
            .and_then(|p| p.as_object())
            .and_then(|p| p.get("width"))
            .and_then(|v| v.as_str())
            .unwrap_or("100%");
        if let Some(pct) = width.strip_suffix('%') {
            if let Ok(val) = pct.trim().parse::<f64>() {
                total += val;
            } else {
                all_parseable = false;
            }
        } else {
            all_parseable = false;
        }
    }

    if all_parseable && (total - 100.0).abs() > 0.01 {
        errors.push(ValidationError {
            path: path.to_string(),
            message: format!(
                "Container widths in ContainerRow must sum to 100%, got {total:.1}%"
            ),
            detail: None,
        });
    }
}

fn validate_element_group_properties(
    obj: &serde_json::Map<String, Value>,
    node_type: &str,
    path: &str,
    errors: &mut Vec<ValidationError>,
) {
    let props = match obj.get("properties").and_then(|v| v.as_object()) {
        Some(p) => p,
        None => return,
    };

    match node_type {
        "Article" => {
            check_enum(
                props,
                "article_style",
                &["Article", "Article2", "Article3", "Article4"],
                path,
                errors,
            );
        }
        "Divider" => {
            check_enum(props, "borderStyle", &["none", "dotted", "dashed", "solid"], path, errors);
        }
        "Social" => {
            check_enum(props, "align", &["left", "center", "right"], path, errors);
            check_enum(props, "size", &["sm", "md", "lg"], path, errors);
            check_enum(
                props,
                "variant",
                &["none", "circle", "rounded", "square"],
                path,
                errors,
            );
        }
        "Video" => {
            check_enum(props, "align", &["left", "center", "right"], path, errors);
        }
        "Feed" => {
            check_enum(props, "feedType", &["dynamic", "static"], path, errors);
            check_enum(props, "order", &["ascending", "descending"], path, errors);
            check_enum(props, "layout", &["block", "postcard", "video-16:9"], path, errors);
        }
        "Carousel" => {
            check_enum(props, "type", &["carousel", "slides"], path, errors);
        }
        "ButtonElement" => {
            check_enum(props, "align", &["left", "center", "right"], path, errors);
            if let Some(url) = props.get("url").and_then(|v| v.as_str()) {
                if url.is_empty() {
                    errors.push(ValidationError {
                        path: path.to_string(),
                        message: "ButtonElement url must not be empty".into(),
                        detail: None,
                    });
                }
            }
        }
        _ => {}
    }
}

fn validate_leaf_properties(
    obj: &serde_json::Map<String, Value>,
    node_type: &str,
    path: &str,
    errors: &mut Vec<ValidationError>,
) {
    let props = match obj.get("properties").and_then(|v| v.as_object()) {
        Some(p) => p,
        None => return,
    };

    match node_type {
        "Image" => {
            check_enum(
                props,
                "align",
                &["left", "center", "right", "left-nowrap", "right-nowrap"],
                path,
                errors,
            );
            // Check src is HTTPS when present and non-empty
            if let Some(src) = props.get("src").and_then(|v| v.as_str()) {
                if !src.is_empty() && !src.starts_with("https://") {
                    errors.push(ValidationError {
                        path: path.to_string(),
                        message: "Image src must be HTTPS".into(),
                        detail: Some(format!("got: \"{src}\"")),
                    });
                }
            }
        }
        "ButtonElement" => {
            check_enum(props, "align", &["left", "center", "right"], path, errors);
            if let Some(url) = props.get("url").and_then(|v| v.as_str()) {
                if url.is_empty() {
                    errors.push(ValidationError {
                        path: path.to_string(),
                        message: "ButtonElement url must not be empty".into(),
                        detail: None,
                    });
                }
            }
        }
        _ => {}
    }
}

// -- Tests -------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn minimal_document() -> Value {
        json!({
            "type": "Document",
            "name": "test",
            "version": 1,
            "properties": {"background_color": "#FFFFFF"},
            "children": [{
                "type": "ContentRegion",
                "name": "Body",
                "version": 1,
                "properties": {},
                "children": [{
                    "type": "ContainerRow",
                    "name": false,
                    "version": 1,
                    "properties": {"stackOnMobile": true},
                    "children": [{
                        "type": "Container",
                        "name": false,
                        "version": 1,
                        "properties": {"width": "100%", "padding": "0px", "valign": "top"},
                        "children": [{
                            "type": "Paragraph",
                            "name": false,
                            "version": 1,
                            "properties": {},
                            "children": [{
                                "type": "Text",
                                "name": "content",
                                "version": 1,
                                "properties": {"value": "<p>Hello</p>"},
                                "children": []
                            }]
                        }]
                    }]
                }]
            }]
        })
    }

    #[test]
    fn valid_minimal_document_passes() {
        assert!(validate_body_json(&minimal_document()).is_ok());
    }

    #[test]
    fn root_must_be_document() {
        let bad = json!({
            "type": "Paragraph",
            "name": false,
            "version": 1,
            "properties": {},
            "children": []
        });
        let errs = validate_body_json(&bad).unwrap_err();
        assert!(errs.iter().any(|e| e.message.contains("root node must be type \"Document\"")));
    }

    #[test]
    fn missing_required_fields() {
        let bad = json!({"type": "Document"});
        let errs = validate_body_json(&bad).unwrap_err();
        assert!(errs.iter().any(|e| e.message.contains("missing required field")));
    }

    #[test]
    fn invalid_version() {
        let bad = json!({
            "type": "Document",
            "name": "test",
            "version": 2,
            "properties": {},
            "children": []
        });
        let errs = validate_body_json(&bad).unwrap_err();
        assert!(errs.iter().any(|e| e.message.contains("version must be 1")));
    }

    #[test]
    fn invalid_child_type_in_document() {
        let bad = json!({
            "type": "Document",
            "name": "test",
            "version": 1,
            "properties": {},
            "children": [{
                "type": "Paragraph",
                "name": false,
                "version": 1,
                "properties": {},
                "children": []
            }]
        });
        let errs = validate_body_json(&bad).unwrap_err();
        assert!(errs.iter().any(|e| e.message.contains("invalid child type")));
    }

    #[test]
    fn invalid_enum_value() {
        let mut doc = minimal_document();
        doc["children"][0]["children"][0]["children"][0]["properties"]["valign"] =
            json!("invalid");
        let errs = validate_body_json(&doc).unwrap_err();
        assert!(errs.iter().any(|e| e.message.contains("enum value")));
    }

    #[test]
    fn container_widths_must_sum_to_100() {
        let mut doc = minimal_document();
        // Add a second container with 60% each
        let row = &mut doc["children"][0]["children"][0];
        row["children"] = json!([
            {
                "type": "Container",
                "name": false,
                "version": 1,
                "properties": {"width": "60%", "valign": "top"},
                "children": [{
                    "type": "Paragraph",
                    "name": false,
                    "version": 1,
                    "properties": {},
                    "children": [{"type": "Text", "name": "content", "version": 1, "properties": {"value": ""}, "children": []}]
                }]
            },
            {
                "type": "Container",
                "name": false,
                "version": 1,
                "properties": {"width": "60%", "valign": "top"},
                "children": [{
                    "type": "Paragraph",
                    "name": false,
                    "version": 1,
                    "properties": {},
                    "children": [{"type": "Text", "name": "content", "version": 1, "properties": {"value": ""}, "children": []}]
                }]
            }
        ]);
        let errs = validate_body_json(&doc).unwrap_err();
        assert!(errs.iter().any(|e| e.message.contains("sum to 100%")));
    }

    #[test]
    fn missing_named_child() {
        let mut doc = minimal_document();
        // Replace Paragraph with Article that's missing children
        doc["children"][0]["children"][0]["children"][0]["children"] = json!([{
            "type": "Article",
            "name": false,
            "version": 1,
            "properties": {},
            "children": []
        }]);
        let errs = validate_body_json(&doc).unwrap_err();
        assert!(errs.iter().any(|e| e.message.contains("missing required child")));
    }

    #[test]
    fn image_src_must_be_https() {
        let mut doc = minimal_document();
        // Replace Paragraph with ImageElement containing http:// src
        doc["children"][0]["children"][0]["children"][0]["children"] = json!([{
            "type": "ImageElement",
            "name": false,
            "version": 1,
            "properties": {},
            "children": [{
                "type": "Image",
                "name": "image",
                "version": 1,
                "properties": {"src": "http://example.com/img.png"},
                "children": []
            }]
        }]);
        let errs = validate_body_json(&doc).unwrap_err();
        assert!(errs.iter().any(|e| e.message.contains("HTTPS")));
    }

    #[test]
    fn invalid_hex_id() {
        let mut doc = minimal_document();
        doc["properties"]["theme_id"] = json!("not-a-valid-id");
        let errs = validate_body_json(&doc).unwrap_err();
        assert!(errs.iter().any(|e| e.message.contains("24-character hex string")));
    }

    #[test]
    fn childless_groups_reject_children() {
        let mut doc = minimal_document();
        doc["children"][0]["children"][0]["children"][0]["children"] = json!([{
            "type": "Divider",
            "name": false,
            "version": 1,
            "properties": {},
            "children": [{"type": "Text", "name": "x", "version": 1, "properties": {}, "children": []}]
        }]);
        let errs = validate_body_json(&doc).unwrap_err();
        assert!(errs.iter().any(|e| e.message.contains("must have no children")));
    }

    #[test]
    fn carousel_requires_image_children() {
        let mut doc = minimal_document();
        doc["children"][0]["children"][0]["children"][0]["children"] = json!([{
            "type": "Carousel",
            "name": false,
            "version": 1,
            "properties": {},
            "children": [{
                "type": "Text",
                "name": "content",
                "version": 1,
                "properties": {},
                "children": []
            }]
        }]);
        let errs = validate_body_json(&doc).unwrap_err();
        assert!(errs.iter().any(|e| e.message.contains("Carousel children must be Image")));
    }
}
