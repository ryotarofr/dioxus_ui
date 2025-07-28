// export type RequiredFields<T, K extends keyof T> = T & Required<Pick<T, K>>;

pub trait RequiredFields<T> {
    type Output;
    fn require_fields(self) -> Self::Output;
}

pub trait WithRequiredFields<T, K> {
    type Output;
    fn with_required_fields(self) -> Self::Output;
}
