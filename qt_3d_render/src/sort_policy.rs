/// C++ type: <span style='color: green;'>```Qt3DRender::QSortPolicy```</span>
#[repr(C)]
pub struct SortPolicy(u8);

impl SortPolicy {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QSortPolicy::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QSortPolicy_metaObject(self as *const ::sort_policy::SortPolicy) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QSortPolicy::QSortPolicy()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::sort_policy::SortPolicy> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QSortPolicy_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QSortPolicy::QSortPolicy(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::sort_policy::SortPolicy> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QSortPolicy_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QSortPolicy::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QSortPolicy_qt_metacall(self as *mut ::sort_policy::SortPolicy,
                                                             arg1 as *const ::qt_core::meta_object::Call,
                                                             arg2,
                                                             arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QSortPolicy::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QSortPolicy_qt_metacast(self as *mut ::sort_policy::SortPolicy, arg1)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QSortPolicy::setSortTypes```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_sort_types(&mut self, &::vector::VectorSortPolicySortType) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QSortPolicy::setSortTypes(const QVector<Qt3DRender::QSortPolicy::SortType>& sortTypes)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_sort_types(&mut self, &::qt_core::vector::VectorCInt) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QSortPolicy::setSortTypes(const QVector<int>& sortTypesInt)```</span>
  ///
  ///
  pub fn set_sort_types<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::SortPolicySetSortTypesArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType> Qt3DRender::QSortPolicy::sortTypes() const```</span>
  ///
  ///
  pub fn sort_types(&self) -> ::vector::VectorSortPolicySortType {
    {
      let mut object: ::vector::VectorSortPolicySortType =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QSortPolicy_sortTypes_to_output(self as *const ::sort_policy::SortPolicy,
                                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<int> Qt3DRender::QSortPolicy::sortTypesInt() const```</span>
  ///
  ///
  pub fn sort_types_int(&self) -> ::qt_core::vector::VectorCInt {
    {
      let mut object: ::qt_core::vector::VectorCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QSortPolicy_sortTypesInt_to_output(self as *const ::sort_policy::SortPolicy,
                                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QSortPolicy::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QSortPolicy_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QSortPolicy::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QSortPolicy_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::sort_policy::SortPolicy {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QSortPolicy_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `SortPolicy`.
  pub struct Signals<'a>(&'a ::sort_policy::SortPolicy);
  /// Represents a built-in Qt signal `Qt3DRender::QSortPolicy::sortTypesChanged`.
  ///
  /// An object of this type can be created from `SortPolicy` with `object.signals().sort_types_changed_vector_vector_sort_policy_sort_type_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SortPolicy` object.
  pub struct SortTypesChangedVectorVectorSortPolicySortTypeRef<'a>(&'a ::sort_policy::SortPolicy);
  impl<'a> ::qt_core::connection::Receiver for SortTypesChangedVectorVectorSortPolicySortTypeRef<'a> {
    type Arguments = (&'static ::vector::VectorSortPolicySortType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sortTypesChanged(const QVector< Qt3DRender::QSortPolicy::SortType >&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SortTypesChangedVectorVectorSortPolicySortTypeRef<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QSortPolicy::sortTypesChanged`.
  ///
  /// An object of this type can be created from `SortPolicy` with `object.signals().sort_types_changed_qt_core_vector_vector_c_int_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SortPolicy` object.
  pub struct SortTypesChangedQtCoreVectorVectorCIntRef<'a>(&'a ::sort_policy::SortPolicy);
  impl<'a> ::qt_core::connection::Receiver for SortTypesChangedQtCoreVectorVectorCIntRef<'a> {
    type Arguments = (&'static ::qt_core::vector::VectorCInt,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sortTypesChanged(const QVector< int >&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SortTypesChangedQtCoreVectorVectorCIntRef<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QSortPolicy::sortTypesChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn sort_types_changed_vector_vector_sort_policy_sort_type_ref
      (&self)
       -> SortTypesChangedVectorVectorSortPolicySortTypeRef {
      SortTypesChangedVectorVectorSortPolicySortTypeRef(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QSortPolicy::sortTypesChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn sort_types_changed_qt_core_vector_vector_c_int_ref(&self) -> SortTypesChangedQtCoreVectorVectorCIntRef {
      SortTypesChangedQtCoreVectorVectorCIntRef(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `SortPolicy`.
  pub struct Slots<'a>(&'a ::sort_policy::SortPolicy);
  /// Represents a built-in Qt slot `Qt3DRender::QSortPolicy::setSortTypes`.
  ///
  /// An object of this type can be created from `SortPolicy` with `object.slots().set_sort_types_vector_vector_sort_policy_sort_type_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SortPolicy` object.
  pub struct SetSortTypesVectorVectorSortPolicySortTypeRef<'a>(&'a ::sort_policy::SortPolicy);
  impl<'a> ::qt_core::connection::Receiver for SetSortTypesVectorVectorSortPolicySortTypeRef<'a> {
    type Arguments = (&'static ::vector::VectorSortPolicySortType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSortTypes(const QVector< Qt3DRender::QSortPolicy::SortType >&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QSortPolicy::setSortTypes`.
  ///
  /// An object of this type can be created from `SortPolicy` with `object.slots().set_sort_types_qt_core_vector_vector_c_int_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SortPolicy` object.
  pub struct SetSortTypesQtCoreVectorVectorCIntRef<'a>(&'a ::sort_policy::SortPolicy);
  impl<'a> ::qt_core::connection::Receiver for SetSortTypesQtCoreVectorVectorCIntRef<'a> {
    type Arguments = (&'static ::qt_core::vector::VectorCInt,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSortTypes(const QVector< int >&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QSortPolicy::setSortTypes`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_sort_types_vector_vector_sort_policy_sort_type_ref(&self)
                                                                  -> SetSortTypesVectorVectorSortPolicySortTypeRef {
      SetSortTypesVectorVectorSortPolicySortTypeRef(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QSortPolicy::setSortTypes`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_sort_types_qt_core_vector_vector_c_int_ref(&self) -> SetSortTypesQtCoreVectorVectorCIntRef {
      SetSortTypesQtCoreVectorVectorCIntRef(self.0)
    }
  }
  impl ::sort_policy::SortPolicy {
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

/// C++ type: <span style='color: green;'>```Qt3DRender::QSortPolicy::SortType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SortType {
  /// C++ enum variant: <span style='color: green;'>```StateChangeCost = 1```</span>
  StateChangeCost = 1,
  /// C++ enum variant: <span style='color: green;'>```BackToFront = 2```</span>
  BackToFront = 2,
  /// C++ enum variant: <span style='color: green;'>```Material = 4```</span>
  Material = 4,
}

impl ::cpp_utils::DynamicCast<::sort_policy::SortPolicy> for ::frame_graph_node::FrameGraphNode {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::sort_policy::SortPolicy> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSortPolicy_G_dynamic_cast_Qt3DRender_QSortPolicy_ptr(self as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::sort_policy::SortPolicy> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSortPolicy_G_dynamic_cast_Qt3DRender_QSortPolicy_ptr(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::sort_policy::SortPolicy {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::sort_policy::SortPolicy)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::sort_policy::SortPolicy as *mut ::sort_policy::SortPolicy) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame_graph_node::FrameGraphNode> for ::sort_policy::SortPolicy {
  fn static_cast_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::sort_policy::SortPolicy) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::sort_policy::SortPolicy as *mut ::sort_policy::SortPolicy) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::sort_policy::SortPolicy {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QSortPolicy_G_static_cast_QObject_ptr(self as *mut ::sort_policy::SortPolicy) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSortPolicy_G_static_cast_QObject_ptr(self as *const ::sort_policy::SortPolicy as *mut ::sort_policy::SortPolicy) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::sort_policy::SortPolicy> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::sort_policy::SortPolicy {
    let ffi_result = ::ffi::qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DRender_QSortPolicy_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::sort_policy::SortPolicy {
    let ffi_result = ::ffi::qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DRender_QSortPolicy_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::sort_policy::SortPolicy> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::sort_policy::SortPolicy {
    let ffi_result = ::ffi::qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DRender_QSortPolicy_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::sort_policy::SortPolicy {
    let ffi_result = ::ffi::qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DRender_QSortPolicy_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::sort_policy::SortPolicy> for ::frame_graph_node::FrameGraphNode {
  unsafe fn static_cast_mut(&mut self) -> &mut ::sort_policy::SortPolicy {
    let ffi_result = ::ffi::qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DRender_QSortPolicy_ptr_Qt3DRender_QFrameGraphNode(self as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::sort_policy::SortPolicy {
    let ffi_result = ::ffi::qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DRender_QSortPolicy_ptr_Qt3DRender_QFrameGraphNode(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::sort_policy::SortPolicy {
  type Target = ::frame_graph_node::FrameGraphNode;
  fn deref(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::sort_policy::SortPolicy as *mut ::sort_policy::SortPolicy) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::sort_policy::SortPolicy {
  fn deref_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::sort_policy::SortPolicy) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [SortPolicy::set_sort_types](../struct.SortPolicy.html#method.set_sort_types) method.
  pub trait SortPolicySetSortTypesArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::sort_policy::SortPolicy) -> ();
  }
  impl<'largs> SortPolicySetSortTypesArgs<'largs> for &'largs ::vector::VectorSortPolicySortType {
    fn exec(self, original_self: &'largs mut ::sort_policy::SortPolicy) -> () {
      let sort_types = self;
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QSortPolicy_setSortTypes_sortTypes(original_self as *mut ::sort_policy::SortPolicy, sort_types as *const ::vector::VectorSortPolicySortType) }
    }
  }
  impl<'largs> SortPolicySetSortTypesArgs<'largs> for &'largs ::qt_core::vector::VectorCInt {
    fn exec(self, original_self: &'largs mut ::sort_policy::SortPolicy) -> () {
      let sort_types_int = self;
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QSortPolicy_setSortTypes_sortTypesInt(original_self as *mut ::sort_policy::SortPolicy, sort_types_int as *const ::qt_core::vector::VectorCInt) }
    }
  }
}
