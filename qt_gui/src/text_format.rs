/// C++ type: <span style='color: green;'>```QTextFormat::FormatType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FormatType {
  /// C++ enum variant: <span style='color: green;'>```InvalidFormat = -1```</span>
  Invalid = -1,
  /// C++ enum variant: <span style='color: green;'>```BlockFormat = 1```</span>
  Block = 1,
  /// C++ enum variant: <span style='color: green;'>```CharFormat = 2```</span>
  Char = 2,
  /// C++ enum variant: <span style='color: green;'>```ListFormat = 3```</span>
  List = 3,
  /// C++ enum variant: <span style='color: green;'>```TableFormat = 4```</span>
  Table = 4,
  /// C++ enum variant: <span style='color: green;'>```FrameFormat = 5```</span>
  Frame = 5,
  /// C++ enum variant: <span style='color: green;'>```UserFormat = 100```</span>
  User = 100,
}

/// C++ type: <span style='color: green;'>```QTextFormat::ObjectTypes```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ObjectTypes {
  /// C++ enum variant: <span style='color: green;'>```NoObject = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```ImageObject = 1```</span>
  Image = 1,
  /// C++ enum variant: <span style='color: green;'>```TableObject = 2```</span>
  Table = 2,
  /// C++ enum variant: <span style='color: green;'>```TableCellObject = 3```</span>
  TableCell = 3,
  /// C++ enum variant: <span style='color: green;'>```UserObject = 4096```</span>
  User = 4096,
}

/// C++ type: <span style='color: green;'>```QTextFormat::PageBreakFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PageBreakFlag {
  /// C++ enum variant: <span style='color: green;'>```PageBreak_Auto = 0```</span>
  Auto = 0,
  /// C++ enum variant: <span style='color: green;'>```PageBreak_AlwaysBefore = 1```</span>
  AlwaysBefore = 1,
  /// C++ enum variant: <span style='color: green;'>```PageBreak_AlwaysAfter = 16```</span>
  AlwaysAfter = 16,
}

impl ::qt_core::flags::FlaggableEnum for PageBreakFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "PageBreakFlag"
  }
}

/// C++ type: <span style='color: green;'>```QTextFormat::Property```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Property {
  /// C++ enum variant: <span style='color: green;'>```ObjectIndex = 0```</span>
  ObjectIndex = 0,
  /// C++ enum variant: <span style='color: green;'>```CssFloat = 2048```</span>
  CssFloat = 2048,
  /// C++ enum variant: <span style='color: green;'>```LayoutDirection = 2049```</span>
  LayoutDirection = 2049,
  /// C++ enum variant: <span style='color: green;'>```OutlinePen = 2064```</span>
  OutlinePen = 2064,
  /// C++ enum variant: <span style='color: green;'>```BackgroundBrush = 2080```</span>
  BackgroundBrush = 2080,
  /// C++ enum variant: <span style='color: green;'>```ForegroundBrush = 2081```</span>
  ForegroundBrush = 2081,
  /// C++ enum variant: <span style='color: green;'>```BackgroundImageUrl = 2083```</span>
  BackgroundImageUrl = 2083,
  /// C++ enum variant: <span style='color: green;'>```BlockAlignment = 4112```</span>
  BlockAlignment = 4112,
  /// C++ enum variant: <span style='color: green;'>```BlockTopMargin = 4144```</span>
  BlockTopMargin = 4144,
  /// C++ enum variant: <span style='color: green;'>```BlockBottomMargin = 4145```</span>
  BlockBottomMargin = 4145,
  /// C++ enum variant: <span style='color: green;'>```BlockLeftMargin = 4146```</span>
  BlockLeftMargin = 4146,
  /// C++ enum variant: <span style='color: green;'>```BlockRightMargin = 4147```</span>
  BlockRightMargin = 4147,
  /// C++ enum variant: <span style='color: green;'>```TextIndent = 4148```</span>
  TextIndent = 4148,
  /// C++ enum variant: <span style='color: green;'>```TabPositions = 4149```</span>
  TabPositions = 4149,
  /// C++ enum variant: <span style='color: green;'>```BlockIndent = 4160```</span>
  BlockIndent = 4160,
  /// C++ enum variant: <span style='color: green;'>```LineHeight = 4168```</span>
  LineHeight = 4168,
  /// C++ enum variant: <span style='color: green;'>```LineHeightType = 4169```</span>
  LineHeightType = 4169,
  /// C++ enum variant: <span style='color: green;'>```BlockNonBreakableLines = 4176```</span>
  BlockNonBreakableLines = 4176,
  /// C++ enum variant: <span style='color: green;'>```BlockTrailingHorizontalRulerWidth = 4192```</span>
  BlockTrailingHorizontalRulerWidth = 4192,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```FirstFontProperty = 8160```</span>
  /// - <span style='color: green;'>```FontCapitalization = 8160```</span>
  ///
  FirstFontProperty = 8160,
  /// C++ enum variant: <span style='color: green;'>```FontLetterSpacing = 8161```</span>
  FontLetterSpacing = 8161,
  /// C++ enum variant: <span style='color: green;'>```FontWordSpacing = 8162```</span>
  FontWordSpacing = 8162,
  /// C++ enum variant: <span style='color: green;'>```FontStyleHint = 8163```</span>
  FontStyleHint = 8163,
  /// C++ enum variant: <span style='color: green;'>```FontStyleStrategy = 8164```</span>
  FontStyleStrategy = 8164,
  /// C++ enum variant: <span style='color: green;'>```FontKerning = 8165```</span>
  FontKerning = 8165,
  /// C++ enum variant: <span style='color: green;'>```FontHintingPreference = 8166```</span>
  FontHintingPreference = 8166,
  /// C++ enum variant: <span style='color: green;'>```FontFamily = 8192```</span>
  FontFamily = 8192,
  /// C++ enum variant: <span style='color: green;'>```FontPointSize = 8193```</span>
  FontPointSize = 8193,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```FontSizeAdjustment = 8194```</span>
  /// - <span style='color: green;'>```FontSizeIncrement = 8194```</span>
  ///
  FontSizeAdjustment = 8194,
  /// C++ enum variant: <span style='color: green;'>```FontWeight = 8195```</span>
  FontWeight = 8195,
  /// C++ enum variant: <span style='color: green;'>```FontItalic = 8196```</span>
  FontItalic = 8196,
  /// C++ enum variant: <span style='color: green;'>```FontUnderline = 8197```</span>
  FontUnderline = 8197,
  /// C++ enum variant: <span style='color: green;'>```FontOverline = 8198```</span>
  FontOverline = 8198,
  /// C++ enum variant: <span style='color: green;'>```FontStrikeOut = 8199```</span>
  FontStrikeOut = 8199,
  /// C++ enum variant: <span style='color: green;'>```FontFixedPitch = 8200```</span>
  FontFixedPitch = 8200,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```FontPixelSize = 8201```</span>
  /// - <span style='color: green;'>```LastFontProperty = 8201```</span>
  ///
  FontPixelSize = 8201,
  /// C++ enum variant: <span style='color: green;'>```TextUnderlineColor = 8208```</span>
  TextUnderlineColor = 8208,
  /// C++ enum variant: <span style='color: green;'>```TextVerticalAlignment = 8225```</span>
  TextVerticalAlignment = 8225,
  /// C++ enum variant: <span style='color: green;'>```TextOutline = 8226```</span>
  TextOutline = 8226,
  /// C++ enum variant: <span style='color: green;'>```TextUnderlineStyle = 8227```</span>
  TextUnderlineStyle = 8227,
  /// C++ enum variant: <span style='color: green;'>```TextToolTip = 8228```</span>
  TextToolTip = 8228,
  /// C++ enum variant: <span style='color: green;'>```IsAnchor = 8240```</span>
  IsAnchor = 8240,
  /// C++ enum variant: <span style='color: green;'>```AnchorHref = 8241```</span>
  AnchorHref = 8241,
  /// C++ enum variant: <span style='color: green;'>```AnchorName = 8242```</span>
  AnchorName = 8242,
  /// C++ enum variant: <span style='color: green;'>```FontLetterSpacingType = 8243```</span>
  FontLetterSpacingType = 8243,
  /// C++ enum variant: <span style='color: green;'>```FontStretch = 8244```</span>
  FontStretch = 8244,
  /// C++ enum variant: <span style='color: green;'>```ObjectType = 12032```</span>
  ObjectType = 12032,
  /// C++ enum variant: <span style='color: green;'>```ListStyle = 12288```</span>
  ListStyle = 12288,
  /// C++ enum variant: <span style='color: green;'>```ListIndent = 12289```</span>
  ListIndent = 12289,
  /// C++ enum variant: <span style='color: green;'>```ListNumberPrefix = 12290```</span>
  ListNumberPrefix = 12290,
  /// C++ enum variant: <span style='color: green;'>```ListNumberSuffix = 12291```</span>
  ListNumberSuffix = 12291,
  /// C++ enum variant: <span style='color: green;'>```FrameBorder = 16384```</span>
  FrameBorder = 16384,
  /// C++ enum variant: <span style='color: green;'>```FrameMargin = 16385```</span>
  FrameMargin = 16385,
  /// C++ enum variant: <span style='color: green;'>```FramePadding = 16386```</span>
  FramePadding = 16386,
  /// C++ enum variant: <span style='color: green;'>```FrameWidth = 16387```</span>
  FrameWidth = 16387,
  /// C++ enum variant: <span style='color: green;'>```FrameHeight = 16388```</span>
  FrameHeight = 16388,
  /// C++ enum variant: <span style='color: green;'>```FrameTopMargin = 16389```</span>
  FrameTopMargin = 16389,
  /// C++ enum variant: <span style='color: green;'>```FrameBottomMargin = 16390```</span>
  FrameBottomMargin = 16390,
  /// C++ enum variant: <span style='color: green;'>```FrameLeftMargin = 16391```</span>
  FrameLeftMargin = 16391,
  /// C++ enum variant: <span style='color: green;'>```FrameRightMargin = 16392```</span>
  FrameRightMargin = 16392,
  /// C++ enum variant: <span style='color: green;'>```FrameBorderBrush = 16393```</span>
  FrameBorderBrush = 16393,
  /// C++ enum variant: <span style='color: green;'>```FrameBorderStyle = 16400```</span>
  FrameBorderStyle = 16400,
  /// C++ enum variant: <span style='color: green;'>```TableColumns = 16640```</span>
  TableColumns = 16640,
  /// C++ enum variant: <span style='color: green;'>```TableColumnWidthConstraints = 16641```</span>
  TableColumnWidthConstraints = 16641,
  /// C++ enum variant: <span style='color: green;'>```TableCellSpacing = 16642```</span>
  TableCellSpacing = 16642,
  /// C++ enum variant: <span style='color: green;'>```TableCellPadding = 16643```</span>
  TableCellPadding = 16643,
  /// C++ enum variant: <span style='color: green;'>```TableHeaderRowCount = 16644```</span>
  TableHeaderRowCount = 16644,
  /// C++ enum variant: <span style='color: green;'>```TableCellRowSpan = 18448```</span>
  TableCellRowSpan = 18448,
  /// C++ enum variant: <span style='color: green;'>```TableCellColumnSpan = 18449```</span>
  TableCellColumnSpan = 18449,
  /// C++ enum variant: <span style='color: green;'>```TableCellTopPadding = 18450```</span>
  TableCellTopPadding = 18450,
  /// C++ enum variant: <span style='color: green;'>```TableCellBottomPadding = 18451```</span>
  TableCellBottomPadding = 18451,
  /// C++ enum variant: <span style='color: green;'>```TableCellLeftPadding = 18452```</span>
  TableCellLeftPadding = 18452,
  /// C++ enum variant: <span style='color: green;'>```TableCellRightPadding = 18453```</span>
  TableCellRightPadding = 18453,
  /// C++ enum variant: <span style='color: green;'>```ImageName = 20480```</span>
  ImageName = 20480,
  /// C++ enum variant: <span style='color: green;'>```ImageWidth = 20496```</span>
  ImageWidth = 20496,
  /// C++ enum variant: <span style='color: green;'>```ImageHeight = 20497```</span>
  ImageHeight = 20497,
  /// C++ enum variant: <span style='color: green;'>```FullWidthSelection = 24576```</span>
  FullWidthSelection = 24576,
  /// C++ enum variant: <span style='color: green;'>```PageBreakPolicy = 28672```</span>
  PageBreakPolicy = 28672,
  /// C++ enum variant: <span style='color: green;'>```UserProperty = 1048576```</span>
  UserProperty = 1048576,
}

/// C++ type: <span style='color: green;'>```QTextFormat```</span>
#[repr(C)]
pub struct TextFormat([u8; ::type_sizes::QT_GUI_TEXT_FORMAT_TEXT_FORMAT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextFormat {
  unsafe fn new_uninitialized() -> TextFormat {
    TextFormat(::std::mem::uninitialized())
  }
}

impl TextFormat {
  /// C++ method: <span style='color: green;'>```QVariant QTextFormat::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_convert_to_QVariant_to_output(self as *const ::text_format::TextFormat,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QBrush QTextFormat::background() const```</span>
  ///
  ///
  pub fn background(&self) -> ::brush::Brush {
    {
      let mut object: ::brush::Brush = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_background_to_output(self as *const ::text_format::TextFormat, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextFormat::boolProperty(int propertyId) const```</span>
  ///
  ///
  pub fn bool_property(&self, property_id: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextFormat_boolProperty(self as *const ::text_format::TextFormat, property_id) }
  }

  /// C++ method: <span style='color: green;'>```QBrush QTextFormat::brushProperty(int propertyId) const```</span>
  ///
  ///
  pub fn brush_property(&self, property_id: ::libc::c_int) -> ::brush::Brush {
    {
      let mut object: ::brush::Brush = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_brushProperty_to_output(self as *const ::text_format::TextFormat,
                                                            property_id,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextFormat::clearBackground()```</span>
  ///
  ///
  pub fn clear_background(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextFormat_clearBackground(self as *mut ::text_format::TextFormat) }
  }

  /// C++ method: <span style='color: green;'>```void QTextFormat::clearForeground()```</span>
  ///
  ///
  pub fn clear_foreground(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextFormat_clearForeground(self as *mut ::text_format::TextFormat) }
  }

  /// C++ method: <span style='color: green;'>```void QTextFormat::clearProperty(int propertyId)```</span>
  ///
  ///
  pub fn clear_property(&mut self, property_id: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextFormat_clearProperty(self as *mut ::text_format::TextFormat, property_id) }
  }

  /// C++ method: <span style='color: green;'>```QColor QTextFormat::colorProperty(int propertyId) const```</span>
  ///
  ///
  pub fn color_property(&self, property_id: ::libc::c_int) -> ::color::Color {
    {
      let mut object: ::color::Color = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_colorProperty_to_output(self as *const ::text_format::TextFormat,
                                                            property_id,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QTextFormat::doubleProperty(int propertyId) const```</span>
  ///
  ///
  pub fn double_property(&self, property_id: ::libc::c_int) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextFormat_doubleProperty(self as *const ::text_format::TextFormat, property_id) }
  }

  /// C++ method: <span style='color: green;'>```QBrush QTextFormat::foreground() const```</span>
  ///
  ///
  pub fn foreground(&self) -> ::brush::Brush {
    {
      let mut object: ::brush::Brush = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_foreground_to_output(self as *const ::text_format::TextFormat, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextFormat::hasProperty(int propertyId) const```</span>
  ///
  ///
  pub fn has_property(&self, property_id: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextFormat_hasProperty(self as *const ::text_format::TextFormat, property_id) }
  }

  /// C++ method: <span style='color: green;'>```int QTextFormat::intProperty(int propertyId) const```</span>
  ///
  ///
  pub fn int_property(&self, property_id: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextFormat_intProperty(self as *const ::text_format::TextFormat, property_id) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextFormat::isBlockFormat() const```</span>
  ///
  ///
  pub fn is_block_format(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextFormat_isBlockFormat(self as *const ::text_format::TextFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextFormat::isCharFormat() const```</span>
  ///
  ///
  pub fn is_char_format(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextFormat_isCharFormat(self as *const ::text_format::TextFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextFormat::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextFormat_isEmpty(self as *const ::text_format::TextFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextFormat::isFrameFormat() const```</span>
  ///
  ///
  pub fn is_frame_format(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextFormat_isFrameFormat(self as *const ::text_format::TextFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextFormat::isImageFormat() const```</span>
  ///
  ///
  pub fn is_image_format(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextFormat_isImageFormat(self as *const ::text_format::TextFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextFormat::isListFormat() const```</span>
  ///
  ///
  pub fn is_list_format(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextFormat_isListFormat(self as *const ::text_format::TextFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextFormat::isTableCellFormat() const```</span>
  ///
  ///
  pub fn is_table_cell_format(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextFormat_isTableCellFormat(self as *const ::text_format::TextFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextFormat::isTableFormat() const```</span>
  ///
  ///
  pub fn is_table_format(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextFormat_isTableFormat(self as *const ::text_format::TextFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextFormat::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextFormat_isValid(self as *const ::text_format::TextFormat) }
  }

  /// C++ method: <span style='color: green;'>```QTextLength QTextFormat::lengthProperty(int propertyId) const```</span>
  ///
  ///
  pub fn length_property(&self, property_id: ::libc::c_int) -> ::text_length::TextLength {
    {
      let mut object: ::text_length::TextLength =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_lengthProperty_to_output(self as *const ::text_format::TextFormat,
                                                             property_id,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLength> QTextFormat::lengthVectorProperty(int propertyId) const```</span>
  ///
  ///
  pub fn length_vector_property(&self, property_id: ::libc::c_int) -> ::vector::VectorTextLength {
    {
      let mut object: ::vector::VectorTextLength =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_lengthVectorProperty_to_output(self as *const ::text_format::TextFormat,
                                                                   property_id,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextFormat::merge(const QTextFormat& other)```</span>
  ///
  ///
  pub fn merge(&mut self, other: &::text_format::TextFormat) {
    unsafe {
      ::ffi::qt_gui_c_QTextFormat_merge(self as *mut ::text_format::TextFormat,
                                        other as *const ::text_format::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextFormat::QTextFormat```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::text_format::TextFormat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextFormat::QTextFormat()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::text_format::TextFormat) -> ::text_format::TextFormat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextFormat::QTextFormat(const QTextFormat& rhs)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::text_format::TextFormat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextFormat::QTextFormat(int type)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::text_format::TextFormat
    where Args: overloading::TextFormatNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```int QTextFormat::objectIndex() const```</span>
  ///
  ///
  pub fn object_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextFormat_objectIndex(self as *const ::text_format::TextFormat) }
  }

  /// C++ method: <span style='color: green;'>```int QTextFormat::objectType() const```</span>
  ///
  ///
  pub fn object_type(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextFormat_objectType(self as *const ::text_format::TextFormat) }
  }

  /// C++ method: <span style='color: green;'>```QTextFormat& QTextFormat::operator=(const QTextFormat& rhs)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, rhs: &'l1 ::text_format::TextFormat) -> &'l0 mut ::text_format::TextFormat {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QTextFormat_operator_assign(self as *mut ::text_format::TextFormat,
                                                  rhs as *const ::text_format::TextFormat)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QTextFormat::operator==(const QTextFormat& rhs) const```</span>
  ///
  ///
  pub fn op_eq(&self, rhs: &::text_format::TextFormat) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextFormat_operator_eq(self as *const ::text_format::TextFormat,
                                              rhs as *const ::text_format::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextFormat::operator!=(const QTextFormat& rhs) const```</span>
  ///
  ///
  pub fn op_neq(&self, rhs: &::text_format::TextFormat) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextFormat_operator_neq(self as *const ::text_format::TextFormat,
                                               rhs as *const ::text_format::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```QPen QTextFormat::penProperty(int propertyId) const```</span>
  ///
  ///
  pub fn pen_property(&self, property_id: ::libc::c_int) -> ::pen::Pen {
    {
      let mut object: ::pen::Pen = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_penProperty_to_output(self as *const ::text_format::TextFormat,
                                                          property_id,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMap<int, QVariant> QTextFormat::properties() const```</span>
  ///
  ///
  pub fn properties(&self) -> ::qt_core::map::MapCIntVariant {
    {
      let mut object: ::qt_core::map::MapCIntVariant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_properties_to_output(self as *const ::text_format::TextFormat, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant QTextFormat::property(int propertyId) const```</span>
  ///
  ///
  pub fn property(&self, property_id: ::libc::c_int) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_property_to_output(self as *const ::text_format::TextFormat,
                                                       property_id,
                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextFormat::propertyCount() const```</span>
  ///
  ///
  pub fn property_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextFormat_propertyCount(self as *const ::text_format::TextFormat) }
  }

  /// C++ method: <span style='color: green;'>```void QTextFormat::setBackground(const QBrush& brush)```</span>
  ///
  ///
  pub fn set_background(&mut self, brush: &::brush::Brush) {
    unsafe {
      ::ffi::qt_gui_c_QTextFormat_setBackground(self as *mut ::text_format::TextFormat,
                                                brush as *const ::brush::Brush)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextFormat::setForeground(const QBrush& brush)```</span>
  ///
  ///
  pub fn set_foreground(&mut self, brush: &::brush::Brush) {
    unsafe {
      ::ffi::qt_gui_c_QTextFormat_setForeground(self as *mut ::text_format::TextFormat,
                                                brush as *const ::brush::Brush)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextFormat::setLayoutDirection(Qt::LayoutDirection direction)```</span>
  ///
  ///
  pub fn set_layout_direction(&mut self, direction: &::qt_core::qt::LayoutDirection) {
    unsafe {
      ::ffi::qt_gui_c_QTextFormat_setLayoutDirection(self as *mut ::text_format::TextFormat,
                                                     direction as *const ::qt_core::qt::LayoutDirection)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextFormat::setObjectIndex(int object)```</span>
  ///
  ///
  pub fn set_object_index(&mut self, object: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextFormat_setObjectIndex(self as *mut ::text_format::TextFormat, object) }
  }

  /// C++ method: <span style='color: green;'>```void QTextFormat::setObjectType(int type)```</span>
  ///
  ///
  pub fn set_object_type(&mut self, type_: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextFormat_setObjectType(self as *mut ::text_format::TextFormat, type_) }
  }

  /// C++ method: <span style='color: green;'>```QTextFormat::setProperty```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_property(&mut self, (::libc::c_int, &::qt_core::variant::Variant)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextFormat::setProperty(int propertyId, const QVariant& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_property(&mut self, (::libc::c_int, &::vector::VectorTextLength)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextFormat::setProperty(int propertyId, const QVector<QTextLength>& lengths)```</span>
  ///
  ///
  pub fn set_property<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextFormatSetPropertyArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString QTextFormat::stringProperty(int propertyId) const```</span>
  ///
  ///
  pub fn string_property(&self, property_id: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_stringProperty_to_output(self as *const ::text_format::TextFormat,
                                                             property_id,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextFormat::swap(QTextFormat& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::text_format::TextFormat) {
    unsafe {
      ::ffi::qt_gui_c_QTextFormat_swap(self as *mut ::text_format::TextFormat,
                                       other as *mut ::text_format::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextBlockFormat QTextFormat::toBlockFormat() const```</span>
  ///
  ///
  pub fn to_block_format(&self) -> ::text_block_format::TextBlockFormat {
    {
      let mut object: ::text_block_format::TextBlockFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_toBlockFormat_to_output(self as *const ::text_format::TextFormat, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCharFormat QTextFormat::toCharFormat() const```</span>
  ///
  ///
  pub fn to_char_format(&self) -> ::text_char_format::TextCharFormat {
    {
      let mut object: ::text_char_format::TextCharFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_toCharFormat_to_output(self as *const ::text_format::TextFormat, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextFrameFormat QTextFormat::toFrameFormat() const```</span>
  ///
  ///
  pub fn to_frame_format(&self) -> ::text_frame_format::TextFrameFormat {
    {
      let mut object: ::text_frame_format::TextFrameFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_toFrameFormat_to_output(self as *const ::text_format::TextFormat, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextImageFormat QTextFormat::toImageFormat() const```</span>
  ///
  ///
  pub fn to_image_format(&self) -> ::text_image_format::TextImageFormat {
    {
      let mut object: ::text_image_format::TextImageFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_toImageFormat_to_output(self as *const ::text_format::TextFormat, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextListFormat QTextFormat::toListFormat() const```</span>
  ///
  ///
  pub fn to_list_format(&self) -> ::text_list_format::TextListFormat {
    {
      let mut object: ::text_list_format::TextListFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_toListFormat_to_output(self as *const ::text_format::TextFormat, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextTableCellFormat QTextFormat::toTableCellFormat() const```</span>
  ///
  ///
  pub fn to_table_cell_format(&self) -> ::text_table_cell_format::TextTableCellFormat {
    {
      let mut object: ::text_table_cell_format::TextTableCellFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_toTableCellFormat_to_output(self as *const ::text_format::TextFormat, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextTableFormat QTextFormat::toTableFormat() const```</span>
  ///
  ///
  pub fn to_table_format(&self) -> ::text_table_format::TextTableFormat {
    {
      let mut object: ::text_table_format::TextTableFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_toTableFormat_to_output(self as *const ::text_format::TextFormat, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextFormat::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextFormat_type(self as *const ::text_format::TextFormat) }
  }
}

impl Drop for ::text_format::TextFormat {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextFormat::~QTextFormat()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextFormat_destructor(self as *mut ::text_format::TextFormat) }
  }
}

/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 ::text_format::TextFormat)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QTextFormat& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 ::text_length::TextLength)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QTextLength& arg2)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::text_format::TextFormat)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QTextFormat& arg2)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::text_length::TextLength)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QTextLength& arg2)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator>>```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shr((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 mut ::text_format::TextFormat)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QTextFormat& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shr((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 mut ::text_length::TextLength)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QTextLength& arg2)```</span>
///
///
pub fn op_shr<'largs, Args>(args: Args) -> &'largs mut ::qt_core::data_stream::DataStream
  where Args: overloading::OpShrArgs<'largs>
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```swap```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn swap((&mut ::text_block_format::TextBlockFormat, &mut ::text_block_format::TextBlockFormat)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void swap(QTextBlockFormat& value1, QTextBlockFormat& value2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn swap((&mut ::text_char_format::TextCharFormat, &mut ::text_char_format::TextCharFormat)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void swap(QTextCharFormat& value1, QTextCharFormat& value2)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn swap((&mut ::text_format::TextFormat, &mut ::text_format::TextFormat)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void swap(QTextFormat& value1, QTextFormat& value2)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn swap((&mut ::text_frame_format::TextFrameFormat, &mut ::text_frame_format::TextFrameFormat)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void swap(QTextFrameFormat& value1, QTextFrameFormat& value2)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn swap((&mut ::text_image_format::TextImageFormat, &mut ::text_image_format::TextImageFormat)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void swap(QTextImageFormat& value1, QTextImageFormat& value2)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn swap((&mut ::text_list_format::TextListFormat, &mut ::text_list_format::TextListFormat)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void swap(QTextListFormat& value1, QTextListFormat& value2)```</span>
///
///
///
/// ## Variant 7
///
/// Rust arguments: ```fn swap((&mut ::text_table_cell_format::TextTableCellFormat, &mut ::text_table_cell_format::TextTableCellFormat)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void swap(QTextTableCellFormat& value1, QTextTableCellFormat& value2)```</span>
///
///
///
/// ## Variant 8
///
/// Rust arguments: ```fn swap((&mut ::text_table_format::TextTableFormat, &mut ::text_table_format::TextTableFormat)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void swap(QTextTableFormat& value1, QTextTableFormat& value2)```</span>
///
///
pub fn swap<Args>(args: Args) -> ()
  where Args: overloading::SwapArgs
{
  args.exec()
}
/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextFormat::new](../struct.TextFormat.html#method.new) method.
  pub trait TextFormatNewArgs {
    fn exec(self) -> ::text_format::TextFormat;
  }
  impl TextFormatNewArgs for () {
    fn exec(self) -> ::text_format::TextFormat {

      {
        let mut object: ::text_format::TextFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextFormat_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> TextFormatNewArgs for &'a ::text_format::TextFormat {
    fn exec(self) -> ::text_format::TextFormat {
      let rhs = self;
      {
        let mut object: ::text_format::TextFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextFormat_constructor_rhs(rhs as *const ::text_format::TextFormat, &mut object);
        }
        object
      }
    }
  }
  impl TextFormatNewArgs for ::libc::c_int {
    fn exec(self) -> ::text_format::TextFormat {
      let type_ = self;
      {
        let mut object: ::text_format::TextFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextFormat_constructor_type(type_, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextFormat::set_property](../struct.TextFormat.html#method.set_property) method.
  pub trait TextFormatSetPropertyArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_format::TextFormat) -> ();
  }
  impl<'largs> TextFormatSetPropertyArgs<'largs> for (::libc::c_int, &'largs ::vector::VectorTextLength) {
    fn exec(self, original_self: &'largs mut ::text_format::TextFormat) -> () {
      let property_id = self.0;
      let lengths = self.1;
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_setProperty_propertyId_lengths(original_self as *mut ::text_format::TextFormat,
                                                                   property_id,
                                                                   lengths as *const ::vector::VectorTextLength)
      }
    }
  }
  impl<'largs> TextFormatSetPropertyArgs<'largs> for (::libc::c_int, &'largs ::qt_core::variant::Variant) {
    fn exec(self, original_self: &'largs mut ::text_format::TextFormat) -> () {
      let property_id = self.0;
      let value = self.1;
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_setProperty_propertyId_value(original_self as *mut ::text_format::TextFormat,
                                                                 property_id,
                                                                 value as *const ::qt_core::variant::Variant)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::qt_core::data_stream::DataStream, &'a ::text_format::TextFormat) {
    type ReturnType = &'a mut ::qt_core::data_stream::DataStream;
    fn exec(self) -> &'a mut ::qt_core::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextFormat_G_operator_shl_QDataStream_QTextFormat(arg1 as *mut ::qt_core::data_stream::DataStream, arg2 as *const ::text_format::TextFormat) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a mut ::qt_core::data_stream::DataStream, &'a ::text_length::TextLength) {
    type ReturnType = &'a mut ::qt_core::data_stream::DataStream;
    fn exec(self) -> &'a mut ::qt_core::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextFormat_G_operator_shl_QDataStream_QTextLength(arg1 as *mut ::qt_core::data_stream::DataStream, arg2 as *const ::text_length::TextLength) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::text_format::TextFormat) {
    type ReturnType = ::qt_core::debug::Debug;
    fn exec(self) -> ::qt_core::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextFormat_G_operator_shl_to_output_QDebug_QTextFormat(arg1 as *const ::qt_core::debug::Debug, arg2 as *const ::text_format::TextFormat, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::text_length::TextLength) {
    type ReturnType = ::qt_core::debug::Debug;
    fn exec(self) -> ::qt_core::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextFormat_G_operator_shl_to_output_QDebug_QTextLength(arg1 as *const ::qt_core::debug::Debug, arg2 as *const ::text_length::TextLength, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shr](../fn.op_shr.html) method.
  pub trait OpShrArgs<'largs> {
    fn exec(self) -> &'largs mut ::qt_core::data_stream::DataStream;
  }
  impl<'largs> OpShrArgs<'largs>
    for (&'largs mut ::qt_core::data_stream::DataStream, &'largs mut ::text_format::TextFormat) {
    fn exec(self) -> &'largs mut ::qt_core::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextFormat_G_operator_shr_QDataStream_QTextFormat(arg1 as *mut ::qt_core::data_stream::DataStream, arg2 as *mut ::text_format::TextFormat) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> OpShrArgs<'largs>
    for (&'largs mut ::qt_core::data_stream::DataStream, &'largs mut ::text_length::TextLength) {
    fn exec(self) -> &'largs mut ::qt_core::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextFormat_G_operator_shr_QDataStream_QTextLength(arg1 as *mut ::qt_core::data_stream::DataStream, arg2 as *mut ::text_length::TextLength) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [swap](../fn.swap.html) method.
  pub trait SwapArgs {
    fn exec(self) -> ();
  }
  impl<'a> SwapArgs for (&'a mut ::text_block_format::TextBlockFormat, &'a mut ::text_block_format::TextBlockFormat) {
    fn exec(self) -> () {
      let value1 = self.0;
      let value2 = self.1;
      unsafe { ::ffi::qt_gui_c_QTextFormat_G_swap_QTextBlockFormat_QTextBlockFormat(value1 as *mut ::text_block_format::TextBlockFormat, value2 as *mut ::text_block_format::TextBlockFormat) }
    }
  }
  impl<'a> SwapArgs for (&'a mut ::text_char_format::TextCharFormat, &'a mut ::text_char_format::TextCharFormat) {
    fn exec(self) -> () {
      let value1 = self.0;
      let value2 = self.1;
      unsafe { ::ffi::qt_gui_c_QTextFormat_G_swap_QTextCharFormat_QTextCharFormat(value1 as *mut ::text_char_format::TextCharFormat, value2 as *mut ::text_char_format::TextCharFormat) }
    }
  }
  impl<'a> SwapArgs for (&'a mut ::text_format::TextFormat, &'a mut ::text_format::TextFormat) {
    fn exec(self) -> () {
      let value1 = self.0;
      let value2 = self.1;
      unsafe {
        ::ffi::qt_gui_c_QTextFormat_G_swap_QTextFormat_QTextFormat(value1 as *mut ::text_format::TextFormat,
                                                                   value2 as *mut ::text_format::TextFormat)
      }
    }
  }
  impl<'a> SwapArgs for (&'a mut ::text_frame_format::TextFrameFormat, &'a mut ::text_frame_format::TextFrameFormat) {
    fn exec(self) -> () {
      let value1 = self.0;
      let value2 = self.1;
      unsafe { ::ffi::qt_gui_c_QTextFormat_G_swap_QTextFrameFormat_QTextFrameFormat(value1 as *mut ::text_frame_format::TextFrameFormat, value2 as *mut ::text_frame_format::TextFrameFormat) }
    }
  }
  impl<'a> SwapArgs for (&'a mut ::text_image_format::TextImageFormat, &'a mut ::text_image_format::TextImageFormat) {
    fn exec(self) -> () {
      let value1 = self.0;
      let value2 = self.1;
      unsafe { ::ffi::qt_gui_c_QTextFormat_G_swap_QTextImageFormat_QTextImageFormat(value1 as *mut ::text_image_format::TextImageFormat, value2 as *mut ::text_image_format::TextImageFormat) }
    }
  }
  impl<'a> SwapArgs for (&'a mut ::text_list_format::TextListFormat, &'a mut ::text_list_format::TextListFormat) {
    fn exec(self) -> () {
      let value1 = self.0;
      let value2 = self.1;
      unsafe { ::ffi::qt_gui_c_QTextFormat_G_swap_QTextListFormat_QTextListFormat(value1 as *mut ::text_list_format::TextListFormat, value2 as *mut ::text_list_format::TextListFormat) }
    }
  }
  impl<'a> SwapArgs
    for (&'a mut ::text_table_cell_format::TextTableCellFormat, &'a mut ::text_table_cell_format::TextTableCellFormat) {
    fn exec(self) -> () {
      let value1 = self.0;
      let value2 = self.1;
      unsafe { ::ffi::qt_gui_c_QTextFormat_G_swap_QTextTableCellFormat_QTextTableCellFormat(value1 as *mut ::text_table_cell_format::TextTableCellFormat, value2 as *mut ::text_table_cell_format::TextTableCellFormat) }
    }
  }
  impl<'a> SwapArgs for (&'a mut ::text_table_format::TextTableFormat, &'a mut ::text_table_format::TextTableFormat) {
    fn exec(self) -> () {
      let value1 = self.0;
      let value2 = self.1;
      unsafe { ::ffi::qt_gui_c_QTextFormat_G_swap_QTextTableFormat_QTextTableFormat(value1 as *mut ::text_table_format::TextTableFormat, value2 as *mut ::text_table_format::TextTableFormat) }
    }
  }
}
