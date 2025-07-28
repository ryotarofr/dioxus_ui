use dioxus::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

/// ArrowLeft に反応して何かをしていいか判定
///
/// isComposingでない AND (targetが入力欄でない OR カーソルが入力欄の先頭にある) ならtrue
pub fn is_arrow_left_actionable(event: &Event<KeyboardData>) -> bool {
    // Simple implementation without target element access for now
    // In a real implementation, you would need to access the DOM target
    !is_composing(event)
}

/// ArrowRight に反応して何かをしていいか判定
///
/// isComposingでない AND (targetが入力欄でない OR カーソルが入力欄の末尾にある) ならtrue
pub fn is_arrow_right_actionable(event: &Event<KeyboardData>) -> bool {
    // Simple implementation without target element access for now
    // In a real implementation, you would need to access the DOM target
    !is_composing(event)
}

/// 文字入力欄かどうか
pub fn is_str_input_element(element: &web_sys::Element) -> bool {
    if let Some(input_element) = element.dyn_ref::<HtmlInputElement>() {
        !NON_STR_INPUT_TYPES.contains(&input_element.type_().as_str())
    } else {
        false
    }
}

/// カーソルが先頭にある文字入力欄かどうか (今発生したeventのtargetが渡されている前提)
pub fn is_input_with_cursor_on_start(element: &web_sys::Element) -> bool {
    // カーソルが取得できるinputでなければ判定不可能
    if !is_cursor_gettable_input_element(element) {
        return false;
    }
    
    // For now, return false as this requires complex DOM selection API access
    // In a real implementation, you would use the Selection API to check cursor position
    false
}

/// カーソルが末尾にある文字入力欄かどうか (今発生したeventのtargetが渡されている前提)
pub fn is_input_with_cursor_on_end(element: &web_sys::Element) -> bool {
    // カーソルが取得できるinputでなければ判定不可能
    if !is_cursor_gettable_input_element(element) {
        return false;
    }
    
    // For now, return false as this requires complex DOM selection API access
    // In a real implementation, you would use the Selection API to check cursor position
    false
}

fn is_composing(event: &Event<KeyboardData>) -> bool {
    event.data().is_composing()
}

/// カーソル位置を取得できるinputかどうか
fn is_cursor_gettable_input_element(element: &web_sys::Element) -> bool {
    if let Some(input_element) = element.dyn_ref::<HtmlInputElement>() {
        NORMAL_CURSOR_INPUT_TYPES.contains(&input_element.type_().as_str())
    } else {
        false
    }
}

/// 明らかに文字入力欄でなく、文字カーソルや左右移動が存在しなさそうなinput.type一覧 (仮)
const NON_STR_INPUT_TYPES: &[&str] = &[
    "checkbox", "file", "radio", "reset", "submit",
];

/// カーソル位置を取得できるinput.type一覧
const NORMAL_CURSOR_INPUT_TYPES: &[&str] = &[
    // https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input
    // 上記の、placeholderに対応しているtype
    "text", "search", "url", "tel", "password", "email", "number",

    // https://developer.mozilla.org/ja/docs/Web/API/HTMLInputElement/selectionStart
    // 上記の、selectionStartに対応しているtype
    // number非対応なのが痛い
    // "text", "search", "url", "tel", "password",
];

// Extended functions with proper DOM access (to be implemented when needed)

/// Advanced version with proper target element access
/// This would be used in an actual implementation with access to the DOM event target
pub fn is_arrow_left_actionable_with_element(
    event: &Event<KeyboardData>, 
    target_element: Option<&web_sys::Element>
) -> bool {
    if let Some(element) = target_element {
        !is_composing(event) && (!is_str_input_element(element) || is_input_with_cursor_on_start(element))
    } else {
        !is_composing(event)
    }
}

/// Advanced version with proper target element access
/// This would be used in an actual implementation with access to the DOM event target
pub fn is_arrow_right_actionable_with_element(
    event: &Event<KeyboardData>, 
    target_element: Option<&web_sys::Element>
) -> bool {
    if let Some(element) = target_element {
        !is_composing(event) && (!is_str_input_element(element) || is_input_with_cursor_on_end(element))
    } else {
        !is_composing(event)
    }
}