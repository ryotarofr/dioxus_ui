use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DataCellProps {
    #[props(default = false)]
    pub hidden: bool,
    pub align: String,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn DataCell(props: DataCellProps) -> Element {
    let mut class_list = vec!["data-cell".to_string()];
    
    if props.hidden {
        class_list.push("hidden".to_string());
    }
    
    if let Some(additional_class) = &props.class {
        class_list.push(additional_class.clone());
    }
    
    let base_style = "padding: 0.5em; padding-right: 15px; border: var(--border-faint); border-width: 0 1px 0 0; overflow: hidden; white-space: nowrap;";
    let mut style_str = format!("{} text-align: {};", base_style, props.align);
    
    if props.hidden {
        style_str.push_str(" display: none;");
    }
    
    if let Some(additional_style) = &props.style {
        style_str.push_str(&format!(" {}", additional_style));
    }

    rsx! {
        div {
            class: class_list.join(" "),
            style: style_str,
            {props.children}
        }
    }
}
