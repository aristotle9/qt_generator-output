/// C++ type: <span style='color: green;'>```QProcess::ExitStatus```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ExitStatus {
  /// C++ enum variant: <span style='color: green;'>```NormalExit = 0```</span>
  Normal = 0,
  /// C++ enum variant: <span style='color: green;'>```CrashExit = 1```</span>
  Crash = 1,
}

/// C++ type: <span style='color: green;'>```QProcess::InputChannelMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum InputChannelMode {
  /// C++ enum variant: <span style='color: green;'>```ManagedInputChannel = 0```</span>
  Managed = 0,
  /// C++ enum variant: <span style='color: green;'>```ForwardedInputChannel = 1```</span>
  Forwarded = 1,
}

/// C++ type: <span style='color: green;'>```QProcess```</span>
#[repr(C)]
pub struct Process(u8);

impl Process {
  /// C++ method: <span style='color: green;'>```QStringList QProcess::arguments() const```</span>
  ///
  ///
  pub fn arguments(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QProcess_arguments_to_output(self as *const ::process::Process, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QProcess::atEnd() const```</span>
  ///
  ///
  pub fn at_end(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QProcess_atEnd(self as *const ::process::Process) }
  }

  /// C++ method: <span style='color: green;'>```virtual qint64 QProcess::bytesAvailable() const```</span>
  ///
  ///
  pub fn bytes_available(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QProcess_bytesAvailable(self as *const ::process::Process) }
  }

  /// C++ method: <span style='color: green;'>```virtual qint64 QProcess::bytesToWrite() const```</span>
  ///
  ///
  pub fn bytes_to_write(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QProcess_bytesToWrite(self as *const ::process::Process) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QProcess::canReadLine() const```</span>
  ///
  ///
  pub fn can_read_line(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QProcess_canReadLine(self as *const ::process::Process) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QProcess::close()```</span>
  ///
  ///
  pub fn close(&mut self) {
    unsafe { ::ffi::qt_core_c_QProcess_close(self as *mut ::process::Process) }
  }

  /// C++ method: <span style='color: green;'>```void QProcess::closeReadChannel(QProcess::ProcessChannel channel)```</span>
  ///
  ///
  pub fn close_read_channel(&mut self, channel: ::process::ProcessChannel) {
    unsafe { ::ffi::qt_core_c_QProcess_closeReadChannel(self as *mut ::process::Process, channel) }
  }

  /// C++ method: <span style='color: green;'>```void QProcess::closeWriteChannel()```</span>
  ///
  ///
  pub fn close_write_channel(&mut self) {
    unsafe { ::ffi::qt_core_c_QProcess_closeWriteChannel(self as *mut ::process::Process) }
  }

  /// C++ method: <span style='color: green;'>```QStringList QProcess::environment() const```</span>
  ///
  ///
  pub fn environment(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QProcess_environment_to_output(self as *const ::process::Process, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QProcess::execute```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn execute(&::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QProcess::execute(const QString& command)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn execute((&::string::String, &::string_list::StringList)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QProcess::execute(const QString& program, const QStringList& arguments)```</span>
  ///
  ///
  pub fn execute<Args>(args: Args) -> ::libc::c_int
    where Args: overloading::ProcessExecuteArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```int QProcess::exitCode() const```</span>
  ///
  ///
  pub fn exit_code(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QProcess_exitCode(self as *const ::process::Process) }
  }

  /// C++ method: <span style='color: green;'>```QProcess::InputChannelMode QProcess::inputChannelMode() const```</span>
  ///
  ///
  pub fn input_channel_mode(&self) -> ::process::InputChannelMode {
    unsafe { ::ffi::qt_core_c_QProcess_inputChannelMode(self as *const ::process::Process) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QProcess::isSequential() const```</span>
  ///
  ///
  pub fn is_sequential(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QProcess_isSequential(self as *const ::process::Process) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QProcess::kill()```</span>
  ///
  ///
  pub fn kill(&mut self) {
    unsafe { ::ffi::qt_core_c_QProcess_kill(self as *mut ::process::Process) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QProcess::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QProcess_metaObject(self as *const ::process::Process) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QProcess::QProcess()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::process::Process> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QProcess_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QProcess::QProcess(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::object::Object) -> ::cpp_utils::CppBox<::process::Process> {
    let ffi_result = ::ffi::qt_core_c_QProcess_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```static QString QProcess::nullDevice()```</span>
  ///
  ///
  pub fn null_device() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QProcess_nullDevice_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QProcess::open```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn open(&mut self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QProcess::open()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn open(&mut self, ::flags::Flags<::io_device::OpenModeFlag>) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QProcess::open(QFlags<QIODevice::OpenModeFlag> mode = ?)```</span>
  ///
  ///
  pub fn open<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::ProcessOpenArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QProcess::ProcessChannelMode QProcess::processChannelMode() const```</span>
  ///
  ///
  pub fn process_channel_mode(&self) -> ::process::ProcessChannelMode {
    unsafe { ::ffi::qt_core_c_QProcess_processChannelMode(self as *const ::process::Process) }
  }

  /// C++ method: <span style='color: green;'>```QProcessEnvironment QProcess::processEnvironment() const```</span>
  ///
  ///
  pub fn process_environment(&self) -> ::process_environment::ProcessEnvironment {
    {
      let mut object: ::process_environment::ProcessEnvironment =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QProcess_processEnvironment_to_output(self as *const ::process::Process, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```qint64 QProcess::processId() const```</span>
  ///
  ///
  pub fn process_id(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QProcess_processId(self as *const ::process::Process) }
  }

  /// C++ method: <span style='color: green;'>```QString QProcess::program() const```</span>
  ///
  ///
  pub fn program(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QProcess_program_to_output(self as *const ::process::Process, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QProcess::readAllStandardError()```</span>
  ///
  ///
  pub fn read_all_standard_error(&mut self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QProcess_readAllStandardError_to_output(self as *mut ::process::Process, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QProcess::readAllStandardOutput()```</span>
  ///
  ///
  pub fn read_all_standard_output(&mut self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QProcess_readAllStandardOutput_to_output(self as *mut ::process::Process, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QProcess::ProcessChannel QProcess::readChannel() const```</span>
  ///
  ///
  pub fn read_channel(&self) -> ::process::ProcessChannel {
    unsafe { ::ffi::qt_core_c_QProcess_readChannel(self as *const ::process::Process) }
  }

  /// C++ method: <span style='color: green;'>```QProcess::ProcessChannelMode QProcess::readChannelMode() const```</span>
  ///
  ///
  pub fn read_channel_mode(&self) -> ::process::ProcessChannelMode {
    unsafe { ::ffi::qt_core_c_QProcess_readChannelMode(self as *const ::process::Process) }
  }

  /// C++ method: <span style='color: green;'>```void QProcess::setArguments(const QStringList& arguments)```</span>
  ///
  ///
  pub fn set_arguments(&mut self, arguments: &::string_list::StringList) {
    unsafe {
      ::ffi::qt_core_c_QProcess_setArguments(self as *mut ::process::Process,
                                             arguments as *const ::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```void QProcess::setEnvironment(const QStringList& environment)```</span>
  ///
  ///
  pub fn set_environment(&mut self, environment: &::string_list::StringList) {
    unsafe {
      ::ffi::qt_core_c_QProcess_setEnvironment(self as *mut ::process::Process,
                                               environment as *const ::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```void QProcess::setInputChannelMode(QProcess::InputChannelMode mode)```</span>
  ///
  ///
  pub fn set_input_channel_mode(&mut self, mode: ::process::InputChannelMode) {
    unsafe { ::ffi::qt_core_c_QProcess_setInputChannelMode(self as *mut ::process::Process, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QProcess::setProcessChannelMode(QProcess::ProcessChannelMode mode)```</span>
  ///
  ///
  pub fn set_process_channel_mode(&mut self, mode: ::process::ProcessChannelMode) {
    unsafe { ::ffi::qt_core_c_QProcess_setProcessChannelMode(self as *mut ::process::Process, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QProcess::setProcessEnvironment(const QProcessEnvironment& environment)```</span>
  ///
  ///
  pub fn set_process_environment(&mut self, environment: &::process_environment::ProcessEnvironment) {
    unsafe {
      ::ffi::qt_core_c_QProcess_setProcessEnvironment(self as *mut ::process::Process,
                                                      environment as *const ::process_environment::ProcessEnvironment)
    }
  }

  /// C++ method: <span style='color: green;'>```void QProcess::setProgram(const QString& program)```</span>
  ///
  ///
  pub fn set_program(&mut self, program: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QProcess_setProgram(self as *mut ::process::Process,
                                           program as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QProcess::setReadChannel(QProcess::ProcessChannel channel)```</span>
  ///
  ///
  pub fn set_read_channel(&mut self, channel: ::process::ProcessChannel) {
    unsafe { ::ffi::qt_core_c_QProcess_setReadChannel(self as *mut ::process::Process, channel) }
  }

  /// C++ method: <span style='color: green;'>```void QProcess::setReadChannelMode(QProcess::ProcessChannelMode mode)```</span>
  ///
  ///
  pub fn set_read_channel_mode(&mut self, mode: ::process::ProcessChannelMode) {
    unsafe { ::ffi::qt_core_c_QProcess_setReadChannelMode(self as *mut ::process::Process, mode) }
  }

  /// C++ method: <span style='color: green;'>```QProcess::setStandardErrorFile```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_standard_error_file(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QProcess::setStandardErrorFile(const QString& fileName)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_standard_error_file(&mut self, (&::string::String, ::flags::Flags<::io_device::OpenModeFlag>)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QProcess::setStandardErrorFile(const QString& fileName, QFlags<QIODevice::OpenModeFlag> mode = ?)```</span>
  ///
  ///
  pub fn set_standard_error_file<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ProcessSetStandardErrorFileArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QProcess::setStandardInputFile(const QString& fileName)```</span>
  ///
  ///
  pub fn set_standard_input_file(&mut self, file_name: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QProcess_setStandardInputFile(self as *mut ::process::Process,
                                                     file_name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QProcess::setStandardOutputFile```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_standard_output_file(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QProcess::setStandardOutputFile(const QString& fileName)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_standard_output_file(&mut self, (&::string::String, ::flags::Flags<::io_device::OpenModeFlag>)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QProcess::setStandardOutputFile(const QString& fileName, QFlags<QIODevice::OpenModeFlag> mode = ?)```</span>
  ///
  ///
  pub fn set_standard_output_file<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ProcessSetStandardOutputFileArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QProcess::setStandardOutputProcess(QProcess* destination)```</span>
  ///
  ///
  pub unsafe fn set_standard_output_process(&mut self, destination: *mut ::process::Process) {
    ::ffi::qt_core_c_QProcess_setStandardOutputProcess(self as *mut ::process::Process, destination)
  }

  /// C++ method: <span style='color: green;'>```void QProcess::setWorkingDirectory(const QString& dir)```</span>
  ///
  ///
  pub fn set_working_directory(&mut self, dir: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QProcess_setWorkingDirectory(self as *mut ::process::Process,
                                                    dir as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QProcess::start```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn start(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QProcess::start()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn start(&mut self, ::flags::Flags<::io_device::OpenModeFlag>) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QProcess::start(QFlags<QIODevice::OpenModeFlag> mode = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn start(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QProcess::start(const QString& command)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn start(&mut self, (&::string::String, ::flags::Flags<::io_device::OpenModeFlag>)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QProcess::start(const QString& command, QFlags<QIODevice::OpenModeFlag> mode = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn start(&mut self, (&::string::String, &::string_list::StringList)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QProcess::start(const QString& program, const QStringList& arguments)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn start(&mut self, (&::string::String, &::string_list::StringList, ::flags::Flags<::io_device::OpenModeFlag>)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QProcess::start(const QString& program, const QStringList& arguments, QFlags<QIODevice::OpenModeFlag> mode = ?)```</span>
  ///
  ///
  pub fn start<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ProcessStartArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QProcess::startDetached```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn start_detached(&::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QProcess::startDetached(const QString& command)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn start_detached((&::string::String, &::string_list::StringList)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QProcess::startDetached(const QString& program, const QStringList& arguments)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn start_detached((&::string::String, &::string_list::StringList, &::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QProcess::startDetached(const QString& program, const QStringList& arguments, const QString& workingDirectory)```</span>
  ///
  ///
  pub fn start_detached<Args>(args: Args) -> bool
    where Args: overloading::ProcessStartDetachedArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static bool QProcess::startDetached(const QString& program, const QStringList& arguments, const QString& workingDirectory, qint64* pid = ?)```</span>
  ///
  ///
  pub unsafe fn start_detached_unsafe(program: &::string::String,
                                      arguments: &::string_list::StringList,
                                      working_directory: &::string::String,
                                      pid: *mut i64)
                                      -> bool {
    ::ffi::qt_core_c_QProcess_startDetached_program_arguments_workingDirectory_pid(program as *const ::string::String, arguments as *const ::string_list::StringList, working_directory as *const ::string::String, pid)
  }

  /// C++ method: <span style='color: green;'>```static QStringList QProcess::systemEnvironment()```</span>
  ///
  ///
  pub fn system_environment() -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QProcess_systemEnvironment_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QProcess::terminate()```</span>
  ///
  ///
  pub fn terminate(&mut self) {
    unsafe { ::ffi::qt_core_c_QProcess_terminate(self as *mut ::process::Process) }
  }

  /// C++ method: <span style='color: green;'>```static QString QProcess::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QProcess_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QProcess::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QProcess_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QProcess::waitForBytesWritten```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn wait_for_bytes_written(&mut self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QProcess::waitForBytesWritten()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn wait_for_bytes_written(&mut self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QProcess::waitForBytesWritten(int msecs = ?)```</span>
  ///
  ///
  pub fn wait_for_bytes_written<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::ProcessWaitForBytesWrittenArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QProcess::waitForFinished```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn wait_for_finished(&mut self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QProcess::waitForFinished()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn wait_for_finished(&mut self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QProcess::waitForFinished(int msecs = ?)```</span>
  ///
  ///
  pub fn wait_for_finished<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::ProcessWaitForFinishedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QProcess::waitForReadyRead```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn wait_for_ready_read(&mut self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QProcess::waitForReadyRead()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn wait_for_ready_read(&mut self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QProcess::waitForReadyRead(int msecs = ?)```</span>
  ///
  ///
  pub fn wait_for_ready_read<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::ProcessWaitForReadyReadArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QProcess::waitForStarted```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn wait_for_started(&mut self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QProcess::waitForStarted()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn wait_for_started(&mut self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QProcess::waitForStarted(int msecs = ?)```</span>
  ///
  ///
  pub fn wait_for_started<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::ProcessWaitForStartedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString QProcess::workingDirectory() const```</span>
  ///
  ///
  pub fn working_directory(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QProcess_workingDirectory_to_output(self as *const ::process::Process, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::process::Process {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QProcess_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Process`.
  pub struct Signals<'a>(&'a ::process::Process);
  /// Represents a built-in Qt signal `QProcess::readyReadStandardOutput`.
  ///
  /// An object of this type can be created from `Process` with `object.signals().ready_read_standard_output()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Process` object.
  pub struct ReadyReadStandardOutput<'a>(&'a ::process::Process);
  impl<'a> ::connection::Receiver for ReadyReadStandardOutput<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2readyReadStandardOutput()\0"
    }
  }
  impl<'a> ::connection::Signal for ReadyReadStandardOutput<'a> {}
  /// Represents a built-in Qt signal `QProcess::channelReadyRead`.
  ///
  /// An object of this type can be created from `Process` with `object.signals().channel_ready_read()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Process` object.
  pub struct ChannelReadyRead<'a>(&'a ::process::Process);
  impl<'a> ::connection::Receiver for ChannelReadyRead<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2channelReadyRead(int)\0"
    }
  }
  impl<'a> ::connection::Signal for ChannelReadyRead<'a> {}
  /// Represents a built-in Qt signal `QProcess::error`.
  ///
  /// An object of this type can be created from `Process` with `object.signals().error()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Process` object.
  pub struct Error<'a>(&'a ::process::Process);
  impl<'a> ::connection::Receiver for Error<'a> {
    type Arguments = (&'static ::process::ProcessError,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2error(QProcess::ProcessError)\0"
    }
  }
  impl<'a> ::connection::Signal for Error<'a> {}
  /// Represents a built-in Qt signal `QProcess::readyRead`.
  ///
  /// An object of this type can be created from `Process` with `object.signals().ready_read()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Process` object.
  pub struct ReadyRead<'a>(&'a ::process::Process);
  impl<'a> ::connection::Receiver for ReadyRead<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2readyRead()\0"
    }
  }
  impl<'a> ::connection::Signal for ReadyRead<'a> {}
  /// Represents a built-in Qt signal `QProcess::started`.
  ///
  /// An object of this type can be created from `Process` with `object.signals().started()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Process` object.
  pub struct Started<'a>(&'a ::process::Process);
  impl<'a> ::connection::Receiver for Started<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2started()\0"
    }
  }
  impl<'a> ::connection::Signal for Started<'a> {}
  /// Represents a built-in Qt signal `QProcess::errorOccurred`.
  ///
  /// An object of this type can be created from `Process` with `object.signals().error_occurred()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Process` object.
  pub struct ErrorOccurred<'a>(&'a ::process::Process);
  impl<'a> ::connection::Receiver for ErrorOccurred<'a> {
    type Arguments = (&'static ::process::ProcessError,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2errorOccurred(QProcess::ProcessError)\0"
    }
  }
  impl<'a> ::connection::Signal for ErrorOccurred<'a> {}
  /// Represents a built-in Qt signal `QProcess::channelBytesWritten`.
  ///
  /// An object of this type can be created from `Process` with `object.signals().channel_bytes_written()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Process` object.
  pub struct ChannelBytesWritten<'a>(&'a ::process::Process);
  impl<'a> ::connection::Receiver for ChannelBytesWritten<'a> {
    type Arguments = (::libc::c_int, i64);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2channelBytesWritten(int,qint64)\0"
    }
  }
  impl<'a> ::connection::Signal for ChannelBytesWritten<'a> {}
  /// Represents a built-in Qt signal `QProcess::bytesWritten`.
  ///
  /// An object of this type can be created from `Process` with `object.signals().bytes_written()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Process` object.
  pub struct BytesWritten<'a>(&'a ::process::Process);
  impl<'a> ::connection::Receiver for BytesWritten<'a> {
    type Arguments = (i64,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2bytesWritten(qint64)\0"
    }
  }
  impl<'a> ::connection::Signal for BytesWritten<'a> {}
  /// Represents a built-in Qt signal `QProcess::stateChanged`.
  ///
  /// An object of this type can be created from `Process` with `object.signals().state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Process` object.
  pub struct StateChanged<'a>(&'a ::process::Process);
  impl<'a> ::connection::Receiver for StateChanged<'a> {
    type Arguments = (&'static ::process::ProcessState,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2stateChanged(QProcess::ProcessState)\0"
    }
  }
  impl<'a> ::connection::Signal for StateChanged<'a> {}
  /// Represents a built-in Qt signal `QProcess::readChannelFinished`.
  ///
  /// An object of this type can be created from `Process` with `object.signals().read_channel_finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Process` object.
  pub struct ReadChannelFinished<'a>(&'a ::process::Process);
  impl<'a> ::connection::Receiver for ReadChannelFinished<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2readChannelFinished()\0"
    }
  }
  impl<'a> ::connection::Signal for ReadChannelFinished<'a> {}
  /// Represents a built-in Qt signal `QProcess::finished`.
  ///
  /// An object of this type can be created from `Process` with `object.signals().finished_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Process` object.
  pub struct FinishedCInt<'a>(&'a ::process::Process);
  impl<'a> ::connection::Receiver for FinishedCInt<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2finished(int)\0"
    }
  }
  impl<'a> ::connection::Signal for FinishedCInt<'a> {}
  /// Represents a built-in Qt signal `QProcess::finished`.
  ///
  /// An object of this type can be created from `Process` with `object.signals().finished_c_int_exit_status_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Process` object.
  pub struct FinishedCIntExitStatusRef<'a>(&'a ::process::Process);
  impl<'a> ::connection::Receiver for FinishedCIntExitStatusRef<'a> {
    type Arguments = (::libc::c_int, &'static ::process::ExitStatus);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2finished(int,QProcess::ExitStatus)\0"
    }
  }
  impl<'a> ::connection::Signal for FinishedCIntExitStatusRef<'a> {}
  /// Represents a built-in Qt signal `QProcess::aboutToClose`.
  ///
  /// An object of this type can be created from `Process` with `object.signals().about_to_close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Process` object.
  pub struct AboutToClose<'a>(&'a ::process::Process);
  impl<'a> ::connection::Receiver for AboutToClose<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2aboutToClose()\0"
    }
  }
  impl<'a> ::connection::Signal for AboutToClose<'a> {}
  /// Represents a built-in Qt signal `QProcess::readyReadStandardError`.
  ///
  /// An object of this type can be created from `Process` with `object.signals().ready_read_standard_error()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Process` object.
  pub struct ReadyReadStandardError<'a>(&'a ::process::Process);
  impl<'a> ::connection::Receiver for ReadyReadStandardError<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2readyReadStandardError()\0"
    }
  }
  impl<'a> ::connection::Signal for ReadyReadStandardError<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QProcess::readyReadStandardOutput`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn ready_read_standard_output(&self) -> ReadyReadStandardOutput {
      ReadyReadStandardOutput(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProcess::channelReadyRead`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn channel_ready_read(&self) -> ChannelReadyRead {
      ChannelReadyRead(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProcess::error`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn error(&self) -> Error {
      Error(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProcess::readyRead`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn ready_read(&self) -> ReadyRead {
      ReadyRead(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProcess::started`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn started(&self) -> Started {
      Started(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProcess::errorOccurred`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn error_occurred(&self) -> ErrorOccurred {
      ErrorOccurred(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProcess::channelBytesWritten`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn channel_bytes_written(&self) -> ChannelBytesWritten {
      ChannelBytesWritten(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProcess::bytesWritten`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bytes_written(&self) -> BytesWritten {
      BytesWritten(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProcess::stateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn state_changed(&self) -> StateChanged {
      StateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProcess::readChannelFinished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn read_channel_finished(&self) -> ReadChannelFinished {
      ReadChannelFinished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProcess::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished_c_int(&self) -> FinishedCInt {
      FinishedCInt(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProcess::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished_c_int_exit_status_ref(&self) -> FinishedCIntExitStatusRef {
      FinishedCIntExitStatusRef(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProcess::aboutToClose`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn about_to_close(&self) -> AboutToClose {
      AboutToClose(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProcess::readyReadStandardError`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn ready_read_standard_error(&self) -> ReadyReadStandardError {
      ReadyReadStandardError(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Process`.
  pub struct Slots<'a>(&'a ::process::Process);
  /// Represents a built-in Qt slot `QProcess::kill`.
  ///
  /// An object of this type can be created from `Process` with `object.slots().kill()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Process` object.
  pub struct Kill<'a>(&'a ::process::Process);
  impl<'a> ::connection::Receiver for Kill<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1kill()\0"
    }
  }
  /// Represents a built-in Qt slot `QProcess::terminate`.
  ///
  /// An object of this type can be created from `Process` with `object.slots().terminate()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Process` object.
  pub struct Terminate<'a>(&'a ::process::Process);
  impl<'a> ::connection::Receiver for Terminate<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1terminate()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QProcess::kill`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn kill(&self) -> Kill {
      Kill(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProcess::terminate`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn terminate(&self) -> Terminate {
      Terminate(self.0)
    }
  }
  impl ::process::Process {
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

/// C++ type: <span style='color: green;'>```QProcess::ProcessChannel```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ProcessChannel {
  /// C++ enum variant: <span style='color: green;'>```StandardOutput = 0```</span>
  Output = 0,
  /// C++ enum variant: <span style='color: green;'>```StandardError = 1```</span>
  Error = 1,
}

/// C++ type: <span style='color: green;'>```QProcess::ProcessChannelMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ProcessChannelMode {
  /// C++ enum variant: <span style='color: green;'>```SeparateChannels = 0```</span>
  SeparateChannels = 0,
  /// C++ enum variant: <span style='color: green;'>```MergedChannels = 1```</span>
  MergedChannels = 1,
  /// C++ enum variant: <span style='color: green;'>```ForwardedChannels = 2```</span>
  ForwardedChannels = 2,
  /// C++ enum variant: <span style='color: green;'>```ForwardedOutputChannel = 3```</span>
  ForwardedOutputChannel = 3,
  /// C++ enum variant: <span style='color: green;'>```ForwardedErrorChannel = 4```</span>
  ForwardedErrorChannel = 4,
}

/// C++ type: <span style='color: green;'>```QProcess::ProcessError```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ProcessError {
  /// C++ enum variant: <span style='color: green;'>```FailedToStart = 0```</span>
  FailedToStart = 0,
  /// C++ enum variant: <span style='color: green;'>```Crashed = 1```</span>
  Crashed = 1,
  /// C++ enum variant: <span style='color: green;'>```Timedout = 2```</span>
  Timedout = 2,
  /// C++ enum variant: <span style='color: green;'>```ReadError = 3```</span>
  ReadError = 3,
  /// C++ enum variant: <span style='color: green;'>```WriteError = 4```</span>
  WriteError = 4,
  /// C++ enum variant: <span style='color: green;'>```UnknownError = 5```</span>
  UnknownError = 5,
}

/// C++ type: <span style='color: green;'>```QProcess::ProcessState```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ProcessState {
  /// C++ enum variant: <span style='color: green;'>```NotRunning = 0```</span>
  NotRunning = 0,
  /// C++ enum variant: <span style='color: green;'>```Starting = 1```</span>
  Starting = 1,
  /// C++ enum variant: <span style='color: green;'>```Running = 2```</span>
  Running = 2,
}

/// C++ method: <span style='color: green;'>```void swap(QProcessEnvironment& value1, QProcessEnvironment& value2)```</span>
///
///
pub fn swap(value1: &mut ::process_environment::ProcessEnvironment,
            value2: &mut ::process_environment::ProcessEnvironment) {
  unsafe {
    ::ffi::qt_core_c_QProcess_G_swap(value1 as *mut ::process_environment::ProcessEnvironment,
                                     value2 as *mut ::process_environment::ProcessEnvironment)
  }
}

impl ::cpp_utils::DynamicCast<::process::Process> for ::io_device::IODevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::process::Process> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QProcess_G_dynamic_cast_QProcess_ptr_QIODevice(self as *mut ::io_device::IODevice) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::process::Process> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QProcess_G_dynamic_cast_QProcess_ptr_QIODevice(self as *const ::io_device::IODevice as *mut ::io_device::IODevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::process::Process> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::process::Process> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QProcess_G_dynamic_cast_QProcess_ptr_QObject(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::process::Process> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QProcess_G_dynamic_cast_QProcess_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::io_device::IODevice> for ::process::Process {
  fn static_cast_mut(&mut self) -> &mut ::io_device::IODevice {
    let ffi_result = unsafe { ::ffi::qt_core_c_QProcess_G_static_cast_QIODevice_ptr(self as *mut ::process::Process) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::io_device::IODevice {
    let ffi_result = unsafe { ::ffi::qt_core_c_QProcess_G_static_cast_QIODevice_ptr(self as *const ::process::Process as *mut ::process::Process) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::process::Process {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QProcess_G_static_cast_QObject_ptr(self as *mut ::process::Process) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QProcess_G_static_cast_QObject_ptr(self as *const ::process::Process as *mut ::process::Process) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::process::Process> for ::io_device::IODevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::process::Process {
    let ffi_result =
      ::ffi::qt_core_c_QProcess_G_static_cast_QProcess_ptr_QIODevice(self as *mut ::io_device::IODevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::process::Process {
    let ffi_result = ::ffi::qt_core_c_QProcess_G_static_cast_QProcess_ptr_QIODevice(self as *const ::io_device::IODevice as *mut ::io_device::IODevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::process::Process> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::process::Process {
    let ffi_result = ::ffi::qt_core_c_QProcess_G_static_cast_QProcess_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::process::Process {
    let ffi_result = ::ffi::qt_core_c_QProcess_G_static_cast_QProcess_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::process::Process {
  type Target = ::io_device::IODevice;
  fn deref(&self) -> &::io_device::IODevice {
    let ffi_result = unsafe { ::ffi::qt_core_c_QProcess_G_static_cast_QIODevice_ptr(self as *const ::process::Process as *mut ::process::Process) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::process::Process {
  fn deref_mut(&mut self) -> &mut ::io_device::IODevice {
    let ffi_result = unsafe { ::ffi::qt_core_c_QProcess_G_static_cast_QIODevice_ptr(self as *mut ::process::Process) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Process::execute](../struct.Process.html#method.execute) method.
  pub trait ProcessExecuteArgs {
    fn exec(self) -> ::libc::c_int;
  }
  impl<'a> ProcessExecuteArgs for &'a ::string::String {
    fn exec(self) -> ::libc::c_int {
      let command = self;
      unsafe { ::ffi::qt_core_c_QProcess_execute_command(command as *const ::string::String) }
    }
  }
  impl<'a> ProcessExecuteArgs for (&'a ::string::String, &'a ::string_list::StringList) {
    fn exec(self) -> ::libc::c_int {
      let program = self.0;
      let arguments = self.1;
      unsafe {
        ::ffi::qt_core_c_QProcess_execute_program_arguments(program as *const ::string::String,
                                                            arguments as *const ::string_list::StringList)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Process::open](../struct.Process.html#method.open) method.
  pub trait ProcessOpenArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::process::Process) -> bool;
  }
  impl<'largs> ProcessOpenArgs<'largs> for ::flags::Flags<::io_device::OpenModeFlag> {
    fn exec(self, original_self: &'largs mut ::process::Process) -> bool {
      let mode = self;
      unsafe {
        ::ffi::qt_core_c_QProcess_open_mode(original_self as *mut ::process::Process,
                                            mode.to_int() as ::libc::c_uint)
      }
    }
  }
  impl<'largs> ProcessOpenArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::process::Process) -> bool {

      unsafe { ::ffi::qt_core_c_QProcess_open_no_args(original_self as *mut ::process::Process) }
    }
  }
  /// This trait represents a set of arguments accepted by [Process::set_standard_error_file](../struct.Process.html#method.set_standard_error_file) method.
  pub trait ProcessSetStandardErrorFileArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::process::Process) -> ();
  }
  impl<'largs> ProcessSetStandardErrorFileArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::process::Process) -> () {
      let file_name = self;
      unsafe {
        ::ffi::qt_core_c_QProcess_setStandardErrorFile_fileName(original_self as *mut ::process::Process,
                                                                file_name as *const ::string::String)
      }
    }
  }
  impl<'largs> ProcessSetStandardErrorFileArgs<'largs>
    for (&'largs ::string::String, ::flags::Flags<::io_device::OpenModeFlag>) {
    fn exec(self, original_self: &'largs mut ::process::Process) -> () {
      let file_name = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_core_c_QProcess_setStandardErrorFile_fileName_mode(original_self as *mut ::process::Process,
                                                                     file_name as *const ::string::String,
                                                                     mode.to_int() as ::libc::c_uint)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Process::set_standard_output_file](../struct.Process.html#method.set_standard_output_file) method.
  pub trait ProcessSetStandardOutputFileArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::process::Process) -> ();
  }
  impl<'largs> ProcessSetStandardOutputFileArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::process::Process) -> () {
      let file_name = self;
      unsafe {
        ::ffi::qt_core_c_QProcess_setStandardOutputFile_fileName(original_self as *mut ::process::Process,
                                                                 file_name as *const ::string::String)
      }
    }
  }
  impl<'largs> ProcessSetStandardOutputFileArgs<'largs>
    for (&'largs ::string::String, ::flags::Flags<::io_device::OpenModeFlag>) {
    fn exec(self, original_self: &'largs mut ::process::Process) -> () {
      let file_name = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_core_c_QProcess_setStandardOutputFile_fileName_mode(original_self as *mut ::process::Process,
                                                                      file_name as *const ::string::String,
                                                                      mode.to_int() as ::libc::c_uint)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Process::start](../struct.Process.html#method.start) method.
  pub trait ProcessStartArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::process::Process) -> ();
  }
  impl<'largs> ProcessStartArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::process::Process) -> () {
      let command = self;
      unsafe {
        ::ffi::qt_core_c_QProcess_start_command(original_self as *mut ::process::Process,
                                                command as *const ::string::String)
      }
    }
  }
  impl<'largs> ProcessStartArgs<'largs> for (&'largs ::string::String, ::flags::Flags<::io_device::OpenModeFlag>) {
    fn exec(self, original_self: &'largs mut ::process::Process) -> () {
      let command = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_core_c_QProcess_start_command_mode(original_self as *mut ::process::Process,
                                                     command as *const ::string::String,
                                                     mode.to_int() as ::libc::c_uint)
      }
    }
  }
  impl<'largs> ProcessStartArgs<'largs> for ::flags::Flags<::io_device::OpenModeFlag> {
    fn exec(self, original_self: &'largs mut ::process::Process) -> () {
      let mode = self;
      unsafe {
        ::ffi::qt_core_c_QProcess_start_mode(original_self as *mut ::process::Process,
                                             mode.to_int() as ::libc::c_uint)
      }
    }
  }
  impl<'largs> ProcessStartArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::process::Process) -> () {

      unsafe { ::ffi::qt_core_c_QProcess_start_no_args(original_self as *mut ::process::Process) }
    }
  }
  impl<'largs> ProcessStartArgs<'largs> for (&'largs ::string::String, &'largs ::string_list::StringList) {
    fn exec(self, original_self: &'largs mut ::process::Process) -> () {
      let program = self.0;
      let arguments = self.1;
      unsafe {
        ::ffi::qt_core_c_QProcess_start_program_arguments(original_self as *mut ::process::Process,
                                                          program as *const ::string::String,
                                                          arguments as *const ::string_list::StringList)
      }
    }
  }
  impl<'largs> ProcessStartArgs<'largs>
    for (&'largs ::string::String, &'largs ::string_list::StringList, ::flags::Flags<::io_device::OpenModeFlag>) {
    fn exec(self, original_self: &'largs mut ::process::Process) -> () {
      let program = self.0;
      let arguments = self.1;
      let mode = self.2;
      unsafe {
        ::ffi::qt_core_c_QProcess_start_program_arguments_mode(original_self as *mut ::process::Process,
                                                               program as *const ::string::String,
                                                               arguments as *const ::string_list::StringList,
                                                               mode.to_int() as ::libc::c_uint)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Process::start_detached](../struct.Process.html#method.start_detached) method.
  pub trait ProcessStartDetachedArgs {
    fn exec(self) -> bool;
  }
  impl<'a> ProcessStartDetachedArgs for &'a ::string::String {
    fn exec(self) -> bool {
      let command = self;
      unsafe { ::ffi::qt_core_c_QProcess_startDetached_command(command as *const ::string::String) }
    }
  }
  impl<'a> ProcessStartDetachedArgs for (&'a ::string::String, &'a ::string_list::StringList) {
    fn exec(self) -> bool {
      let program = self.0;
      let arguments = self.1;
      unsafe {
        ::ffi::qt_core_c_QProcess_startDetached_program_arguments(program as *const ::string::String,
                                                                  arguments as *const ::string_list::StringList)
      }
    }
  }
  impl<'a> ProcessStartDetachedArgs for (&'a ::string::String, &'a ::string_list::StringList, &'a ::string::String) {
    fn exec(self) -> bool {
      let program = self.0;
      let arguments = self.1;
      let working_directory = self.2;
      unsafe { ::ffi::qt_core_c_QProcess_startDetached_program_arguments_workingDirectory(program as *const ::string::String, arguments as *const ::string_list::StringList, working_directory as *const ::string::String) }
    }
  }
  /// This trait represents a set of arguments accepted by [Process::wait_for_bytes_written](../struct.Process.html#method.wait_for_bytes_written) method.
  pub trait ProcessWaitForBytesWrittenArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::process::Process) -> bool;
  }
  impl<'largs> ProcessWaitForBytesWrittenArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::process::Process) -> bool {
      let msecs = self;
      unsafe { ::ffi::qt_core_c_QProcess_waitForBytesWritten_msecs(original_self as *mut ::process::Process, msecs) }
    }
  }
  impl<'largs> ProcessWaitForBytesWrittenArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::process::Process) -> bool {

      unsafe { ::ffi::qt_core_c_QProcess_waitForBytesWritten_no_args(original_self as *mut ::process::Process) }
    }
  }
  /// This trait represents a set of arguments accepted by [Process::wait_for_finished](../struct.Process.html#method.wait_for_finished) method.
  pub trait ProcessWaitForFinishedArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::process::Process) -> bool;
  }
  impl<'largs> ProcessWaitForFinishedArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::process::Process) -> bool {
      let msecs = self;
      unsafe { ::ffi::qt_core_c_QProcess_waitForFinished_msecs(original_self as *mut ::process::Process, msecs) }
    }
  }
  impl<'largs> ProcessWaitForFinishedArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::process::Process) -> bool {

      unsafe { ::ffi::qt_core_c_QProcess_waitForFinished_no_args(original_self as *mut ::process::Process) }
    }
  }
  /// This trait represents a set of arguments accepted by [Process::wait_for_ready_read](../struct.Process.html#method.wait_for_ready_read) method.
  pub trait ProcessWaitForReadyReadArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::process::Process) -> bool;
  }
  impl<'largs> ProcessWaitForReadyReadArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::process::Process) -> bool {
      let msecs = self;
      unsafe { ::ffi::qt_core_c_QProcess_waitForReadyRead_msecs(original_self as *mut ::process::Process, msecs) }
    }
  }
  impl<'largs> ProcessWaitForReadyReadArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::process::Process) -> bool {

      unsafe { ::ffi::qt_core_c_QProcess_waitForReadyRead_no_args(original_self as *mut ::process::Process) }
    }
  }
  /// This trait represents a set of arguments accepted by [Process::wait_for_started](../struct.Process.html#method.wait_for_started) method.
  pub trait ProcessWaitForStartedArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::process::Process) -> bool;
  }
  impl<'largs> ProcessWaitForStartedArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::process::Process) -> bool {
      let msecs = self;
      unsafe { ::ffi::qt_core_c_QProcess_waitForStarted_msecs(original_self as *mut ::process::Process, msecs) }
    }
  }
  impl<'largs> ProcessWaitForStartedArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::process::Process) -> bool {

      unsafe { ::ffi::qt_core_c_QProcess_waitForStarted_no_args(original_self as *mut ::process::Process) }
    }
  }
}
