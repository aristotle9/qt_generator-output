/// C++ type: <span style='color: green;'>```Qt3DExtras::QText2DEntity```</span>
#[repr(C)]
pub struct Text2DEntity(u8);

impl Text2DEntity {
  /// C++ method: <span style='color: green;'>```QColor Qt3DExtras::QText2DEntity::color() const```</span>
  ///
  ///
  pub fn color(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QText2DEntity_color_to_output(self as *const ::text_2d_entity::Text2DEntity,
                                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFont Qt3DExtras::QText2DEntity::font() const```</span>
  ///
  ///
  pub fn font(&self) -> ::qt_gui::font::Font {
    {
      let mut object: ::qt_gui::font::Font =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QText2DEntity_font_to_output(self as *const ::text_2d_entity::Text2DEntity,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QText2DEntity::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QText2DEntity_height(self as *const ::text_2d_entity::Text2DEntity) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QText2DEntity::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QText2DEntity_metaObject(self as *const ::text_2d_entity::Text2DEntity) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QText2DEntity::QText2DEntity()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::text_2d_entity::Text2DEntity> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QText2DEntity_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QText2DEntity::QText2DEntity(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::text_2d_entity::Text2DEntity> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QText2DEntity_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QText2DEntity::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QText2DEntity_qt_metacall(self as *mut ::text_2d_entity::Text2DEntity,
                                                               arg1 as *const ::qt_core::meta_object::Call,
                                                               arg2,
                                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QText2DEntity::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QText2DEntity_qt_metacast(self as *mut ::text_2d_entity::Text2DEntity, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QText2DEntity::setColor(const QColor& color)```</span>
  ///
  ///
  pub fn set_color(&mut self, color: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QText2DEntity_setColor(self as *mut ::text_2d_entity::Text2DEntity,
                                                              color as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QText2DEntity::setFont(const QFont& font)```</span>
  ///
  ///
  pub fn set_font(&mut self, font: &::qt_gui::font::Font) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QText2DEntity_setFont(self as *mut ::text_2d_entity::Text2DEntity,
                                                             font as *const ::qt_gui::font::Font)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QText2DEntity::setHeight(float height)```</span>
  ///
  ///
  pub fn set_height(&mut self, height: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QText2DEntity_setHeight(self as *mut ::text_2d_entity::Text2DEntity, height)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QText2DEntity::setText(const QString& text)```</span>
  ///
  ///
  pub fn set_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QText2DEntity_setText(self as *mut ::text_2d_entity::Text2DEntity,
                                                             text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QText2DEntity::setWidth(float width)```</span>
  ///
  ///
  pub fn set_width(&mut self, width: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QText2DEntity_setWidth(self as *mut ::text_2d_entity::Text2DEntity, width)
    }
  }

  /// C++ method: <span style='color: green;'>```QString Qt3DExtras::QText2DEntity::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QText2DEntity_text_to_output(self as *const ::text_2d_entity::Text2DEntity,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QText2DEntity::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QText2DEntity_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QText2DEntity::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QText2DEntity_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QText2DEntity::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QText2DEntity_width(self as *const ::text_2d_entity::Text2DEntity) }
  }
}

impl ::cpp_utils::CppDeletable for ::text_2d_entity::Text2DEntity {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QText2DEntity_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Text2DEntity`.
  pub struct Signals<'a>(&'a ::text_2d_entity::Text2DEntity);
  /// Represents a built-in Qt signal `Qt3DExtras::QText2DEntity::textChanged`.
  ///
  /// An object of this type can be created from `Text2DEntity` with `object.signals().text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Text2DEntity` object.
  pub struct TextChanged<'a>(&'a ::text_2d_entity::Text2DEntity);
  impl<'a> ::qt_core::connection::Receiver for TextChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2textChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TextChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QText2DEntity::fontChanged`.
  ///
  /// An object of this type can be created from `Text2DEntity` with `object.signals().font_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Text2DEntity` object.
  pub struct FontChanged<'a>(&'a ::text_2d_entity::Text2DEntity);
  impl<'a> ::qt_core::connection::Receiver for FontChanged<'a> {
    type Arguments = (&'static ::qt_gui::font::Font,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2fontChanged(const QFont&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FontChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QText2DEntity::widthChanged`.
  ///
  /// An object of this type can be created from `Text2DEntity` with `object.signals().width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Text2DEntity` object.
  pub struct WidthChanged<'a>(&'a ::text_2d_entity::Text2DEntity);
  impl<'a> ::qt_core::connection::Receiver for WidthChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2widthChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WidthChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QText2DEntity::colorChanged`.
  ///
  /// An object of this type can be created from `Text2DEntity` with `object.signals().color_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Text2DEntity` object.
  pub struct ColorChanged<'a>(&'a ::text_2d_entity::Text2DEntity);
  impl<'a> ::qt_core::connection::Receiver for ColorChanged<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2colorChanged(const QColor&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ColorChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QText2DEntity::heightChanged`.
  ///
  /// An object of this type can be created from `Text2DEntity` with `object.signals().height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Text2DEntity` object.
  pub struct HeightChanged<'a>(&'a ::text_2d_entity::Text2DEntity);
  impl<'a> ::qt_core::connection::Receiver for HeightChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2heightChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HeightChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QText2DEntity::textChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn text_changed(&self) -> TextChanged {
      TextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QText2DEntity::fontChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn font_changed(&self) -> FontChanged {
      FontChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QText2DEntity::widthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn width_changed(&self) -> WidthChanged {
      WidthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QText2DEntity::colorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn color_changed(&self) -> ColorChanged {
      ColorChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QText2DEntity::heightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn height_changed(&self) -> HeightChanged {
      HeightChanged(self.0)
    }
  }
  impl ::text_2d_entity::Text2DEntity {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_3d_core::entity::Entity> for ::text_2d_entity::Text2DEntity {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::entity::Entity {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QText2DEntity_G_static_cast_Qt3DCore_QEntity_ptr(self as *mut ::text_2d_entity::Text2DEntity) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::entity::Entity {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QText2DEntity_G_static_cast_Qt3DCore_QEntity_ptr(self as *const ::text_2d_entity::Text2DEntity as *mut ::text_2d_entity::Text2DEntity) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::text_2d_entity::Text2DEntity {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QText2DEntity_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::text_2d_entity::Text2DEntity) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QText2DEntity_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::text_2d_entity::Text2DEntity as *mut ::text_2d_entity::Text2DEntity) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::text_2d_entity::Text2DEntity {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QText2DEntity_G_static_cast_QObject_ptr(self as *mut ::text_2d_entity::Text2DEntity)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QText2DEntity_G_static_cast_QObject_ptr(self as *const ::text_2d_entity::Text2DEntity as *mut ::text_2d_entity::Text2DEntity) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_2d_entity::Text2DEntity> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_2d_entity::Text2DEntity {
    let ffi_result = ::ffi::qt_3d_extras_c_QText2DEntity_G_static_cast_Qt3DExtras_QText2DEntity_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_2d_entity::Text2DEntity {
    let ffi_result = ::ffi::qt_3d_extras_c_QText2DEntity_G_static_cast_Qt3DExtras_QText2DEntity_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_2d_entity::Text2DEntity> for ::qt_3d_core::entity::Entity {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_2d_entity::Text2DEntity {
    let ffi_result = ::ffi::qt_3d_extras_c_QText2DEntity_G_static_cast_Qt3DExtras_QText2DEntity_ptr_Qt3DCore_QEntity(self as *mut ::qt_3d_core::entity::Entity);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_2d_entity::Text2DEntity {
    let ffi_result = ::ffi::qt_3d_extras_c_QText2DEntity_G_static_cast_Qt3DExtras_QText2DEntity_ptr_Qt3DCore_QEntity(self as *const ::qt_3d_core::entity::Entity as *mut ::qt_3d_core::entity::Entity);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_2d_entity::Text2DEntity> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_2d_entity::Text2DEntity {
    let ffi_result = ::ffi::qt_3d_extras_c_QText2DEntity_G_static_cast_Qt3DExtras_QText2DEntity_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_2d_entity::Text2DEntity {
    let ffi_result = ::ffi::qt_3d_extras_c_QText2DEntity_G_static_cast_Qt3DExtras_QText2DEntity_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::text_2d_entity::Text2DEntity {
  type Target = ::qt_3d_core::entity::Entity;
  fn deref(&self) -> &::qt_3d_core::entity::Entity {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QText2DEntity_G_static_cast_Qt3DCore_QEntity_ptr(self as *const ::text_2d_entity::Text2DEntity as *mut ::text_2d_entity::Text2DEntity) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::text_2d_entity::Text2DEntity {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::entity::Entity {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QText2DEntity_G_static_cast_Qt3DCore_QEntity_ptr(self as *mut ::text_2d_entity::Text2DEntity) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
