use gtk::gdk;

pub fn texture_for_icon(icon_name: &str, size: i32) -> gdk::Texture {
    let theme = gtk::IconTheme::for_display(&gdk::Display::default().unwrap());

    let icon = theme.lookup_icon(
        icon_name,
        &[],
        size,
        1,
        gtk::TextDirection::None,
        gtk::IconLookupFlags::empty(),
    );

    // gtk returns correct file even if icon is not exist
    gdk::Texture::from_file(&icon.file().unwrap()).unwrap()
}
