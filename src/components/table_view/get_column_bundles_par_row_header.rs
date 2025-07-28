#[derive(Debug, Clone, PartialEq)]
pub struct ColumnBundle<T> {
    pub headers: Vec<T>,
    pub contents: Vec<T>,
    pub index_until: usize,
}

impl<T> ColumnBundle<T> {
    pub fn init() -> Self {
        Self {
            headers: Vec::new(),
            contents: Vec::new(),
            index_until: 0,
        }
    }

    pub fn get_index_until(src: &[ColumnBundle<T>]) -> usize {
        src.iter()
            .map(|bundle| bundle.headers.len() + bundle.contents.len())
            .sum()
    }
}

pub trait HasRowHeader {
    fn is_row_header(&self) -> bool;
}

/// 連続する isRowHeader 毎に区切って配列化
pub fn get_column_bundles_par_row_header<T: HasRowHeader + Clone>(
    options: Vec<T>
) -> Vec<ColumnBundle<T>> {
    let mut result = vec![ColumnBundle::init()];
    let mut before: Option<bool> = None;

    for option in options {
        let current = option.is_row_header();
        let is_header = option.is_row_header();
        
        if is_header {
            if current == before.unwrap_or(true) || before.is_none() {
                if let Some(tail) = result.last_mut() {
                    tail.headers.push(option);
                }
            } else {
                let index_until = ColumnBundle::get_index_until(&result);
                result.push(ColumnBundle {
                    headers: vec![option],
                    contents: Vec::new(),
                    index_until,
                });
            }
        } else if let Some(tail) = result.last_mut() {
            tail.contents.push(option);
        }
        
        before = Some(current);
    }

    result
}
