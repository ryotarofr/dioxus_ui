use dioxus::prelude::*;

/// Simple TableView component converted from React
/// This is a basic implementation with core table functionality

#[derive(Props, Clone, PartialEq)]
pub struct TableViewProps {
    /// Table data as strings for simplicity
    pub data: Vec<Vec<String>>,
    
    /// Column headers
    #[props(default = vec![])]
    pub headers: Vec<String>,
    
    /// Table title
    #[props(default)]
    pub title: Option<String>,
    
    /// Additional CSS class
    #[props(default)]
    pub class: Option<String>,
    
    /// Additional inline style
    #[props(default)]
    pub style: Option<String>,
    
    /// Tab index for focus management
    #[props(default)]
    pub tab_index: Option<i32>,
}

#[component]
pub fn TableView(props: TableViewProps) -> Element {
    // Build CSS class
    let table_class = match &props.class {
        Some(additional_class) => format!("Table {}", additional_class),
        None => "Table".to_string(),
    };
    
    // Build CSS style
    let table_style = props.style.clone().unwrap_or_default();
    
    // Keyboard event handler
    let handle_key_down = move |event: KeyboardEvent| {
        match event.key() {
            Key::ArrowDown | Key::ArrowUp | Key::ArrowLeft | Key::ArrowRight => {
                event.prevent_default();
                // Basic keyboard navigation would go here
            }
            Key::Enter => {
                event.prevent_default();
                // Enter key handling would go here
            }
            Key::Character(ch) if ch == " " => {
                event.prevent_default();
                // Space key handling would go here
            }
            _ => {}
        }
    };

    rsx! {
        div {
            "data-testid": "TableView",
            class: "{table_class}",
            style: "{table_style}",
            tabindex: props.tab_index.unwrap_or(0),
            onkeydown: handle_key_down,
            
            // Title section
            if let Some(title) = &props.title {
                div {
                    class: "Header",
                    div {
                        class: "Title",
                        "{title}"
                    }
                }
            }
            
            // Table content
            div {
                class: "Grid",
                style: "overflow: auto;",
                
                // Header row
                if !props.headers.is_empty() {
                    div {
                        class: "GridHeaderRow",
                        for header in &props.headers {
                            div {
                                class: "HeaderCell",
                                "{header}"
                            }
                        }
                    }
                }
                
                // Data rows
                div {
                    class: "VirtualGrid",
                    for (row_index, row) in props.data.iter().enumerate() {
                        div {
                            key: "{row_index}",
                            class: if row_index % 2 == 0 { "ContentRow" } else { "ContentRow Even" },
                            for (col_index, cell) in row.iter().enumerate() {
                                div {
                                    key: "{col_index}",
                                    class: "DataCell",
                                    "{cell}"
                                }
                            }
                        }
                    }
                }
            }
            
            // Paginator placeholder
            div {
                class: "Paginator",
                button {
                    onclick: move |_| {
                        // Previous page logic would go here
                    },
                    "Previous"
                }
                span {
                    class: "PageInfo",
                    "Page 1 / 1"
                }
                button {
                    onclick: move |_| {
                        // Next page logic would go here
                    },
                    "Next"
                }
            }
        }
    }
}

// Export for backward compatibility with existing code that may import complex types
pub use super::use_table::TableViewStateProps;