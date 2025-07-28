use dioxus::prelude::*;
use super::use_sort::SortOrder;
use crate::function::obj_calc::{Calc, RhsValue};
use crate::types::position::{Position, PartialPosition};
use crate::types::size::{Size, PartialSize};

#[derive(Clone, PartialEq)]
pub struct ResizeOrigin {
    pub cursor_position: Position,
    pub element_size: Size,
}

impl Default for ResizeOrigin {
    fn default() -> Self {
        Self {
            cursor_position: Position::init(),
            element_size: Size::init(),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct HeaderCellProps {
    #[props(default = false)]
    pub hidden: bool,
    pub sort_order_is_changeable: bool,
    pub sort_order: Option<SortOrder>,
    pub shift_sort_order: EventHandler<usize>,
    pub toggle_sort_order_and_apply_once: EventHandler<()>,
    pub set_width: EventHandler<Option<String>>,
    pub focused: bool,
    pub is_tail: bool,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub children: Element,
}

/// ヘッダーセル
#[component]
pub fn HeaderCell(props: HeaderCellProps) -> Element {
    const MIN_WIDTH: f64 = 20.0;
    const SHOW_RESIZE_THUMB: bool = true;
    
    let mut resize_origin = use_signal(ResizeOrigin::default);
    
    let data_sort_order = if props.sort_order_is_changeable {
        props.sort_order.as_ref().map(|o| o.to_str())
    } else {
        None
    };
    
    let mut class_list = vec!["header-cell".to_string()];
    
    if props.hidden {
        class_list.push("hidden".to_string());
    }
    
    if props.focused {
        class_list.push("focused".to_string());
    }
    
    if let Some(additional_class) = &props.class {
        class_list.push(additional_class.clone());
    }
    
    let mut resize_thumb_classes = vec![
        "resize-thumb".to_string(),
        "show-in-hover-only".to_string(),
    ];
    
    if !SHOW_RESIZE_THUMB {
        resize_thumb_classes.push("hidden".to_string());
    }

    let base_style = "cursor: pointer; position: relative; padding: 0.5em; padding-right: 15px; border: var(--border-em); border-width: 0 1px 0 0; overflow: hidden; white-space: nowrap;";
    let mut style_str = base_style.to_string();
    
    if props.focused {
        style_str.push_str(" outline: 2px solid black;");
    }
    
    if props.hidden {
        style_str.push_str(" display: none;");
    }
    
    let sort_indicator_style = "cursor: pointer; width: 9px; position: relative;";
    let sort_before_after_base = "content: ''; position: absolute; top: 50%; right: 3px; width: 0; height: 0; border: 5px solid transparent;";
    
    let resize_thumb_style = if SHOW_RESIZE_THUMB {
        "cursor: col-resize; position: absolute; top: 0; right: 0; bottom: 0; width: 3px; background-color: var(--color-em);"
    } else {
        "display: none;"
    };

    rsx! {
        div {
            class: class_list.join(" "),
            style: style_str,
            onclick: move |event| {
                if let Some(onclick) = props.onclick {
                    onclick.call(event);
                }
                
                if !props.sort_order_is_changeable {
                    return;
                }
                
                // Note: Dioxus doesn't have direct access to ctrlKey in MouseEvent
                // This would need to be handled differently in a real implementation
                props.toggle_sort_order_and_apply_once.call(());
            },
            
            {props.children}
            
            div {
                class: "sort-order-indicator",
                style: sort_indicator_style,
                "data-sort-order": data_sort_order.unwrap_or(""),
                
                // Sort order up arrow (::before equivalent)
                div {
                    style: format!("{} margin-top: -10px; border-bottom-color: currentcolor; {}", 
                        sort_before_after_base,
                        if data_sort_order == Some("none") || data_sort_order == Some("asc") { "" } else { "display: none;" }
                    ),
                }
                
                // Sort order down arrow (::after equivalent) 
                div {
                    style: format!("{} margin-top: 2px; border-top-color: currentcolor; {}", 
                        sort_before_after_base,
                        if data_sort_order == Some("none") || data_sort_order == Some("desc") { "" } else { "display: none;" }
                    ),
                }
            }
            
            div {
                class: resize_thumb_classes.join(" "),
                style: resize_thumb_style,
                onclick: |event| event.stop_propagation(),
                onmousedown: move |event| {
                    let cursor_pos = Position::from_partial(PartialPosition {
                        x: Some(event.client_coordinates().x),
                        y: Some(event.client_coordinates().y),
                    });
                    
                    // Note: In a real implementation, you'd need to get the parent element size
                    // This is a simplified version
                    let element_size = Size::from_partial(PartialSize {
                        width: Some(100.0), // Would be actual element width
                        height: Some(30.0), // Would be actual element height
                    });
                    
                    resize_origin.set(ResizeOrigin {
                        cursor_position: cursor_pos,
                        element_size,
                    });
                },
                onmousemove: move |event| {
                    let current_cursor = Position::from_partial(PartialPosition {
                        x: Some(event.client_coordinates().x),
                        y: Some(event.client_coordinates().y),
                    });
                    
                    let origin = resize_origin.read();
                    let cursor_pos_map = std::collections::HashMap::from([
                        ("x".to_string(), current_cursor.x),
                        ("y".to_string(), current_cursor.y),
                    ]);
                    let origin_cursor_map = std::collections::HashMap::from([
                        ("x".to_string(), origin.cursor_position.x),
                        ("y".to_string(), origin.cursor_position.y),
                    ]);
                    
                    let vector = Calc::minus(cursor_pos_map, RhsValue::Object(origin_cursor_map));
                    let vector_position = Position {
                        x: vector.get("x").copied().unwrap_or(0.0),
                        y: vector.get("y").copied().unwrap_or(0.0),
                    };
                    
                    let origin_size_map = std::collections::HashMap::from([
                        ("width".to_string(), origin.element_size.width),
                        ("height".to_string(), origin.element_size.height),
                    ]);
                    let vector_size = Size::from_position(vector_position);
                    let vector_size_map = std::collections::HashMap::from([
                        ("width".to_string(), vector_size.width),
                        ("height".to_string(), vector_size.height),
                    ]);
                    
                    let resized = Calc::plus(origin_size_map, RhsValue::Object(vector_size_map));
                    let new_width = resized.get("width").copied().unwrap_or(MIN_WIDTH);
                    
                    let final_width = new_width.max(MIN_WIDTH);
                    props.set_width.call(Some(format!("{}px", final_width)));
                }
            }
        }
    }
}