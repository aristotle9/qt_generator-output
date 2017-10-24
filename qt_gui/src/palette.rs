/// C++ type: <span style='color: green;'>```QPalette::ColorGroup```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ColorGroup {
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Active = 0```</span>
  /// - <span style='color: green;'>```Normal = 0```</span>
  ///
  Active = 0,
  /// C++ enum variant: <span style='color: green;'>```Disabled = 1```</span>
  Disabled = 1,
  /// C++ enum variant: <span style='color: green;'>```Inactive = 2```</span>
  Inactive = 2,
  /// C++ enum variant: <span style='color: green;'>```NColorGroups = 3```</span>
  NColorGroups = 3,
  /// C++ enum variant: <span style='color: green;'>```Current = 4```</span>
  Current = 4,
  /// C++ enum variant: <span style='color: green;'>```All = 5```</span>
  All = 5,
}

/// C++ type: <span style='color: green;'>```QPalette::ColorRole```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ColorRole {
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```WindowText = 0```</span>
  /// - <span style='color: green;'>```Foreground = 0```</span>
  ///
  WindowText = 0,
  /// C++ enum variant: <span style='color: green;'>```Button = 1```</span>
  Button = 1,
  /// C++ enum variant: <span style='color: green;'>```Light = 2```</span>
  Light = 2,
  /// C++ enum variant: <span style='color: green;'>```Midlight = 3```</span>
  Midlight = 3,
  /// C++ enum variant: <span style='color: green;'>```Dark = 4```</span>
  Dark = 4,
  /// C++ enum variant: <span style='color: green;'>```Mid = 5```</span>
  Mid = 5,
  /// C++ enum variant: <span style='color: green;'>```Text = 6```</span>
  Text = 6,
  /// C++ enum variant: <span style='color: green;'>```BrightText = 7```</span>
  BrightText = 7,
  /// C++ enum variant: <span style='color: green;'>```ButtonText = 8```</span>
  ButtonText = 8,
  /// C++ enum variant: <span style='color: green;'>```Base = 9```</span>
  Base = 9,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Window = 10```</span>
  /// - <span style='color: green;'>```Background = 10```</span>
  ///
  Window = 10,
  /// C++ enum variant: <span style='color: green;'>```Shadow = 11```</span>
  Shadow = 11,
  /// C++ enum variant: <span style='color: green;'>```Highlight = 12```</span>
  Highlight = 12,
  /// C++ enum variant: <span style='color: green;'>```HighlightedText = 13```</span>
  HighlightedText = 13,
  /// C++ enum variant: <span style='color: green;'>```Link = 14```</span>
  Link = 14,
  /// C++ enum variant: <span style='color: green;'>```LinkVisited = 15```</span>
  LinkVisited = 15,
  /// C++ enum variant: <span style='color: green;'>```AlternateBase = 16```</span>
  AlternateBase = 16,
  /// C++ enum variant: <span style='color: green;'>```NoRole = 17```</span>
  NoRole = 17,
  /// C++ enum variant: <span style='color: green;'>```ToolTipBase = 18```</span>
  ToolTipBase = 18,
  /// C++ enum variant: <span style='color: green;'>```ToolTipText = 19```</span>
  ToolTipText = 19,
  /// C++ enum variant: <span style='color: green;'>```NColorRoles = 20```</span>
  NColorRoles = 20,
}

/// C++ type: <span style='color: green;'>```QPalette```</span>
#[repr(C)]
pub struct Palette([u8; ::type_sizes::QT_GUI_PALETTE_PALETTE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Palette {
  unsafe fn new_uninitialized() -> Palette {
    Palette(::std::mem::uninitialized())
  }
}

impl Palette {
  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::alternateBase() const```</span>
  ///
  ///
  pub fn alternate_base<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_alternateBase(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVariant QPalette::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPalette_convert_to_QVariant_to_output(self as *const ::palette::Palette, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::background() const```</span>
  ///
  ///
  pub fn background<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_background(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::base() const```</span>
  ///
  ///
  pub fn base<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_base(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::brightText() const```</span>
  ///
  ///
  pub fn bright_text<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_brightText(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPalette::brush```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn brush(&self, (::palette::ColorGroup, ::palette::ColorRole)) -> &'l0 ::brush::Brush```<br>
  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::brush(QPalette::ColorGroup cg, QPalette::ColorRole cr) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn brush(&self, ::palette::ColorRole) -> &'l0 ::brush::Brush```<br>
  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::brush(QPalette::ColorRole cr) const```</span>
  ///
  ///
  pub fn brush<'largs, Args>(&'largs self, args: Args) -> &'largs ::brush::Brush
    where Args: overloading::PaletteBrushArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::button() const```</span>
  ///
  ///
  pub fn button<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_button(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::buttonText() const```</span>
  ///
  ///
  pub fn button_text<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_buttonText(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```qint64 QPalette::cacheKey() const```</span>
  ///
  ///
  pub fn cache_key(&self) -> i64 {
    unsafe { ::ffi::qt_gui_c_QPalette_cacheKey(self as *const ::palette::Palette) }
  }

  /// C++ method: <span style='color: green;'>```QPalette::color```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn color(&self, (::palette::ColorGroup, ::palette::ColorRole)) -> &'l0 ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```const QColor& QPalette::color(QPalette::ColorGroup cg, QPalette::ColorRole cr) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn color(&self, ::palette::ColorRole) -> &'l0 ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```const QColor& QPalette::color(QPalette::ColorRole cr) const```</span>
  ///
  ///
  pub fn color<'largs, Args>(&'largs self, args: Args) -> &'largs ::color::Color
    where Args: overloading::PaletteColorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPalette::ColorGroup QPalette::currentColorGroup() const```</span>
  ///
  ///
  pub fn current_color_group(&self) -> ::palette::ColorGroup {
    unsafe { ::ffi::qt_gui_c_QPalette_currentColorGroup(self as *const ::palette::Palette) }
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::dark() const```</span>
  ///
  ///
  pub fn dark<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_dark(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::foreground() const```</span>
  ///
  ///
  pub fn foreground<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_foreground(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::highlight() const```</span>
  ///
  ///
  pub fn highlight<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_highlight(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::highlightedText() const```</span>
  ///
  ///
  pub fn highlighted_text<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_highlightedText(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QPalette::isBrushSet(QPalette::ColorGroup cg, QPalette::ColorRole cr) const```</span>
  ///
  ///
  pub fn is_brush_set(&self, cg: ::palette::ColorGroup, cr: ::palette::ColorRole) -> bool {
    unsafe { ::ffi::qt_gui_c_QPalette_isBrushSet(self as *const ::palette::Palette, cg, cr) }
  }

  /// C++ method: <span style='color: green;'>```bool QPalette::isCopyOf(const QPalette& p) const```</span>
  ///
  ///
  pub fn is_copy_of(&self, p: &::palette::Palette) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPalette_isCopyOf(self as *const ::palette::Palette,
                                        p as *const ::palette::Palette)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPalette::isEqual(QPalette::ColorGroup cr1, QPalette::ColorGroup cr2) const```</span>
  ///
  ///
  pub fn is_equal(&self, cr1: ::palette::ColorGroup, cr2: ::palette::ColorGroup) -> bool {
    unsafe { ::ffi::qt_gui_c_QPalette_isEqual(self as *const ::palette::Palette, cr1, cr2) }
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::light() const```</span>
  ///
  ///
  pub fn light<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_light(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::link() const```</span>
  ///
  ///
  pub fn link<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_link(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::linkVisited() const```</span>
  ///
  ///
  pub fn link_visited<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_linkVisited(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::mid() const```</span>
  ///
  ///
  pub fn mid<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_mid(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::midlight() const```</span>
  ///
  ///
  pub fn midlight<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_midlight(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPalette::QPalette```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::palette::Palette```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPalette::QPalette()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::qt::GlobalColor) -> ::palette::Palette```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPalette::QPalette(Qt::GlobalColor button)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::brush::Brush, &::brush::Brush, &::brush::Brush, &::brush::Brush, &::brush::Brush, &::brush::Brush, &::brush::Brush, &::brush::Brush, &::brush::Brush)) -> ::palette::Palette```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPalette::QPalette(const QBrush& windowText, const QBrush& button, const QBrush& light, const QBrush& dark, const QBrush& mid, const QBrush& text, const QBrush& bright_text, const QBrush& base, const QBrush& window)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::color::Color) -> ::palette::Palette```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPalette::QPalette(const QColor& button)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((&::color::Color, &::color::Color)) -> ::palette::Palette```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPalette::QPalette(const QColor& button, const QColor& window)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new((&::color::Color, &::color::Color, &::color::Color, &::color::Color, &::color::Color, &::color::Color, &::color::Color)) -> ::palette::Palette```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPalette::QPalette(const QColor& windowText, const QColor& window, const QColor& light, const QColor& dark, const QColor& mid, const QColor& text, const QColor& base)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new(&::palette::Palette) -> ::palette::Palette```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPalette::QPalette(const QPalette& palette)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::palette::Palette
    where Args: overloading::PaletteNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPalette& QPalette::operator=(const QPalette& palette)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, palette: &'l1 ::palette::Palette) -> &'l0 mut ::palette::Palette {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QPalette_operator_assign(self as *mut ::palette::Palette,
                                               palette as *const ::palette::Palette)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QPalette::operator==(const QPalette& p) const```</span>
  ///
  ///
  pub fn op_eq(&self, p: &::palette::Palette) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPalette_operator_eq(self as *const ::palette::Palette,
                                           p as *const ::palette::Palette)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPalette::operator!=(const QPalette& p) const```</span>
  ///
  ///
  pub fn op_neq(&self, p: &::palette::Palette) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPalette_operator_neq(self as *const ::palette::Palette,
                                            p as *const ::palette::Palette)
    }
  }

  /// C++ method: <span style='color: green;'>```QPalette::resolve```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn resolve(&self, &::palette::Palette) -> ::palette::Palette```<br>
  /// C++ method: <span style='color: green;'>```QPalette QPalette::resolve(const QPalette& arg1) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn resolve(&self, ()) -> ::libc::c_uint```<br>
  /// C++ method: <span style='color: green;'>```unsigned int QPalette::resolve() const```</span>
  ///
  ///
  pub fn resolve<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::PaletteResolveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QPalette::resolve(unsigned int mask)```</span>
  ///
  ///
  pub fn resolve_mut(&mut self, mask: ::libc::c_uint) {
    unsafe { ::ffi::qt_gui_c_QPalette_resolve_mask(self as *mut ::palette::Palette, mask) }
  }

  /// C++ method: <span style='color: green;'>```QPalette::setBrush```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_brush(&mut self, (::palette::ColorGroup, ::palette::ColorRole, &::brush::Brush)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPalette::setBrush(QPalette::ColorGroup cg, QPalette::ColorRole cr, const QBrush& brush)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_brush(&mut self, (::palette::ColorRole, &::brush::Brush)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPalette::setBrush(QPalette::ColorRole cr, const QBrush& brush)```</span>
  ///
  ///
  pub fn set_brush<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PaletteSetBrushArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPalette::setColor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_color(&mut self, (::palette::ColorGroup, ::palette::ColorRole, &::color::Color)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPalette::setColor(QPalette::ColorGroup cg, QPalette::ColorRole cr, const QColor& color)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_color(&mut self, (::palette::ColorRole, &::color::Color)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPalette::setColor(QPalette::ColorRole cr, const QColor& color)```</span>
  ///
  ///
  pub fn set_color<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PaletteSetColorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QPalette::setColorGroup(QPalette::ColorGroup cr, const QBrush& windowText, const QBrush& button, const QBrush& light, const QBrush& dark, const QBrush& mid, const QBrush& text, const QBrush& bright_text, const QBrush& base, const QBrush& window)```</span>
  ///
  ///
  pub fn set_color_group(&mut self,
                         cr: ::palette::ColorGroup,
                         window_text: &::brush::Brush,
                         button: &::brush::Brush,
                         light: &::brush::Brush,
                         dark: &::brush::Brush,
                         mid: &::brush::Brush,
                         text: &::brush::Brush,
                         bright_text: &::brush::Brush,
                         base: &::brush::Brush,
                         window: &::brush::Brush) {
    unsafe {
      ::ffi::qt_gui_c_QPalette_setColorGroup(self as *mut ::palette::Palette,
                                             cr,
                                             window_text as *const ::brush::Brush,
                                             button as *const ::brush::Brush,
                                             light as *const ::brush::Brush,
                                             dark as *const ::brush::Brush,
                                             mid as *const ::brush::Brush,
                                             text as *const ::brush::Brush,
                                             bright_text as *const ::brush::Brush,
                                             base as *const ::brush::Brush,
                                             window as *const ::brush::Brush)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPalette::setCurrentColorGroup(QPalette::ColorGroup cg)```</span>
  ///
  ///
  pub fn set_current_color_group(&mut self, cg: ::palette::ColorGroup) {
    unsafe { ::ffi::qt_gui_c_QPalette_setCurrentColorGroup(self as *mut ::palette::Palette, cg) }
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::shadow() const```</span>
  ///
  ///
  pub fn shadow<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_shadow(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QPalette::swap(QPalette& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::palette::Palette) {
    unsafe {
      ::ffi::qt_gui_c_QPalette_swap(self as *mut ::palette::Palette,
                                    other as *mut ::palette::Palette)
    }
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::text() const```</span>
  ///
  ///
  pub fn text<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_text(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::toolTipBase() const```</span>
  ///
  ///
  pub fn tool_tip_base<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_toolTipBase(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::toolTipText() const```</span>
  ///
  ///
  pub fn tool_tip_text<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_toolTipText(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::window() const```</span>
  ///
  ///
  pub fn window<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_window(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QBrush& QPalette::windowText() const```</span>
  ///
  ///
  pub fn window_text<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_windowText(self as *const ::palette::Palette) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl Drop for ::palette::Palette {
  /// C++ method: <span style='color: green;'>```[destructor] void QPalette::~QPalette()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPalette_destructor(self as *mut ::palette::Palette) }
  }
}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QPalette& arg2)```</span>
///
///
pub fn op_shl(arg1: &::qt_core::debug::Debug, arg2: &::palette::Palette) -> ::qt_core::debug::Debug {
  {
    let mut object: ::qt_core::debug::Debug =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_gui_c_QPalette_G_operator_shl_to_output(arg1 as *const ::qt_core::debug::Debug,
                                                        arg2 as *const ::palette::Palette,
                                                        &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& ds, QPalette& p)```</span>
///
///
pub fn op_shr<'l0, 'l1>(ds: &'l0 mut ::qt_core::data_stream::DataStream,
                        p: &'l1 mut ::palette::Palette)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_gui_c_QPalette_G_operator_shr(ds as *mut ::qt_core::data_stream::DataStream,
                                            p as *mut ::palette::Palette)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```void swap(QPalette& value1, QPalette& value2)```</span>
///
///
pub fn swap(value1: &mut ::palette::Palette, value2: &mut ::palette::Palette) {
  unsafe {
    ::ffi::qt_gui_c_QPalette_G_swap(value1 as *mut ::palette::Palette,
                                    value2 as *mut ::palette::Palette)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Palette::brush](../struct.Palette.html#method.brush) method.
  pub trait PaletteBrushArgs<'largs> {
    fn exec(self, original_self: &'largs ::palette::Palette) -> &'largs ::brush::Brush;
  }
  impl<'largs> PaletteBrushArgs<'largs> for (::palette::ColorGroup, ::palette::ColorRole) {
    fn exec(self, original_self: &'largs ::palette::Palette) -> &'largs ::brush::Brush {
      let cg = self.0;
      let cr = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QPalette_brush_cg_cr(original_self as *const ::palette::Palette, cg, cr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> PaletteBrushArgs<'largs> for ::palette::ColorRole {
    fn exec(self, original_self: &'largs ::palette::Palette) -> &'largs ::brush::Brush {
      let cr = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_brush_cr(original_self as *const ::palette::Palette, cr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Palette::color](../struct.Palette.html#method.color) method.
  pub trait PaletteColorArgs<'largs> {
    fn exec(self, original_self: &'largs ::palette::Palette) -> &'largs ::color::Color;
  }
  impl<'largs> PaletteColorArgs<'largs> for (::palette::ColorGroup, ::palette::ColorRole) {
    fn exec(self, original_self: &'largs ::palette::Palette) -> &'largs ::color::Color {
      let cg = self.0;
      let cr = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QPalette_color_cg_cr(original_self as *const ::palette::Palette, cg, cr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> PaletteColorArgs<'largs> for ::palette::ColorRole {
    fn exec(self, original_self: &'largs ::palette::Palette) -> &'largs ::color::Color {
      let cr = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPalette_color_cr(original_self as *const ::palette::Palette, cr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Palette::new](../struct.Palette.html#method.new) method.
  pub trait PaletteNewArgs {
    fn exec(self) -> ::palette::Palette;
  }
  impl<'a> PaletteNewArgs
    for (&'a ::brush::Brush,
                                   &'a ::brush::Brush,
                                   &'a ::brush::Brush,
                                   &'a ::brush::Brush,
                                   &'a ::brush::Brush,
                                   &'a ::brush::Brush,
                                   &'a ::brush::Brush,
                                   &'a ::brush::Brush,
                                   &'a ::brush::Brush) {
    fn exec(self) -> ::palette::Palette {
      let window_text = self.0;
      let button = self.1;
      let light = self.2;
      let dark = self.3;
      let mid = self.4;
      let text = self.5;
      let bright_text = self.6;
      let base = self.7;
      let window = self.8;
      {
        let mut object: ::palette::Palette =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPalette_constructor_QBrush_QBrush_QBrush_QBrush_QBrush_QBrush_QBrush_QBrush_QBrush(window_text as *const ::brush::Brush, button as *const ::brush::Brush, light as *const ::brush::Brush, dark as *const ::brush::Brush, mid as *const ::brush::Brush, text as *const ::brush::Brush, bright_text as *const ::brush::Brush, base as *const ::brush::Brush, window as *const ::brush::Brush, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PaletteNewArgs for &'a ::color::Color {
    fn exec(self) -> ::palette::Palette {
      let button = self;
      {
        let mut object: ::palette::Palette =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPalette_constructor_QColor(button as *const ::color::Color, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PaletteNewArgs for (&'a ::color::Color, &'a ::color::Color) {
    fn exec(self) -> ::palette::Palette {
      let button = self.0;
      let window = self.1;
      {
        let mut object: ::palette::Palette =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPalette_constructor_QColor_QColor(button as *const ::color::Color,
                                                             window as *const ::color::Color,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'a> PaletteNewArgs
    for (&'a ::color::Color,
                                   &'a ::color::Color,
                                   &'a ::color::Color,
                                   &'a ::color::Color,
                                   &'a ::color::Color,
                                   &'a ::color::Color,
                                   &'a ::color::Color) {
    fn exec(self) -> ::palette::Palette {
      let window_text = self.0;
      let window = self.1;
      let light = self.2;
      let dark = self.3;
      let mid = self.4;
      let text = self.5;
      let base = self.6;
      {
        let mut object: ::palette::Palette =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPalette_constructor_QColor_QColor_QColor_QColor_QColor_QColor_QColor(window_text as *const ::color::Color, window as *const ::color::Color, light as *const ::color::Color, dark as *const ::color::Color, mid as *const ::color::Color, text as *const ::color::Color, base as *const ::color::Color, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PaletteNewArgs for &'a ::palette::Palette {
    fn exec(self) -> ::palette::Palette {
      let palette = self;
      {
        let mut object: ::palette::Palette =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPalette_constructor_QPalette(palette as *const ::palette::Palette, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PaletteNewArgs for &'a ::qt_core::qt::GlobalColor {
    fn exec(self) -> ::palette::Palette {
      let button = self;
      {
        let mut object: ::palette::Palette =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPalette_constructor_Qt_GlobalColor(button as *const ::qt_core::qt::GlobalColor, &mut object);
        }
        object
      }
    }
  }
  impl PaletteNewArgs for () {
    fn exec(self) -> ::palette::Palette {

      {
        let mut object: ::palette::Palette =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPalette_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Palette::resolve](../struct.Palette.html#method.resolve) method.
  pub trait PaletteResolveArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs ::palette::Palette) -> Self::ReturnType;
  }
  impl<'largs> PaletteResolveArgs<'largs> for () {
    type ReturnType = ::libc::c_uint;
    fn exec(self, original_self: &'largs ::palette::Palette) -> ::libc::c_uint {

      unsafe { ::ffi::qt_gui_c_QPalette_resolve_no_args(original_self as *const ::palette::Palette) }
    }
  }
  impl<'largs> PaletteResolveArgs<'largs> for &'largs ::palette::Palette {
    type ReturnType = ::palette::Palette;
    fn exec(self, original_self: &'largs ::palette::Palette) -> ::palette::Palette {
      let arg1 = self;
      {
        let mut object: ::palette::Palette =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPalette_resolve_to_output(original_self as *const ::palette::Palette,
                                                     arg1 as *const ::palette::Palette,
                                                     &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Palette::set_brush](../struct.Palette.html#method.set_brush) method.
  pub trait PaletteSetBrushArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::palette::Palette) -> ();
  }
  impl<'largs> PaletteSetBrushArgs<'largs> for (::palette::ColorGroup, ::palette::ColorRole, &'largs ::brush::Brush) {
    fn exec(self, original_self: &'largs mut ::palette::Palette) -> () {
      let cg = self.0;
      let cr = self.1;
      let brush = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPalette_setBrush_cg_cr_brush(original_self as *mut ::palette::Palette,
                                                      cg,
                                                      cr,
                                                      brush as *const ::brush::Brush)
      }
    }
  }
  impl<'largs> PaletteSetBrushArgs<'largs> for (::palette::ColorRole, &'largs ::brush::Brush) {
    fn exec(self, original_self: &'largs mut ::palette::Palette) -> () {
      let cr = self.0;
      let brush = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPalette_setBrush_cr_brush(original_self as *mut ::palette::Palette,
                                                   cr,
                                                   brush as *const ::brush::Brush)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Palette::set_color](../struct.Palette.html#method.set_color) method.
  pub trait PaletteSetColorArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::palette::Palette) -> ();
  }
  impl<'largs> PaletteSetColorArgs<'largs> for (::palette::ColorGroup, ::palette::ColorRole, &'largs ::color::Color) {
    fn exec(self, original_self: &'largs mut ::palette::Palette) -> () {
      let cg = self.0;
      let cr = self.1;
      let color = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPalette_setColor_cg_cr_color(original_self as *mut ::palette::Palette,
                                                      cg,
                                                      cr,
                                                      color as *const ::color::Color)
      }
    }
  }
  impl<'largs> PaletteSetColorArgs<'largs> for (::palette::ColorRole, &'largs ::color::Color) {
    fn exec(self, original_self: &'largs mut ::palette::Palette) -> () {
      let cr = self.0;
      let color = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPalette_setColor_cr_color(original_self as *mut ::palette::Palette,
                                                   cr,
                                                   color as *const ::color::Color)
      }
    }
  }
}
