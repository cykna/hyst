use taffy::TaffyError;

#[derive(Debug)]
pub enum LayoutError {
    InvalidStyleName(String),
    Taffy(TaffyError),
}
