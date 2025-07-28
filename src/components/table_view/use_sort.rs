use dioxus::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::cmp::Ordering;
use std::cell::RefCell;

use crate::function::objects::Objects;
use crate::types::setter::{SetStateAction, SetterUtils};

const ORDERS: [&str; 3] = ["none", "asc", "desc"];

#[derive(Debug, Clone, PartialEq)]
pub enum Order {
    None,
    Asc,
    Desc,
}

impl std::str::FromStr for Order {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "asc" => Ok(Order::Asc),
            "desc" => Ok(Order::Desc),
            "none" => Ok(Order::None),
            _ => Err(()),
        }
    }
}

impl Order {
    pub fn to_str(&self) -> &'static str {
        match self {
            Order::None => "none",
            Order::Asc => "asc",
            Order::Desc => "desc",
        }
    }
}

pub type SortOrder = Order;

#[derive(Clone, PartialEq)]
pub struct SortedWithIndex<T> {
    pub data: T,
    pub index: usize,
}

// Type aliases for complex function types
type InitFn = Rc<dyn FnMut()>;
type SetOrderMapFn<K> = Rc<dyn FnMut(SetStateAction<HashMap<K, Order>>)>;
type SetOrderFn<K> = Rc<dyn Fn(K) -> Rc<RefCell<dyn FnMut(SetStateAction<Order>)>>>;
type ShiftOrderFn<K> = Rc<dyn Fn(K) -> Rc<dyn Fn(SetStateAction<usize>)>>;
type GetSortedByIndicesFn<D> = Rc<dyn Fn(Vec<D>) -> Vec<D>>;
type AscSorterMap<K, T> = HashMap<K, Rc<dyn Fn(&T, &T) -> Ordering>>;

#[derive(Clone)]
pub struct UseSortResult<K, D>
where
    K: Clone + Hash + Eq + 'static,
    D: Clone + 'static,
{
    pub init: InitFn,
    pub order_entries: Vec<(K, Order)>,
    pub order_map: HashMap<K, Order>,
    pub set_order_map: SetOrderMapFn<K>,
    pub set_order: SetOrderFn<K>,
    pub set_order_once: SetOrderFn<K>,
    pub shift_order: ShiftOrderFn<K>,
    pub get_sorted_by_indices: GetSortedByIndicesFn<D>,
}

pub fn use_sort<T, K>(
    data: Vec<T>,
    init: impl Fn() -> Vec<(K, Order)> + Clone + 'static,
    asc_sorter_map: AscSorterMap<K, T>,
) -> UseSortResult<K, T>
where
    T: Clone + PartialEq + 'static,
    K: Clone + Hash + Eq + 'static,
{
    let order_entries = use_signal(&init);
    
    let get_order_map_from_orders = |orders: &Vec<(K, Order)>| -> HashMap<K, Order> {
        Objects::from_entries(orders.clone())
    };
    
    let order_map = get_order_map_from_orders(&order_entries.read());
    
    let set_order_map = {
        let mut order_entries = order_entries;
        Rc::new(move |set_state_action: SetStateAction<HashMap<K, Order>>| {
            order_entries.with_mut(|prev_orders| {
                let prev = get_order_map_from_orders(prev_orders);
                let next = SetterUtils::to_value(set_state_action, prev);
                let next_orders = Objects::entries(&next)
                    .into_iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                *prev_orders = next_orders;
            });
        }) as SetOrderMapFn<K>
    };
    
    let get_sorted_with_index = use_memo(move || {
        get_sorted_with_index_fn(order_entries.read().clone(), asc_sorter_map.clone(), data.clone())
    });
    
    let sorted_with_index = get_sorted_with_index();
    let sorted_indices: Vec<usize> = sorted_with_index.iter().map(|it| it.index).collect();
    
    let set_order = {
        Rc::new(move |key: K| -> Rc<RefCell<dyn FnMut(SetStateAction<Order>)>> {
            let mut order_entries = order_entries;
            let key = key.clone();
            let closure = move |set_state_action: SetStateAction<Order>| {
                order_entries.with_mut(|prev_orders| {
                    let prev_order = prev_orders
                        .iter()
                        .find(|(it_key, _)| it_key == &key)
                        .map(|(_, order)| order.clone())
                        .unwrap_or(Order::None);
                    let next_order = SetterUtils::to_value(set_state_action, prev_order);
                    
                    let mut result = vec![(key.clone(), next_order)];
                    result.extend(
                        prev_orders
                            .iter()
                            .filter(|(it_key, _)| it_key != &key)
                            .cloned()
                    );
                    *prev_orders = result;
                });
            };
            Rc::new(RefCell::new(closure))
        })
    };
    
    let set_order_once = {
        Rc::new(move |key: K| -> Rc<RefCell<dyn FnMut(SetStateAction<Order>)>> {
            let mut order_entries = order_entries;
            let key = key.clone();
            let closure = move |set_state_action: SetStateAction<Order>| {
                order_entries.with_mut(|prev_orders| {
                    let prev_order = prev_orders
                        .iter()
                        .find(|(it_key, _)| it_key == &key)
                        .map(|(_, order)| order.clone())
                        .unwrap_or(Order::None);
                    let next_order = SetterUtils::to_value(set_state_action, prev_order);
                    
                    let mut result = vec![(key.clone(), next_order)];
                    result.extend(
                        prev_orders
                            .iter()
                            .filter(|(it_key, _)| it_key != &key)
                            .map(|(it_key, _)| (it_key.clone(), Order::None))
                    );
                    *prev_orders = result;
                });
            };
            Rc::new(RefCell::new(closure))
        })
    };
    
    let shift_order = {
        let set_order = set_order.clone();
        Rc::new(move |key: K| -> Rc<dyn Fn(SetStateAction<usize>)> {
            let set_order = set_order.clone();
            let key = key.clone();
            Rc::new(move |set_state_action: SetStateAction<usize>| {
                let set_order_fn = set_order(key.clone());
                set_order_fn.borrow_mut()(SetStateAction::Function(Rc::new(move |prev_order| {
                    let prev_index = match prev_order {
                        Order::None => 0,
                        Order::Asc => 1,
                        Order::Desc => 2,
                    };
                    let next_index_raw = SetterUtils::to_value(set_state_action.clone(), prev_index);
                    let next_index = next_index_raw % ORDERS.len();
                    ORDERS[next_index].parse().unwrap_or(Order::None)
                })));
            })
        })
    };
    
    let init_fn = {
        let mut order_entries = order_entries;
        let init = init.clone();
        Rc::new(move || {
            order_entries.set(init());
        }) as InitFn
    };
    let get_sorted_by_indices = {
        let sorted_indices = sorted_indices.clone();
        Rc::new(move |data: Vec<T>| {
            sorted_indices
                .iter()
                .filter_map(|&index| data.get(index).cloned())
                .collect()
        })
    };
    
    let current_order_entries = order_entries.read().clone();
    
    UseSortResult {
        init: init_fn,
        order_entries: current_order_entries,
        order_map,
        set_order_map,
        set_order,
        set_order_once,
        shift_order,
        get_sorted_by_indices,
    }
}

fn get_sorted_with_index_fn<T, K>(
    sort_order_entries: Vec<(K, Order)>,
    asc_sorter_map: AscSorterMap<K, T>,
    data: Vec<T>,
) -> Vec<SortedWithIndex<T>>
where
    T: Clone,
    K: Clone + Hash + Eq,
{
    let mut with_index: Vec<SortedWithIndex<T>> = data
        .into_iter()
        .enumerate()
        .map(|(index, data)| SortedWithIndex { data, index })
        .collect();
    
    for (key, order) in sort_order_entries {
        if order == Order::None {
            continue;
        }
        
        if let Some(asc_sorter) = asc_sorter_map.get(&key) {
            with_index.sort_by(|prev, next| {
                let asc_sort_result = asc_sorter(&prev.data, &next.data);
                match order {
                    Order::Asc => asc_sort_result,
                    Order::Desc => asc_sort_result.reverse(),
                    Order::None => Ordering::Equal,
                }
            });
        }
    }
    
    with_index
}
