/// C++ type: <span style='color: green;'>```QFont::Capitalization```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Capitalization {
  /// C++ enum variant: <span style='color: green;'>```MixedCase = 0```</span>
  MixedCase = 0,
  /// C++ enum variant: <span style='color: green;'>```AllUppercase = 1```</span>
  AllUppercase = 1,
  /// C++ enum variant: <span style='color: green;'>```AllLowercase = 2```</span>
  AllLowercase = 2,
  /// C++ enum variant: <span style='color: green;'>```SmallCaps = 3```</span>
  SmallCaps = 3,
  /// C++ enum variant: <span style='color: green;'>```Capitalize = 4```</span>
  Capitalize = 4,
}

/// C++ type: <span style='color: green;'>```QFont```</span>
#[repr(C)]
pub struct Font([u8; ::type_sizes::QT_GUI_FONT_FONT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Font {
  unsafe fn new_uninitialized() -> Font {
    Font(::std::mem::uninitialized())
  }
}

impl Font {
  /// C++ method: <span style='color: green;'>```QVariant QFont::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFont_convert_to_QVariant_to_output(self as *const ::font::Font, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFont::bold() const```</span>
  ///
  ///
  pub fn bold(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFont_bold(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```static void QFont::cacheStatistics()```</span>
  ///
  ///
  pub fn cache_statistics() {
    unsafe { ::ffi::qt_gui_c_QFont_cacheStatistics() }
  }

  /// C++ method: <span style='color: green;'>```QFont::Capitalization QFont::capitalization() const```</span>
  ///
  ///
  pub fn capitalization(&self) -> ::font::Capitalization {
    unsafe { ::ffi::qt_gui_c_QFont_capitalization(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```static void QFont::cleanup()```</span>
  ///
  ///
  pub fn cleanup() {
    unsafe { ::ffi::qt_gui_c_QFont_cleanup() }
  }

  /// C++ method: <span style='color: green;'>```QString QFont::defaultFamily() const```</span>
  ///
  ///
  pub fn default_family(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFont_defaultFamily_to_output(self as *const ::font::Font, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFont::exactMatch() const```</span>
  ///
  ///
  pub fn exact_match(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFont_exactMatch(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```QString QFont::family() const```</span>
  ///
  ///
  pub fn family(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFont_family_to_output(self as *const ::font::Font, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFont::fixedPitch() const```</span>
  ///
  ///
  pub fn fixed_pitch(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFont_fixedPitch(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```bool QFont::fromString(const QString& arg1)```</span>
  ///
  ///
  pub fn from_string(&mut self, arg1: &::qt_core::string::String) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QFont_fromString(self as *mut ::font::Font,
                                       arg1 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QFont::HintingPreference QFont::hintingPreference() const```</span>
  ///
  ///
  pub fn hinting_preference(&self) -> ::font::HintingPreference {
    unsafe { ::ffi::qt_gui_c_QFont_hintingPreference(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```static void QFont::initialize()```</span>
  ///
  ///
  pub fn initialize() {
    unsafe { ::ffi::qt_gui_c_QFont_initialize() }
  }

  /// C++ method: <span style='color: green;'>```static void QFont::insertSubstitution(const QString& arg1, const QString& arg2)```</span>
  ///
  ///
  pub fn insert_substitution(arg1: &::qt_core::string::String, arg2: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QFont_insertSubstitution(arg1 as *const ::qt_core::string::String,
                                               arg2 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static void QFont::insertSubstitutions(const QString& arg1, const QStringList& arg2)```</span>
  ///
  ///
  pub fn insert_substitutions(arg1: &::qt_core::string::String, arg2: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_gui_c_QFont_insertSubstitutions(arg1 as *const ::qt_core::string::String,
                                                arg2 as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFont::isCopyOf(const QFont& arg1) const```</span>
  ///
  ///
  pub fn is_copy_of(&self, arg1: &::font::Font) -> bool {
    unsafe { ::ffi::qt_gui_c_QFont_isCopyOf(self as *const ::font::Font, arg1 as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```bool QFont::italic() const```</span>
  ///
  ///
  pub fn italic(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFont_italic(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```bool QFont::kerning() const```</span>
  ///
  ///
  pub fn kerning(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFont_kerning(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```QString QFont::key() const```</span>
  ///
  ///
  pub fn key(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFont_key_to_output(self as *const ::font::Font, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFont::lastResortFamily() const```</span>
  ///
  ///
  pub fn last_resort_family(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFont_lastResortFamily_to_output(self as *const ::font::Font, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFont::lastResortFont() const```</span>
  ///
  ///
  pub fn last_resort_font(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFont_lastResortFont_to_output(self as *const ::font::Font, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QFont::letterSpacing() const```</span>
  ///
  ///
  pub fn letter_spacing(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFont_letterSpacing(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```QFont::SpacingType QFont::letterSpacingType() const```</span>
  ///
  ///
  pub fn letter_spacing_type(&self) -> ::font::SpacingType {
    unsafe { ::ffi::qt_gui_c_QFont_letterSpacingType(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```QFont::QFont```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::font::Font```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFont::QFont()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::font::Font) -> ::font::Font```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFont::QFont(const QFont& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::font::Font```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFont::QFont(const QString& family)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::qt_core::string::String, ::libc::c_int)) -> ::font::Font```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFont::QFont(const QString& family, int pointSize = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((&::qt_core::string::String, ::libc::c_int, ::libc::c_int)) -> ::font::Font```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFont::QFont(const QString& family, int pointSize = ?, int weight = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new((&::qt_core::string::String, ::libc::c_int, ::libc::c_int, bool)) -> ::font::Font```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFont::QFont(const QString& family, int pointSize = ?, int weight = ?, bool italic = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::font::Font
    where Args: overloading::FontNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QFont::QFont(const QFont& arg1, QPaintDevice* pd)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(arg1: &::font::Font, pd: *mut ::paint_device::PaintDevice) -> ::font::Font {
    {
      let mut object: ::font::Font = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QFont_constructor_arg1_pd(arg1 as *const ::font::Font, pd, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFont& QFont::operator=(const QFont& arg1)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, arg1: &'l1 ::font::Font) -> &'l0 mut ::font::Font {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QFont_operator_assign(self as *mut ::font::Font, arg1 as *const ::font::Font) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QFont::operator==(const QFont& arg1) const```</span>
  ///
  ///
  pub fn op_eq(&self, arg1: &::font::Font) -> bool {
    unsafe { ::ffi::qt_gui_c_QFont_operator_eq(self as *const ::font::Font, arg1 as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```bool QFont::operator<(const QFont& arg1) const```</span>
  ///
  ///
  pub fn op_lt(&self, arg1: &::font::Font) -> bool {
    unsafe { ::ffi::qt_gui_c_QFont_operator_lt(self as *const ::font::Font, arg1 as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```bool QFont::operator!=(const QFont& arg1) const```</span>
  ///
  ///
  pub fn op_neq(&self, arg1: &::font::Font) -> bool {
    unsafe { ::ffi::qt_gui_c_QFont_operator_neq(self as *const ::font::Font, arg1 as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```bool QFont::overline() const```</span>
  ///
  ///
  pub fn overline(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFont_overline(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```int QFont::pixelSize() const```</span>
  ///
  ///
  pub fn pixel_size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFont_pixelSize(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```int QFont::pointSize() const```</span>
  ///
  ///
  pub fn point_size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFont_pointSize(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```double QFont::pointSizeF() const```</span>
  ///
  ///
  pub fn point_size_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFont_pointSizeF(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```bool QFont::rawMode() const```</span>
  ///
  ///
  pub fn raw_mode(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFont_rawMode(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```QString QFont::rawName() const```</span>
  ///
  ///
  pub fn raw_name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFont_rawName_to_output(self as *const ::font::Font, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static void QFont::removeSubstitutions(const QString& arg1)```</span>
  ///
  ///
  pub fn remove_substitutions(arg1: &::qt_core::string::String) {
    unsafe { ::ffi::qt_gui_c_QFont_removeSubstitutions(arg1 as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QFont::resolve```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn resolve(&self, &::font::Font) -> ::font::Font```<br>
  /// C++ method: <span style='color: green;'>```QFont QFont::resolve(const QFont& arg1) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn resolve(&self, ()) -> ::libc::c_uint```<br>
  /// C++ method: <span style='color: green;'>```unsigned int QFont::resolve() const```</span>
  ///
  ///
  pub fn resolve<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::FontResolveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QFont::resolve(unsigned int mask)```</span>
  ///
  ///
  pub fn resolve_mut(&mut self, mask: ::libc::c_uint) {
    unsafe { ::ffi::qt_gui_c_QFont_resolve_mask(self as *mut ::font::Font, mask) }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setBold(bool arg1)```</span>
  ///
  ///
  pub fn set_bold(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_gui_c_QFont_setBold(self as *mut ::font::Font, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setCapitalization(QFont::Capitalization arg1)```</span>
  ///
  ///
  pub fn set_capitalization(&mut self, arg1: ::font::Capitalization) {
    unsafe { ::ffi::qt_gui_c_QFont_setCapitalization(self as *mut ::font::Font, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setFamily(const QString& arg1)```</span>
  ///
  ///
  pub fn set_family(&mut self, arg1: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QFont_setFamily(self as *mut ::font::Font,
                                      arg1 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setFixedPitch(bool arg1)```</span>
  ///
  ///
  pub fn set_fixed_pitch(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_gui_c_QFont_setFixedPitch(self as *mut ::font::Font, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setHintingPreference(QFont::HintingPreference hintingPreference)```</span>
  ///
  ///
  pub fn set_hinting_preference(&mut self, hinting_preference: ::font::HintingPreference) {
    unsafe { ::ffi::qt_gui_c_QFont_setHintingPreference(self as *mut ::font::Font, hinting_preference) }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setItalic(bool b)```</span>
  ///
  ///
  pub fn set_italic(&mut self, b: bool) {
    unsafe { ::ffi::qt_gui_c_QFont_setItalic(self as *mut ::font::Font, b) }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setKerning(bool arg1)```</span>
  ///
  ///
  pub fn set_kerning(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_gui_c_QFont_setKerning(self as *mut ::font::Font, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setLetterSpacing(QFont::SpacingType type, double spacing)```</span>
  ///
  ///
  pub fn set_letter_spacing(&mut self, type_: ::font::SpacingType, spacing: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QFont_setLetterSpacing(self as *mut ::font::Font, type_, spacing) }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setOverline(bool arg1)```</span>
  ///
  ///
  pub fn set_overline(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_gui_c_QFont_setOverline(self as *mut ::font::Font, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setPixelSize(int arg1)```</span>
  ///
  ///
  pub fn set_pixel_size(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QFont_setPixelSize(self as *mut ::font::Font, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setPointSize(int arg1)```</span>
  ///
  ///
  pub fn set_point_size(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QFont_setPointSize(self as *mut ::font::Font, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setPointSizeF(double arg1)```</span>
  ///
  ///
  pub fn set_point_size_f(&mut self, arg1: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QFont_setPointSizeF(self as *mut ::font::Font, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setRawMode(bool arg1)```</span>
  ///
  ///
  pub fn set_raw_mode(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_gui_c_QFont_setRawMode(self as *mut ::font::Font, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setRawName(const QString& arg1)```</span>
  ///
  ///
  pub fn set_raw_name(&mut self, arg1: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QFont_setRawName(self as *mut ::font::Font,
                                       arg1 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setStretch(int arg1)```</span>
  ///
  ///
  pub fn set_stretch(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QFont_setStretch(self as *mut ::font::Font, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setStrikeOut(bool arg1)```</span>
  ///
  ///
  pub fn set_strike_out(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_gui_c_QFont_setStrikeOut(self as *mut ::font::Font, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setStyle(QFont::Style style)```</span>
  ///
  ///
  pub fn set_style(&mut self, style: ::font::Style) {
    unsafe { ::ffi::qt_gui_c_QFont_setStyle(self as *mut ::font::Font, style) }
  }

  /// C++ method: <span style='color: green;'>```QFont::setStyleHint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_style_hint(&mut self, ::font::StyleHint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFont::setStyleHint(QFont::StyleHint arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_style_hint(&mut self, (::font::StyleHint, ::font::StyleStrategy)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFont::setStyleHint(QFont::StyleHint arg1, QFont::StyleStrategy arg2 = ?)```</span>
  ///
  ///
  pub fn set_style_hint<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::FontSetStyleHintArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QFont::setStyleName(const QString& arg1)```</span>
  ///
  ///
  pub fn set_style_name(&mut self, arg1: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QFont_setStyleName(self as *mut ::font::Font,
                                         arg1 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setStyleStrategy(QFont::StyleStrategy s)```</span>
  ///
  ///
  pub fn set_style_strategy(&mut self, s: ::font::StyleStrategy) {
    unsafe { ::ffi::qt_gui_c_QFont_setStyleStrategy(self as *mut ::font::Font, s) }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setUnderline(bool arg1)```</span>
  ///
  ///
  pub fn set_underline(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_gui_c_QFont_setUnderline(self as *mut ::font::Font, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setWeight(int arg1)```</span>
  ///
  ///
  pub fn set_weight(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QFont_setWeight(self as *mut ::font::Font, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QFont::setWordSpacing(double spacing)```</span>
  ///
  ///
  pub fn set_word_spacing(&mut self, spacing: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QFont_setWordSpacing(self as *mut ::font::Font, spacing) }
  }

  /// C++ method: <span style='color: green;'>```int QFont::stretch() const```</span>
  ///
  ///
  pub fn stretch(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFont_stretch(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```bool QFont::strikeOut() const```</span>
  ///
  ///
  pub fn strike_out(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFont_strikeOut(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```QFont::Style QFont::style() const```</span>
  ///
  ///
  pub fn style(&self) -> ::font::Style {
    unsafe { ::ffi::qt_gui_c_QFont_style(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```QFont::StyleHint QFont::styleHint() const```</span>
  ///
  ///
  pub fn style_hint(&self) -> ::font::StyleHint {
    unsafe { ::ffi::qt_gui_c_QFont_styleHint(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```QString QFont::styleName() const```</span>
  ///
  ///
  pub fn style_name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFont_styleName_to_output(self as *const ::font::Font, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFont::StyleStrategy QFont::styleStrategy() const```</span>
  ///
  ///
  pub fn style_strategy(&self) -> ::font::StyleStrategy {
    unsafe { ::ffi::qt_gui_c_QFont_styleStrategy(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```static QString QFont::substitute(const QString& arg1)```</span>
  ///
  ///
  pub fn substitute(arg1: &::qt_core::string::String) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFont_substitute_to_output(arg1 as *const ::qt_core::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QStringList QFont::substitutes(const QString& arg1)```</span>
  ///
  ///
  pub fn substitutes(arg1: &::qt_core::string::String) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFont_substitutes_to_output(arg1 as *const ::qt_core::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QStringList QFont::substitutions()```</span>
  ///
  ///
  pub fn substitutions() -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFont_substitutions_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QFont::swap(QFont& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::font::Font) {
    unsafe { ::ffi::qt_gui_c_QFont_swap(self as *mut ::font::Font, other as *mut ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```QString QFont::toString() const```</span>
  ///
  ///
  pub fn to_string(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFont_toString_to_output(self as *const ::font::Font, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFont::underline() const```</span>
  ///
  ///
  pub fn underline(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFont_underline(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```int QFont::weight() const```</span>
  ///
  ///
  pub fn weight(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFont_weight(self as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```double QFont::wordSpacing() const```</span>
  ///
  ///
  pub fn word_spacing(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFont_wordSpacing(self as *const ::font::Font) }
  }
}

impl Drop for ::font::Font {
  /// C++ method: <span style='color: green;'>```[destructor] void QFont::~QFont()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QFont_destructor(self as *mut ::font::Font) }
  }
}

/// C++ type: <span style='color: green;'>```QFont::HintingPreference```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum HintingPreference {
  /// C++ enum variant: <span style='color: green;'>```PreferDefaultHinting = 0```</span>
  Default = 0,
  /// C++ enum variant: <span style='color: green;'>```PreferNoHinting = 1```</span>
  No = 1,
  /// C++ enum variant: <span style='color: green;'>```PreferVerticalHinting = 2```</span>
  Vertical = 2,
  /// C++ enum variant: <span style='color: green;'>```PreferFullHinting = 3```</span>
  Full = 3,
}

/// C++ type: <span style='color: green;'>```QFont::ResolveProperties```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ResolveProperties {
  /// C++ enum variant: <span style='color: green;'>```FamilyResolved = 1```</span>
  Family = 1,
  /// C++ enum variant: <span style='color: green;'>```SizeResolved = 2```</span>
  Size = 2,
  /// C++ enum variant: <span style='color: green;'>```StyleHintResolved = 4```</span>
  StyleHint = 4,
  /// C++ enum variant: <span style='color: green;'>```StyleStrategyResolved = 8```</span>
  StyleStrategy = 8,
  /// C++ enum variant: <span style='color: green;'>```WeightResolved = 16```</span>
  Weight = 16,
  /// C++ enum variant: <span style='color: green;'>```StyleResolved = 32```</span>
  Style = 32,
  /// C++ enum variant: <span style='color: green;'>```UnderlineResolved = 64```</span>
  Underline = 64,
  /// C++ enum variant: <span style='color: green;'>```OverlineResolved = 128```</span>
  Overline = 128,
  /// C++ enum variant: <span style='color: green;'>```StrikeOutResolved = 256```</span>
  StrikeOut = 256,
  /// C++ enum variant: <span style='color: green;'>```FixedPitchResolved = 512```</span>
  FixedPitch = 512,
  /// C++ enum variant: <span style='color: green;'>```StretchResolved = 1024```</span>
  Stretch = 1024,
  /// C++ enum variant: <span style='color: green;'>```KerningResolved = 2048```</span>
  Kerning = 2048,
  /// C++ enum variant: <span style='color: green;'>```CapitalizationResolved = 4096```</span>
  Capitalization = 4096,
  /// C++ enum variant: <span style='color: green;'>```LetterSpacingResolved = 8192```</span>
  LetterSpacing = 8192,
  /// C++ enum variant: <span style='color: green;'>```WordSpacingResolved = 16384```</span>
  WordSpacing = 16384,
  /// C++ enum variant: <span style='color: green;'>```HintingPreferenceResolved = 32768```</span>
  HintingPreference = 32768,
  /// C++ enum variant: <span style='color: green;'>```StyleNameResolved = 65536```</span>
  StyleName = 65536,
  /// C++ enum variant: <span style='color: green;'>```AllPropertiesResolved = 131071```</span>
  AllProperties = 131071,
}

/// C++ type: <span style='color: green;'>```QFont::SpacingType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SpacingType {
  /// C++ enum variant: <span style='color: green;'>```PercentageSpacing = 0```</span>
  Percentage = 0,
  /// C++ enum variant: <span style='color: green;'>```AbsoluteSpacing = 1```</span>
  Absolute = 1,
}

/// C++ type: <span style='color: green;'>```QFont::Stretch```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Stretch {
  /// C++ enum variant: <span style='color: green;'>```AnyStretch = 0```</span>
  AnyStretch = 0,
  /// C++ enum variant: <span style='color: green;'>```UltraCondensed = 50```</span>
  UltraCondensed = 50,
  /// C++ enum variant: <span style='color: green;'>```ExtraCondensed = 62```</span>
  ExtraCondensed = 62,
  /// C++ enum variant: <span style='color: green;'>```Condensed = 75```</span>
  Condensed = 75,
  /// C++ enum variant: <span style='color: green;'>```SemiCondensed = 87```</span>
  SemiCondensed = 87,
  /// C++ enum variant: <span style='color: green;'>```Unstretched = 100```</span>
  Unstretched = 100,
  /// C++ enum variant: <span style='color: green;'>```SemiExpanded = 112```</span>
  SemiExpanded = 112,
  /// C++ enum variant: <span style='color: green;'>```Expanded = 125```</span>
  Expanded = 125,
  /// C++ enum variant: <span style='color: green;'>```ExtraExpanded = 150```</span>
  ExtraExpanded = 150,
  /// C++ enum variant: <span style='color: green;'>```UltraExpanded = 200```</span>
  UltraExpanded = 200,
}

/// C++ type: <span style='color: green;'>```QFont::Style```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Style {
  /// C++ enum variant: <span style='color: green;'>```StyleNormal = 0```</span>
  Normal = 0,
  /// C++ enum variant: <span style='color: green;'>```StyleItalic = 1```</span>
  Italic = 1,
  /// C++ enum variant: <span style='color: green;'>```StyleOblique = 2```</span>
  Oblique = 2,
}

/// C++ type: <span style='color: green;'>```QFont::StyleHint```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleHint {
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Helvetica = 0```</span>
  /// - <span style='color: green;'>```SansSerif = 0```</span>
  ///
  Helvetica = 0,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Times = 1```</span>
  /// - <span style='color: green;'>```Serif = 1```</span>
  ///
  Times = 1,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Courier = 2```</span>
  /// - <span style='color: green;'>```TypeWriter = 2```</span>
  ///
  Courier = 2,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```OldEnglish = 3```</span>
  /// - <span style='color: green;'>```Decorative = 3```</span>
  ///
  OldEnglish = 3,
  /// C++ enum variant: <span style='color: green;'>```System = 4```</span>
  System = 4,
  /// C++ enum variant: <span style='color: green;'>```AnyStyle = 5```</span>
  AnyStyle = 5,
  /// C++ enum variant: <span style='color: green;'>```Cursive = 6```</span>
  Cursive = 6,
  /// C++ enum variant: <span style='color: green;'>```Monospace = 7```</span>
  Monospace = 7,
  /// C++ enum variant: <span style='color: green;'>```Fantasy = 8```</span>
  Fantasy = 8,
}

/// C++ type: <span style='color: green;'>```QFont::StyleStrategy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleStrategy {
  /// C++ enum variant: <span style='color: green;'>```PreferDefault = 1```</span>
  PreferDefault = 1,
  /// C++ enum variant: <span style='color: green;'>```PreferBitmap = 2```</span>
  PreferBitmap = 2,
  /// C++ enum variant: <span style='color: green;'>```PreferDevice = 4```</span>
  PreferDevice = 4,
  /// C++ enum variant: <span style='color: green;'>```PreferOutline = 8```</span>
  PreferOutline = 8,
  /// C++ enum variant: <span style='color: green;'>```ForceOutline = 16```</span>
  ForceOutline = 16,
  /// C++ enum variant: <span style='color: green;'>```PreferMatch = 32```</span>
  PreferMatch = 32,
  /// C++ enum variant: <span style='color: green;'>```PreferQuality = 64```</span>
  PreferQuality = 64,
  /// C++ enum variant: <span style='color: green;'>```PreferAntialias = 128```</span>
  PreferAntialias = 128,
  /// C++ enum variant: <span style='color: green;'>```NoAntialias = 256```</span>
  NoAntialias = 256,
  /// C++ enum variant: <span style='color: green;'>```OpenGLCompatible = 512```</span>
  OpenGLCompatible = 512,
  /// C++ enum variant: <span style='color: green;'>```ForceIntegerMetrics = 1024```</span>
  ForceIntegerMetrics = 1024,
  /// C++ enum variant: <span style='color: green;'>```NoSubpixelAntialias = 2048```</span>
  NoSubpixelAntialias = 2048,
  /// C++ enum variant: <span style='color: green;'>```NoFontMerging = 32768```</span>
  NoFontMerging = 32768,
}

/// C++ type: <span style='color: green;'>```QFont::Weight```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Weight {
  /// C++ enum variant: <span style='color: green;'>```Thin = 0```</span>
  Thin = 0,
  /// C++ enum variant: <span style='color: green;'>```ExtraLight = 12```</span>
  ExtraLight = 12,
  /// C++ enum variant: <span style='color: green;'>```Light = 25```</span>
  Light = 25,
  /// C++ enum variant: <span style='color: green;'>```Normal = 50```</span>
  Normal = 50,
  /// C++ enum variant: <span style='color: green;'>```Medium = 57```</span>
  Medium = 57,
  /// C++ enum variant: <span style='color: green;'>```DemiBold = 63```</span>
  DemiBold = 63,
  /// C++ enum variant: <span style='color: green;'>```Bold = 75```</span>
  Bold = 75,
  /// C++ enum variant: <span style='color: green;'>```ExtraBold = 81```</span>
  ExtraBold = 81,
  /// C++ enum variant: <span style='color: green;'>```Black = 87```</span>
  Black = 87,
}

/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash(&::font::Font) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QFont& font)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash((&::font::Font, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QFont& font, unsigned int seed = ?)```</span>
///
///
pub fn hash<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::HashArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QFont& arg2)```</span>
///
///
pub fn op_shl(arg1: &::qt_core::debug::Debug, arg2: &::font::Font) -> ::qt_core::debug::Debug {
  {
    let mut object: ::qt_core::debug::Debug =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_gui_c_QFont_G_operator_shl_to_output(arg1 as *const ::qt_core::debug::Debug,
                                                     arg2 as *const ::font::Font,
                                                     &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```void swap(QFont& value1, QFont& value2)```</span>
///
///
pub fn swap(value1: &mut ::font::Font, value2: &mut ::font::Font) {
  unsafe { ::ffi::qt_gui_c_QFont_G_swap(value1 as *mut ::font::Font, value2 as *mut ::font::Font) }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Font::new](../struct.Font.html#method.new) method.
  pub trait FontNewArgs {
    fn exec(self) -> ::font::Font;
  }
  impl<'a> FontNewArgs for &'a ::font::Font {
    fn exec(self) -> ::font::Font {
      let arg1 = self;
      {
        let mut object: ::font::Font = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFont_constructor_arg1(arg1 as *const ::font::Font, &mut object);
        }
        object
      }
    }
  }
  impl<'a> FontNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::font::Font {
      let family = self;
      {
        let mut object: ::font::Font = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFont_constructor_family(family as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> FontNewArgs for (&'a ::qt_core::string::String, ::libc::c_int) {
    fn exec(self) -> ::font::Font {
      let family = self.0;
      let point_size = self.1;
      {
        let mut object: ::font::Font = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFont_constructor_family_pointSize(family as *const ::qt_core::string::String,
                                                             point_size,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'a> FontNewArgs for (&'a ::qt_core::string::String, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::font::Font {
      let family = self.0;
      let point_size = self.1;
      let weight = self.2;
      {
        let mut object: ::font::Font = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFont_constructor_family_pointSize_weight(family as *const ::qt_core::string::String,
                                                                    point_size,
                                                                    weight,
                                                                    &mut object);
        }
        object
      }
    }
  }
  impl<'a> FontNewArgs for (&'a ::qt_core::string::String, ::libc::c_int, ::libc::c_int, bool) {
    fn exec(self) -> ::font::Font {
      let family = self.0;
      let point_size = self.1;
      let weight = self.2;
      let italic = self.3;
      {
        let mut object: ::font::Font = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFont_constructor_family_pointSize_weight_italic(family as *const ::qt_core::string::String, point_size, weight, italic, &mut object);
        }
        object
      }
    }
  }
  impl FontNewArgs for () {
    fn exec(self) -> ::font::Font {

      {
        let mut object: ::font::Font = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFont_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Font::resolve](../struct.Font.html#method.resolve) method.
  pub trait FontResolveArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs ::font::Font) -> Self::ReturnType;
  }
  impl<'largs> FontResolveArgs<'largs> for () {
    type ReturnType = ::libc::c_uint;
    fn exec(self, original_self: &'largs ::font::Font) -> ::libc::c_uint {

      unsafe { ::ffi::qt_gui_c_QFont_resolve_no_args(original_self as *const ::font::Font) }
    }
  }
  impl<'largs> FontResolveArgs<'largs> for &'largs ::font::Font {
    type ReturnType = ::font::Font;
    fn exec(self, original_self: &'largs ::font::Font) -> ::font::Font {
      let arg1 = self;
      {
        let mut object: ::font::Font = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFont_resolve_to_output(original_self as *const ::font::Font,
                                                  arg1 as *const ::font::Font,
                                                  &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Font::set_style_hint](../struct.Font.html#method.set_style_hint) method.
  pub trait FontSetStyleHintArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::font::Font) -> ();
  }
  impl<'largs> FontSetStyleHintArgs<'largs> for ::font::StyleHint {
    fn exec(self, original_self: &'largs mut ::font::Font) -> () {
      let arg1 = self;
      unsafe { ::ffi::qt_gui_c_QFont_setStyleHint_arg1(original_self as *mut ::font::Font, arg1) }
    }
  }
  impl<'largs> FontSetStyleHintArgs<'largs> for (::font::StyleHint, ::font::StyleStrategy) {
    fn exec(self, original_self: &'largs mut ::font::Font) -> () {
      let arg1 = self.0;
      let arg2 = self.1;
      unsafe { ::ffi::qt_gui_c_QFont_setStyleHint_arg1_arg2(original_self as *mut ::font::Font, arg1, arg2) }
    }
  }
  /// This trait represents a set of arguments accepted by [hash](../fn.hash.html) method.
  pub trait HashArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> HashArgs for &'a ::font::Font {
    fn exec(self) -> ::libc::c_uint {
      let font = self;
      unsafe { ::ffi::qt_gui_c_QFont_G_qHash_font(font as *const ::font::Font) }
    }
  }
  impl<'a> HashArgs for (&'a ::font::Font, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let font = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_gui_c_QFont_G_qHash_font_seed(font as *const ::font::Font, seed) }
    }
  }
}
