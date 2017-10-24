/// C++ type: <span style='color: green;'>```QTextBrowser```</span>
#[repr(C)]
pub struct TextBrowser(u8);

impl TextBrowser {
  /// C++ method: <span style='color: green;'>```virtual [slot] void QTextBrowser::backward()```</span>
  ///
  ///
  pub fn backward(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTextBrowser_backward(self as *mut ::text_browser::TextBrowser) }
  }

  /// C++ method: <span style='color: green;'>```int QTextBrowser::backwardHistoryCount() const```</span>
  ///
  ///
  pub fn backward_history_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTextBrowser_backwardHistoryCount(self as *const ::text_browser::TextBrowser) }
  }

  /// C++ method: <span style='color: green;'>```void QTextBrowser::clearHistory()```</span>
  ///
  ///
  pub fn clear_history(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTextBrowser_clearHistory(self as *mut ::text_browser::TextBrowser) }
  }

  /// C++ method: <span style='color: green;'>```virtual [slot] void QTextBrowser::forward()```</span>
  ///
  ///
  pub fn forward(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTextBrowser_forward(self as *mut ::text_browser::TextBrowser) }
  }

  /// C++ method: <span style='color: green;'>```int QTextBrowser::forwardHistoryCount() const```</span>
  ///
  ///
  pub fn forward_history_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTextBrowser_forwardHistoryCount(self as *const ::text_browser::TextBrowser) }
  }

  /// C++ method: <span style='color: green;'>```QString QTextBrowser::historyTitle(int arg1) const```</span>
  ///
  ///
  pub fn history_title(&self, arg1: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTextBrowser_historyTitle_to_output(self as *const ::text_browser::TextBrowser,
                                                                arg1,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QUrl QTextBrowser::historyUrl(int arg1) const```</span>
  ///
  ///
  pub fn history_url(&self, arg1: ::libc::c_int) -> ::qt_core::url::Url {
    {
      let mut object: ::qt_core::url::Url =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTextBrowser_historyUrl_to_output(self as *const ::text_browser::TextBrowser,
                                                              arg1,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual [slot] void QTextBrowser::home()```</span>
  ///
  ///
  pub fn home(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTextBrowser_home(self as *mut ::text_browser::TextBrowser) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextBrowser::isBackwardAvailable() const```</span>
  ///
  ///
  pub fn is_backward_available(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTextBrowser_isBackwardAvailable(self as *const ::text_browser::TextBrowser) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextBrowser::isForwardAvailable() const```</span>
  ///
  ///
  pub fn is_forward_available(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTextBrowser_isForwardAvailable(self as *const ::text_browser::TextBrowser) }
  }

  /// C++ method: <span style='color: green;'>```virtual QVariant QTextBrowser::loadResource(int type, const QUrl& name)```</span>
  ///
  ///
  pub fn load_resource(&mut self, type_: ::libc::c_int, name: &::qt_core::url::Url) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTextBrowser_loadResource_to_output(self as *mut ::text_browser::TextBrowser,
                                                                type_,
                                                                name as *const ::qt_core::url::Url,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTextBrowser::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QTextBrowser_metaObject(self as *const ::text_browser::TextBrowser) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTextBrowser::QTextBrowser()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::text_browser::TextBrowser> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextBrowser_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTextBrowser::QTextBrowser(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::text_browser::TextBrowser> {
    let ffi_result = ::ffi::qt_widgets_c_QTextBrowser_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```bool QTextBrowser::openExternalLinks() const```</span>
  ///
  ///
  pub fn open_external_links(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTextBrowser_openExternalLinks(self as *const ::text_browser::TextBrowser) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextBrowser::openLinks() const```</span>
  ///
  ///
  pub fn open_links(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTextBrowser_openLinks(self as *const ::text_browser::TextBrowser) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QTextBrowser::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QTextBrowser_qt_metacall(self as *mut ::text_browser::TextBrowser,
                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                 arg2,
                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QTextBrowser::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QTextBrowser_qt_metacast(self as *mut ::text_browser::TextBrowser, arg1)
  }

  /// C++ method: <span style='color: green;'>```virtual [slot] void QTextBrowser::reload()```</span>
  ///
  ///
  pub fn reload(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTextBrowser_reload(self as *mut ::text_browser::TextBrowser) }
  }

  /// C++ method: <span style='color: green;'>```QStringList QTextBrowser::searchPaths() const```</span>
  ///
  ///
  pub fn search_paths(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTextBrowser_searchPaths_to_output(self as *const ::text_browser::TextBrowser, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextBrowser::setOpenExternalLinks(bool open)```</span>
  ///
  ///
  pub fn set_open_external_links(&mut self, open: bool) {
    unsafe { ::ffi::qt_widgets_c_QTextBrowser_setOpenExternalLinks(self as *mut ::text_browser::TextBrowser, open) }
  }

  /// C++ method: <span style='color: green;'>```void QTextBrowser::setOpenLinks(bool open)```</span>
  ///
  ///
  pub fn set_open_links(&mut self, open: bool) {
    unsafe { ::ffi::qt_widgets_c_QTextBrowser_setOpenLinks(self as *mut ::text_browser::TextBrowser, open) }
  }

  /// C++ method: <span style='color: green;'>```void QTextBrowser::setSearchPaths(const QStringList& paths)```</span>
  ///
  ///
  pub fn set_search_paths(&mut self, paths: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_widgets_c_QTextBrowser_setSearchPaths(self as *mut ::text_browser::TextBrowser,
                                                      paths as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual [slot] void QTextBrowser::setSource(const QUrl& name)```</span>
  ///
  ///
  pub fn set_source(&mut self, name: &::qt_core::url::Url) {
    unsafe {
      ::ffi::qt_widgets_c_QTextBrowser_setSource(self as *mut ::text_browser::TextBrowser,
                                                 name as *const ::qt_core::url::Url)
    }
  }

  /// C++ method: <span style='color: green;'>```QUrl QTextBrowser::source() const```</span>
  ///
  ///
  pub fn source(&self) -> ::qt_core::url::Url {
    {
      let mut object: ::qt_core::url::Url =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTextBrowser_source_to_output(self as *const ::text_browser::TextBrowser, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTextBrowser::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTextBrowser_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTextBrowser::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTextBrowser_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::text_browser::TextBrowser {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QTextBrowser_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TextBrowser`.
  pub struct Signals<'a>(&'a ::text_browser::TextBrowser);
  /// Represents a built-in Qt signal `QTextBrowser::backwardAvailable`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.signals().backward_available()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct BackwardAvailable<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for BackwardAvailable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2backwardAvailable(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BackwardAvailable<'a> {}
  /// Represents a built-in Qt signal `QTextBrowser::selectionChanged`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.signals().selection_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct SelectionChanged<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for SelectionChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2selectionChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SelectionChanged<'a> {}
  /// Represents a built-in Qt signal `QTextBrowser::redoAvailable`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.signals().redo_available()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct RedoAvailable<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for RedoAvailable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2redoAvailable(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RedoAvailable<'a> {}
  /// Represents a built-in Qt signal `QTextBrowser::undoAvailable`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.signals().undo_available()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct UndoAvailable<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for UndoAvailable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2undoAvailable(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for UndoAvailable<'a> {}
  /// Represents a built-in Qt signal `QTextBrowser::historyChanged`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.signals().history_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct HistoryChanged<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for HistoryChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2historyChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HistoryChanged<'a> {}
  /// Represents a built-in Qt signal `QTextBrowser::anchorClicked`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.signals().anchor_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct AnchorClicked<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for AnchorClicked<'a> {
    type Arguments = (&'static ::qt_core::url::Url,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2anchorClicked(const QUrl&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AnchorClicked<'a> {}
  /// Represents a built-in Qt signal `QTextBrowser::copyAvailable`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.signals().copy_available()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct CopyAvailable<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for CopyAvailable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2copyAvailable(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CopyAvailable<'a> {}
  /// Represents a built-in Qt signal `QTextBrowser::highlighted`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.signals().highlighted_qt_core_url_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct HighlightedQtCoreUrlRef<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for HighlightedQtCoreUrlRef<'a> {
    type Arguments = (&'static ::qt_core::url::Url,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2highlighted(const QUrl&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HighlightedQtCoreUrlRef<'a> {}
  /// Represents a built-in Qt signal `QTextBrowser::highlighted`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.signals().highlighted_qt_core_string_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct HighlightedQtCoreStringRef<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for HighlightedQtCoreStringRef<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2highlighted(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HighlightedQtCoreStringRef<'a> {}
  /// Represents a built-in Qt signal `QTextBrowser::cursorPositionChanged`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.signals().cursor_position_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct CursorPositionChanged<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for CursorPositionChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2cursorPositionChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CursorPositionChanged<'a> {}
  /// Represents a built-in Qt signal `QTextBrowser::forwardAvailable`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.signals().forward_available()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct ForwardAvailable<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for ForwardAvailable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2forwardAvailable(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ForwardAvailable<'a> {}
  /// Represents a built-in Qt signal `QTextBrowser::textChanged`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.signals().text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct TextChanged<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for TextChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2textChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TextChanged<'a> {}
  /// Represents a built-in Qt signal `QTextBrowser::sourceChanged`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.signals().source_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct SourceChanged<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for SourceChanged<'a> {
    type Arguments = (&'static ::qt_core::url::Url,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sourceChanged(const QUrl&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SourceChanged<'a> {}
  /// Represents a built-in Qt signal `QTextBrowser::currentCharFormatChanged`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.signals().current_char_format_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct CurrentCharFormatChanged<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for CurrentCharFormatChanged<'a> {
    type Arguments = (&'static ::qt_gui::text_char_format::TextCharFormat,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentCharFormatChanged(const QTextCharFormat&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentCharFormatChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QTextBrowser::backwardAvailable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn backward_available(&self) -> BackwardAvailable {
      BackwardAvailable(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextBrowser::selectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn selection_changed(&self) -> SelectionChanged {
      SelectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextBrowser::redoAvailable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn redo_available(&self) -> RedoAvailable {
      RedoAvailable(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextBrowser::undoAvailable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn undo_available(&self) -> UndoAvailable {
      UndoAvailable(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextBrowser::historyChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn history_changed(&self) -> HistoryChanged {
      HistoryChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextBrowser::anchorClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn anchor_clicked(&self) -> AnchorClicked {
      AnchorClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextBrowser::copyAvailable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn copy_available(&self) -> CopyAvailable {
      CopyAvailable(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextBrowser::highlighted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn highlighted_qt_core_url_ref(&self) -> HighlightedQtCoreUrlRef {
      HighlightedQtCoreUrlRef(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextBrowser::highlighted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn highlighted_qt_core_string_ref(&self) -> HighlightedQtCoreStringRef {
      HighlightedQtCoreStringRef(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextBrowser::cursorPositionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cursor_position_changed(&self) -> CursorPositionChanged {
      CursorPositionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextBrowser::forwardAvailable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn forward_available(&self) -> ForwardAvailable {
      ForwardAvailable(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextBrowser::textChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn text_changed(&self) -> TextChanged {
      TextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextBrowser::sourceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_changed(&self) -> SourceChanged {
      SourceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextBrowser::currentCharFormatChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_char_format_changed(&self) -> CurrentCharFormatChanged {
      CurrentCharFormatChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `TextBrowser`.
  pub struct Slots<'a>(&'a ::text_browser::TextBrowser);
  /// Represents a built-in Qt slot `QTextBrowser::setFontWeight`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().set_font_weight()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct SetFontWeight<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for SetFontWeight<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFontWeight(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::redo`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().redo()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct Redo<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for Redo<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1redo()\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::clear`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().clear()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct Clear<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for Clear<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clear()\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::setFontPointSize`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().set_font_point_size()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct SetFontPointSize<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for SetFontPointSize<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFontPointSize(double)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::undo`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().undo()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct Undo<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for Undo<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1undo()\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::insertHtml`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().insert_html()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct InsertHtml<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for InsertHtml<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1insertHtml(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::append`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().append()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct Append<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for Append<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1append(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::setPlainText`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().set_plain_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct SetPlainText<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for SetPlainText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPlainText(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::zoomIn`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().zoom_in()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct ZoomIn<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for ZoomIn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1zoomIn(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::setTextBackgroundColor`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().set_text_background_color()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct SetTextBackgroundColor<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for SetTextBackgroundColor<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTextBackgroundColor(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::paste`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().paste()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct Paste<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for Paste<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1paste()\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::setSource`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().set_source()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct SetSource<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for SetSource<'a> {
    type Arguments = (&'static ::qt_core::url::Url,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSource(const QUrl&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::insertPlainText`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().insert_plain_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct InsertPlainText<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for InsertPlainText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1insertPlainText(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::backward`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().backward()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct Backward<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for Backward<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1backward()\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::forward`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().forward()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct Forward<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for Forward<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1forward()\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::setFontUnderline`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().set_font_underline()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct SetFontUnderline<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for SetFontUnderline<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFontUnderline(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::setHtml`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().set_html()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct SetHtml<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for SetHtml<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHtml(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::setText`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().set_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct SetText<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for SetText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setText(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::copy`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().copy()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct Copy<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for Copy<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1copy()\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::setCurrentFont`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().set_current_font()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct SetCurrentFont<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentFont<'a> {
    type Arguments = (&'static ::qt_gui::font::Font,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentFont(const QFont&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::reload`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().reload()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct Reload<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for Reload<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reload()\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::home`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().home()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct Home<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for Home<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1home()\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::zoomOut`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().zoom_out()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct ZoomOut<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for ZoomOut<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1zoomOut(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::setFontItalic`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().set_font_italic()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct SetFontItalic<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for SetFontItalic<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFontItalic(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::setFontFamily`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().set_font_family()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct SetFontFamily<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for SetFontFamily<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFontFamily(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::cut`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().cut()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct Cut<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for Cut<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1cut()\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::selectAll`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().select_all()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct SelectAll<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for SelectAll<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1selectAll()\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::scrollToAnchor`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().scroll_to_anchor()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct ScrollToAnchor<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for ScrollToAnchor<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollToAnchor(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextBrowser::setTextColor`.
  ///
  /// An object of this type can be created from `TextBrowser` with `object.slots().set_text_color()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextBrowser` object.
  pub struct SetTextColor<'a>(&'a ::text_browser::TextBrowser);
  impl<'a> ::qt_core::connection::Receiver for SetTextColor<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTextColor(const QColor&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QTextBrowser::setFontWeight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_font_weight(&self) -> SetFontWeight {
      SetFontWeight(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::redo`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn redo(&self) -> Redo {
      Redo(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::clear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear(&self) -> Clear {
      Clear(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::setFontPointSize`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_font_point_size(&self) -> SetFontPointSize {
      SetFontPointSize(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::undo`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn undo(&self) -> Undo {
      Undo(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::insertHtml`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn insert_html(&self) -> InsertHtml {
      InsertHtml(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::append`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn append(&self) -> Append {
      Append(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::setPlainText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_plain_text(&self) -> SetPlainText {
      SetPlainText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::zoomIn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn zoom_in(&self) -> ZoomIn {
      ZoomIn(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::setTextBackgroundColor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_text_background_color(&self) -> SetTextBackgroundColor {
      SetTextBackgroundColor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::paste`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn paste(&self) -> Paste {
      Paste(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::setSource`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_source(&self) -> SetSource {
      SetSource(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::insertPlainText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn insert_plain_text(&self) -> InsertPlainText {
      InsertPlainText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::backward`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn backward(&self) -> Backward {
      Backward(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::forward`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn forward(&self) -> Forward {
      Forward(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::setFontUnderline`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_font_underline(&self) -> SetFontUnderline {
      SetFontUnderline(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::setHtml`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_html(&self) -> SetHtml {
      SetHtml(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::setText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_text(&self) -> SetText {
      SetText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::copy`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn copy(&self) -> Copy {
      Copy(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::setCurrentFont`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_font(&self) -> SetCurrentFont {
      SetCurrentFont(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::reload`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reload(&self) -> Reload {
      Reload(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::home`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn home(&self) -> Home {
      Home(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::zoomOut`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn zoom_out(&self) -> ZoomOut {
      ZoomOut(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::setFontItalic`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_font_italic(&self) -> SetFontItalic {
      SetFontItalic(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::setFontFamily`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_font_family(&self) -> SetFontFamily {
      SetFontFamily(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::cut`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cut(&self) -> Cut {
      Cut(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::selectAll`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn select_all(&self) -> SelectAll {
      SelectAll(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::scrollToAnchor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_anchor(&self) -> ScrollToAnchor {
      ScrollToAnchor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextBrowser::setTextColor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_text_color(&self) -> SetTextColor {
      SetTextColor(self.0)
    }
  }
  impl ::text_browser::TextBrowser {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::text_browser::TextBrowser> for ::abstract_scroll_area::AbstractScrollArea {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::text_browser::TextBrowser> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextBrowser_G_dynamic_cast_QTextBrowser_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::text_browser::TextBrowser> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextBrowser_G_dynamic_cast_QTextBrowser_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::text_browser::TextBrowser> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::text_browser::TextBrowser> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTextBrowser_G_dynamic_cast_QTextBrowser_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::text_browser::TextBrowser> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextBrowser_G_dynamic_cast_QTextBrowser_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::text_browser::TextBrowser> for ::text_edit::TextEdit {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::text_browser::TextBrowser> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTextBrowser_G_dynamic_cast_QTextBrowser_ptr_QTextEdit(self as *mut ::text_edit::TextEdit)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::text_browser::TextBrowser> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextBrowser_G_dynamic_cast_QTextBrowser_ptr_QTextEdit(self as *const ::text_edit::TextEdit as *mut ::text_edit::TextEdit) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::text_browser::TextBrowser> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::text_browser::TextBrowser> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTextBrowser_G_dynamic_cast_QTextBrowser_ptr_QWidget(self as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::text_browser::TextBrowser> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextBrowser_G_dynamic_cast_QTextBrowser_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::text_browser::TextBrowser {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QObject_ptr(self as *mut ::text_browser::TextBrowser) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QObject_ptr(self as *const ::text_browser::TextBrowser as *mut ::text_browser::TextBrowser) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::text_browser::TextBrowser {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QPaintDevice_ptr(self as *mut ::text_browser::TextBrowser)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QPaintDevice_ptr(self as *const ::text_browser::TextBrowser as *mut ::text_browser::TextBrowser) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_scroll_area::AbstractScrollArea> for ::text_browser::TextBrowser {
  fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::text_browser::TextBrowser)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QAbstractScrollArea_ptr(self as *const ::text_browser::TextBrowser as *mut ::text_browser::TextBrowser) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::text_browser::TextBrowser {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QFrame_ptr(self as *mut ::text_browser::TextBrowser) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QFrame_ptr(self as *const ::text_browser::TextBrowser as *mut ::text_browser::TextBrowser) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::text_edit::TextEdit> for ::text_browser::TextBrowser {
  fn static_cast_mut(&mut self) -> &mut ::text_edit::TextEdit {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QTextEdit_ptr(self as *mut ::text_browser::TextBrowser) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::text_edit::TextEdit {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QTextEdit_ptr(self as *const ::text_browser::TextBrowser as *mut ::text_browser::TextBrowser) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::text_browser::TextBrowser {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QWidget_ptr(self as *mut ::text_browser::TextBrowser) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QWidget_ptr(self as *const ::text_browser::TextBrowser as *mut ::text_browser::TextBrowser) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_browser::TextBrowser> for ::abstract_scroll_area::AbstractScrollArea {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_browser::TextBrowser {
    let ffi_result = ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_browser::TextBrowser {
    let ffi_result = ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_browser::TextBrowser> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_browser::TextBrowser {
    let ffi_result =
      ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_browser::TextBrowser {
    let ffi_result = ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_browser::TextBrowser> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_browser::TextBrowser {
    let ffi_result =
      ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_browser::TextBrowser {
    let ffi_result = ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_browser::TextBrowser> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_browser::TextBrowser {
    let ffi_result = ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_browser::TextBrowser {
    let ffi_result = ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_browser::TextBrowser> for ::text_edit::TextEdit {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_browser::TextBrowser {
    let ffi_result =
      ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QTextEdit(self as *mut ::text_edit::TextEdit);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_browser::TextBrowser {
    let ffi_result = ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QTextEdit(self as *const ::text_edit::TextEdit as *mut ::text_edit::TextEdit);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_browser::TextBrowser> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_browser::TextBrowser {
    let ffi_result =
      ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_browser::TextBrowser {
    let ffi_result = ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QTextBrowser_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::text_browser::TextBrowser {
  type Target = ::text_edit::TextEdit;
  fn deref(&self) -> &::text_edit::TextEdit {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QTextEdit_ptr(self as *const ::text_browser::TextBrowser as *mut ::text_browser::TextBrowser) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::text_browser::TextBrowser {
  fn deref_mut(&mut self) -> &mut ::text_edit::TextEdit {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTextBrowser_G_static_cast_QTextEdit_ptr(self as *mut ::text_browser::TextBrowser) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
