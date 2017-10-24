/// C++ type: <span style='color: green;'>```QFontDatabase```</span>
#[repr(C)]
pub struct FontDatabase(u8);

impl FontDatabase {
  /// C++ method: <span style='color: green;'>```static int QFontDatabase::addApplicationFont(const QString& fileName)```</span>
  ///
  ///
  pub fn add_application_font(file_name: &::qt_core::string::String) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontDatabase_addApplicationFont(file_name as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```static int QFontDatabase::addApplicationFontFromData(const QByteArray& fontData)```</span>
  ///
  ///
  pub fn add_application_font_from_data(font_data: &::qt_core::byte_array::ByteArray) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QFontDatabase_addApplicationFontFromData(font_data as *const ::qt_core::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```static QStringList QFontDatabase::applicationFontFamilies(int id)```</span>
  ///
  ///
  pub fn application_font_families(id: ::libc::c_int) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFontDatabase_applicationFontFamilies_to_output(id, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFontDatabase::bold(const QString& family, const QString& style) const```</span>
  ///
  ///
  pub fn bold(&self, family: &::qt_core::string::String, style: &::qt_core::string::String) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QFontDatabase_bold(self as *const ::font_database::FontDatabase,
                                         family as *const ::qt_core::string::String,
                                         style as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QFontDatabase::families```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn families(&self, ()) -> ::qt_core::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QFontDatabase::families() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn families(&self, ::font_database::WritingSystem) -> ::qt_core::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QFontDatabase::families(QFontDatabase::WritingSystem writingSystem = ?) const```</span>
  ///
  ///
  pub fn families<'largs, Args>(&'largs self, args: Args) -> ::qt_core::string_list::StringList
    where Args: overloading::FontDatabaseFamiliesArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QFont QFontDatabase::font(const QString& family, const QString& style, int pointSize) const```</span>
  ///
  ///
  pub fn font(&self,
              family: &::qt_core::string::String,
              style: &::qt_core::string::String,
              point_size: ::libc::c_int)
              -> ::font::Font {
    {
      let mut object: ::font::Font = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFontDatabase_font_to_output(self as *const ::font_database::FontDatabase,
                                                     family as *const ::qt_core::string::String,
                                                     style as *const ::qt_core::string::String,
                                                     point_size,
                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFontDatabase::hasFamily(const QString& family) const```</span>
  ///
  ///
  pub fn has_family(&self, family: &::qt_core::string::String) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QFontDatabase_hasFamily(self as *const ::font_database::FontDatabase,
                                              family as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QFontDatabase::isBitmapScalable```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn is_bitmap_scalable(&self, &::qt_core::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QFontDatabase::isBitmapScalable(const QString& family) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn is_bitmap_scalable(&self, (&::qt_core::string::String, &::qt_core::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QFontDatabase::isBitmapScalable(const QString& family, const QString& style = ?) const```</span>
  ///
  ///
  pub fn is_bitmap_scalable<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::FontDatabaseIsBitmapScalableArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QFontDatabase::isFixedPitch```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn is_fixed_pitch(&self, &::qt_core::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QFontDatabase::isFixedPitch(const QString& family) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn is_fixed_pitch(&self, (&::qt_core::string::String, &::qt_core::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QFontDatabase::isFixedPitch(const QString& family, const QString& style = ?) const```</span>
  ///
  ///
  pub fn is_fixed_pitch<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::FontDatabaseIsFixedPitchArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QFontDatabase::isPrivateFamily(const QString& family) const```</span>
  ///
  ///
  pub fn is_private_family(&self, family: &::qt_core::string::String) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QFontDatabase_isPrivateFamily(self as *const ::font_database::FontDatabase,
                                                    family as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QFontDatabase::isScalable```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn is_scalable(&self, &::qt_core::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QFontDatabase::isScalable(const QString& family) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn is_scalable(&self, (&::qt_core::string::String, &::qt_core::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QFontDatabase::isScalable(const QString& family, const QString& style = ?) const```</span>
  ///
  ///
  pub fn is_scalable<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::FontDatabaseIsScalableArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QFontDatabase::isSmoothlyScalable```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn is_smoothly_scalable(&self, &::qt_core::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QFontDatabase::isSmoothlyScalable(const QString& family) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn is_smoothly_scalable(&self, (&::qt_core::string::String, &::qt_core::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QFontDatabase::isSmoothlyScalable(const QString& family, const QString& style = ?) const```</span>
  ///
  ///
  pub fn is_smoothly_scalable<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::FontDatabaseIsSmoothlyScalableArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QFontDatabase::italic(const QString& family, const QString& style) const```</span>
  ///
  ///
  pub fn italic(&self, family: &::qt_core::string::String, style: &::qt_core::string::String) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QFontDatabase_italic(self as *const ::font_database::FontDatabase,
                                           family as *const ::qt_core::string::String,
                                           style as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QFontDatabase::QFontDatabase()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::font_database::FontDatabase> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QFontDatabase_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QFontDatabase::pointSizes```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn point_sizes(&mut self, &::qt_core::string::String) -> ::qt_core::list::ListCInt```<br>
  /// C++ method: <span style='color: green;'>```QList<int> QFontDatabase::pointSizes(const QString& family)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn point_sizes(&mut self, (&::qt_core::string::String, &::qt_core::string::String)) -> ::qt_core::list::ListCInt```<br>
  /// C++ method: <span style='color: green;'>```QList<int> QFontDatabase::pointSizes(const QString& family, const QString& style = ?)```</span>
  ///
  ///
  pub fn point_sizes<'largs, Args>(&'largs mut self, args: Args) -> ::qt_core::list::ListCInt
    where Args: overloading::FontDatabasePointSizesArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static bool QFontDatabase::removeAllApplicationFonts()```</span>
  ///
  ///
  pub fn remove_all_application_fonts() -> bool {
    unsafe { ::ffi::qt_gui_c_QFontDatabase_removeAllApplicationFonts() }
  }

  /// C++ method: <span style='color: green;'>```static bool QFontDatabase::removeApplicationFont(int id)```</span>
  ///
  ///
  pub fn remove_application_font(id: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_gui_c_QFontDatabase_removeApplicationFont(id) }
  }

  /// C++ method: <span style='color: green;'>```QList<int> QFontDatabase::smoothSizes(const QString& family, const QString& style)```</span>
  ///
  ///
  pub fn smooth_sizes(&mut self,
                      family: &::qt_core::string::String,
                      style: &::qt_core::string::String)
                      -> ::qt_core::list::ListCInt {
    {
      let mut object: ::qt_core::list::ListCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFontDatabase_smoothSizes_to_output(self as *mut ::font_database::FontDatabase,
                                                            family as *const ::qt_core::string::String,
                                                            style as *const ::qt_core::string::String,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QList<int> QFontDatabase::standardSizes()```</span>
  ///
  ///
  pub fn standard_sizes() -> ::qt_core::list::ListCInt {
    {
      let mut object: ::qt_core::list::ListCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFontDatabase_standardSizes_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFontDatabase::styleString```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn style_string(&mut self, &::font::Font) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QFontDatabase::styleString(const QFont& font)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn style_string(&mut self, &::font_info::FontInfo) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QFontDatabase::styleString(const QFontInfo& fontInfo)```</span>
  ///
  ///
  pub fn style_string<'largs, Args>(&'largs mut self, args: Args) -> ::qt_core::string::String
    where Args: overloading::FontDatabaseStyleStringArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringList QFontDatabase::styles(const QString& family) const```</span>
  ///
  ///
  pub fn styles(&self, family: &::qt_core::string::String) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFontDatabase_styles_to_output(self as *const ::font_database::FontDatabase,
                                                       family as *const ::qt_core::string::String,
                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static bool QFontDatabase::supportsThreadedFontRendering()```</span>
  ///
  ///
  pub fn supports_threaded_font_rendering() -> bool {
    unsafe { ::ffi::qt_gui_c_QFontDatabase_supportsThreadedFontRendering() }
  }

  /// C++ method: <span style='color: green;'>```static QFont QFontDatabase::systemFont(QFontDatabase::SystemFont type)```</span>
  ///
  ///
  pub fn system_font(type_: ::font_database::SystemFont) -> ::font::Font {
    {
      let mut object: ::font::Font = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFontDatabase_systemFont_to_output(type_, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QFontDatabase::weight(const QString& family, const QString& style) const```</span>
  ///
  ///
  pub fn weight(&self, family: &::qt_core::string::String, style: &::qt_core::string::String) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QFontDatabase_weight(self as *const ::font_database::FontDatabase,
                                           family as *const ::qt_core::string::String,
                                           style as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFontDatabase::writingSystemName(QFontDatabase::WritingSystem writingSystem)```</span>
  ///
  ///
  pub fn writing_system_name(writing_system: ::font_database::WritingSystem) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFontDatabase_writingSystemName_to_output(writing_system, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFontDatabase::writingSystemSample(QFontDatabase::WritingSystem writingSystem)```</span>
  ///
  ///
  pub fn writing_system_sample(writing_system: ::font_database::WritingSystem) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFontDatabase_writingSystemSample_to_output(writing_system, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFontDatabase::writingSystems```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn writing_systems(&self, ()) -> ::list::ListFontDatabaseWritingSystem```<br>
  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem> QFontDatabase::writingSystems() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn writing_systems(&self, &::qt_core::string::String) -> ::list::ListFontDatabaseWritingSystem```<br>
  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem> QFontDatabase::writingSystems(const QString& family) const```</span>
  ///
  ///
  pub fn writing_systems<'largs, Args>(&'largs self, args: Args) -> ::list::ListFontDatabaseWritingSystem
    where Args: overloading::FontDatabaseWritingSystemsArgs<'largs>
  {
    args.exec(self)
  }
}

impl ::cpp_utils::CppDeletable for ::font_database::FontDatabase {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QFontDatabase_delete
  }
}

/// C++ type: <span style='color: green;'>```QFontDatabase::SystemFont```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SystemFont {
  /// C++ enum variant: <span style='color: green;'>```GeneralFont = 0```</span>
  General = 0,
  /// C++ enum variant: <span style='color: green;'>```FixedFont = 1```</span>
  Fixed = 1,
  /// C++ enum variant: <span style='color: green;'>```TitleFont = 2```</span>
  Title = 2,
  /// C++ enum variant: <span style='color: green;'>```SmallestReadableFont = 3```</span>
  SmallestReadable = 3,
}

/// C++ type: <span style='color: green;'>```QFontDatabase::WritingSystem```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum WritingSystem {
  /// C++ enum variant: <span style='color: green;'>```Any = 0```</span>
  Any = 0,
  /// C++ enum variant: <span style='color: green;'>```Latin = 1```</span>
  Latin = 1,
  /// C++ enum variant: <span style='color: green;'>```Greek = 2```</span>
  Greek = 2,
  /// C++ enum variant: <span style='color: green;'>```Cyrillic = 3```</span>
  Cyrillic = 3,
  /// C++ enum variant: <span style='color: green;'>```Armenian = 4```</span>
  Armenian = 4,
  /// C++ enum variant: <span style='color: green;'>```Hebrew = 5```</span>
  Hebrew = 5,
  /// C++ enum variant: <span style='color: green;'>```Arabic = 6```</span>
  Arabic = 6,
  /// C++ enum variant: <span style='color: green;'>```Syriac = 7```</span>
  Syriac = 7,
  /// C++ enum variant: <span style='color: green;'>```Thaana = 8```</span>
  Thaana = 8,
  /// C++ enum variant: <span style='color: green;'>```Devanagari = 9```</span>
  Devanagari = 9,
  /// C++ enum variant: <span style='color: green;'>```Bengali = 10```</span>
  Bengali = 10,
  /// C++ enum variant: <span style='color: green;'>```Gurmukhi = 11```</span>
  Gurmukhi = 11,
  /// C++ enum variant: <span style='color: green;'>```Gujarati = 12```</span>
  Gujarati = 12,
  /// C++ enum variant: <span style='color: green;'>```Oriya = 13```</span>
  Oriya = 13,
  /// C++ enum variant: <span style='color: green;'>```Tamil = 14```</span>
  Tamil = 14,
  /// C++ enum variant: <span style='color: green;'>```Telugu = 15```</span>
  Telugu = 15,
  /// C++ enum variant: <span style='color: green;'>```Kannada = 16```</span>
  Kannada = 16,
  /// C++ enum variant: <span style='color: green;'>```Malayalam = 17```</span>
  Malayalam = 17,
  /// C++ enum variant: <span style='color: green;'>```Sinhala = 18```</span>
  Sinhala = 18,
  /// C++ enum variant: <span style='color: green;'>```Thai = 19```</span>
  Thai = 19,
  /// C++ enum variant: <span style='color: green;'>```Lao = 20```</span>
  Lao = 20,
  /// C++ enum variant: <span style='color: green;'>```Tibetan = 21```</span>
  Tibetan = 21,
  /// C++ enum variant: <span style='color: green;'>```Myanmar = 22```</span>
  Myanmar = 22,
  /// C++ enum variant: <span style='color: green;'>```Georgian = 23```</span>
  Georgian = 23,
  /// C++ enum variant: <span style='color: green;'>```Khmer = 24```</span>
  Khmer = 24,
  /// C++ enum variant: <span style='color: green;'>```SimplifiedChinese = 25```</span>
  SimplifiedChinese = 25,
  /// C++ enum variant: <span style='color: green;'>```TraditionalChinese = 26```</span>
  TraditionalChinese = 26,
  /// C++ enum variant: <span style='color: green;'>```Japanese = 27```</span>
  Japanese = 27,
  /// C++ enum variant: <span style='color: green;'>```Korean = 28```</span>
  Korean = 28,
  /// C++ enum variant: <span style='color: green;'>```Vietnamese = 29```</span>
  Vietnamese = 29,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Symbol = 30```</span>
  /// - <span style='color: green;'>```Other = 30```</span>
  ///
  Symbol = 30,
  /// C++ enum variant: <span style='color: green;'>```Ogham = 31```</span>
  Ogham = 31,
  /// C++ enum variant: <span style='color: green;'>```Runic = 32```</span>
  Runic = 32,
  /// C++ enum variant: <span style='color: green;'>```Nko = 33```</span>
  Nko = 33,
  /// C++ enum variant: <span style='color: green;'>```WritingSystemsCount = 34```</span>
  WritingSystemsCount = 34,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [FontDatabase::families](../struct.FontDatabase.html#method.families) method.
  pub trait FontDatabaseFamiliesArgs<'largs> {
    fn exec(self, original_self: &'largs ::font_database::FontDatabase) -> ::qt_core::string_list::StringList;
  }
  impl<'largs> FontDatabaseFamiliesArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::font_database::FontDatabase) -> ::qt_core::string_list::StringList {

      {
        let mut object: ::qt_core::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontDatabase_families_to_output_no_args(original_self as *const ::font_database::FontDatabase, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FontDatabaseFamiliesArgs<'largs> for ::font_database::WritingSystem {
    fn exec(self, original_self: &'largs ::font_database::FontDatabase) -> ::qt_core::string_list::StringList {
      let writing_system = self;
      {
        let mut object: ::qt_core::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontDatabase_families_to_output_writingSystem(original_self as *const ::font_database::FontDatabase, writing_system, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FontDatabase::is_bitmap_scalable](../struct.FontDatabase.html#method.is_bitmap_scalable) method.
  pub trait FontDatabaseIsBitmapScalableArgs<'largs> {
    fn exec(self, original_self: &'largs ::font_database::FontDatabase) -> bool;
  }
  impl<'largs> FontDatabaseIsBitmapScalableArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs ::font_database::FontDatabase) -> bool {
      let family = self;
      unsafe {
        ::ffi::qt_gui_c_QFontDatabase_isBitmapScalable_family(original_self as *const ::font_database::FontDatabase,
                                                              family as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> FontDatabaseIsBitmapScalableArgs<'largs>
    for (&'largs ::qt_core::string::String, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs ::font_database::FontDatabase) -> bool {
      let family = self.0;
      let style = self.1;
      unsafe { ::ffi::qt_gui_c_QFontDatabase_isBitmapScalable_family_style(original_self as *const ::font_database::FontDatabase, family as *const ::qt_core::string::String, style as *const ::qt_core::string::String) }
    }
  }
  /// This trait represents a set of arguments accepted by [FontDatabase::is_fixed_pitch](../struct.FontDatabase.html#method.is_fixed_pitch) method.
  pub trait FontDatabaseIsFixedPitchArgs<'largs> {
    fn exec(self, original_self: &'largs ::font_database::FontDatabase) -> bool;
  }
  impl<'largs> FontDatabaseIsFixedPitchArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs ::font_database::FontDatabase) -> bool {
      let family = self;
      unsafe {
        ::ffi::qt_gui_c_QFontDatabase_isFixedPitch_family(original_self as *const ::font_database::FontDatabase,
                                                          family as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> FontDatabaseIsFixedPitchArgs<'largs>
    for (&'largs ::qt_core::string::String, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs ::font_database::FontDatabase) -> bool {
      let family = self.0;
      let style = self.1;
      unsafe {
        ::ffi::qt_gui_c_QFontDatabase_isFixedPitch_family_style(original_self as *const ::font_database::FontDatabase,
                                                                family as *const ::qt_core::string::String,
                                                                style as *const ::qt_core::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FontDatabase::is_scalable](../struct.FontDatabase.html#method.is_scalable) method.
  pub trait FontDatabaseIsScalableArgs<'largs> {
    fn exec(self, original_self: &'largs ::font_database::FontDatabase) -> bool;
  }
  impl<'largs> FontDatabaseIsScalableArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs ::font_database::FontDatabase) -> bool {
      let family = self;
      unsafe {
        ::ffi::qt_gui_c_QFontDatabase_isScalable_family(original_self as *const ::font_database::FontDatabase,
                                                        family as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> FontDatabaseIsScalableArgs<'largs>
    for (&'largs ::qt_core::string::String, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs ::font_database::FontDatabase) -> bool {
      let family = self.0;
      let style = self.1;
      unsafe {
        ::ffi::qt_gui_c_QFontDatabase_isScalable_family_style(original_self as *const ::font_database::FontDatabase,
                                                              family as *const ::qt_core::string::String,
                                                              style as *const ::qt_core::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FontDatabase::is_smoothly_scalable](../struct.FontDatabase.html#method.is_smoothly_scalable) method.
  pub trait FontDatabaseIsSmoothlyScalableArgs<'largs> {
    fn exec(self, original_self: &'largs ::font_database::FontDatabase) -> bool;
  }
  impl<'largs> FontDatabaseIsSmoothlyScalableArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs ::font_database::FontDatabase) -> bool {
      let family = self;
      unsafe {
        ::ffi::qt_gui_c_QFontDatabase_isSmoothlyScalable_family(original_self as *const ::font_database::FontDatabase,
                                                                family as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> FontDatabaseIsSmoothlyScalableArgs<'largs>
    for (&'largs ::qt_core::string::String, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs ::font_database::FontDatabase) -> bool {
      let family = self.0;
      let style = self.1;
      unsafe { ::ffi::qt_gui_c_QFontDatabase_isSmoothlyScalable_family_style(original_self as *const ::font_database::FontDatabase, family as *const ::qt_core::string::String, style as *const ::qt_core::string::String) }
    }
  }
  /// This trait represents a set of arguments accepted by [FontDatabase::point_sizes](../struct.FontDatabase.html#method.point_sizes) method.
  pub trait FontDatabasePointSizesArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::font_database::FontDatabase) -> ::qt_core::list::ListCInt;
  }
  impl<'largs> FontDatabasePointSizesArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::font_database::FontDatabase) -> ::qt_core::list::ListCInt {
      let family = self;
      {
        let mut object: ::qt_core::list::ListCInt =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontDatabase_pointSizes_to_output_family(original_self as *mut ::font_database::FontDatabase, family as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FontDatabasePointSizesArgs<'largs>
    for (&'largs ::qt_core::string::String, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::font_database::FontDatabase) -> ::qt_core::list::ListCInt {
      let family = self.0;
      let style = self.1;
      {
        let mut object: ::qt_core::list::ListCInt =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontDatabase_pointSizes_to_output_family_style(original_self as *mut ::font_database::FontDatabase, family as *const ::qt_core::string::String, style as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FontDatabase::style_string](../struct.FontDatabase.html#method.style_string) method.
  pub trait FontDatabaseStyleStringArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::font_database::FontDatabase) -> ::qt_core::string::String;
  }
  impl<'largs> FontDatabaseStyleStringArgs<'largs> for &'largs ::font::Font {
    fn exec(self, original_self: &'largs mut ::font_database::FontDatabase) -> ::qt_core::string::String {
      let font = self;
      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontDatabase_styleString_to_output_font(original_self as *mut ::font_database::FontDatabase, font as *const ::font::Font, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FontDatabaseStyleStringArgs<'largs> for &'largs ::font_info::FontInfo {
    fn exec(self, original_self: &'largs mut ::font_database::FontDatabase) -> ::qt_core::string::String {
      let font_info = self;
      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontDatabase_styleString_to_output_fontInfo(original_self as *mut ::font_database::FontDatabase, font_info as *const ::font_info::FontInfo, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FontDatabase::writing_systems](../struct.FontDatabase.html#method.writing_systems) method.
  pub trait FontDatabaseWritingSystemsArgs<'largs> {
    fn exec(self, original_self: &'largs ::font_database::FontDatabase) -> ::list::ListFontDatabaseWritingSystem;
  }
  impl<'largs> FontDatabaseWritingSystemsArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs ::font_database::FontDatabase) -> ::list::ListFontDatabaseWritingSystem {
      let family = self;
      {
        let mut object: ::list::ListFontDatabaseWritingSystem =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontDatabase_writingSystems_to_output_family(original_self as *const ::font_database::FontDatabase, family as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FontDatabaseWritingSystemsArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::font_database::FontDatabase) -> ::list::ListFontDatabaseWritingSystem {

      {
        let mut object: ::list::ListFontDatabaseWritingSystem =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontDatabase_writingSystems_to_output_no_args(original_self as *const ::font_database::FontDatabase, &mut object);
        }
        object
      }
    }
  }
}
