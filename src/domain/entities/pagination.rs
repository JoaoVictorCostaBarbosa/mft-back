pub struct Paginated<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    pub items_per_page: u32,
    pub current_page: u32,
}

impl<T> Paginated<T> {
    pub fn new(items: Vec<T>, total_items: i64, items_per_page: u32, current_page: u32) -> Self {
        Self {
            items,
            total_items,
            items_per_page,
            current_page,
        }
    }

    pub fn total_pages(&self) -> u32 {
        if self.total_items <= 0 {
            return 0;
        }

        ((self.total_items + self.items_per_page as i64 - 1) / self.items_per_page as i64) as u32
    }

    pub fn has_next(&self) -> bool {
        self.current_page < self.total_pages()
    }

    pub fn has_previous(&self) -> bool {
        self.current_page > 1 && self.total_pages() > 0
    }
}
