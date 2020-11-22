pub fn unordered_list<'a, I, T, Message: 'a>(iter: I) -> iced::Column<'a, Message>
where
    I: std::iter::IntoIterator<Item = T>,

    T: AsRef<str>,
{
    let mut col = iced::Column::new();
    for s in iter.into_iter() {
        let s = s.as_ref();
        col = col.push(crate::widget::text(s));
        col = col.push(iced::Space::with_height(iced::Length::Units(25)));
    }

    col
}
