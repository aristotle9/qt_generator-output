/// C++ type: <span style='color: green;'>```QCoreApplication```</span>
#[repr(C)]
pub struct CoreApplication(u8);

impl CoreApplication {
  /// C++ method: <span style='color: green;'>```static void QCoreApplication::addLibraryPath(const QString& arg1)```</span>
  ///
  ///
  pub fn add_library_path(arg1: &::string::String) {
    unsafe { ::ffi::qt_core_c_QCoreApplication_addLibraryPath(arg1 as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```static QString QCoreApplication::applicationDirPath()```</span>
  ///
  ///
  pub fn application_dir_path() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCoreApplication_applicationDirPath_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QCoreApplication::applicationFilePath()```</span>
  ///
  ///
  pub fn application_file_path() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCoreApplication_applicationFilePath_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QCoreApplication::applicationName()```</span>
  ///
  ///
  pub fn application_name() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCoreApplication_applicationName_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static qint64 QCoreApplication::applicationPid()```</span>
  ///
  ///
  pub fn application_pid() -> i64 {
    unsafe { ::ffi::qt_core_c_QCoreApplication_applicationPid() }
  }

  /// C++ method: <span style='color: green;'>```static QString QCoreApplication::applicationVersion()```</span>
  ///
  ///
  pub fn application_version() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCoreApplication_applicationVersion_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QStringList QCoreApplication::arguments()```</span>
  ///
  ///
  pub fn arguments() -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCoreApplication_arguments_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static bool QCoreApplication::closingDown()```</span>
  ///
  ///
  pub fn closing_down() -> bool {
    unsafe { ::ffi::qt_core_c_QCoreApplication_closingDown() }
  }

  /// C++ method: <span style='color: green;'>```static QAbstractEventDispatcher* QCoreApplication::eventDispatcher()```</span>
  ///
  ///
  pub fn event_dispatcher() -> *mut ::abstract_event_dispatcher::AbstractEventDispatcher {
    unsafe { ::ffi::qt_core_c_QCoreApplication_eventDispatcher() }
  }

  /// C++ method: <span style='color: green;'>```static int QCoreApplication::exec()```</span>
  ///
  ///
  pub fn exec() -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QCoreApplication_exec() }
  }

  /// C++ method: <span style='color: green;'>```QCoreApplication::exit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn exit(()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QCoreApplication::exit()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn exit(::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QCoreApplication::exit(int retcode = ?)```</span>
  ///
  ///
  pub fn exit<Args>(args: Args) -> ()
    where Args: overloading::CoreApplicationExitArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static void QCoreApplication::flush()```</span>
  ///
  ///
  pub fn flush() {
    unsafe { ::ffi::qt_core_c_QCoreApplication_flush() }
  }

  /// C++ method: <span style='color: green;'>```static bool QCoreApplication::hasPendingEvents()```</span>
  ///
  ///
  pub fn has_pending_events() -> bool {
    unsafe { ::ffi::qt_core_c_QCoreApplication_hasPendingEvents() }
  }

  /// C++ method: <span style='color: green;'>```void QCoreApplication::installNativeEventFilter(QAbstractNativeEventFilter* filterObj)```</span>
  ///
  ///
pub unsafe fn install_native_event_filter(&mut self, filter_obj: *mut ::abstract_native_event_filter::AbstractNativeEventFilter) {
    ::ffi::qt_core_c_QCoreApplication_installNativeEventFilter(self as *mut ::core_application::CoreApplication,
                                                               filter_obj)
  }

  /// C++ method: <span style='color: green;'>```static bool QCoreApplication::installTranslator(QTranslator* messageFile)```</span>
  ///
  ///
  pub unsafe fn install_translator(message_file: *mut ::translator::Translator) -> bool {
    ::ffi::qt_core_c_QCoreApplication_installTranslator(message_file)
  }

  /// C++ method: <span style='color: green;'>```static QCoreApplication* QCoreApplication::instance()```</span>
  ///
  ///
  pub fn instance() -> *mut ::core_application::CoreApplication {
    unsafe { ::ffi::qt_core_c_QCoreApplication_instance() }
  }

  /// C++ method: <span style='color: green;'>```static bool QCoreApplication::isQuitLockEnabled()```</span>
  ///
  ///
  pub fn is_quit_lock_enabled() -> bool {
    unsafe { ::ffi::qt_core_c_QCoreApplication_isQuitLockEnabled() }
  }

  /// C++ method: <span style='color: green;'>```static bool QCoreApplication::isSetuidAllowed()```</span>
  ///
  ///
  pub fn is_setuid_allowed() -> bool {
    unsafe { ::ffi::qt_core_c_QCoreApplication_isSetuidAllowed() }
  }

  /// C++ method: <span style='color: green;'>```static QStringList QCoreApplication::libraryPaths()```</span>
  ///
  ///
  pub fn library_paths() -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCoreApplication_libraryPaths_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QCoreApplication::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QCoreApplication_metaObject(self as *const ::core_application::CoreApplication) }
  }

  /// C++ method: <span style='color: green;'>```QCoreApplication::QCoreApplication```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new((&mut ::libc::c_int, *mut *mut ::libc::c_char)) -> ::cpp_utils::CppBox<::core_application::CoreApplication>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCoreApplication::QCoreApplication(int& argc, char** argv)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&mut ::libc::c_int, *mut *mut ::libc::c_char, ::libc::c_int)) -> ::cpp_utils::CppBox<::core_application::CoreApplication>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCoreApplication::QCoreApplication(int& argc, char** argv, int arg3 = ?)```</span>
  ///
  ///
  pub unsafe fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::core_application::CoreApplication>
    where Args: overloading::CoreApplicationNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual bool QCoreApplication::notify(QObject* arg1, QEvent* arg2)```</span>
  ///
  ///
  pub unsafe fn notify(&mut self, arg1: *mut ::object::Object, arg2: *mut ::event::Event) -> bool {
    ::ffi::qt_core_c_QCoreApplication_notify(self as *mut ::core_application::CoreApplication, arg1, arg2)
  }

  /// C++ method: <span style='color: green;'>```static QString QCoreApplication::organizationDomain()```</span>
  ///
  ///
  pub fn organization_domain() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCoreApplication_organizationDomain_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QCoreApplication::organizationName()```</span>
  ///
  ///
  pub fn organization_name() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCoreApplication_organizationName_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCoreApplication::postEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn post_event((*mut ::object::Object, *mut ::event::Event)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QCoreApplication::postEvent(QObject* receiver, QEvent* event)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn post_event((*mut ::object::Object, *mut ::event::Event, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QCoreApplication::postEvent(QObject* receiver, QEvent* event, int priority = ?)```</span>
  ///
  ///
  pub unsafe fn post_event<Args>(args: Args) -> ()
    where Args: overloading::CoreApplicationPostEventArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static [slot] void QCoreApplication::quit()```</span>
  ///
  ///
  pub fn quit() {
    unsafe { ::ffi::qt_core_c_QCoreApplication_quit() }
  }

  /// C++ method: <span style='color: green;'>```static void QCoreApplication::removeLibraryPath(const QString& arg1)```</span>
  ///
  ///
  pub fn remove_library_path(arg1: &::string::String) {
    unsafe { ::ffi::qt_core_c_QCoreApplication_removeLibraryPath(arg1 as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```void QCoreApplication::removeNativeEventFilter(QAbstractNativeEventFilter* filterObj)```</span>
  ///
  ///
pub unsafe fn remove_native_event_filter(&mut self, filter_obj: *mut ::abstract_native_event_filter::AbstractNativeEventFilter) {
    ::ffi::qt_core_c_QCoreApplication_removeNativeEventFilter(self as *mut ::core_application::CoreApplication,
                                                              filter_obj)
  }

  /// C++ method: <span style='color: green;'>```QCoreApplication::removePostedEvents```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove_posted_events(*mut ::object::Object) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QCoreApplication::removePostedEvents(QObject* receiver)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove_posted_events((*mut ::object::Object, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QCoreApplication::removePostedEvents(QObject* receiver, int eventType = ?)```</span>
  ///
  ///
  pub unsafe fn remove_posted_events<Args>(args: Args) -> ()
    where Args: overloading::CoreApplicationRemovePostedEventsArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static bool QCoreApplication::removeTranslator(QTranslator* messageFile)```</span>
  ///
  ///
  pub unsafe fn remove_translator(message_file: *mut ::translator::Translator) -> bool {
    ::ffi::qt_core_c_QCoreApplication_removeTranslator(message_file)
  }

  /// C++ method: <span style='color: green;'>```static bool QCoreApplication::sendEvent(QObject* receiver, QEvent* event)```</span>
  ///
  ///
  pub unsafe fn send_event(receiver: *mut ::object::Object, event: *mut ::event::Event) -> bool {
    ::ffi::qt_core_c_QCoreApplication_sendEvent(receiver, event)
  }

  /// C++ method: <span style='color: green;'>```static void QCoreApplication::sendPostedEvents()```</span>
  ///
  ///
  pub fn send_posted_events() {
    unsafe { ::ffi::qt_core_c_QCoreApplication_sendPostedEvents_no_args() }
  }

  /// C++ method: <span style='color: green;'>```QCoreApplication::sendPostedEvents```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn send_posted_events_unsafe(*mut ::object::Object) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QCoreApplication::sendPostedEvents(QObject* receiver = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn send_posted_events_unsafe((*mut ::object::Object, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QCoreApplication::sendPostedEvents(QObject* receiver = ?, int event_type = ?)```</span>
  ///
  ///
  pub unsafe fn send_posted_events_unsafe<Args>(args: Args) -> ()
    where Args: overloading::CoreApplicationSendPostedEventsUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static void QCoreApplication::setApplicationName(const QString& application)```</span>
  ///
  ///
  pub fn set_application_name(application: &::string::String) {
    unsafe { ::ffi::qt_core_c_QCoreApplication_setApplicationName(application as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```static void QCoreApplication::setApplicationVersion(const QString& version)```</span>
  ///
  ///
  pub fn set_application_version(version: &::string::String) {
    unsafe { ::ffi::qt_core_c_QCoreApplication_setApplicationVersion(version as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QCoreApplication::setAttribute```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_attribute(&::qt::ApplicationAttribute) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QCoreApplication::setAttribute(Qt::ApplicationAttribute attribute)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_attribute((&::qt::ApplicationAttribute, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QCoreApplication::setAttribute(Qt::ApplicationAttribute attribute, bool on = ?)```</span>
  ///
  ///
  pub fn set_attribute<Args>(args: Args) -> ()
    where Args: overloading::CoreApplicationSetAttributeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static void QCoreApplication::setEventDispatcher(QAbstractEventDispatcher* eventDispatcher)```</span>
  ///
  ///
  pub unsafe fn set_event_dispatcher(event_dispatcher: *mut ::abstract_event_dispatcher::AbstractEventDispatcher) {
    ::ffi::qt_core_c_QCoreApplication_setEventDispatcher(event_dispatcher)
  }

  /// C++ method: <span style='color: green;'>```static void QCoreApplication::setLibraryPaths(const QStringList& arg1)```</span>
  ///
  ///
  pub fn set_library_paths(arg1: &::string_list::StringList) {
    unsafe { ::ffi::qt_core_c_QCoreApplication_setLibraryPaths(arg1 as *const ::string_list::StringList) }
  }

  /// C++ method: <span style='color: green;'>```static void QCoreApplication::setOrganizationDomain(const QString& orgDomain)```</span>
  ///
  ///
  pub fn set_organization_domain(org_domain: &::string::String) {
    unsafe { ::ffi::qt_core_c_QCoreApplication_setOrganizationDomain(org_domain as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```static void QCoreApplication::setOrganizationName(const QString& orgName)```</span>
  ///
  ///
  pub fn set_organization_name(org_name: &::string::String) {
    unsafe { ::ffi::qt_core_c_QCoreApplication_setOrganizationName(org_name as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```static void QCoreApplication::setQuitLockEnabled(bool enabled)```</span>
  ///
  ///
  pub fn set_quit_lock_enabled(enabled: bool) {
    unsafe { ::ffi::qt_core_c_QCoreApplication_setQuitLockEnabled(enabled) }
  }

  /// C++ method: <span style='color: green;'>```static void QCoreApplication::setSetuidAllowed(bool allow)```</span>
  ///
  ///
  pub fn set_setuid_allowed(allow: bool) {
    unsafe { ::ffi::qt_core_c_QCoreApplication_setSetuidAllowed(allow) }
  }

  /// C++ method: <span style='color: green;'>```static bool QCoreApplication::startingUp()```</span>
  ///
  ///
  pub fn starting_up() -> bool {
    unsafe { ::ffi::qt_core_c_QCoreApplication_startingUp() }
  }

  /// C++ method: <span style='color: green;'>```static bool QCoreApplication::testAttribute(Qt::ApplicationAttribute attribute)```</span>
  ///
  ///
  pub fn test_attribute(attribute: &::qt::ApplicationAttribute) -> bool {
    unsafe { ::ffi::qt_core_c_QCoreApplication_testAttribute(attribute as *const ::qt::ApplicationAttribute) }
  }

  /// C++ method: <span style='color: green;'>```static QString QCoreApplication::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QCoreApplication_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QCoreApplication::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QCoreApplication_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCoreApplication::translate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translate((*const ::libc::c_char, *const ::libc::c_char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QCoreApplication::translate(const char* context, const char* key)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translate((*const ::libc::c_char, *const ::libc::c_char, *const ::libc::c_char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QCoreApplication::translate(const char* context, const char* key, const char* disambiguation = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn translate((*const ::libc::c_char, *const ::libc::c_char, *const ::libc::c_char, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QCoreApplication::translate(const char* context, const char* key, const char* disambiguation = ?, int n = ?)```</span>
  ///
  ///
  pub unsafe fn translate<Args>(args: Args) -> ::string::String
    where Args: overloading::CoreApplicationTranslateArgs
  {
    args.exec()
  }
}

impl ::cpp_utils::CppDeletable for ::core_application::CoreApplication {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QCoreApplication_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `CoreApplication`.
  pub struct Signals<'a>(&'a ::core_application::CoreApplication);
  /// Represents a built-in Qt signal `QCoreApplication::applicationVersionChanged`.
  ///
  /// An object of this type can be created from `CoreApplication` with `object.signals().application_version_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CoreApplication` object.
  pub struct ApplicationVersionChanged<'a>(&'a ::core_application::CoreApplication);
  impl<'a> ::connection::Receiver for ApplicationVersionChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2applicationVersionChanged()\0"
    }
  }
  impl<'a> ::connection::Signal for ApplicationVersionChanged<'a> {}
  /// Represents a built-in Qt signal `QCoreApplication::objectNameChanged`.
  ///
  /// An object of this type can be created from `CoreApplication` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CoreApplication` object.
  pub struct ObjectNameChanged<'a>(&'a ::core_application::CoreApplication);
  impl<'a> ::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::string::String,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::connection::Signal for ObjectNameChanged<'a> {}
  /// Represents a built-in Qt signal `QCoreApplication::aboutToQuit`.
  ///
  /// An object of this type can be created from `CoreApplication` with `object.signals().about_to_quit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CoreApplication` object.
  pub struct AboutToQuit<'a>(&'a ::core_application::CoreApplication);
  impl<'a> ::connection::Receiver for AboutToQuit<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2aboutToQuit()\0"
    }
  }
  impl<'a> ::connection::Signal for AboutToQuit<'a> {}
  /// Represents a built-in Qt signal `QCoreApplication::organizationNameChanged`.
  ///
  /// An object of this type can be created from `CoreApplication` with `object.signals().organization_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CoreApplication` object.
  pub struct OrganizationNameChanged<'a>(&'a ::core_application::CoreApplication);
  impl<'a> ::connection::Receiver for OrganizationNameChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2organizationNameChanged()\0"
    }
  }
  impl<'a> ::connection::Signal for OrganizationNameChanged<'a> {}
  /// Represents a built-in Qt signal `QCoreApplication::organizationDomainChanged`.
  ///
  /// An object of this type can be created from `CoreApplication` with `object.signals().organization_domain_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CoreApplication` object.
  pub struct OrganizationDomainChanged<'a>(&'a ::core_application::CoreApplication);
  impl<'a> ::connection::Receiver for OrganizationDomainChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2organizationDomainChanged()\0"
    }
  }
  impl<'a> ::connection::Signal for OrganizationDomainChanged<'a> {}
  /// Represents a built-in Qt signal `QCoreApplication::applicationNameChanged`.
  ///
  /// An object of this type can be created from `CoreApplication` with `object.signals().application_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CoreApplication` object.
  pub struct ApplicationNameChanged<'a>(&'a ::core_application::CoreApplication);
  impl<'a> ::connection::Receiver for ApplicationNameChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2applicationNameChanged()\0"
    }
  }
  impl<'a> ::connection::Signal for ApplicationNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QCoreApplication::applicationVersionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn application_version_changed(&self) -> ApplicationVersionChanged {
      ApplicationVersionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCoreApplication::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCoreApplication::aboutToQuit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn about_to_quit(&self) -> AboutToQuit {
      AboutToQuit(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCoreApplication::organizationNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn organization_name_changed(&self) -> OrganizationNameChanged {
      OrganizationNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCoreApplication::organizationDomainChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn organization_domain_changed(&self) -> OrganizationDomainChanged {
      OrganizationDomainChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCoreApplication::applicationNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn application_name_changed(&self) -> ApplicationNameChanged {
      ApplicationNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `CoreApplication`.
  pub struct Slots<'a>(&'a ::core_application::CoreApplication);
  /// Represents a built-in Qt slot `QCoreApplication::quit`.
  ///
  /// An object of this type can be created from `CoreApplication` with `object.slots().quit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CoreApplication` object.
  pub struct Quit<'a>(&'a ::core_application::CoreApplication);
  impl<'a> ::connection::Receiver for Quit<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1quit()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QCoreApplication::quit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn quit(&self) -> Quit {
      Quit(self.0)
    }
  }
  impl ::core_application::CoreApplication {
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

/// C++ method: <span style='color: green;'>```void qAddPostRoutine(void (*FN_PTR)() arg1)```</span>
///
///
pub unsafe fn add_post_routine(arg1: extern "C" fn()) {
  ::ffi::qt_core_c_QCoreApplication_G_qAddPostRoutine(arg1)
}

/// C++ method: <span style='color: green;'>```void qAddPreRoutine(void (*FN_PTR)() arg1)```</span>
///
///
pub unsafe fn add_pre_routine(arg1: extern "C" fn()) {
  ::ffi::qt_core_c_QCoreApplication_G_qAddPreRoutine(arg1)
}

/// C++ method: <span style='color: green;'>```void qRemovePostRoutine(void (*FN_PTR)() arg1)```</span>
///
///
pub unsafe fn remove_post_routine(arg1: extern "C" fn()) {
  ::ffi::qt_core_c_QCoreApplication_G_qRemovePostRoutine(arg1)
}

impl ::cpp_utils::DynamicCast<::core_application::CoreApplication> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::core_application::CoreApplication> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QCoreApplication_G_dynamic_cast_QCoreApplication_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::core_application::CoreApplication> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QCoreApplication_G_dynamic_cast_QCoreApplication_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::core_application::CoreApplication {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QCoreApplication_G_static_cast_QObject_ptr(self as *mut ::core_application::CoreApplication)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QCoreApplication_G_static_cast_QObject_ptr(self as *const ::core_application::CoreApplication as *mut ::core_application::CoreApplication) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::core_application::CoreApplication> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::core_application::CoreApplication {
    let ffi_result =
      ::ffi::qt_core_c_QCoreApplication_G_static_cast_QCoreApplication_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::core_application::CoreApplication {
    let ffi_result = ::ffi::qt_core_c_QCoreApplication_G_static_cast_QCoreApplication_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::core_application::CoreApplication {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QCoreApplication_G_static_cast_QObject_ptr(self as *const ::core_application::CoreApplication as *mut ::core_application::CoreApplication) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::core_application::CoreApplication {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QCoreApplication_G_static_cast_QObject_ptr(self as *mut ::core_application::CoreApplication)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [CoreApplication::exit](../struct.CoreApplication.html#method.exit) method.
  pub trait CoreApplicationExitArgs {
    fn exec(self) -> ();
  }
  impl CoreApplicationExitArgs for () {
    fn exec(self) -> () {

      unsafe { ::ffi::qt_core_c_QCoreApplication_exit_no_args() }
    }
  }
  impl CoreApplicationExitArgs for ::libc::c_int {
    fn exec(self) -> () {
      let retcode = self;
      unsafe { ::ffi::qt_core_c_QCoreApplication_exit_retcode(retcode) }
    }
  }
  /// This trait represents a set of arguments accepted by [CoreApplication::new](../struct.CoreApplication.html#method.new) method.
  pub trait CoreApplicationNewArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::core_application::CoreApplication>;
  }
  impl<'a> CoreApplicationNewArgs for (&'a mut ::libc::c_int, *mut *mut ::libc::c_char) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::core_application::CoreApplication> {
      let argc = self.0;
      let argv = self.1;
      let ffi_result = ::ffi::qt_core_c_QCoreApplication_new_argc_argv(argc as *mut ::libc::c_int, argv);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> CoreApplicationNewArgs for (&'a mut ::libc::c_int, *mut *mut ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::core_application::CoreApplication> {
      let argc = self.0;
      let argv = self.1;
      let arg3 = self.2;
      let ffi_result = ::ffi::qt_core_c_QCoreApplication_new_argc_argv_arg3(argc as *mut ::libc::c_int, argv, arg3);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [CoreApplication::post_event](../struct.CoreApplication.html#method.post_event) method.
  pub trait CoreApplicationPostEventArgs {
    unsafe fn exec(self) -> ();
  }
  impl CoreApplicationPostEventArgs for (*mut ::object::Object, *mut ::event::Event) {
    unsafe fn exec(self) -> () {
      let receiver = self.0;
      let event = self.1;
      ::ffi::qt_core_c_QCoreApplication_postEvent_receiver_event(receiver, event)
    }
  }
  impl CoreApplicationPostEventArgs for (*mut ::object::Object, *mut ::event::Event, ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let receiver = self.0;
      let event = self.1;
      let priority = self.2;
      ::ffi::qt_core_c_QCoreApplication_postEvent_receiver_event_priority(receiver, event, priority)
    }
  }
  /// This trait represents a set of arguments accepted by [CoreApplication::remove_posted_events](../struct.CoreApplication.html#method.remove_posted_events) method.
  pub trait CoreApplicationRemovePostedEventsArgs {
    unsafe fn exec(self) -> ();
  }
  impl CoreApplicationRemovePostedEventsArgs for *mut ::object::Object {
    unsafe fn exec(self) -> () {
      let receiver = self;
      ::ffi::qt_core_c_QCoreApplication_removePostedEvents_receiver(receiver)
    }
  }
  impl CoreApplicationRemovePostedEventsArgs for (*mut ::object::Object, ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let receiver = self.0;
      let event_type = self.1;
      ::ffi::qt_core_c_QCoreApplication_removePostedEvents_receiver_eventType(receiver, event_type)
    }
  }
  /// This trait represents a set of arguments accepted by [CoreApplication::send_posted_events_unsafe](../struct.CoreApplication.html#method.send_posted_events_unsafe) method.
  pub trait CoreApplicationSendPostedEventsUnsafeArgs {
    unsafe fn exec(self) -> ();
  }
  impl CoreApplicationSendPostedEventsUnsafeArgs for *mut ::object::Object {
    unsafe fn exec(self) -> () {
      let receiver = self;
      ::ffi::qt_core_c_QCoreApplication_sendPostedEvents_receiver(receiver)
    }
  }
  impl CoreApplicationSendPostedEventsUnsafeArgs for (*mut ::object::Object, ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let receiver = self.0;
      let event_type = self.1;
      ::ffi::qt_core_c_QCoreApplication_sendPostedEvents_receiver_event_type(receiver, event_type)
    }
  }
  /// This trait represents a set of arguments accepted by [CoreApplication::set_attribute](../struct.CoreApplication.html#method.set_attribute) method.
  pub trait CoreApplicationSetAttributeArgs {
    fn exec(self) -> ();
  }
  impl<'a> CoreApplicationSetAttributeArgs for &'a ::qt::ApplicationAttribute {
    fn exec(self) -> () {
      let attribute = self;
      unsafe {
        ::ffi::qt_core_c_QCoreApplication_setAttribute_attribute(attribute as *const ::qt::ApplicationAttribute)
      }
    }
  }
  impl<'a> CoreApplicationSetAttributeArgs for (&'a ::qt::ApplicationAttribute, bool) {
    fn exec(self) -> () {
      let attribute = self.0;
      let on = self.1;
      unsafe {
        ::ffi::qt_core_c_QCoreApplication_setAttribute_attribute_on(attribute as *const ::qt::ApplicationAttribute, on)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [CoreApplication::translate](../struct.CoreApplication.html#method.translate) method.
  pub trait CoreApplicationTranslateArgs {
    unsafe fn exec(self) -> ::string::String;
  }
  impl CoreApplicationTranslateArgs for (*const ::libc::c_char, *const ::libc::c_char) {
    unsafe fn exec(self) -> ::string::String {
      let context = self.0;
      let key = self.1;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QCoreApplication_translate_to_output_context_key(context, key, &mut object);
        object
      }
    }
  }
  impl CoreApplicationTranslateArgs for (*const ::libc::c_char, *const ::libc::c_char, *const ::libc::c_char) {
    unsafe fn exec(self) -> ::string::String {
      let context = self.0;
      let key = self.1;
      let disambiguation = self.2;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QCoreApplication_translate_to_output_context_key_disambiguation(context,
                                                                                         key,
                                                                                         disambiguation,
                                                                                         &mut object);
        object
      }
    }
  }
  impl CoreApplicationTranslateArgs
    for (*const ::libc::c_char, *const ::libc::c_char, *const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self) -> ::string::String {
      let context = self.0;
      let key = self.1;
      let disambiguation = self.2;
      let n = self.3;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QCoreApplication_translate_to_output_context_key_disambiguation_n(context,
                                                                                           key,
                                                                                           disambiguation,
                                                                                           n,
                                                                                           &mut object);
        object
      }
    }
  }
}



/// A struct providing valid `argc` and `argv` values for Qt application
/// objects.
///
/// Constructors of `qt_core::core_application::CoreApplication`,
/// `qt_gui::gui_application::GuiApplication` and `qt_widgets::application::Application`
/// require `argc` and `argv` values that are available in C++'s `main` function but
/// not available in Rust. More importantly, `argc` and `argv` must be valid for the entire
/// life of the application. This struct stores list of arguments in a format compatible with
/// `argc` and `argv`, and can be used to initialize Qt application objects.
/// `CoreApplicationArgs` must live longer than the application object.
///
/// `CoreApplication::create_and_exit` convenience function
/// and similar functions in the other application types
/// can be used instead of `CoreApplicationArgs`.
pub struct CoreApplicationArgs {
  _values: Vec<Vec<u8>>,
  argc: Box<::libc::c_int>,
  argv: Vec<*mut ::libc::c_char>,
}

impl CoreApplicationArgs {
  /// Creates an object containing `args`.
  pub fn from(mut args: Vec<Vec<u8>>) -> CoreApplicationArgs {
    for arg in &mut args {
      if !arg.ends_with(&[0]) {
        arg.push(0);
      }
    }
    CoreApplicationArgs {
      argc: Box::new(args.len() as ::libc::c_int),
      argv: args.iter_mut()
        .map(|x| x.as_mut_ptr() as *mut ::libc::c_char)
        .collect(),
      _values: args,
    }
  }
  /// Creates an object containing empty list of arguments.
  /// Although this is the cheapest way to construct a `CoreApplicationArgs`
  /// object, it's not clear whether Qt considers empty arguments list valid.
  pub fn empty() -> CoreApplicationArgs {
    CoreApplicationArgs::from(Vec::new())
  }

  /// Returns `(argc, argv)` values in the form accepted by the application objects'
  /// constructors.
  pub fn get(&mut self) -> (&mut ::libc::c_int, *mut *mut ::libc::c_char) {
    (self.argc.as_mut(), self.argv.as_mut_ptr())
  }

  #[cfg(unix)]
  /// Creates an object representing real arguments of the application.
  /// On Windows, this function uses empty argument list for performance reasons because
  /// Qt doesn't use `argc` and `argv` on Windows at all.
  pub fn from_real() -> CoreApplicationArgs {
    use std::os::unix::ffi::OsStringExt;
    let args = ::std::env::args_os().map(|arg| arg.into_vec()).collect();
    CoreApplicationArgs::from(args)
  }
  #[cfg(windows)]
  /// Creates an object representing real arguments of the application.
  /// On Windows, this function uses empty argument list for performance reasons because
  /// Qt doesn't use `argc` and `argv` on Windows at all.
  pub fn from_real() -> CoreApplicationArgs {
    // Qt doesn't use argc and argv on Windows anyway
    // TODO: check this
    CoreApplicationArgs::empty()
  }
}

impl ::core_application::CoreApplication {
  /// A convenience function for performing proper initialization and de-initialization of
  /// a Qt application.
  ///
  /// This function creates `CoreApplication` with valid `argc` and `argv`, calls the passed
  /// closure `f(app)` with the application object and exist the process with the exit code
  /// returned by the closure. The closure should perform the initialization of the application
  /// and either return immediately or call `CoreApplication::exec()` and return its return value:
  /// ```
  /// fn main() {
  ///   CoreApplication::create_and_exit(|app| {
  ///     // initialization goes here
  ///     CoreApplication::exec()
  ///   })
  /// }
  /// ```
  pub fn create_and_exit<F: FnOnce(&mut ::core_application::CoreApplication) -> i32>(f: F) -> ! {
    let exit_code = {
      let mut args = CoreApplicationArgs::from_real();
      let mut app = unsafe { ::core_application::CoreApplication::new(args.get()) };
      f(app.as_mut())
    };
    ::std::process::exit(exit_code)
  }
}
