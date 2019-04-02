/// Implement the `From` trait for any type.
/// Example usage:
/// ```
/// enum MyError<'a> {
///     Bar(BarError<'a>)
///     Foo(FooError<'a>)
/// }
///
/// impl_from!(BarError<'a>, Error::Bar);
/// impl_from!(BarError<'a>, Error::Bar);
///
/// ```
macro_rules! impl_from {
    // From a type with a lifetime to a type which also has a lifetime
    ($a:ident<$c:lifetime>, $b:ident::$enum_type:ident) => {
        impl<$c> From<$a<$c>> for $b<$c> {
            fn from(e: $a<$c>) -> Self {
                $b::$enum_type(e)
            }
        }
    };

    // From a type without a lifetime to a type which also does not have a lifetime
    ($a:ident, $b:ident::$enum_type:ident) => {
        impl From<$a> for $b {
            fn from(e: $a) -> Self {
                $b::$enum_type(e)
            }
        }
    };
}

/// Implement `Display` for an enum.
///
/// Example usage:
/// ```
/// enum Foo<'a> {
///     Bar(&'a str)
///     Baz(i32)
/// }
///
/// impl_display!{ Foo<'a>, {
///     Bar(s) => s,
///     Baz(i) => format!("{}", i)
/// }}
/// ```
macro_rules! impl_display {
    // For a type with a lifetime
    ($enum:ident<$lt:lifetime>, {$($variant:pat => $fmt_string:expr),+$(,)* }) => {

        impl<$lt> ::std::fmt::Display for $enum<$lt> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use self::$enum::*;
                match &self {
                    $(
                        $variant => write!(f, "{}", $fmt_string),
                    )+
                }
            }
        }

    };

    // For a type without a lifetime
    ($enum:ident, {$($variant:pat => $fmt_string:expr),+$(,)* }) => {

        impl ::std::fmt::Display for $enum {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use self::$enum::*;
                match &self {
                    $(
                        $variant => write!(f, "{}", $fmt_string),
                    )+
                }
            }
        }

    };
}

/// Implements `Display, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Hash`
/// for a Callback with a `.0` field:
///
/// ```
/// struct MyCallback<T>(fn (&T));
///
/// // impl <T> Display, Debug, etc. for MyCallback<T>
/// impl_callback!(MyCallback<T>);
/// ```
///
/// This is necessary to work around for https://github.com/rust-lang/rust/issues/54508
macro_rules! impl_callback {($callback_value:ident<$t:ident>) => (

    impl<$t> ::std::fmt::Display for $callback_value<$t> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl<$t> ::std::fmt::Debug for $callback_value<$t> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let callback = stringify!($callback_value);
            write!(f, "{} @ 0x{:x}", callback, self.0 as usize)
        }
    }

    impl<$t> Clone for $callback_value<$t> {
        fn clone(&self) -> Self {
            $callback_value(self.0.clone())
        }
    }

    impl<$t> ::std::hash::Hash for $callback_value<$t> {
        fn hash<H>(&self, state: &mut H) where H: Hasher {
            state.write_usize(self.0 as usize);
        }
    }

    impl<$t> PartialEq for $callback_value<$t> {
        fn eq(&self, rhs: &Self) -> bool {
            self.0 as usize == rhs.0 as usize
        }
    }

    impl<$t> PartialOrd for $callback_value<$t> {
        fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
            Some((self.0 as usize).cmp(&(other.0 as usize)))
        }
    }

    impl<$t> Ord for $callback_value<$t> {
        fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
            (self.0 as usize).cmp(&(other.0 as usize))
        }
    }

    impl<$t> Eq for $callback_value<$t> { }

    impl<$t> Copy for $callback_value<$t> { }
)}

macro_rules! impl_callback_bounded {($callback_value:ident<$t:ident: $trait_bound:ident>) => (
    impl<$t: $trait_bound> ::std::fmt::Display for $callback_value<$t> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl<$t: $trait_bound> ::std::fmt::Debug for $callback_value<$t> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let callback = stringify!($callback_value);
            write!(f, "{} @ 0x{:x}", callback, self.0 as usize)
        }
    }

    impl<$t: $trait_bound> Clone for $callback_value<$t> {
        fn clone(&self) -> Self {
            $callback_value(self.0.clone())
        }
    }

    impl<$t: $trait_bound> ::std::hash::Hash for $callback_value<$t> {
        fn hash<H>(&self, state: &mut H) where H: Hasher {
            state.write_usize(self.0 as usize);
        }
    }

    impl<$t: $trait_bound> PartialEq for $callback_value<$t> {
        fn eq(&self, rhs: &Self) -> bool {
            self.0 as usize == rhs.0 as usize
        }
    }

    impl<$t: $trait_bound> PartialOrd for $callback_value<$t> {
        fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
            Some((self.0 as usize).cmp(&(other.0 as usize)))
        }
    }

    impl<$t: $trait_bound> Ord for $callback_value<$t> {
        fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
            (self.0 as usize).cmp(&(other.0 as usize))
        }
    }

    impl<$t: $trait_bound> Eq for $callback_value<$t> { }

    impl<$t: $trait_bound> Copy for $callback_value<$t> { }
)}

macro_rules! image_api {($struct_name:ident::$struct_field:ident) => (

impl<T: Layout> $struct_name<T> {

    /// See [`AppResources::get_loaded_font_ids`]
    ///
    /// [`AppResources::get_loaded_font_ids`]: ../app_resources/struct.AppResources.html#method.get_loaded_font_ids
    pub fn get_loaded_font_ids(&self) -> Vec<FontId> {
        self.$struct_field.get_loaded_font_ids()
    }

    /// See [`AppResources::get_loaded_image_ids`]
    ///
    /// [`AppResources::get_loaded_image_ids`]: ../app_resources/struct.AppResources.html#method.get_loaded_image_ids
    pub fn get_loaded_image_ids(&self) -> Vec<ImageId> {
        self.$struct_field.get_loaded_image_ids()
    }

    /// See [`AppResources::get_loaded_css_image_ids`]
    ///
    /// [`AppResources::get_loaded_css_image_ids`]: ../app_resources/struct.AppResources.html#method.get_loaded_css_image_ids
    pub fn get_loaded_css_image_ids(&self) -> Vec<CssImageId> {
        self.$struct_field.get_loaded_css_image_ids()
    }

    /// See [`AppResources::get_loaded_css_font_ids`]
    ///
    /// [`AppResources::get_loaded_css_font_ids`]: ../app_resources/struct.AppResources.html#method.get_loaded_css_font_ids
    pub fn get_loaded_css_font_ids(&self) -> Vec<CssImageId> {
        self.$struct_field.get_loaded_css_font_ids()
    }

    /// See [`AppResources::get_loaded_text_ids`]
    ///
    /// [`AppResources::get_loaded_text_ids`]: ../app_resources/struct.AppResources.html#method.get_loaded_text_ids
    pub fn get_loaded_text_ids(&self) -> Vec<TextId> {
        self.$struct_field.get_loaded_text_ids()
    }

    // -- ImageId cache

    /// See [`AppResources::add_image`]
    ///
    /// [`AppResources::add_image`]: ../app_resources/struct.AppResources.html#method.add_image
    #[cfg(feature = "image_loading")]
    pub fn add_image(&mut self, image_id: ImageId, image_source: ImageSource) {
        self.$struct_field.add_image(image_id, image_source)
    }

    /// See [`AppResources::add_image_raw`]
    ///
    /// [`AppResources::add_image_raw`]: ../app_resources/struct.AppResources.html#method.add_image_raw
    pub fn add_image_raw(&mut self, image_id: ImageId, image: RawImage) {
        self.$struct_field.add_image_raw(image_id, image)
    }

    /// See [`AppResources::has_image`]
    ///
    /// [`AppResources::has_image`]: ../app_resources/struct.AppResources.html#method.has_image
    pub fn has_image(&self, image_id: &ImageId) -> bool {
        self.$struct_field.has_image(image_id)
    }

    /// Given an `ImageId`, returns the bytes for that image or `None`, if the `ImageId` is invalid.
    ///
    /// See [`AppResources::get_image_bytes`]
    ///
    /// [`AppResources::get_image_bytes`]: ../app_resources/struct.AppResources.html#method.get_image_bytes
    pub fn get_image_bytes(&self, image_id: &ImageId) -> Option<Result<Vec<u8>, ImageReloadError>> {
        self.$struct_field.get_image_bytes(image_id)
    }

    /// See [`AppResources::delete_image`]
    ///
    /// [`AppResources::delete_image`]: ../app_resources/struct.AppResources.html#method.delete_image
    pub fn delete_image(&mut self, image_id: &ImageId) {
        self.$struct_field.delete_image(image_id)
    }

    /// See [`AppResources::add_css_image_id`]
    ///
    /// [`AppResources::add_css_image_id`]: ../app_resources/struct.AppResources.html#method.add_css_image_id
    pub fn add_css_image_id<S: Into<String>>(&mut self, css_id: S) -> ImageId {
        self.$struct_field.add_css_image_id(css_id)
    }

    /// See [`AppResources::has_css_image_id`]
    ///
    /// [`AppResources::has_css_image_id`]: ../app_resources/struct.AppResources.html#method.has_css_image_id
    pub fn has_css_image_id(&self, css_id: &str) -> bool {
        self.$struct_field.has_css_image_id(css_id)
    }

    /// See [`AppResources::get_css_image_id`]
    ///
    /// [`AppResources::get_css_image_id`]: ../app_resources/struct.AppResources.html#method.get_css_image_id
    pub fn get_css_image_id(&self, css_id: &str) -> Option<&ImageId> {
        self.$struct_field.get_css_image_id(css_id)
    }

    /// See [`AppResources::delete_css_image_id`]
    ///
    /// [`AppResources::delete_css_image_id`]: ../app_resources/struct.AppResources.html#method.delete_css_image_id
    pub fn delete_css_image_id(&mut self, css_id: &str) -> Option<ImageId> {
        self.$struct_field.delete_css_image_id(css_id)
    }

    /// See [`AppResources::add_css_font_id`]
    ///
    /// [`AppResources::add_css_font_id`]: ../app_resources/struct.AppResources.html#method.add_css_font_id
    pub fn add_css_font_id<S: Into<String>>(&mut self, css_id: S) -> FontId {
        self.$struct_field.add_css_font_id(css_id)
    }

    /// See [`AppResources::has_css_font_id`]
    ///
    /// [`AppResources::has_css_font_id`]: ../app_resources/struct.AppResources.html#method.has_css_font_id
    pub fn has_css_font_id(&self, css_id: &str) -> bool {
        self.$struct_field.has_css_font_id(css_id)
    }

    /// See [`AppResources::get_css_font_id`]
    ///
    /// [`AppResources::get_css_font_id`]: ../app_resources/struct.AppResources.html#method.get_css_font_id
    pub fn get_css_font_id(&self, css_id: &str) -> Option<&FontId> {
        self.$struct_field.get_css_font_id(css_id)
    }

    /// See [`AppResources::delete_css_font_id`]
    ///
    /// [`AppResources::delete_css_font_id`]: ../app_resources/struct.AppResources.html#method.delete_css_font_id
    pub fn delete_css_font_id(&mut self, css_id: &str) -> Option<FontId> {
        self.$struct_field.delete_css_font_id(css_id)
    }
}

)}

macro_rules! font_api {($struct_name:ident::$struct_field:ident) => (

impl<T: Layout> $struct_name<T> {

    /// Given a `FontId`, returns the bytes for that font or `None`, if the `FontId` is invalid.
    /// See [`AppResources::get_font_bytes`]
    ///
    /// [`AppResources::get_font_bytes`]: ../app_resources/struct.AppResources.html#method.get_font_bytes
    pub fn get_font_bytes(&self, font_id: &FontId) -> Option<Result<(Vec<u8>, i32), FontReloadError>> {
        self.$struct_field.get_font_bytes(font_id)
    }

    /// See [`AppResources::add_font`]
    ///
    /// [`AppResources::add_font`]: ../app_resources/struct.AppResources.html#method.add_font
    pub fn add_font(&mut self, font_id: FontId, font_source: FontSource) {
        self.$struct_field.add_font(font_id, font_source)
    }

    /// See [`AppResources::has_font`]
    ///
    /// [`AppResources::has_font`]: ../app_resources/struct.AppResources.html#method.has_font
    pub fn has_font(&self, font_id: &FontId) -> bool {
        self.$struct_field.has_font(font_id)
    }

    /// See [`AppResources::delete_font`]
    ///
    /// [`AppResources::delete_font`]: ../app_resources/struct.AppResources.html#method.delete_font
    pub fn delete_font(&mut self, font_id: &FontId) {
        self.$struct_field.delete_font(font_id)
    }
}

)}

macro_rules! text_api {($struct_name:ident::$struct_field:ident) => (

impl<T: Layout> $struct_name<T> {

    /// Adds a string to the internal text cache, but only store it as a string,
    /// without caching the layout of the string.
    ///
    /// See [`AppResources::add_text`].
    ///
    /// [`AppResources::add_text`]: ../app_resources/struct.AppResources.html#method.add_text
    pub fn add_text(&mut self, text: &str) -> TextId {
        self.$struct_field.add_text(text)
    }

    /// Removes a string from both the string cache and the layouted text cache
    ///
    /// See [`AppResources::delete_text`].
    ///
    /// [`AppResources::delete_text`]: ../app_resources/struct.AppResources.html#method.delete_text
    pub fn delete_text(&mut self, id: TextId) {
        self.$struct_field.delete_text(id)
    }

    /// Empties the entire internal text cache, invalidating all `TextId`s.
    /// If the given TextId is used after this call, the text will not render in the UI.
    /// Use with care.
    ///
    /// See [`AppResources::clear_all_texts`].
    ///
    /// [`AppResources::clear_all_texts`]: ../app_resources/struct.AppResources.html#method.clear_all_texts
    pub fn clear_all_texts(&mut self) {
        self.$struct_field.clear_all_texts()
    }
}

)}

macro_rules! clipboard_api {($struct_name:ident::$struct_field:ident) => (

impl<T: Layout> $struct_name<T> {

    /// See [`AppResources::get_clipboard_string`]
    ///
    /// [`AppResources::get_clipboard_string`]: ../app_resources/struct.AppResources.html#method.get_clipboard_string
    pub fn get_clipboard_string(&mut self) -> Result<String, ClipboardError> {
        self.$struct_field.get_clipboard_string()
    }

    /// See [`AppResources::set_clipboard_string`]
    ///
    /// [`AppResources::set_clipboard_string`]: ../app_resources/struct.AppResources.html#method.set_clipboard_string
    pub fn set_clipboard_string<I: Into<String>>(&mut self, contents: I) -> Result<(), ClipboardError> {
        self.$struct_field.set_clipboard_string(contents)
    }
}

)}

macro_rules! timer_api {($struct_name:ident::$struct_field:ident) => (

impl<T: Layout> $struct_name<T> {

    /// See [`AppState::add_timer`]
    ///
    /// [`AppState::add_timer`]: ../app_state/struct.AppState.html#method.add_timer
    pub fn add_timer(&mut self, timer_id: TimerId, timer: Timer<T>) {
        self.$struct_field.add_timer(timer_id, timer)
    }

    /// See [`AppState::has_timer`]
    ///
    /// [`AppState::has_timer`]: ../app_state/struct.AppState.html#method.has_timer
    pub fn has_timer(&self, timer_id: &TimerId) -> bool {
        self.$struct_field.has_timer(timer_id)
    }

    /// See [`AppState::get_timer`]
    ///
    /// [`AppState::get_timer`]: ../app_state/struct.AppState.html#method.get_timer
    pub fn get_timer(&self, timer_id: &TimerId) -> Option<Timer<T>> {
        self.$struct_field.get_timer(timer_id)
    }

    /// See [`AppState::delete_timer`]
    ///
    /// [`AppState::delete_timer`]: ../app_state/struct.AppState.html#method.delete_timer
    pub fn delete_timer(&mut self, timer_id: &TimerId) -> Option<Timer<T>> {
        self.$struct_field.delete_timer(timer_id)
    }
}

)}

macro_rules! impl_deamon_api {() => (

    /// Insert a timer into the list of active timers.
    /// Replaces the existing timer if called with the same TimerId.
    pub fn add_timer(&mut self, id: TimerId, timer: Timer<T>) {
        self.timers.insert(id, timer);
    }

    pub fn has_timer(&self, timer_id: &TimerId) -> bool {
        self.get_timer(timer_id).is_some()
    }

    pub fn get_timer(&self, timer_id: &TimerId) -> Option<Timer<T>> {
        self.timers.get(&timer_id).cloned()
    }

    pub fn delete_timer(&mut self, timer_id: &TimerId) -> Option<Timer<T>> {
        self.timers.remove(timer_id)
    }

    /// Custom tasks can be used when the `AppState` isn't `Send`. For example
    /// `SvgCache` isn't thread-safe, since it has to interact with OpenGL, so
    /// it can't be sent to other threads safely.
    ///
    /// What you can do instead, is take a part of your application data, wrap
    /// that in an `Arc<Mutex<>>` and push a task that takes it onto the queue.
    /// This way you can modify a part of the application state on a different
    /// thread, while not requiring that everything is thread-safe.
    ///
    /// While you can't modify the `SvgCache` from a different thread, you can
    /// modify other things in the `AppState` and leave the SVG cache alone.
    pub fn add_task(&mut self, task: Task<T>) {
        self.tasks.push(task);
    }
)}