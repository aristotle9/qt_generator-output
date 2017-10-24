/// C++ type: <span style='color: green;'>```Qt3DRender::QLevelOfDetailBoundingSphere```</span>
#[repr(C)]
pub struct LevelOfDetailBoundingSphere([u8; ::type_sizes::QT_3D_RENDER_LEVEL_OF_DETAIL_BOUNDING_SPHERE_LEVEL_OF_DETAIL_BOUNDING_SPHERE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for LevelOfDetailBoundingSphere {
  unsafe fn new_uninitialized() -> LevelOfDetailBoundingSphere {
    LevelOfDetailBoundingSphere(::std::mem::uninitialized())
  }
}

impl LevelOfDetailBoundingSphere {
  /// C++ method: <span style='color: green;'>```QVector3D Qt3DRender::QLevelOfDetailBoundingSphere::center() const```</span>
  ///
  ///
  pub fn center(&self) -> ::cpp_utils::CppBox<::qt_gui::vector_3d::Vector3D> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_center_as_ptr(self as *const ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QLevelOfDetailBoundingSphere::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_isEmpty(self as *const ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLevelOfDetailBoundingSphere::QLevelOfDetailBoundingSphere```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QLevelOfDetailBoundingSphere::QLevelOfDetailBoundingSphere()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_gui::vector_3d::Vector3D) -> ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QLevelOfDetailBoundingSphere::QLevelOfDetailBoundingSphere(QVector3D center = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::qt_gui::vector_3d::Vector3D, ::libc::c_float)) -> ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QLevelOfDetailBoundingSphere::QLevelOfDetailBoundingSphere(QVector3D center = ?, float radius = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) -> ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QLevelOfDetailBoundingSphere::QLevelOfDetailBoundingSphere(const Qt3DRender::QLevelOfDetailBoundingSphere& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere
    where Args: overloading::LevelOfDetailBoundingSphereNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QLevelOfDetailBoundingSphere& Qt3DRender::QLevelOfDetailBoundingSphere::operator=(const Qt3DRender::QLevelOfDetailBoundingSphere& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere)
                             -> &'l0 mut ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_operator_assign(self as *mut ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere, other as *const ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QLevelOfDetailBoundingSphere::operator==(const Qt3DRender::QLevelOfDetailBoundingSphere& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_operator_eq(self as *const ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere, other as *const ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QLevelOfDetailBoundingSphere::operator!=(const Qt3DRender::QLevelOfDetailBoundingSphere& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_operator_neq(self as *const ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere, other as *const ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QLevelOfDetailBoundingSphere::radius() const```</span>
  ///
  ///
  pub fn radius(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_radius(self as *const ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) }
  }
}

impl Drop for ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere {
  /// C++ method: <span style='color: green;'>```[destructor] void Qt3DRender::QLevelOfDetailBoundingSphere::~QLevelOfDetailBoundingSphere()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_destructor(self as *mut ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [LevelOfDetailBoundingSphere::new](../struct.LevelOfDetailBoundingSphere.html#method.new) method.
  pub trait LevelOfDetailBoundingSphereNewArgs {
    fn exec(self) -> ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere;
  }
  impl<'a> LevelOfDetailBoundingSphereNewArgs for &'a ::qt_gui::vector_3d::Vector3D {
    fn exec(self) -> ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere {
      let center = self;
      {
        let mut object: ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_constructor_center(center as *const ::qt_gui::vector_3d::Vector3D, &mut object);
        }
        object
      }
    }
  }
  impl<'a> LevelOfDetailBoundingSphereNewArgs for (&'a ::qt_gui::vector_3d::Vector3D, ::libc::c_float) {
    fn exec(self) -> ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere {
      let center = self.0;
      let radius = self.1;
      {
        let mut object: ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_constructor_center_radius(center as *const ::qt_gui::vector_3d::Vector3D, radius, &mut object);
        }
        object
      }
    }
  }
  impl LevelOfDetailBoundingSphereNewArgs for () {
    fn exec(self) -> ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere {

      {
        let mut object: ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> LevelOfDetailBoundingSphereNewArgs for &'a ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere {
    fn exec(self) -> ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere {
      let other = self;
      {
        let mut object: ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_Qt3DRender_QLevelOfDetailBoundingSphere_constructor_other(other as *const ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere, &mut object);
        }
        object
      }
    }
  }
}
