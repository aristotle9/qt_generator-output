/// C++ type: <span style='color: green;'>```QRawFont::AntialiasingType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum AntialiasingType {
  /// C++ enum variant: <span style='color: green;'>```PixelAntialiasing = 0```</span>
  PixelAntialiasing = 0,
  /// C++ enum variant: <span style='color: green;'>```SubPixelAntialiasing = 1```</span>
  SubPixelAntialiasing = 1,
}

/// C++ type: <span style='color: green;'>```QRawFont::LayoutFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum LayoutFlag {
  /// C++ enum variant: <span style='color: green;'>```SeparateAdvances = 0```</span>
  SeparateAdvances = 0,
  /// C++ enum variant: <span style='color: green;'>```KernedAdvances = 1```</span>
  KernedAdvances = 1,
  /// C++ enum variant: <span style='color: green;'>```UseDesignMetrics = 2```</span>
  UseDesignMetrics = 2,
}

impl ::qt_core::flags::FlaggableEnum for LayoutFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "LayoutFlag"
  }
}

/// C++ type: <span style='color: green;'>```QRawFont```</span>
#[repr(C)]
pub struct RawFont([u8; ::type_sizes::QT_GUI_RAW_FONT_RAW_FONT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for RawFont {
  unsafe fn new_uninitialized() -> RawFont {
    RawFont(::std::mem::uninitialized())
  }
}

impl RawFont {
  /// C++ method: <span style='color: green;'>```QRawFont::advancesForGlyphIndexes```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn advances_for_glyph_indexes(&self, &::vector::VectorU32) -> ::qt_core::vector::VectorPointF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPointF> QRawFont::advancesForGlyphIndexes(const QVector<quint32>& glyphIndexes) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn advances_for_glyph_indexes(&self, (&::vector::VectorU32, ::qt_core::flags::Flags<::raw_font::LayoutFlag>)) -> ::qt_core::vector::VectorPointF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPointF> QRawFont::advancesForGlyphIndexes(const QVector<quint32>& glyphIndexes, QFlags<QRawFont::LayoutFlag> layoutFlags) const```</span>
  ///
  ///
  pub fn advances_for_glyph_indexes<'largs, Args>(&'largs self, args: Args) -> ::qt_core::vector::VectorPointF
    where Args: overloading::RawFontAdvancesForGlyphIndexesArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRawFont::advancesForGlyphIndexes```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn advances_for_glyph_indexes_unsafe(&self, (*const u32, *mut ::qt_core::point_f::PointF, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QRawFont::advancesForGlyphIndexes(const quint32* glyphIndexes, QPointF* advances, int numGlyphs) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn advances_for_glyph_indexes_unsafe(&self, (*const u32, *mut ::qt_core::point_f::PointF, ::libc::c_int, ::qt_core::flags::Flags<::raw_font::LayoutFlag>)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QRawFont::advancesForGlyphIndexes(const quint32* glyphIndexes, QPointF* advances, int numGlyphs, QFlags<QRawFont::LayoutFlag> layoutFlags) const```</span>
  ///
  ///
  pub unsafe fn advances_for_glyph_indexes_unsafe<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::RawFontAdvancesForGlyphIndexesUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRawFont::alphaMapForGlyph```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn alpha_map_for_glyph(&self, u32) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QRawFont::alphaMapForGlyph(quint32 glyphIndex) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn alpha_map_for_glyph(&self, (u32, ::raw_font::AntialiasingType)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QRawFont::alphaMapForGlyph(quint32 glyphIndex, QRawFont::AntialiasingType antialiasingType = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn alpha_map_for_glyph(&self, (u32, ::raw_font::AntialiasingType, &::transform::Transform)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QRawFont::alphaMapForGlyph(quint32 glyphIndex, QRawFont::AntialiasingType antialiasingType = ?, const QTransform& transform = ?) const```</span>
  ///
  ///
  pub fn alpha_map_for_glyph<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::image::Image>
    where Args: overloading::RawFontAlphaMapForGlyphArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```double QRawFont::ascent() const```</span>
  ///
  ///
  pub fn ascent(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QRawFont_ascent(self as *const ::raw_font::RawFont) }
  }

  /// C++ method: <span style='color: green;'>```double QRawFont::averageCharWidth() const```</span>
  ///
  ///
  pub fn average_char_width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QRawFont_averageCharWidth(self as *const ::raw_font::RawFont) }
  }

  /// C++ method: <span style='color: green;'>```QRectF QRawFont::boundingRect(quint32 glyphIndex) const```</span>
  ///
  ///
  pub fn bounding_rect(&self, glyph_index: u32) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QRawFont_boundingRect_to_output(self as *const ::raw_font::RawFont, glyph_index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QRawFont::capHeight() const```</span>
  ///
  ///
  pub fn cap_height(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QRawFont_capHeight(self as *const ::raw_font::RawFont) }
  }

  /// C++ method: <span style='color: green;'>```double QRawFont::descent() const```</span>
  ///
  ///
  pub fn descent(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QRawFont_descent(self as *const ::raw_font::RawFont) }
  }

  /// C++ method: <span style='color: green;'>```QString QRawFont::familyName() const```</span>
  ///
  ///
  pub fn family_name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QRawFont_familyName_to_output(self as *const ::raw_font::RawFont, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QRawFont::fontTable(const char* tagName) const```</span>
  ///
  ///
  pub unsafe fn font_table(&self, tag_name: *const ::libc::c_char) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QRawFont_fontTable_to_output(self as *const ::raw_font::RawFont, tag_name, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRawFont::fromFont```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_font(&::font::Font) -> ::raw_font::RawFont```<br>
  /// C++ method: <span style='color: green;'>```static QRawFont QRawFont::fromFont(const QFont& font)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_font((&::font::Font, &::font_database::WritingSystem)) -> ::raw_font::RawFont```<br>
  /// C++ method: <span style='color: green;'>```static QRawFont QRawFont::fromFont(const QFont& font, QFontDatabase::WritingSystem writingSystem = ?)```</span>
  ///
  ///
  pub fn from_font<Args>(args: Args) -> ::raw_font::RawFont
    where Args: overloading::RawFontFromFontArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QRawFont::glyphIndexesForChars(const QChar* chars, int numChars, quint32* glyphIndexes, int* numGlyphs) const```</span>
  ///
  ///
  pub unsafe fn glyph_indexes_for_chars(&self,
                                        chars: *const ::qt_core::char::Char,
                                        num_chars: ::libc::c_int,
                                        glyph_indexes: *mut u32,
                                        num_glyphs: *mut ::libc::c_int)
                                        -> bool {
    ::ffi::qt_gui_c_QRawFont_glyphIndexesForChars(self as *const ::raw_font::RawFont,
                                                  chars,
                                                  num_chars,
                                                  glyph_indexes,
                                                  num_glyphs)
  }

  /// C++ method: <span style='color: green;'>```QVector<quint32> QRawFont::glyphIndexesForString(const QString& text) const```</span>
  ///
  ///
  pub fn glyph_indexes_for_string(&self, text: &::qt_core::string::String) -> ::vector::VectorU32 {
    {
      let mut object: ::vector::VectorU32 =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QRawFont_glyphIndexesForString_to_output(self as *const ::raw_font::RawFont,
                                                                 text as *const ::qt_core::string::String,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QRawFont::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QRawFont_isValid(self as *const ::raw_font::RawFont) }
  }

  /// C++ method: <span style='color: green;'>```double QRawFont::leading() const```</span>
  ///
  ///
  pub fn leading(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QRawFont_leading(self as *const ::raw_font::RawFont) }
  }

  /// C++ method: <span style='color: green;'>```double QRawFont::lineThickness() const```</span>
  ///
  ///
  pub fn line_thickness(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QRawFont_lineThickness(self as *const ::raw_font::RawFont) }
  }

  /// C++ method: <span style='color: green;'>```void QRawFont::loadFromData(const QByteArray& fontData, double pixelSize, QFont::HintingPreference hintingPreference)```</span>
  ///
  ///
  pub fn load_from_data(&mut self,
                        font_data: &::qt_core::byte_array::ByteArray,
                        pixel_size: ::libc::c_double,
                        hinting_preference: &::font::HintingPreference) {
    unsafe {
      ::ffi::qt_gui_c_QRawFont_loadFromData(self as *mut ::raw_font::RawFont,
                                            font_data as *const ::qt_core::byte_array::ByteArray,
                                            pixel_size,
                                            hinting_preference as *const ::font::HintingPreference)
    }
  }

  /// C++ method: <span style='color: green;'>```void QRawFont::loadFromFile(const QString& fileName, double pixelSize, QFont::HintingPreference hintingPreference)```</span>
  ///
  ///
  pub fn load_from_file(&mut self,
                        file_name: &::qt_core::string::String,
                        pixel_size: ::libc::c_double,
                        hinting_preference: &::font::HintingPreference) {
    unsafe {
      ::ffi::qt_gui_c_QRawFont_loadFromFile(self as *mut ::raw_font::RawFont,
                                            file_name as *const ::qt_core::string::String,
                                            pixel_size,
                                            hinting_preference as *const ::font::HintingPreference)
    }
  }

  /// C++ method: <span style='color: green;'>```double QRawFont::maxCharWidth() const```</span>
  ///
  ///
  pub fn max_char_width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QRawFont_maxCharWidth(self as *const ::raw_font::RawFont) }
  }

  /// C++ method: <span style='color: green;'>```QRawFont::QRawFont```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::raw_font::RawFont```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRawFont::QRawFont()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::qt_core::byte_array::ByteArray, ::libc::c_double)) -> ::raw_font::RawFont```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRawFont::QRawFont(const QByteArray& fontData, double pixelSize)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::qt_core::byte_array::ByteArray, ::libc::c_double, &::font::HintingPreference)) -> ::raw_font::RawFont```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRawFont::QRawFont(const QByteArray& fontData, double pixelSize, QFont::HintingPreference hintingPreference = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::raw_font::RawFont) -> ::raw_font::RawFont```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRawFont::QRawFont(const QRawFont& other)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((&::qt_core::string::String, ::libc::c_double)) -> ::raw_font::RawFont```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRawFont::QRawFont(const QString& fileName, double pixelSize)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new((&::qt_core::string::String, ::libc::c_double, &::font::HintingPreference)) -> ::raw_font::RawFont```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRawFont::QRawFont(const QString& fileName, double pixelSize, QFont::HintingPreference hintingPreference = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::raw_font::RawFont
    where Args: overloading::RawFontNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QRawFont& QRawFont::operator=(const QRawFont& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::raw_font::RawFont) -> &'l0 mut ::raw_font::RawFont {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QRawFont_operator_assign(self as *mut ::raw_font::RawFont,
                                               other as *const ::raw_font::RawFont)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QRawFont::operator==(const QRawFont& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::raw_font::RawFont) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QRawFont_operator_eq(self as *const ::raw_font::RawFont,
                                           other as *const ::raw_font::RawFont)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QRawFont::operator!=(const QRawFont& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::raw_font::RawFont) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QRawFont_operator_neq(self as *const ::raw_font::RawFont,
                                            other as *const ::raw_font::RawFont)
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath QRawFont::pathForGlyph(quint32 glyphIndex) const```</span>
  ///
  ///
  pub fn path_for_glyph(&self, glyph_index: u32) -> ::painter_path::PainterPath {
    {
      let mut object: ::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QRawFont_pathForGlyph_to_output(self as *const ::raw_font::RawFont, glyph_index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QRawFont::pixelSize() const```</span>
  ///
  ///
  pub fn pixel_size(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QRawFont_pixelSize(self as *const ::raw_font::RawFont) }
  }

  /// C++ method: <span style='color: green;'>```void QRawFont::setPixelSize(double pixelSize)```</span>
  ///
  ///
  pub fn set_pixel_size(&mut self, pixel_size: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QRawFont_setPixelSize(self as *mut ::raw_font::RawFont, pixel_size) }
  }

  /// C++ method: <span style='color: green;'>```QString QRawFont::styleName() const```</span>
  ///
  ///
  pub fn style_name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QRawFont_styleName_to_output(self as *const ::raw_font::RawFont, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem> QRawFont::supportedWritingSystems() const```</span>
  ///
  ///
  pub fn supported_writing_systems(&self) -> ::list::ListFontDatabaseWritingSystem {
    {
      let mut object: ::list::ListFontDatabaseWritingSystem =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QRawFont_supportedWritingSystems_to_output(self as *const ::raw_font::RawFont, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRawFont::supportsCharacter```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn supports_character(&self, &::qt_core::char::Char) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QRawFont::supportsCharacter(QChar character) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn supports_character(&self, ::libc::c_uint) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QRawFont::supportsCharacter(unsigned int ucs4) const```</span>
  ///
  ///
  pub fn supports_character<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::RawFontSupportsCharacterArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QRawFont::swap(QRawFont& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::raw_font::RawFont) {
    unsafe {
      ::ffi::qt_gui_c_QRawFont_swap(self as *mut ::raw_font::RawFont,
                                    other as *mut ::raw_font::RawFont)
    }
  }

  /// C++ method: <span style='color: green;'>```double QRawFont::underlinePosition() const```</span>
  ///
  ///
  pub fn underline_position(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QRawFont_underlinePosition(self as *const ::raw_font::RawFont) }
  }

  /// C++ method: <span style='color: green;'>```double QRawFont::unitsPerEm() const```</span>
  ///
  ///
  pub fn units_per_em(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QRawFont_unitsPerEm(self as *const ::raw_font::RawFont) }
  }

  /// C++ method: <span style='color: green;'>```int QRawFont::weight() const```</span>
  ///
  ///
  pub fn weight(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QRawFont_weight(self as *const ::raw_font::RawFont) }
  }

  /// C++ method: <span style='color: green;'>```double QRawFont::xHeight() const```</span>
  ///
  ///
  pub fn x_height(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QRawFont_xHeight(self as *const ::raw_font::RawFont) }
  }
}

impl Drop for ::raw_font::RawFont {
  /// C++ method: <span style='color: green;'>```[destructor] void QRawFont::~QRawFont()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QRawFont_destructor(self as *mut ::raw_font::RawFont) }
  }
}

/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash(&::raw_font::RawFont) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QRawFont& font)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash((&::raw_font::RawFont, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QRawFont& font, unsigned int seed = ?)```</span>
///
///
pub fn hash<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::HashArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```void swap(QRawFont& value1, QRawFont& value2)```</span>
///
///
pub fn swap(value1: &mut ::raw_font::RawFont, value2: &mut ::raw_font::RawFont) {
  unsafe {
    ::ffi::qt_gui_c_QRawFont_G_swap(value1 as *mut ::raw_font::RawFont,
                                    value2 as *mut ::raw_font::RawFont)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [RawFont::advances_for_glyph_indexes](../struct.RawFont.html#method.advances_for_glyph_indexes) method.
  pub trait RawFontAdvancesForGlyphIndexesArgs<'largs> {
    fn exec(self, original_self: &'largs ::raw_font::RawFont) -> ::qt_core::vector::VectorPointF;
  }
  impl<'largs> RawFontAdvancesForGlyphIndexesArgs<'largs> for &'largs ::vector::VectorU32 {
    fn exec(self, original_self: &'largs ::raw_font::RawFont) -> ::qt_core::vector::VectorPointF {
      let glyph_indexes = self;
      {
        let mut object: ::qt_core::vector::VectorPointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QRawFont_advancesForGlyphIndexes_to_output_glyphIndexes(original_self as *const ::raw_font::RawFont, glyph_indexes as *const ::vector::VectorU32, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RawFontAdvancesForGlyphIndexesArgs<'largs>
    for (&'largs ::vector::VectorU32, ::qt_core::flags::Flags<::raw_font::LayoutFlag>) {
    fn exec(self, original_self: &'largs ::raw_font::RawFont) -> ::qt_core::vector::VectorPointF {
      let glyph_indexes = self.0;
      let layout_flags = self.1;
      {
        let mut object: ::qt_core::vector::VectorPointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QRawFont_advancesForGlyphIndexes_to_output_glyphIndexes_layoutFlags(original_self as *const ::raw_font::RawFont, glyph_indexes as *const ::vector::VectorU32, layout_flags.to_int() as ::libc::c_uint, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [RawFont::advances_for_glyph_indexes_unsafe](../struct.RawFont.html#method.advances_for_glyph_indexes_unsafe) method.
  pub trait RawFontAdvancesForGlyphIndexesUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::raw_font::RawFont) -> bool;
  }
  impl<'largs> RawFontAdvancesForGlyphIndexesUnsafeArgs<'largs>
    for (*const u32, *mut ::qt_core::point_f::PointF, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::raw_font::RawFont) -> bool {
      let glyph_indexes = self.0;
      let advances = self.1;
      let num_glyphs = self.2;
      ::ffi::qt_gui_c_QRawFont_advancesForGlyphIndexes_glyphIndexes_advances_numGlyphs(original_self as *const ::raw_font::RawFont, glyph_indexes, advances, num_glyphs)
    }
  }
  impl<'largs> RawFontAdvancesForGlyphIndexesUnsafeArgs<'largs> for (*const u32,*mut ::qt_core::point_f::PointF,::libc::c_int,::qt_core::flags::Flags<::raw_font::LayoutFlag>) {

  unsafe fn exec(self, original_self: &'largs ::raw_font::RawFont) -> bool {
    let glyph_indexes = self.0;
let advances = self.1;
let num_glyphs = self.2;
let layout_flags = self.3;
    ::ffi::qt_gui_c_QRawFont_advancesForGlyphIndexes_glyphIndexes_advances_numGlyphs_layoutFlags(original_self as *const ::raw_font::RawFont, glyph_indexes, advances, num_glyphs, layout_flags.to_int() as ::libc::c_uint)
  }
}
  /// This trait represents a set of arguments accepted by [RawFont::alpha_map_for_glyph](../struct.RawFont.html#method.alpha_map_for_glyph) method.
  pub trait RawFontAlphaMapForGlyphArgs<'largs> {
    fn exec(self, original_self: &'largs ::raw_font::RawFont) -> ::cpp_utils::CppBox<::image::Image>;
  }
  impl<'largs> RawFontAlphaMapForGlyphArgs<'largs> for u32 {
    fn exec(self, original_self: &'largs ::raw_font::RawFont) -> ::cpp_utils::CppBox<::image::Image> {
      let glyph_index = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QRawFont_alphaMapForGlyph_as_ptr_glyphIndex(original_self as *const ::raw_font::RawFont,
                                                                      glyph_index)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> RawFontAlphaMapForGlyphArgs<'largs> for (u32, ::raw_font::AntialiasingType) {
    fn exec(self, original_self: &'largs ::raw_font::RawFont) -> ::cpp_utils::CppBox<::image::Image> {
      let glyph_index = self.0;
      let antialiasing_type = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QRawFont_alphaMapForGlyph_as_ptr_glyphIndex_antialiasingType(original_self as *const ::raw_font::RawFont, glyph_index, antialiasing_type) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> RawFontAlphaMapForGlyphArgs<'largs>
    for (u32, ::raw_font::AntialiasingType, &'largs ::transform::Transform) {
    fn exec(self, original_self: &'largs ::raw_font::RawFont) -> ::cpp_utils::CppBox<::image::Image> {
      let glyph_index = self.0;
      let antialiasing_type = self.1;
      let transform = self.2;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QRawFont_alphaMapForGlyph_as_ptr_glyphIndex_antialiasingType_transform(original_self as *const ::raw_font::RawFont, glyph_index, antialiasing_type, transform as *const ::transform::Transform) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [RawFont::from_font](../struct.RawFont.html#method.from_font) method.
  pub trait RawFontFromFontArgs {
    fn exec(self) -> ::raw_font::RawFont;
  }
  impl<'a> RawFontFromFontArgs for &'a ::font::Font {
    fn exec(self) -> ::raw_font::RawFont {
      let font = self;
      {
        let mut object: ::raw_font::RawFont =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QRawFont_fromFont_to_output_font(font as *const ::font::Font, &mut object);
        }
        object
      }
    }
  }
  impl<'a> RawFontFromFontArgs for (&'a ::font::Font, &'a ::font_database::WritingSystem) {
    fn exec(self) -> ::raw_font::RawFont {
      let font = self.0;
      let writing_system = self.1;
      {
        let mut object: ::raw_font::RawFont =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QRawFont_fromFont_to_output_font_writingSystem(font as *const ::font::Font, writing_system as *const ::font_database::WritingSystem, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [RawFont::new](../struct.RawFont.html#method.new) method.
  pub trait RawFontNewArgs {
    fn exec(self) -> ::raw_font::RawFont;
  }
  impl<'a> RawFontNewArgs for (&'a ::qt_core::string::String, ::libc::c_double) {
    fn exec(self) -> ::raw_font::RawFont {
      let file_name = self.0;
      let pixel_size = self.1;
      {
        let mut object: ::raw_font::RawFont =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QRawFont_constructor_fileName_pixelSize(file_name as *const ::qt_core::string::String,
                                                                  pixel_size,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'a> RawFontNewArgs for (&'a ::qt_core::string::String, ::libc::c_double, &'a ::font::HintingPreference) {
    fn exec(self) -> ::raw_font::RawFont {
      let file_name = self.0;
      let pixel_size = self.1;
      let hinting_preference = self.2;
      {
        let mut object: ::raw_font::RawFont =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QRawFont_constructor_fileName_pixelSize_hintingPreference(file_name as *const ::qt_core::string::String, pixel_size, hinting_preference as *const ::font::HintingPreference, &mut object);
        }
        object
      }
    }
  }
  impl<'a> RawFontNewArgs for (&'a ::qt_core::byte_array::ByteArray, ::libc::c_double) {
    fn exec(self) -> ::raw_font::RawFont {
      let font_data = self.0;
      let pixel_size = self.1;
      {
        let mut object: ::raw_font::RawFont =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QRawFont_constructor_fontData_pixelSize(font_data as *const ::qt_core::byte_array::ByteArray, pixel_size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> RawFontNewArgs for (&'a ::qt_core::byte_array::ByteArray, ::libc::c_double, &'a ::font::HintingPreference) {
    fn exec(self) -> ::raw_font::RawFont {
      let font_data = self.0;
      let pixel_size = self.1;
      let hinting_preference = self.2;
      {
        let mut object: ::raw_font::RawFont =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QRawFont_constructor_fontData_pixelSize_hintingPreference(font_data as *const ::qt_core::byte_array::ByteArray, pixel_size, hinting_preference as *const ::font::HintingPreference, &mut object);
        }
        object
      }
    }
  }
  impl RawFontNewArgs for () {
    fn exec(self) -> ::raw_font::RawFont {

      {
        let mut object: ::raw_font::RawFont =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QRawFont_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> RawFontNewArgs for &'a ::raw_font::RawFont {
    fn exec(self) -> ::raw_font::RawFont {
      let other = self;
      {
        let mut object: ::raw_font::RawFont =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QRawFont_constructor_other(other as *const ::raw_font::RawFont, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [RawFont::supports_character](../struct.RawFont.html#method.supports_character) method.
  pub trait RawFontSupportsCharacterArgs<'largs> {
    fn exec(self, original_self: &'largs ::raw_font::RawFont) -> bool;
  }
  impl<'largs> RawFontSupportsCharacterArgs<'largs> for &'largs ::qt_core::char::Char {
    fn exec(self, original_self: &'largs ::raw_font::RawFont) -> bool {
      let character = self;
      unsafe {
        ::ffi::qt_gui_c_QRawFont_supportsCharacter_character(original_self as *const ::raw_font::RawFont,
                                                             character as *const ::qt_core::char::Char)
      }
    }
  }
  impl<'largs> RawFontSupportsCharacterArgs<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs ::raw_font::RawFont) -> bool {
      let ucs4 = self;
      unsafe { ::ffi::qt_gui_c_QRawFont_supportsCharacter_ucs4(original_self as *const ::raw_font::RawFont, ucs4) }
    }
  }
  /// This trait represents a set of arguments accepted by [hash](../fn.hash.html) method.
  pub trait HashArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> HashArgs for &'a ::raw_font::RawFont {
    fn exec(self) -> ::libc::c_uint {
      let font = self;
      unsafe { ::ffi::qt_gui_c_QRawFont_G_qHash_font(font as *const ::raw_font::RawFont) }
    }
  }
  impl<'a> HashArgs for (&'a ::raw_font::RawFont, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let font = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_gui_c_QRawFont_G_qHash_font_seed(font as *const ::raw_font::RawFont, seed) }
    }
  }
}
