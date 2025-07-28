use dioxus::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

use crate::types::setter::{SetStateAction, SetterUtils};

#[derive(Clone)]
pub struct UseSelectResult {
    pub init: Rc<RefCell<dyn FnMut()>>,
    pub ids: Vec<String>,
    pub set_ids: Rc<RefCell<dyn FnMut(SetStateAction<Vec<String>>) -> SelectResult>>,
    pub unset_all: Rc<RefCell<dyn FnMut()>>,
    pub set_by_id: Rc<dyn Fn(String) -> Rc<RefCell<dyn FnMut(SetStateAction<bool>) -> SelectResult>>>,
    pub toggle_by_id: Rc<RefCell<dyn FnMut(String)>>,
}

pub struct SelectResult {
    pub default_prevented: bool,
}

pub struct SelectEvent {
    prevent_default: Rc<RefCell<bool>>,
    apply_callback: Rc<RefCell<dyn FnMut()>>,
}

impl SelectEvent {
    pub fn prevent_default(&self) {
        *self.prevent_default.borrow_mut() = true;
    }
    
    pub fn apply_callback(&self) {
        self.apply_callback.borrow_mut()();
    }
}

pub fn use_select(
    init: impl Fn() -> Vec<String> + Clone + 'static,
    select_many: bool,
    changeable: bool,
    cancelable: bool,
    on_change: Option<Rc<dyn Fn(&SelectEvent)>>,
) -> UseSelectResult {
    let selected_ids = use_signal(|| init());
    
    let set_selected_ids = {
        let mut selected_ids_for_closure = selected_ids;
        let _init = init.clone();
        let on_change = on_change.clone();
        
        Rc::new(RefCell::new(move |set_state_action: SetStateAction<Vec<String>>| -> SelectResult {
            let default_prevented = Rc::new(RefCell::new(false));
            let default_prevented_clone = default_prevented.clone();
            
            let mut should_return_early = false;
            let mut next_value: Vec<String> = Vec::new();
            
            // First, compute the next value
            let current = selected_ids_for_closure.read().clone();
            let next_raw = SetterUtils::to_value(set_state_action, current.clone());
            let next = if select_many {
                next_raw
            } else {
                next_raw.into_iter().take(1).collect()
            };
            
            if changeable && current != next {
                if let Some(ref on_change_fn) = on_change {
                    let next_for_callback = next.clone();
                    let apply_callback = {
                        Rc::new(RefCell::new(move || {
                            // This will be called later by the user
                            // We can't modify the signal here due to borrowing rules
                            // The actual update happens below
                        }))
                    };
                    
                    let event = SelectEvent {
                        prevent_default: default_prevented_clone.clone(),
                        apply_callback,
                    };
                    
                    on_change_fn(&event);
                    
                    if *default_prevented_clone.borrow() {
                        should_return_early = true;
                    }
                }
            }
            
            if !should_return_early {
                selected_ids_for_closure.set(next);
            }
            
            SelectResult {
                default_prevented: should_return_early,
            }
        }))
    };
    
    let set_by_id = {
        let set_selected_ids = set_selected_ids.clone();
        
        Rc::new(move |id: String| -> Rc<RefCell<dyn FnMut(SetStateAction<bool>) -> SelectResult>> {
            let set_selected_ids = set_selected_ids.clone();
            let id = id.clone();
            
            Rc::new(RefCell::new(move |set_state_action: SetStateAction<bool>| -> SelectResult {
                set_selected_ids.borrow_mut()(SetStateAction::Function(Rc::new({
                    let id = id.clone();
                    move |prevs: Vec<String>| {
                        let prev = prevs.contains(&id);
                        let next = SetterUtils::to_value(set_state_action.clone(), prev);
                        
                        if prev == next {
                            return prevs;
                        }
                        
                        if !cancelable && prev {
                            return prevs;
                        }
                        
                        if next {
                            let mut result = vec![id.clone()];
                            result.extend(prevs);
                            result
                        } else {
                            prevs.into_iter().filter(|it| it != &id).collect()
                        }
                    }
                })))
            }))
        })
    };
    
    let init_fn = {
        let mut selected_ids = selected_ids;
        let init = init.clone();
        
        Rc::new(RefCell::new(move || {
            selected_ids.set(init());
        }))
    };
    
    let unset_all = {
        let set_selected_ids = set_selected_ids.clone();
        
        Rc::new(RefCell::new(move || {
            set_selected_ids.borrow_mut()(SetStateAction::Value(Vec::new()));
        }))
    };
    
    let toggle_by_id = {
        let mut selected_ids = selected_ids;
        
        Rc::new(RefCell::new(move |id: String| {
            selected_ids.with_mut(|prev_ids| {
                let prev = prev_ids.contains(&id);
                
                if !cancelable && prev {
                    return;
                }
                
                if prev {
                    prev_ids.retain(|it| it != &id);
                } else {
                    prev_ids.insert(0, id);
                }
            });
        }))
    };
    
    let current_ids = selected_ids.read().clone();
    
    UseSelectResult {
        init: init_fn,
        ids: current_ids,
        set_ids: set_selected_ids,
        unset_all,
        set_by_id,
        toggle_by_id,
    }
}
