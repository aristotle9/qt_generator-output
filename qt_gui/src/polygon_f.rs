/// C++ type: <span style='color: green;'>```QPolygonF```</span>
#[repr(C)]
pub struct PolygonF([u8; ::type_sizes::QT_GUI_POLYGON_F_POLYGON_F]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PolygonF {
  unsafe fn new_uninitialized() -> PolygonF {
    PolygonF(::std::mem::uninitialized())
  }
}

impl PolygonF {
  /// C++ method: <span style='color: green;'>```QVariant QPolygonF::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPolygonF_convert_to_QVariant_to_output(self as *const ::polygon_f::PolygonF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QPolygonF::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPolygonF_boundingRect_to_output(self as *const ::polygon_f::PolygonF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPolygonF::containsPoint(const QPointF& pt, Qt::FillRule fillRule) const```</span>
  ///
  ///
  pub fn contains_point(&self, pt: &::qt_core::point_f::PointF, fill_rule: &::qt_core::qt::FillRule) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPolygonF_containsPoint(self as *const ::polygon_f::PolygonF,
                                              pt as *const ::qt_core::point_f::PointF,
                                              fill_rule as *const ::qt_core::qt::FillRule)
    }
  }

  /// C++ method: <span style='color: green;'>```QPolygonF QPolygonF::intersected(const QPolygonF& r) const```</span>
  ///
  ///
  pub fn intersected(&self, r: &::polygon_f::PolygonF) -> ::polygon_f::PolygonF {
    {
      let mut object: ::polygon_f::PolygonF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPolygonF_intersected_to_output(self as *const ::polygon_f::PolygonF,
                                                        r as *const ::polygon_f::PolygonF,
                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPolygonF::isClosed() const```</span>
  ///
  ///
  pub fn is_closed(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPolygonF_isClosed(self as *const ::polygon_f::PolygonF) }
  }

  /// C++ method: <span style='color: green;'>```QPolygonF::QPolygonF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPolygonF::QPolygonF()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::polygon::Polygon) -> ::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPolygonF::QPolygonF(const QPolygon& a)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::polygon_f::PolygonF) -> ::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPolygonF::QPolygonF(const QPolygonF& a)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::qt_core::rect_f::RectF) -> ::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPolygonF::QPolygonF(const QRectF& r)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new(&::qt_core::vector::VectorPointF) -> ::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPolygonF::QPolygonF(const QVector<QPointF>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPolygonF::QPolygonF(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::polygon_f::PolygonF
    where Args: overloading::PolygonFNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPolygonF& QPolygonF::operator=(const QPolygonF& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::polygon_f::PolygonF) -> &'l0 mut ::polygon_f::PolygonF {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QPolygonF_operator_assign(self as *mut ::polygon_f::PolygonF,
                                                other as *const ::polygon_f::PolygonF)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPolygonF QPolygonF::subtracted(const QPolygonF& r) const```</span>
  ///
  ///
  pub fn subtracted(&self, r: &::polygon_f::PolygonF) -> ::polygon_f::PolygonF {
    {
      let mut object: ::polygon_f::PolygonF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPolygonF_subtracted_to_output(self as *const ::polygon_f::PolygonF,
                                                       r as *const ::polygon_f::PolygonF,
                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QPolygonF::swap(QPolygonF& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::polygon_f::PolygonF) {
    unsafe {
      ::ffi::qt_gui_c_QPolygonF_swap(self as *mut ::polygon_f::PolygonF,
                                     other as *mut ::polygon_f::PolygonF)
    }
  }

  /// C++ method: <span style='color: green;'>```QPolygon QPolygonF::toPolygon() const```</span>
  ///
  ///
  pub fn to_polygon(&self) -> ::polygon::Polygon {
    {
      let mut object: ::polygon::Polygon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPolygonF_toPolygon_to_output(self as *const ::polygon_f::PolygonF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPolygonF::translate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translate(&mut self, &::qt_core::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPolygonF::translate(const QPointF& offset)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translate(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPolygonF::translate(double dx, double dy)```</span>
  ///
  ///
  pub fn translate<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PolygonFTranslateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPolygonF::translated```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translated(&self, &::qt_core::point_f::PointF) -> ::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QPolygonF::translated(const QPointF& offset) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translated(&self, (::libc::c_double, ::libc::c_double)) -> ::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QPolygonF::translated(double dx, double dy) const```</span>
  ///
  ///
  pub fn translated<'largs, Args>(&'largs self, args: Args) -> ::polygon_f::PolygonF
    where Args: overloading::PolygonFTranslatedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPolygonF QPolygonF::united(const QPolygonF& r) const```</span>
  ///
  ///
  pub fn united(&self, r: &::polygon_f::PolygonF) -> ::polygon_f::PolygonF {
    {
      let mut object: ::polygon_f::PolygonF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPolygonF_united_to_output(self as *const ::polygon_f::PolygonF,
                                                   r as *const ::polygon_f::PolygonF,
                                                   &mut object);
      }
      object
    }
  }
}

impl Drop for ::polygon_f::PolygonF {
  /// C++ method: <span style='color: green;'>```[destructor] void QPolygonF::~QPolygonF()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPolygonF_destructor(self as *mut ::polygon_f::PolygonF) }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::vector::VectorPointF> for ::polygon_f::PolygonF {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::vector::VectorPointF {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QPolygonF_G_static_cast_QVector_QPointF_ptr(self as *mut ::polygon_f::PolygonF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::vector::VectorPointF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPolygonF_G_static_cast_QVector_QPointF_ptr(self as *const ::polygon_f::PolygonF as *mut ::polygon_f::PolygonF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::polygon_f::PolygonF> for ::qt_core::vector::VectorPointF {
  unsafe fn static_cast_mut(&mut self) -> &mut ::polygon_f::PolygonF {
    let ffi_result =
      ::ffi::qt_gui_c_QPolygonF_G_static_cast_QPolygonF_ptr(self as *mut ::qt_core::vector::VectorPointF);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::polygon_f::PolygonF {
    let ffi_result = ::ffi::qt_gui_c_QPolygonF_G_static_cast_QPolygonF_ptr(self as *const ::qt_core::vector::VectorPointF as *mut ::qt_core::vector::VectorPointF);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::polygon_f::PolygonF {
  type Target = ::qt_core::vector::VectorPointF;
  fn deref(&self) -> &::qt_core::vector::VectorPointF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPolygonF_G_static_cast_QVector_QPointF_ptr(self as *const ::polygon_f::PolygonF as *mut ::polygon_f::PolygonF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::polygon_f::PolygonF {
  fn deref_mut(&mut self) -> &mut ::qt_core::vector::VectorPointF {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QPolygonF_G_static_cast_QVector_QPointF_ptr(self as *mut ::polygon_f::PolygonF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PolygonF::new](../struct.PolygonF.html#method.new) method.
  pub trait PolygonFNewArgs {
    fn exec(self) -> ::polygon_f::PolygonF;
  }
  impl<'a> PolygonFNewArgs for &'a ::polygon::Polygon {
    fn exec(self) -> ::polygon_f::PolygonF {
      let a = self;
      {
        let mut object: ::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPolygonF_constructor_QPolygon(a as *const ::polygon::Polygon, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PolygonFNewArgs for &'a ::polygon_f::PolygonF {
    fn exec(self) -> ::polygon_f::PolygonF {
      let a = self;
      {
        let mut object: ::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPolygonF_constructor_QPolygonF(a as *const ::polygon_f::PolygonF, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PolygonFNewArgs for &'a ::qt_core::rect_f::RectF {
    fn exec(self) -> ::polygon_f::PolygonF {
      let r = self;
      {
        let mut object: ::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPolygonF_constructor_QRectF(r as *const ::qt_core::rect_f::RectF, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PolygonFNewArgs for &'a ::qt_core::vector::VectorPointF {
    fn exec(self) -> ::polygon_f::PolygonF {
      let v = self;
      {
        let mut object: ::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPolygonF_constructor_QVector_QPointF(v as *const ::qt_core::vector::VectorPointF,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl PolygonFNewArgs for ::libc::c_int {
    fn exec(self) -> ::polygon_f::PolygonF {
      let size = self;
      {
        let mut object: ::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPolygonF_constructor_int(size, &mut object);
        }
        object
      }
    }
  }
  impl PolygonFNewArgs for () {
    fn exec(self) -> ::polygon_f::PolygonF {

      {
        let mut object: ::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPolygonF_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PolygonF::translate](../struct.PolygonF.html#method.translate) method.
  pub trait PolygonFTranslateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::polygon_f::PolygonF) -> ();
  }
  impl<'largs> PolygonFTranslateArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::polygon_f::PolygonF) -> () {
      let dx = self.0;
      let dy = self.1;
      unsafe { ::ffi::qt_gui_c_QPolygonF_translate_dx_dy(original_self as *mut ::polygon_f::PolygonF, dx, dy) }
    }
  }
  impl<'largs> PolygonFTranslateArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::polygon_f::PolygonF) -> () {
      let offset = self;
      unsafe {
        ::ffi::qt_gui_c_QPolygonF_translate_offset(original_self as *mut ::polygon_f::PolygonF,
                                                   offset as *const ::qt_core::point_f::PointF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PolygonF::translated](../struct.PolygonF.html#method.translated) method.
  pub trait PolygonFTranslatedArgs<'largs> {
    fn exec(self, original_self: &'largs ::polygon_f::PolygonF) -> ::polygon_f::PolygonF;
  }
  impl<'largs> PolygonFTranslatedArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs ::polygon_f::PolygonF) -> ::polygon_f::PolygonF {
      let dx = self.0;
      let dy = self.1;
      {
        let mut object: ::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPolygonF_translated_to_output_dx_dy(original_self as *const ::polygon_f::PolygonF,
                                                               dx,
                                                               dy,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PolygonFTranslatedArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs ::polygon_f::PolygonF) -> ::polygon_f::PolygonF {
      let offset = self;
      {
        let mut object: ::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPolygonF_translated_to_output_offset(original_self as *const ::polygon_f::PolygonF,
                                                                offset as *const ::qt_core::point_f::PointF,
                                                                &mut object);
        }
        object
      }
    }
  }
}
