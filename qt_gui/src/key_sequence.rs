/// C++ type: <span style='color: green;'>```QKeySequence```</span>
#[repr(C)]
pub struct KeySequence([u8; ::type_sizes::QT_GUI_KEY_SEQUENCE_KEY_SEQUENCE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for KeySequence {
  unsafe fn new_uninitialized() -> KeySequence {
    KeySequence(::std::mem::uninitialized())
  }
}

impl KeySequence {
  /// C++ method: <span style='color: green;'>```QVariant QKeySequence::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QKeySequence_convert_to_QVariant_to_output(self as *const ::key_sequence::KeySequence,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QKeySequence::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QKeySequence_count(self as *const ::key_sequence::KeySequence) }
  }

  /// C++ method: <span style='color: green;'>```QKeySequence::fromString```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_string(&::qt_core::string::String) -> ::key_sequence::KeySequence```<br>
  /// C++ method: <span style='color: green;'>```static QKeySequence QKeySequence::fromString(const QString& str)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_string((&::qt_core::string::String, ::key_sequence::SequenceFormat)) -> ::key_sequence::KeySequence```<br>
  /// C++ method: <span style='color: green;'>```static QKeySequence QKeySequence::fromString(const QString& str, QKeySequence::SequenceFormat format = ?)```</span>
  ///
  ///
  pub fn from_string<Args>(args: Args) -> ::key_sequence::KeySequence
    where Args: overloading::KeySequenceFromStringArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QKeySequence::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QKeySequence_isEmpty(self as *const ::key_sequence::KeySequence) }
  }

  /// C++ method: <span style='color: green;'>```static QList<QKeySequence> QKeySequence::keyBindings(QKeySequence::StandardKey key)```</span>
  ///
  ///
  pub fn key_bindings(key: ::key_sequence::StandardKey) -> ::list::ListKeySequence {
    {
      let mut object: ::list::ListKeySequence =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QKeySequence_keyBindings_to_output(key, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QKeySequence::listFromString```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn list_from_string(&::qt_core::string::String) -> ::list::ListKeySequence```<br>
  /// C++ method: <span style='color: green;'>```static QList<QKeySequence> QKeySequence::listFromString(const QString& str)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn list_from_string((&::qt_core::string::String, ::key_sequence::SequenceFormat)) -> ::list::ListKeySequence```<br>
  /// C++ method: <span style='color: green;'>```static QList<QKeySequence> QKeySequence::listFromString(const QString& str, QKeySequence::SequenceFormat format = ?)```</span>
  ///
  ///
  pub fn list_from_string<Args>(args: Args) -> ::list::ListKeySequence
    where Args: overloading::KeySequenceListFromStringArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QKeySequence::listToString```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn list_to_string(&::list::ListKeySequence) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QKeySequence::listToString(const QList<QKeySequence>& list)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn list_to_string((&::list::ListKeySequence, ::key_sequence::SequenceFormat)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QKeySequence::listToString(const QList<QKeySequence>& list, QKeySequence::SequenceFormat format = ?)```</span>
  ///
  ///
  pub fn list_to_string<Args>(args: Args) -> ::qt_core::string::String
    where Args: overloading::KeySequenceListToStringArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QKeySequence::SequenceMatch QKeySequence::matches(const QKeySequence& seq) const```</span>
  ///
  ///
  pub fn matches(&self, seq: &::key_sequence::KeySequence) -> ::key_sequence::SequenceMatch {
    unsafe {
      ::ffi::qt_gui_c_QKeySequence_matches(self as *const ::key_sequence::KeySequence,
                                           seq as *const ::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```static QKeySequence QKeySequence::mnemonic(const QString& text)```</span>
  ///
  ///
  pub fn mnemonic(text: &::qt_core::string::String) -> ::key_sequence::KeySequence {
    {
      let mut object: ::key_sequence::KeySequence =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QKeySequence_mnemonic_to_output(text as *const ::qt_core::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QKeySequence::QKeySequence```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::key_sequence::KeySequence```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QKeySequence::QKeySequence()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::key_sequence::StandardKey) -> ::key_sequence::KeySequence```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QKeySequence::QKeySequence(QKeySequence::StandardKey key)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::key_sequence::KeySequence) -> ::key_sequence::KeySequence```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QKeySequence::QKeySequence(const QKeySequence& ks)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::key_sequence::KeySequence```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QKeySequence::QKeySequence(const QString& key)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((&::qt_core::string::String, ::key_sequence::SequenceFormat)) -> ::key_sequence::KeySequence```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QKeySequence::QKeySequence(const QString& key, QKeySequence::SequenceFormat format = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::key_sequence::KeySequence```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QKeySequence::QKeySequence(int k1)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int)) -> ::key_sequence::KeySequence```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QKeySequence::QKeySequence(int k1, int k2 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::key_sequence::KeySequence```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QKeySequence::QKeySequence(int k1, int k2 = ?, int k3 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::key_sequence::KeySequence```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QKeySequence::QKeySequence(int k1, int k2 = ?, int k3 = ?, int k4 = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::key_sequence::KeySequence
    where Args: overloading::KeySequenceNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QKeySequence& QKeySequence::operator=(const QKeySequence& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::key_sequence::KeySequence)
                             -> &'l0 mut ::key_sequence::KeySequence {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QKeySequence_operator_assign(self as *mut ::key_sequence::KeySequence,
                                                   other as *const ::key_sequence::KeySequence)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QKeySequence::operator==(const QKeySequence& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::key_sequence::KeySequence) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QKeySequence_operator_eq(self as *const ::key_sequence::KeySequence,
                                               other as *const ::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QKeySequence::operator>=(const QKeySequence& other) const```</span>
  ///
  ///
  pub fn op_ge(&self, other: &::key_sequence::KeySequence) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QKeySequence_operator_ge(self as *const ::key_sequence::KeySequence,
                                               other as *const ::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QKeySequence::operator>(const QKeySequence& other) const```</span>
  ///
  ///
  pub fn op_gt(&self, other: &::key_sequence::KeySequence) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QKeySequence_operator_gt(self as *const ::key_sequence::KeySequence,
                                               other as *const ::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```int QKeySequence::operator[](unsigned int i) const```</span>
  ///
  ///
  pub fn op_index(&self, i: ::libc::c_uint) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QKeySequence_operator_index(self as *const ::key_sequence::KeySequence, i) }
  }

  /// C++ method: <span style='color: green;'>```bool QKeySequence::operator<=(const QKeySequence& other) const```</span>
  ///
  ///
  pub fn op_le(&self, other: &::key_sequence::KeySequence) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QKeySequence_operator_le(self as *const ::key_sequence::KeySequence,
                                               other as *const ::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QKeySequence::operator<(const QKeySequence& ks) const```</span>
  ///
  ///
  pub fn op_lt(&self, ks: &::key_sequence::KeySequence) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QKeySequence_operator_lt(self as *const ::key_sequence::KeySequence,
                                               ks as *const ::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QKeySequence::operator!=(const QKeySequence& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::key_sequence::KeySequence) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QKeySequence_operator_neq(self as *const ::key_sequence::KeySequence,
                                                other as *const ::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```void QKeySequence::swap(QKeySequence& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::key_sequence::KeySequence) {
    unsafe {
      ::ffi::qt_gui_c_QKeySequence_swap(self as *mut ::key_sequence::KeySequence,
                                        other as *mut ::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```QKeySequence::toString```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_string(&self, ()) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QKeySequence::toString() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_string(&self, ::key_sequence::SequenceFormat) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QKeySequence::toString(QKeySequence::SequenceFormat format = ?) const```</span>
  ///
  ///
  pub fn to_string<'largs, Args>(&'largs self, args: Args) -> ::qt_core::string::String
    where Args: overloading::KeySequenceToStringArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::key_sequence::KeySequence {
  /// C++ method: <span style='color: green;'>```[destructor] void QKeySequence::~QKeySequence()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QKeySequence_destructor(self as *mut ::key_sequence::KeySequence) }
  }
}

/// C++ type: <span style='color: green;'>```QKeySequence::SequenceFormat```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SequenceFormat {
  /// C++ enum variant: <span style='color: green;'>```NativeText = 0```</span>
  Native = 0,
  /// C++ enum variant: <span style='color: green;'>```PortableText = 1```</span>
  Portable = 1,
}

/// C++ type: <span style='color: green;'>```QKeySequence::SequenceMatch```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SequenceMatch {
  /// C++ enum variant: <span style='color: green;'>```NoMatch = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```PartialMatch = 1```</span>
  Partial = 1,
  /// C++ enum variant: <span style='color: green;'>```ExactMatch = 2```</span>
  Exact = 2,
}

/// C++ type: <span style='color: green;'>```QKeySequence::StandardKey```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StandardKey {
  /// C++ enum variant: <span style='color: green;'>```UnknownKey = 0```</span>
  UnknownKey = 0,
  /// C++ enum variant: <span style='color: green;'>```HelpContents = 1```</span>
  HelpContents = 1,
  /// C++ enum variant: <span style='color: green;'>```WhatsThis = 2```</span>
  WhatsThis = 2,
  /// C++ enum variant: <span style='color: green;'>```Open = 3```</span>
  Open = 3,
  /// C++ enum variant: <span style='color: green;'>```Close = 4```</span>
  Close = 4,
  /// C++ enum variant: <span style='color: green;'>```Save = 5```</span>
  Save = 5,
  /// C++ enum variant: <span style='color: green;'>```New = 6```</span>
  New = 6,
  /// C++ enum variant: <span style='color: green;'>```Delete = 7```</span>
  Delete = 7,
  /// C++ enum variant: <span style='color: green;'>```Cut = 8```</span>
  Cut = 8,
  /// C++ enum variant: <span style='color: green;'>```Copy = 9```</span>
  Copy = 9,
  /// C++ enum variant: <span style='color: green;'>```Paste = 10```</span>
  Paste = 10,
  /// C++ enum variant: <span style='color: green;'>```Undo = 11```</span>
  Undo = 11,
  /// C++ enum variant: <span style='color: green;'>```Redo = 12```</span>
  Redo = 12,
  /// C++ enum variant: <span style='color: green;'>```Back = 13```</span>
  Back = 13,
  /// C++ enum variant: <span style='color: green;'>```Forward = 14```</span>
  Forward = 14,
  /// C++ enum variant: <span style='color: green;'>```Refresh = 15```</span>
  Refresh = 15,
  /// C++ enum variant: <span style='color: green;'>```ZoomIn = 16```</span>
  ZoomIn = 16,
  /// C++ enum variant: <span style='color: green;'>```ZoomOut = 17```</span>
  ZoomOut = 17,
  /// C++ enum variant: <span style='color: green;'>```Print = 18```</span>
  Print = 18,
  /// C++ enum variant: <span style='color: green;'>```AddTab = 19```</span>
  AddTab = 19,
  /// C++ enum variant: <span style='color: green;'>```NextChild = 20```</span>
  NextChild = 20,
  /// C++ enum variant: <span style='color: green;'>```PreviousChild = 21```</span>
  PreviousChild = 21,
  /// C++ enum variant: <span style='color: green;'>```Find = 22```</span>
  Find = 22,
  /// C++ enum variant: <span style='color: green;'>```FindNext = 23```</span>
  FindNext = 23,
  /// C++ enum variant: <span style='color: green;'>```FindPrevious = 24```</span>
  FindPrevious = 24,
  /// C++ enum variant: <span style='color: green;'>```Replace = 25```</span>
  Replace = 25,
  /// C++ enum variant: <span style='color: green;'>```SelectAll = 26```</span>
  SelectAll = 26,
  /// C++ enum variant: <span style='color: green;'>```Bold = 27```</span>
  Bold = 27,
  /// C++ enum variant: <span style='color: green;'>```Italic = 28```</span>
  Italic = 28,
  /// C++ enum variant: <span style='color: green;'>```Underline = 29```</span>
  Underline = 29,
  /// C++ enum variant: <span style='color: green;'>```MoveToNextChar = 30```</span>
  MoveToNextChar = 30,
  /// C++ enum variant: <span style='color: green;'>```MoveToPreviousChar = 31```</span>
  MoveToPreviousChar = 31,
  /// C++ enum variant: <span style='color: green;'>```MoveToNextWord = 32```</span>
  MoveToNextWord = 32,
  /// C++ enum variant: <span style='color: green;'>```MoveToPreviousWord = 33```</span>
  MoveToPreviousWord = 33,
  /// C++ enum variant: <span style='color: green;'>```MoveToNextLine = 34```</span>
  MoveToNextLine = 34,
  /// C++ enum variant: <span style='color: green;'>```MoveToPreviousLine = 35```</span>
  MoveToPreviousLine = 35,
  /// C++ enum variant: <span style='color: green;'>```MoveToNextPage = 36```</span>
  MoveToNextPage = 36,
  /// C++ enum variant: <span style='color: green;'>```MoveToPreviousPage = 37```</span>
  MoveToPreviousPage = 37,
  /// C++ enum variant: <span style='color: green;'>```MoveToStartOfLine = 38```</span>
  MoveToStartOfLine = 38,
  /// C++ enum variant: <span style='color: green;'>```MoveToEndOfLine = 39```</span>
  MoveToEndOfLine = 39,
  /// C++ enum variant: <span style='color: green;'>```MoveToStartOfBlock = 40```</span>
  MoveToStartOfBlock = 40,
  /// C++ enum variant: <span style='color: green;'>```MoveToEndOfBlock = 41```</span>
  MoveToEndOfBlock = 41,
  /// C++ enum variant: <span style='color: green;'>```MoveToStartOfDocument = 42```</span>
  MoveToStartOfDocument = 42,
  /// C++ enum variant: <span style='color: green;'>```MoveToEndOfDocument = 43```</span>
  MoveToEndOfDocument = 43,
  /// C++ enum variant: <span style='color: green;'>```SelectNextChar = 44```</span>
  SelectNextChar = 44,
  /// C++ enum variant: <span style='color: green;'>```SelectPreviousChar = 45```</span>
  SelectPreviousChar = 45,
  /// C++ enum variant: <span style='color: green;'>```SelectNextWord = 46```</span>
  SelectNextWord = 46,
  /// C++ enum variant: <span style='color: green;'>```SelectPreviousWord = 47```</span>
  SelectPreviousWord = 47,
  /// C++ enum variant: <span style='color: green;'>```SelectNextLine = 48```</span>
  SelectNextLine = 48,
  /// C++ enum variant: <span style='color: green;'>```SelectPreviousLine = 49```</span>
  SelectPreviousLine = 49,
  /// C++ enum variant: <span style='color: green;'>```SelectNextPage = 50```</span>
  SelectNextPage = 50,
  /// C++ enum variant: <span style='color: green;'>```SelectPreviousPage = 51```</span>
  SelectPreviousPage = 51,
  /// C++ enum variant: <span style='color: green;'>```SelectStartOfLine = 52```</span>
  SelectStartOfLine = 52,
  /// C++ enum variant: <span style='color: green;'>```SelectEndOfLine = 53```</span>
  SelectEndOfLine = 53,
  /// C++ enum variant: <span style='color: green;'>```SelectStartOfBlock = 54```</span>
  SelectStartOfBlock = 54,
  /// C++ enum variant: <span style='color: green;'>```SelectEndOfBlock = 55```</span>
  SelectEndOfBlock = 55,
  /// C++ enum variant: <span style='color: green;'>```SelectStartOfDocument = 56```</span>
  SelectStartOfDocument = 56,
  /// C++ enum variant: <span style='color: green;'>```SelectEndOfDocument = 57```</span>
  SelectEndOfDocument = 57,
  /// C++ enum variant: <span style='color: green;'>```DeleteStartOfWord = 58```</span>
  DeleteStartOfWord = 58,
  /// C++ enum variant: <span style='color: green;'>```DeleteEndOfWord = 59```</span>
  DeleteEndOfWord = 59,
  /// C++ enum variant: <span style='color: green;'>```DeleteEndOfLine = 60```</span>
  DeleteEndOfLine = 60,
  /// C++ enum variant: <span style='color: green;'>```InsertParagraphSeparator = 61```</span>
  InsertParagraphSeparator = 61,
  /// C++ enum variant: <span style='color: green;'>```InsertLineSeparator = 62```</span>
  InsertLineSeparator = 62,
  /// C++ enum variant: <span style='color: green;'>```SaveAs = 63```</span>
  SaveAs = 63,
  /// C++ enum variant: <span style='color: green;'>```Preferences = 64```</span>
  Preferences = 64,
  /// C++ enum variant: <span style='color: green;'>```Quit = 65```</span>
  Quit = 65,
  /// C++ enum variant: <span style='color: green;'>```FullScreen = 66```</span>
  FullScreen = 66,
  /// C++ enum variant: <span style='color: green;'>```Deselect = 67```</span>
  Deselect = 67,
  /// C++ enum variant: <span style='color: green;'>```DeleteCompleteLine = 68```</span>
  DeleteCompleteLine = 68,
  /// C++ enum variant: <span style='color: green;'>```Backspace = 69```</span>
  Backspace = 69,
  /// C++ enum variant: <span style='color: green;'>```Cancel = 70```</span>
  Cancel = 70,
}

/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash(&::key_sequence::KeySequence) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QKeySequence& key)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash((&::key_sequence::KeySequence, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QKeySequence& key, unsigned int seed = ?)```</span>
///
///
pub fn hash<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::HashArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 ::key_sequence::KeySequence)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& in, const QKeySequence& ks)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::key_sequence::KeySequence)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QKeySequence& arg2)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& out, QKeySequence& ks)```</span>
///
///
pub fn op_shr<'l0, 'l1>(out: &'l0 mut ::qt_core::data_stream::DataStream,
                        ks: &'l1 mut ::key_sequence::KeySequence)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_gui_c_QKeySequence_G_operator_shr(out as *mut ::qt_core::data_stream::DataStream,
                                                ks as *mut ::key_sequence::KeySequence)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```void swap(QKeySequence& value1, QKeySequence& value2)```</span>
///
///
pub fn swap(value1: &mut ::key_sequence::KeySequence, value2: &mut ::key_sequence::KeySequence) {
  unsafe {
    ::ffi::qt_gui_c_QKeySequence_G_swap(value1 as *mut ::key_sequence::KeySequence,
                                        value2 as *mut ::key_sequence::KeySequence)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [KeySequence::from_string](../struct.KeySequence.html#method.from_string) method.
  pub trait KeySequenceFromStringArgs {
    fn exec(self) -> ::key_sequence::KeySequence;
  }
  impl<'a> KeySequenceFromStringArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::key_sequence::KeySequence {
      let str = self;
      {
        let mut object: ::key_sequence::KeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QKeySequence_fromString_to_output_str(str as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> KeySequenceFromStringArgs for (&'a ::qt_core::string::String, ::key_sequence::SequenceFormat) {
    fn exec(self) -> ::key_sequence::KeySequence {
      let str = self.0;
      let format = self.1;
      {
        let mut object: ::key_sequence::KeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QKeySequence_fromString_to_output_str_format(str as *const ::qt_core::string::String,
                                                                       format,
                                                                       &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [KeySequence::list_from_string](../struct.KeySequence.html#method.list_from_string) method.
  pub trait KeySequenceListFromStringArgs {
    fn exec(self) -> ::list::ListKeySequence;
  }
  impl<'a> KeySequenceListFromStringArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::list::ListKeySequence {
      let str = self;
      {
        let mut object: ::list::ListKeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QKeySequence_listFromString_to_output_str(str as *const ::qt_core::string::String,
                                                                    &mut object);
        }
        object
      }
    }
  }
  impl<'a> KeySequenceListFromStringArgs for (&'a ::qt_core::string::String, ::key_sequence::SequenceFormat) {
    fn exec(self) -> ::list::ListKeySequence {
      let str = self.0;
      let format = self.1;
      {
        let mut object: ::list::ListKeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QKeySequence_listFromString_to_output_str_format(str as *const ::qt_core::string::String,
                                                                           format,
                                                                           &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [KeySequence::list_to_string](../struct.KeySequence.html#method.list_to_string) method.
  pub trait KeySequenceListToStringArgs {
    fn exec(self) -> ::qt_core::string::String;
  }
  impl<'a> KeySequenceListToStringArgs for &'a ::list::ListKeySequence {
    fn exec(self) -> ::qt_core::string::String {
      let list = self;
      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QKeySequence_listToString_to_output_list(list as *const ::list::ListKeySequence, &mut object);
        }
        object
      }
    }
  }
  impl<'a> KeySequenceListToStringArgs for (&'a ::list::ListKeySequence, ::key_sequence::SequenceFormat) {
    fn exec(self) -> ::qt_core::string::String {
      let list = self.0;
      let format = self.1;
      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QKeySequence_listToString_to_output_list_format(list as *const ::list::ListKeySequence,
                                                                          format,
                                                                          &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [KeySequence::new](../struct.KeySequence.html#method.new) method.
  pub trait KeySequenceNewArgs {
    fn exec(self) -> ::key_sequence::KeySequence;
  }
  impl<'a> KeySequenceNewArgs for &'a ::key_sequence::KeySequence {
    fn exec(self) -> ::key_sequence::KeySequence {
      let ks = self;
      {
        let mut object: ::key_sequence::KeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QKeySequence_constructor_QKeySequence(ks as *const ::key_sequence::KeySequence, &mut object);
        }
        object
      }
    }
  }
  impl KeySequenceNewArgs for ::key_sequence::StandardKey {
    fn exec(self) -> ::key_sequence::KeySequence {
      let key = self;
      {
        let mut object: ::key_sequence::KeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QKeySequence_constructor_QKeySequence_StandardKey(key, &mut object);
        }
        object
      }
    }
  }
  impl<'a> KeySequenceNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::key_sequence::KeySequence {
      let key = self;
      {
        let mut object: ::key_sequence::KeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QKeySequence_constructor_QString(key as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> KeySequenceNewArgs for (&'a ::qt_core::string::String, ::key_sequence::SequenceFormat) {
    fn exec(self) -> ::key_sequence::KeySequence {
      let key = self.0;
      let format = self.1;
      {
        let mut object: ::key_sequence::KeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QKeySequence_constructor_QString_QKeySequence_SequenceFormat(key as *const ::qt_core::string::String, format, &mut object);
        }
        object
      }
    }
  }
  impl KeySequenceNewArgs for ::libc::c_int {
    fn exec(self) -> ::key_sequence::KeySequence {
      let k1 = self;
      {
        let mut object: ::key_sequence::KeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QKeySequence_constructor_int(k1, &mut object);
        }
        object
      }
    }
  }
  impl KeySequenceNewArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::key_sequence::KeySequence {
      let k1 = self.0;
      let k2 = self.1;
      {
        let mut object: ::key_sequence::KeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QKeySequence_constructor_int_int(k1, k2, &mut object);
        }
        object
      }
    }
  }
  impl KeySequenceNewArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::key_sequence::KeySequence {
      let k1 = self.0;
      let k2 = self.1;
      let k3 = self.2;
      {
        let mut object: ::key_sequence::KeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QKeySequence_constructor_int_int_int(k1, k2, k3, &mut object);
        }
        object
      }
    }
  }
  impl KeySequenceNewArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::key_sequence::KeySequence {
      let k1 = self.0;
      let k2 = self.1;
      let k3 = self.2;
      let k4 = self.3;
      {
        let mut object: ::key_sequence::KeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QKeySequence_constructor_int_int_int_int(k1, k2, k3, k4, &mut object);
        }
        object
      }
    }
  }
  impl KeySequenceNewArgs for () {
    fn exec(self) -> ::key_sequence::KeySequence {

      {
        let mut object: ::key_sequence::KeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QKeySequence_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [KeySequence::to_string](../struct.KeySequence.html#method.to_string) method.
  pub trait KeySequenceToStringArgs<'largs> {
    fn exec(self, original_self: &'largs ::key_sequence::KeySequence) -> ::qt_core::string::String;
  }
  impl<'largs> KeySequenceToStringArgs<'largs> for ::key_sequence::SequenceFormat {
    fn exec(self, original_self: &'largs ::key_sequence::KeySequence) -> ::qt_core::string::String {
      let format = self;
      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QKeySequence_toString_to_output_format(original_self as *const ::key_sequence::KeySequence,
                                                                 format,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'largs> KeySequenceToStringArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::key_sequence::KeySequence) -> ::qt_core::string::String {

      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QKeySequence_toString_to_output_no_args(original_self as *const ::key_sequence::KeySequence, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [hash](../fn.hash.html) method.
  pub trait HashArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> HashArgs for &'a ::key_sequence::KeySequence {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_gui_c_QKeySequence_G_qHash_key(key as *const ::key_sequence::KeySequence) }
    }
  }
  impl<'a> HashArgs for (&'a ::key_sequence::KeySequence, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_gui_c_QKeySequence_G_qHash_key_seed(key as *const ::key_sequence::KeySequence, seed) }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::qt_core::data_stream::DataStream, &'a ::key_sequence::KeySequence) {
    type ReturnType = &'a mut ::qt_core::data_stream::DataStream;
    fn exec(self) -> &'a mut ::qt_core::data_stream::DataStream {
      let in_ = self.0;
      let ks = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QKeySequence_G_operator_shl(in_ as *mut ::qt_core::data_stream::DataStream,
                                                    ks as *const ::key_sequence::KeySequence)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::key_sequence::KeySequence) {
    type ReturnType = ::qt_core::debug::Debug;
    fn exec(self) -> ::qt_core::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QKeySequence_G_operator_shl_to_output(arg1 as *const ::qt_core::debug::Debug,
                                                                arg2 as *const ::key_sequence::KeySequence,
                                                                &mut object);
        }
        object
      }
    }
  }
}
