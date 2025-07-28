use dioxus::prelude::*;
use web_sys;
use std::rc::Rc;

use ui::Hero;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

/// 以下コードは期待通り動作しない
/// dioxusでfocusの動作をどうやってやるかを調べる

// create_forwarded_ref関数を自分で実装
pub fn create_forwarded_ref<T: 'static>() -> (Signal<Option<T>>, Signal<Option<T>>) {
    let signal = use_signal(|| None);
    (signal, signal)
}

pub fn forward_ref_component<T: 'static, P: 'static>(
    render_fn: impl Fn(P, Option<Signal<Option<T>>>) -> dioxus::prelude::Element + 'static,
) -> impl Fn(P, Option<Signal<Option<T>>>) -> dioxus::prelude::Element {
    move |props, forwarded_ref| render_fn(props, forwarded_ref)
}

// MyButtonコンポーネントを追加
#[derive(Props, PartialEq, Clone)]
struct MyButtonProps<T: PartialEq + Clone + 'static> {
    text: String,
    onclick: EventHandler<MouseEvent>,
    #[props(optional)]
    forwarded_ref: Option<Signal<Option<web_sys::HtmlElement>>>,
    #[props(optional)]
    _phantom: std::marker::PhantomData<T>,
}

#[component]
pub fn MyButton<T: PartialEq + Clone + 'static>(props: MyButtonProps<T>) -> dioxus::prelude::Element {
    rsx! {
        button {
            onclick: move |evt| props.onclick.call(evt),
            onmounted: move |evt| {
                if let Some(mut ref_signal) = props.forwarded_ref {
                    if let Some(element) = evt.data.downcast::<web_sys::HtmlElement>() {
                        ref_signal.set(Some(element.clone()));
                    }
                }
            },
            {props.text}
        }
    }
}

// 使用例
#[derive(Props, PartialEq, Clone)]
struct InputProps {
    placeholder: String,
    value: String,
    onchange: EventHandler<FormEvent>,
    #[props(optional)]
    forwarded_ref: Option<Signal<Option<web_sys::HtmlInputElement>>>,
}

#[component]
pub fn ForwardedInput(props: InputProps) -> dioxus::prelude::Element {
    println!("ForwardedInput rendered with placeholder: {}", props.value);
    rsx! {
        input {
            placeholder: {props.placeholder},
            value: {props.value},
            oninput: move |evt| props.onchange.call(evt),
            onmounted: move |evt| {
                if let Some(mut ref_signal) = props.forwarded_ref {
                    if let Some(element) = evt.data.downcast::<web_sys::HtmlInputElement>() {
                        ref_signal.set(Some(element.clone()));
                    }
                }
            }
        }
    }
}

// 使用方法の例
#[component]
pub fn App() -> dioxus::prelude::Element {
    let (button_ref, button_ref_forward) = create_forwarded_ref::<web_sys::HtmlElement>();
    let (input_ref, input_ref_forward) = create_forwarded_ref::<web_sys::HtmlInputElement>();

    rsx! {
        div {
            MyButton::<String> {
                text: "Click me".to_string(),
                onclick: move |_| {
                    if let Some(button_element) = button_ref.read().as_ref() {
                        let _ = button_element.focus();
                    }
                },
                forwarded_ref: Some(button_ref_forward),
                _phantom: std::marker::PhantomData
            }
            
            ForwardedInput {
                placeholder: "Enter text".to_string(),
                value: "".to_string(),
                onchange: move |_| {},
                forwarded_ref: Some(input_ref_forward)
            }
            
            button {
                onclick: move |_| {
                    if let Some(input_element) = input_ref.read().as_ref() {
                        let _ = input_element.focus();
                    }
                },
                "Focus Input"
            }
        }
    }
}