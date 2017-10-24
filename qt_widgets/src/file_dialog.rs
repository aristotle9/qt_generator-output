/// C++ type: <span style='color: green;'>```QFileDialog::AcceptMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum AcceptMode {
  /// C++ enum variant: <span style='color: green;'>```AcceptOpen = 0```</span>
  Open = 0,
  /// C++ enum variant: <span style='color: green;'>```AcceptSave = 1```</span>
  Save = 1,
}

/// C++ type: <span style='color: green;'>```QFileDialog::DialogLabel```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DialogLabel {
  /// C++ enum variant: <span style='color: green;'>```LookIn = 0```</span>
  LookIn = 0,
  /// C++ enum variant: <span style='color: green;'>```FileName = 1```</span>
  FileName = 1,
  /// C++ enum variant: <span style='color: green;'>```FileType = 2```</span>
  FileType = 2,
  /// C++ enum variant: <span style='color: green;'>```Accept = 3```</span>
  Accept = 3,
  /// C++ enum variant: <span style='color: green;'>```Reject = 4```</span>
  Reject = 4,
}

/// C++ type: <span style='color: green;'>```QFileDialog```</span>
#[repr(C)]
pub struct FileDialog(u8);

impl FileDialog {
  /// C++ method: <span style='color: green;'>```QFileDialog::AcceptMode QFileDialog::acceptMode() const```</span>
  ///
  ///
  pub fn accept_mode(&self) -> ::file_dialog::AcceptMode {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_acceptMode(self as *const ::file_dialog::FileDialog) }
  }

  /// C++ method: <span style='color: green;'>```bool QFileDialog::confirmOverwrite() const```</span>
  ///
  ///
  pub fn confirm_overwrite(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_confirmOverwrite(self as *const ::file_dialog::FileDialog) }
  }

  /// C++ method: <span style='color: green;'>```QString QFileDialog::defaultSuffix() const```</span>
  ///
  ///
  pub fn default_suffix(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_defaultSuffix_to_output(self as *const ::file_dialog::FileDialog, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDir QFileDialog::directory() const```</span>
  ///
  ///
  pub fn directory(&self) -> ::qt_core::dir::Dir {
    {
      let mut object: ::qt_core::dir::Dir =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_directory_to_output(self as *const ::file_dialog::FileDialog, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QUrl QFileDialog::directoryUrl() const```</span>
  ///
  ///
  pub fn directory_url(&self) -> ::qt_core::url::Url {
    {
      let mut object: ::qt_core::url::Url =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_directoryUrl_to_output(self as *const ::file_dialog::FileDialog, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileDialog::FileMode QFileDialog::fileMode() const```</span>
  ///
  ///
  pub fn file_mode(&self) -> ::file_dialog::FileMode {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_fileMode(self as *const ::file_dialog::FileDialog) }
  }

  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getExistingDirectory()```</span>
  ///
  ///
  pub fn get_existing_directory() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_getExistingDirectory_to_output_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileDialog::getExistingDirectory```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_existing_directory_unsafe(*mut ::widget::Widget) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getExistingDirectory(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_existing_directory_unsafe((*mut ::widget::Widget, &::qt_core::string::String)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getExistingDirectory(QWidget* parent = ?, const QString& caption = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn get_existing_directory_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getExistingDirectory(QWidget* parent = ?, const QString& caption = ?, const QString& dir = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn get_existing_directory_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, ::qt_core::flags::Flags<::file_dialog::Option>)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getExistingDirectory(QWidget* parent = ?, const QString& caption = ?, const QString& dir = ?, QFlags<QFileDialog::Option> options = ?)```</span>
  ///
  ///
  pub unsafe fn get_existing_directory_unsafe<Args>(args: Args) -> ::qt_core::string::String
    where Args: overloading::FileDialogGetExistingDirectoryUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getExistingDirectoryUrl()```</span>
  ///
  ///
  pub fn get_existing_directory_url() -> ::qt_core::url::Url {
    {
      let mut object: ::qt_core::url::Url =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_getExistingDirectoryUrl_to_output_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileDialog::getExistingDirectoryUrl```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_existing_directory_url_unsafe(*mut ::widget::Widget) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getExistingDirectoryUrl(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_existing_directory_url_unsafe((*mut ::widget::Widget, &::qt_core::string::String)) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getExistingDirectoryUrl(QWidget* parent = ?, const QString& caption = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn get_existing_directory_url_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::url::Url)) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getExistingDirectoryUrl(QWidget* parent = ?, const QString& caption = ?, const QUrl& dir = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn get_existing_directory_url_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::url::Url, ::qt_core::flags::Flags<::file_dialog::Option>)) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getExistingDirectoryUrl(QWidget* parent = ?, const QString& caption = ?, const QUrl& dir = ?, QFlags<QFileDialog::Option> options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn get_existing_directory_url_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::url::Url, ::qt_core::flags::Flags<::file_dialog::Option>, &::qt_core::string_list::StringList)) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getExistingDirectoryUrl(QWidget* parent = ?, const QString& caption = ?, const QUrl& dir = ?, QFlags<QFileDialog::Option> options = ?, const QStringList& supportedSchemes = ?)```</span>
  ///
  ///
  pub unsafe fn get_existing_directory_url_unsafe<Args>(args: Args) -> ::qt_core::url::Url
    where Args: overloading::FileDialogGetExistingDirectoryUrlUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getOpenFileName()```</span>
  ///
  ///
  pub fn get_open_file_name() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileName_to_output_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileDialog::getOpenFileName```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_open_file_name_unsafe(*mut ::widget::Widget) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getOpenFileName(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_open_file_name_unsafe((*mut ::widget::Widget, &::qt_core::string::String)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getOpenFileName(QWidget* parent = ?, const QString& caption = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn get_open_file_name_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getOpenFileName(QWidget* parent = ?, const QString& caption = ?, const QString& dir = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn get_open_file_name_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getOpenFileName(QWidget* parent = ?, const QString& caption = ?, const QString& dir = ?, const QString& filter = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn get_open_file_name_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, *mut ::qt_core::string::String)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getOpenFileName(QWidget* parent = ?, const QString& caption = ?, const QString& dir = ?, const QString& filter = ?, QString* selectedFilter = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn get_open_file_name_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, *mut ::qt_core::string::String, ::qt_core::flags::Flags<::file_dialog::Option>)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getOpenFileName(QWidget* parent = ?, const QString& caption = ?, const QString& dir = ?, const QString& filter = ?, QString* selectedFilter = ?, QFlags<QFileDialog::Option> options = ?)```</span>
  ///
  ///
  pub unsafe fn get_open_file_name_unsafe<Args>(args: Args) -> ::qt_core::string::String
    where Args: overloading::FileDialogGetOpenFileNameUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QStringList QFileDialog::getOpenFileNames()```</span>
  ///
  ///
  pub fn get_open_file_names() -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileNames_to_output_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileDialog::getOpenFileNames```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_open_file_names_unsafe(*mut ::widget::Widget) -> ::qt_core::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```static QStringList QFileDialog::getOpenFileNames(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_open_file_names_unsafe((*mut ::widget::Widget, &::qt_core::string::String)) -> ::qt_core::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```static QStringList QFileDialog::getOpenFileNames(QWidget* parent = ?, const QString& caption = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn get_open_file_names_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String)) -> ::qt_core::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```static QStringList QFileDialog::getOpenFileNames(QWidget* parent = ?, const QString& caption = ?, const QString& dir = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn get_open_file_names_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String)) -> ::qt_core::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```static QStringList QFileDialog::getOpenFileNames(QWidget* parent = ?, const QString& caption = ?, const QString& dir = ?, const QString& filter = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn get_open_file_names_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, *mut ::qt_core::string::String)) -> ::qt_core::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```static QStringList QFileDialog::getOpenFileNames(QWidget* parent = ?, const QString& caption = ?, const QString& dir = ?, const QString& filter = ?, QString* selectedFilter = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn get_open_file_names_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, *mut ::qt_core::string::String, ::qt_core::flags::Flags<::file_dialog::Option>)) -> ::qt_core::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```static QStringList QFileDialog::getOpenFileNames(QWidget* parent = ?, const QString& caption = ?, const QString& dir = ?, const QString& filter = ?, QString* selectedFilter = ?, QFlags<QFileDialog::Option> options = ?)```</span>
  ///
  ///
  pub unsafe fn get_open_file_names_unsafe<Args>(args: Args) -> ::qt_core::string_list::StringList
    where Args: overloading::FileDialogGetOpenFileNamesUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getOpenFileUrl()```</span>
  ///
  ///
  pub fn get_open_file_url() -> ::qt_core::url::Url {
    {
      let mut object: ::qt_core::url::Url =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileUrl_to_output_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileDialog::getOpenFileUrl```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_open_file_url_unsafe(*mut ::widget::Widget) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getOpenFileUrl(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_open_file_url_unsafe((*mut ::widget::Widget, &::qt_core::string::String)) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getOpenFileUrl(QWidget* parent = ?, const QString& caption = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn get_open_file_url_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::url::Url)) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getOpenFileUrl(QWidget* parent = ?, const QString& caption = ?, const QUrl& dir = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn get_open_file_url_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::url::Url, &::qt_core::string::String)) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getOpenFileUrl(QWidget* parent = ?, const QString& caption = ?, const QUrl& dir = ?, const QString& filter = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn get_open_file_url_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::url::Url, &::qt_core::string::String, *mut ::qt_core::string::String)) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getOpenFileUrl(QWidget* parent = ?, const QString& caption = ?, const QUrl& dir = ?, const QString& filter = ?, QString* selectedFilter = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn get_open_file_url_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::url::Url, &::qt_core::string::String, *mut ::qt_core::string::String, ::qt_core::flags::Flags<::file_dialog::Option>)) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getOpenFileUrl(QWidget* parent = ?, const QString& caption = ?, const QUrl& dir = ?, const QString& filter = ?, QString* selectedFilter = ?, QFlags<QFileDialog::Option> options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn get_open_file_url_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::url::Url, &::qt_core::string::String, *mut ::qt_core::string::String, ::qt_core::flags::Flags<::file_dialog::Option>, &::qt_core::string_list::StringList)) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getOpenFileUrl(QWidget* parent = ?, const QString& caption = ?, const QUrl& dir = ?, const QString& filter = ?, QString* selectedFilter = ?, QFlags<QFileDialog::Option> options = ?, const QStringList& supportedSchemes = ?)```</span>
  ///
  ///
  pub unsafe fn get_open_file_url_unsafe<Args>(args: Args) -> ::qt_core::url::Url
    where Args: overloading::FileDialogGetOpenFileUrlUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QList<QUrl> QFileDialog::getOpenFileUrls()```</span>
  ///
  ///
  pub fn get_open_file_urls() -> ::qt_core::list::ListUrl {
    {
      let mut object: ::qt_core::list::ListUrl =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileUrls_to_output_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileDialog::getOpenFileUrls```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_open_file_urls_unsafe(*mut ::widget::Widget) -> ::qt_core::list::ListUrl```<br>
  /// C++ method: <span style='color: green;'>```static QList<QUrl> QFileDialog::getOpenFileUrls(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_open_file_urls_unsafe((*mut ::widget::Widget, &::qt_core::string::String)) -> ::qt_core::list::ListUrl```<br>
  /// C++ method: <span style='color: green;'>```static QList<QUrl> QFileDialog::getOpenFileUrls(QWidget* parent = ?, const QString& caption = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn get_open_file_urls_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::url::Url)) -> ::qt_core::list::ListUrl```<br>
  /// C++ method: <span style='color: green;'>```static QList<QUrl> QFileDialog::getOpenFileUrls(QWidget* parent = ?, const QString& caption = ?, const QUrl& dir = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn get_open_file_urls_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::url::Url, &::qt_core::string::String)) -> ::qt_core::list::ListUrl```<br>
  /// C++ method: <span style='color: green;'>```static QList<QUrl> QFileDialog::getOpenFileUrls(QWidget* parent = ?, const QString& caption = ?, const QUrl& dir = ?, const QString& filter = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn get_open_file_urls_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::url::Url, &::qt_core::string::String, *mut ::qt_core::string::String)) -> ::qt_core::list::ListUrl```<br>
  /// C++ method: <span style='color: green;'>```static QList<QUrl> QFileDialog::getOpenFileUrls(QWidget* parent = ?, const QString& caption = ?, const QUrl& dir = ?, const QString& filter = ?, QString* selectedFilter = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn get_open_file_urls_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::url::Url, &::qt_core::string::String, *mut ::qt_core::string::String, ::qt_core::flags::Flags<::file_dialog::Option>)) -> ::qt_core::list::ListUrl```<br>
  /// C++ method: <span style='color: green;'>```static QList<QUrl> QFileDialog::getOpenFileUrls(QWidget* parent = ?, const QString& caption = ?, const QUrl& dir = ?, const QString& filter = ?, QString* selectedFilter = ?, QFlags<QFileDialog::Option> options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn get_open_file_urls_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::url::Url, &::qt_core::string::String, *mut ::qt_core::string::String, ::qt_core::flags::Flags<::file_dialog::Option>, &::qt_core::string_list::StringList)) -> ::qt_core::list::ListUrl```<br>
  /// C++ method: <span style='color: green;'>```static QList<QUrl> QFileDialog::getOpenFileUrls(QWidget* parent = ?, const QString& caption = ?, const QUrl& dir = ?, const QString& filter = ?, QString* selectedFilter = ?, QFlags<QFileDialog::Option> options = ?, const QStringList& supportedSchemes = ?)```</span>
  ///
  ///
  pub unsafe fn get_open_file_urls_unsafe<Args>(args: Args) -> ::qt_core::list::ListUrl
    where Args: overloading::FileDialogGetOpenFileUrlsUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getSaveFileName()```</span>
  ///
  ///
  pub fn get_save_file_name() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_getSaveFileName_to_output_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileDialog::getSaveFileName```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_save_file_name_unsafe(*mut ::widget::Widget) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getSaveFileName(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_save_file_name_unsafe((*mut ::widget::Widget, &::qt_core::string::String)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getSaveFileName(QWidget* parent = ?, const QString& caption = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn get_save_file_name_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getSaveFileName(QWidget* parent = ?, const QString& caption = ?, const QString& dir = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn get_save_file_name_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getSaveFileName(QWidget* parent = ?, const QString& caption = ?, const QString& dir = ?, const QString& filter = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn get_save_file_name_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, *mut ::qt_core::string::String)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getSaveFileName(QWidget* parent = ?, const QString& caption = ?, const QString& dir = ?, const QString& filter = ?, QString* selectedFilter = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn get_save_file_name_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String, *mut ::qt_core::string::String, ::qt_core::flags::Flags<::file_dialog::Option>)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QFileDialog::getSaveFileName(QWidget* parent = ?, const QString& caption = ?, const QString& dir = ?, const QString& filter = ?, QString* selectedFilter = ?, QFlags<QFileDialog::Option> options = ?)```</span>
  ///
  ///
  pub unsafe fn get_save_file_name_unsafe<Args>(args: Args) -> ::qt_core::string::String
    where Args: overloading::FileDialogGetSaveFileNameUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getSaveFileUrl()```</span>
  ///
  ///
  pub fn get_save_file_url() -> ::qt_core::url::Url {
    {
      let mut object: ::qt_core::url::Url =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_getSaveFileUrl_to_output_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileDialog::getSaveFileUrl```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_save_file_url_unsafe(*mut ::widget::Widget) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getSaveFileUrl(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_save_file_url_unsafe((*mut ::widget::Widget, &::qt_core::string::String)) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getSaveFileUrl(QWidget* parent = ?, const QString& caption = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn get_save_file_url_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::url::Url)) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getSaveFileUrl(QWidget* parent = ?, const QString& caption = ?, const QUrl& dir = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn get_save_file_url_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::url::Url, &::qt_core::string::String)) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getSaveFileUrl(QWidget* parent = ?, const QString& caption = ?, const QUrl& dir = ?, const QString& filter = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn get_save_file_url_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::url::Url, &::qt_core::string::String, *mut ::qt_core::string::String)) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getSaveFileUrl(QWidget* parent = ?, const QString& caption = ?, const QUrl& dir = ?, const QString& filter = ?, QString* selectedFilter = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn get_save_file_url_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::url::Url, &::qt_core::string::String, *mut ::qt_core::string::String, ::qt_core::flags::Flags<::file_dialog::Option>)) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getSaveFileUrl(QWidget* parent = ?, const QString& caption = ?, const QUrl& dir = ?, const QString& filter = ?, QString* selectedFilter = ?, QFlags<QFileDialog::Option> options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn get_save_file_url_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::url::Url, &::qt_core::string::String, *mut ::qt_core::string::String, ::qt_core::flags::Flags<::file_dialog::Option>, &::qt_core::string_list::StringList)) -> ::qt_core::url::Url```<br>
  /// C++ method: <span style='color: green;'>```static QUrl QFileDialog::getSaveFileUrl(QWidget* parent = ?, const QString& caption = ?, const QUrl& dir = ?, const QString& filter = ?, QString* selectedFilter = ?, QFlags<QFileDialog::Option> options = ?, const QStringList& supportedSchemes = ?)```</span>
  ///
  ///
  pub unsafe fn get_save_file_url_unsafe<Args>(args: Args) -> ::qt_core::url::Url
    where Args: overloading::FileDialogGetSaveFileUrlUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStringList QFileDialog::history() const```</span>
  ///
  ///
  pub fn history(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_history_to_output(self as *const ::file_dialog::FileDialog, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileIconProvider* QFileDialog::iconProvider() const```</span>
  ///
  ///
  pub fn icon_provider(&self) -> *mut ::file_icon_provider::FileIconProvider {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_iconProvider(self as *const ::file_dialog::FileDialog) }
  }

  /// C++ method: <span style='color: green;'>```bool QFileDialog::isNameFilterDetailsVisible() const```</span>
  ///
  ///
  pub fn is_name_filter_details_visible(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_isNameFilterDetailsVisible(self as *const ::file_dialog::FileDialog) }
  }

  /// C++ method: <span style='color: green;'>```bool QFileDialog::isReadOnly() const```</span>
  ///
  ///
  pub fn is_read_only(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_isReadOnly(self as *const ::file_dialog::FileDialog) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemDelegate* QFileDialog::itemDelegate() const```</span>
  ///
  ///
  pub fn item_delegate(&self) -> *mut ::abstract_item_delegate::AbstractItemDelegate {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_itemDelegate(self as *const ::file_dialog::FileDialog) }
  }

  /// C++ method: <span style='color: green;'>```QString QFileDialog::labelText(QFileDialog::DialogLabel label) const```</span>
  ///
  ///
  pub fn label_text(&self, label: ::file_dialog::DialogLabel) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_labelText_to_output(self as *const ::file_dialog::FileDialog,
                                                            label,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QFileDialog::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_metaObject(self as *const ::file_dialog::FileDialog) }
  }

  /// C++ method: <span style='color: green;'>```QStringList QFileDialog::mimeTypeFilters() const```</span>
  ///
  ///
  pub fn mime_type_filters(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_mimeTypeFilters_to_output(self as *const ::file_dialog::FileDialog,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QFileDialog::nameFilters() const```</span>
  ///
  ///
  pub fn name_filters(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_nameFilters_to_output(self as *const ::file_dialog::FileDialog, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QFileDialog::QFileDialog()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::file_dialog::FileDialog> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFileDialog_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QFileDialog::QFileDialog```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::file_dialog::FileDialog>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFileDialog::QFileDialog(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::widget::Widget, &::qt_core::string::String)) -> ::cpp_utils::CppBox<::file_dialog::FileDialog>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFileDialog::QFileDialog(QWidget* parent = ?, const QString& caption = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String)) -> ::cpp_utils::CppBox<::file_dialog::FileDialog>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFileDialog::QFileDialog(QWidget* parent = ?, const QString& caption = ?, const QString& directory = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::widget::Widget, &::qt_core::string::String, &::qt_core::string::String, &::qt_core::string::String)) -> ::cpp_utils::CppBox<::file_dialog::FileDialog>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFileDialog::QFileDialog(QWidget* parent = ?, const QString& caption = ?, const QString& directory = ?, const QString& filter = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::file_dialog::FileDialog>
    where Args: overloading::FileDialogNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QFileDialog::open(QObject* receiver, const char* member)```</span>
  ///
  ///
  pub unsafe fn open(&mut self, receiver: *mut ::qt_core::object::Object, member: *const ::libc::c_char) {
    ::ffi::qt_widgets_c_QFileDialog_open(self as *mut ::file_dialog::FileDialog, receiver, member)
  }

  /// C++ method: <span style='color: green;'>```QFlags<QFileDialog::Option> QFileDialog::options() const```</span>
  ///
  ///
  pub fn options(&self) -> ::qt_core::flags::Flags<::file_dialog::Option> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFileDialog_options(self as *const ::file_dialog::FileDialog) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```QAbstractProxyModel* QFileDialog::proxyModel() const```</span>
  ///
  ///
  pub fn proxy_model(&self) -> *mut ::qt_core::abstract_proxy_model::AbstractProxyModel {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_proxyModel(self as *const ::file_dialog::FileDialog) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QFileDialog::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QFileDialog_qt_metacall(self as *mut ::file_dialog::FileDialog,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QFileDialog::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QFileDialog_qt_metacast(self as *mut ::file_dialog::FileDialog, arg1)
  }

  /// C++ method: <span style='color: green;'>```bool QFileDialog::resolveSymlinks() const```</span>
  ///
  ///
  pub fn resolve_symlinks(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_resolveSymlinks(self as *const ::file_dialog::FileDialog) }
  }

  /// C++ method: <span style='color: green;'>```bool QFileDialog::restoreState(const QByteArray& state)```</span>
  ///
  ///
  pub fn restore_state(&mut self, state: &::qt_core::byte_array::ByteArray) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QFileDialog_restoreState(self as *mut ::file_dialog::FileDialog,
                                                   state as *const ::qt_core::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QFileDialog::saveState() const```</span>
  ///
  ///
  pub fn save_state(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_saveState_to_output(self as *const ::file_dialog::FileDialog, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::selectFile(const QString& filename)```</span>
  ///
  ///
  pub fn select_file(&mut self, filename: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QFileDialog_selectFile(self as *mut ::file_dialog::FileDialog,
                                                 filename as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::selectMimeTypeFilter(const QString& filter)```</span>
  ///
  ///
  pub fn select_mime_type_filter(&mut self, filter: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QFileDialog_selectMimeTypeFilter(self as *mut ::file_dialog::FileDialog,
                                                           filter as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::selectNameFilter(const QString& filter)```</span>
  ///
  ///
  pub fn select_name_filter(&mut self, filter: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QFileDialog_selectNameFilter(self as *mut ::file_dialog::FileDialog,
                                                       filter as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::selectUrl(const QUrl& url)```</span>
  ///
  ///
  pub fn select_url(&mut self, url: &::qt_core::url::Url) {
    unsafe {
      ::ffi::qt_widgets_c_QFileDialog_selectUrl(self as *mut ::file_dialog::FileDialog,
                                                url as *const ::qt_core::url::Url)
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QFileDialog::selectedFiles() const```</span>
  ///
  ///
  pub fn selected_files(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_selectedFiles_to_output(self as *const ::file_dialog::FileDialog, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFileDialog::selectedMimeTypeFilter() const```</span>
  ///
  ///
  pub fn selected_mime_type_filter(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_selectedMimeTypeFilter_to_output(self as *const ::file_dialog::FileDialog,
                                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFileDialog::selectedNameFilter() const```</span>
  ///
  ///
  pub fn selected_name_filter(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_selectedNameFilter_to_output(self as *const ::file_dialog::FileDialog,
                                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QUrl> QFileDialog::selectedUrls() const```</span>
  ///
  ///
  pub fn selected_urls(&self) -> ::qt_core::list::ListUrl {
    {
      let mut object: ::qt_core::list::ListUrl =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_selectedUrls_to_output(self as *const ::file_dialog::FileDialog, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::setAcceptMode(QFileDialog::AcceptMode mode)```</span>
  ///
  ///
  pub fn set_accept_mode(&mut self, mode: ::file_dialog::AcceptMode) {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_setAcceptMode(self as *mut ::file_dialog::FileDialog, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::setConfirmOverwrite(bool enabled)```</span>
  ///
  ///
  pub fn set_confirm_overwrite(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_setConfirmOverwrite(self as *mut ::file_dialog::FileDialog, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::setDefaultSuffix(const QString& suffix)```</span>
  ///
  ///
  pub fn set_default_suffix(&mut self, suffix: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QFileDialog_setDefaultSuffix(self as *mut ::file_dialog::FileDialog,
                                                       suffix as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QFileDialog::setDirectory```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_directory(&mut self, &::qt_core::dir::Dir) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFileDialog::setDirectory(const QDir& directory)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_directory(&mut self, &::qt_core::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFileDialog::setDirectory(const QString& directory)```</span>
  ///
  ///
  pub fn set_directory<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::FileDialogSetDirectoryArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QFileDialog::setDirectoryUrl(const QUrl& directory)```</span>
  ///
  ///
  pub fn set_directory_url(&mut self, directory: &::qt_core::url::Url) {
    unsafe {
      ::ffi::qt_widgets_c_QFileDialog_setDirectoryUrl(self as *mut ::file_dialog::FileDialog,
                                                      directory as *const ::qt_core::url::Url)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::setFileMode(QFileDialog::FileMode mode)```</span>
  ///
  ///
  pub fn set_file_mode(&mut self, mode: ::file_dialog::FileMode) {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_setFileMode(self as *mut ::file_dialog::FileDialog, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::setHistory(const QStringList& paths)```</span>
  ///
  ///
  pub fn set_history(&mut self, paths: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_widgets_c_QFileDialog_setHistory(self as *mut ::file_dialog::FileDialog,
                                                 paths as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::setIconProvider(QFileIconProvider* provider)```</span>
  ///
  ///
  pub unsafe fn set_icon_provider(&mut self, provider: *mut ::file_icon_provider::FileIconProvider) {
    ::ffi::qt_widgets_c_QFileDialog_setIconProvider(self as *mut ::file_dialog::FileDialog, provider)
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::setItemDelegate(QAbstractItemDelegate* delegate)```</span>
  ///
  ///
  pub unsafe fn set_item_delegate(&mut self, delegate: *mut ::abstract_item_delegate::AbstractItemDelegate) {
    ::ffi::qt_widgets_c_QFileDialog_setItemDelegate(self as *mut ::file_dialog::FileDialog, delegate)
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::setLabelText(QFileDialog::DialogLabel label, const QString& text)```</span>
  ///
  ///
  pub fn set_label_text(&mut self, label: ::file_dialog::DialogLabel, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QFileDialog_setLabelText(self as *mut ::file_dialog::FileDialog,
                                                   label,
                                                   text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::setMimeTypeFilters(const QStringList& filters)```</span>
  ///
  ///
  pub fn set_mime_type_filters(&mut self, filters: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_widgets_c_QFileDialog_setMimeTypeFilters(self as *mut ::file_dialog::FileDialog,
                                                         filters as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::setNameFilter(const QString& filter)```</span>
  ///
  ///
  pub fn set_name_filter(&mut self, filter: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QFileDialog_setNameFilter(self as *mut ::file_dialog::FileDialog,
                                                    filter as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::setNameFilterDetailsVisible(bool enabled)```</span>
  ///
  ///
  pub fn set_name_filter_details_visible(&mut self, enabled: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QFileDialog_setNameFilterDetailsVisible(self as *mut ::file_dialog::FileDialog, enabled)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::setNameFilters(const QStringList& filters)```</span>
  ///
  ///
  pub fn set_name_filters(&mut self, filters: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_widgets_c_QFileDialog_setNameFilters(self as *mut ::file_dialog::FileDialog,
                                                     filters as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```QFileDialog::setOption```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_option(&mut self, ::file_dialog::Option) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFileDialog::setOption(QFileDialog::Option option)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_option(&mut self, (::file_dialog::Option, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFileDialog::setOption(QFileDialog::Option option, bool on = ?)```</span>
  ///
  ///
  pub fn set_option<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::FileDialogSetOptionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QFileDialog::setOptions(QFlags<QFileDialog::Option> options)```</span>
  ///
  ///
  pub fn set_options(&mut self, options: ::qt_core::flags::Flags<::file_dialog::Option>) {
    unsafe {
      ::ffi::qt_widgets_c_QFileDialog_setOptions(self as *mut ::file_dialog::FileDialog,
                                                 options.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::setProxyModel(QAbstractProxyModel* model)```</span>
  ///
  ///
  pub unsafe fn set_proxy_model(&mut self, model: *mut ::qt_core::abstract_proxy_model::AbstractProxyModel) {
    ::ffi::qt_widgets_c_QFileDialog_setProxyModel(self as *mut ::file_dialog::FileDialog, model)
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::setReadOnly(bool enabled)```</span>
  ///
  ///
  pub fn set_read_only(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_setReadOnly(self as *mut ::file_dialog::FileDialog, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::setResolveSymlinks(bool enabled)```</span>
  ///
  ///
  pub fn set_resolve_symlinks(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_setResolveSymlinks(self as *mut ::file_dialog::FileDialog, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::setSidebarUrls(const QList<QUrl>& urls)```</span>
  ///
  ///
  pub fn set_sidebar_urls(&mut self, urls: &::qt_core::list::ListUrl) {
    unsafe {
      ::ffi::qt_widgets_c_QFileDialog_setSidebarUrls(self as *mut ::file_dialog::FileDialog,
                                                     urls as *const ::qt_core::list::ListUrl)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::setSupportedSchemes(const QStringList& schemes)```</span>
  ///
  ///
  pub fn set_supported_schemes(&mut self, schemes: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_widgets_c_QFileDialog_setSupportedSchemes(self as *mut ::file_dialog::FileDialog,
                                                          schemes as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileDialog::setViewMode(QFileDialog::ViewMode mode)```</span>
  ///
  ///
  pub fn set_view_mode(&mut self, mode: ::file_dialog::ViewMode) {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_setViewMode(self as *mut ::file_dialog::FileDialog, mode) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QFileDialog::setVisible(bool visible)```</span>
  ///
  ///
  pub fn set_visible(&mut self, visible: bool) {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_setVisible(self as *mut ::file_dialog::FileDialog, visible) }
  }

  /// C++ method: <span style='color: green;'>```QList<QUrl> QFileDialog::sidebarUrls() const```</span>
  ///
  ///
  pub fn sidebar_urls(&self) -> ::qt_core::list::ListUrl {
    {
      let mut object: ::qt_core::list::ListUrl =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_sidebarUrls_to_output(self as *const ::file_dialog::FileDialog, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QFileDialog::supportedSchemes() const```</span>
  ///
  ///
  pub fn supported_schemes(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_supportedSchemes_to_output(self as *const ::file_dialog::FileDialog,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFileDialog::testOption(QFileDialog::Option option) const```</span>
  ///
  ///
  pub fn test_option(&self, option: ::file_dialog::Option) -> bool {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_testOption(self as *const ::file_dialog::FileDialog, option) }
  }

  /// C++ method: <span style='color: green;'>```static QString QFileDialog::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QFileDialog_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFileDialog::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QFileDialog_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileDialog::ViewMode QFileDialog::viewMode() const```</span>
  ///
  ///
  pub fn view_mode(&self) -> ::file_dialog::ViewMode {
    unsafe { ::ffi::qt_widgets_c_QFileDialog_viewMode(self as *const ::file_dialog::FileDialog) }
  }
}

impl ::cpp_utils::CppDeletable for ::file_dialog::FileDialog {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QFileDialog_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `FileDialog`.
  pub struct Signals<'a>(&'a ::file_dialog::FileDialog);
  /// Represents a built-in Qt signal `QFileDialog::currentUrlChanged`.
  ///
  /// An object of this type can be created from `FileDialog` with `object.signals().current_url_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDialog` object.
  pub struct CurrentUrlChanged<'a>(&'a ::file_dialog::FileDialog);
  impl<'a> ::qt_core::connection::Receiver for CurrentUrlChanged<'a> {
    type Arguments = (&'static ::qt_core::url::Url,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentUrlChanged(const QUrl&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentUrlChanged<'a> {}
  /// Represents a built-in Qt signal `QFileDialog::filterSelected`.
  ///
  /// An object of this type can be created from `FileDialog` with `object.signals().filter_selected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDialog` object.
  pub struct FilterSelected<'a>(&'a ::file_dialog::FileDialog);
  impl<'a> ::qt_core::connection::Receiver for FilterSelected<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2filterSelected(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FilterSelected<'a> {}
  /// Represents a built-in Qt signal `QFileDialog::urlSelected`.
  ///
  /// An object of this type can be created from `FileDialog` with `object.signals().url_selected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDialog` object.
  pub struct UrlSelected<'a>(&'a ::file_dialog::FileDialog);
  impl<'a> ::qt_core::connection::Receiver for UrlSelected<'a> {
    type Arguments = (&'static ::qt_core::url::Url,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2urlSelected(const QUrl&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for UrlSelected<'a> {}
  /// Represents a built-in Qt signal `QFileDialog::finished`.
  ///
  /// An object of this type can be created from `FileDialog` with `object.signals().finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDialog` object.
  pub struct Finished<'a>(&'a ::file_dialog::FileDialog);
  impl<'a> ::qt_core::connection::Receiver for Finished<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2finished(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Finished<'a> {}
  /// Represents a built-in Qt signal `QFileDialog::rejected`.
  ///
  /// An object of this type can be created from `FileDialog` with `object.signals().rejected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDialog` object.
  pub struct Rejected<'a>(&'a ::file_dialog::FileDialog);
  impl<'a> ::qt_core::connection::Receiver for Rejected<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rejected()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Rejected<'a> {}
  /// Represents a built-in Qt signal `QFileDialog::currentChanged`.
  ///
  /// An object of this type can be created from `FileDialog` with `object.signals().current_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDialog` object.
  pub struct CurrentChanged<'a>(&'a ::file_dialog::FileDialog);
  impl<'a> ::qt_core::connection::Receiver for CurrentChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentChanged<'a> {}
  /// Represents a built-in Qt signal `QFileDialog::directoryEntered`.
  ///
  /// An object of this type can be created from `FileDialog` with `object.signals().directory_entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDialog` object.
  pub struct DirectoryEntered<'a>(&'a ::file_dialog::FileDialog);
  impl<'a> ::qt_core::connection::Receiver for DirectoryEntered<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2directoryEntered(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DirectoryEntered<'a> {}
  /// Represents a built-in Qt signal `QFileDialog::fileSelected`.
  ///
  /// An object of this type can be created from `FileDialog` with `object.signals().file_selected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDialog` object.
  pub struct FileSelected<'a>(&'a ::file_dialog::FileDialog);
  impl<'a> ::qt_core::connection::Receiver for FileSelected<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2fileSelected(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FileSelected<'a> {}
  /// Represents a built-in Qt signal `QFileDialog::directoryUrlEntered`.
  ///
  /// An object of this type can be created from `FileDialog` with `object.signals().directory_url_entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDialog` object.
  pub struct DirectoryUrlEntered<'a>(&'a ::file_dialog::FileDialog);
  impl<'a> ::qt_core::connection::Receiver for DirectoryUrlEntered<'a> {
    type Arguments = (&'static ::qt_core::url::Url,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2directoryUrlEntered(const QUrl&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DirectoryUrlEntered<'a> {}
  /// Represents a built-in Qt signal `QFileDialog::accepted`.
  ///
  /// An object of this type can be created from `FileDialog` with `object.signals().accepted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDialog` object.
  pub struct Accepted<'a>(&'a ::file_dialog::FileDialog);
  impl<'a> ::qt_core::connection::Receiver for Accepted<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2accepted()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Accepted<'a> {}
  /// Represents a built-in Qt signal `QFileDialog::filesSelected`.
  ///
  /// An object of this type can be created from `FileDialog` with `object.signals().files_selected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDialog` object.
  pub struct FilesSelected<'a>(&'a ::file_dialog::FileDialog);
  impl<'a> ::qt_core::connection::Receiver for FilesSelected<'a> {
    type Arguments = (&'static ::qt_core::string_list::StringList,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2filesSelected(const QStringList&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FilesSelected<'a> {}
  /// Represents a built-in Qt signal `QFileDialog::urlsSelected`.
  ///
  /// An object of this type can be created from `FileDialog` with `object.signals().urls_selected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDialog` object.
  pub struct UrlsSelected<'a>(&'a ::file_dialog::FileDialog);
  impl<'a> ::qt_core::connection::Receiver for UrlsSelected<'a> {
    type Arguments = (&'static ::qt_core::list::ListUrl,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2urlsSelected(const QList< QUrl >&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for UrlsSelected<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QFileDialog::currentUrlChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_url_changed(&self) -> CurrentUrlChanged {
      CurrentUrlChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileDialog::filterSelected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn filter_selected(&self) -> FilterSelected {
      FilterSelected(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileDialog::urlSelected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn url_selected(&self) -> UrlSelected {
      UrlSelected(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileDialog::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished(&self) -> Finished {
      Finished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileDialog::rejected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rejected(&self) -> Rejected {
      Rejected(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileDialog::currentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_changed(&self) -> CurrentChanged {
      CurrentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileDialog::directoryEntered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn directory_entered(&self) -> DirectoryEntered {
      DirectoryEntered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileDialog::fileSelected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn file_selected(&self) -> FileSelected {
      FileSelected(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileDialog::directoryUrlEntered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn directory_url_entered(&self) -> DirectoryUrlEntered {
      DirectoryUrlEntered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileDialog::accepted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accepted(&self) -> Accepted {
      Accepted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileDialog::filesSelected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn files_selected(&self) -> FilesSelected {
      FilesSelected(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileDialog::urlsSelected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn urls_selected(&self) -> UrlsSelected {
      UrlsSelected(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `FileDialog`.
  pub struct Slots<'a>(&'a ::file_dialog::FileDialog);
  /// Represents a built-in Qt slot `QFileDialog::showExtension`.
  ///
  /// An object of this type can be created from `FileDialog` with `object.slots().show_extension()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDialog` object.
  pub struct ShowExtension<'a>(&'a ::file_dialog::FileDialog);
  impl<'a> ::qt_core::connection::Receiver for ShowExtension<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showExtension(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QFileDialog::open`.
  ///
  /// An object of this type can be created from `FileDialog` with `object.slots().open()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDialog` object.
  pub struct Open<'a>(&'a ::file_dialog::FileDialog);
  impl<'a> ::qt_core::connection::Receiver for Open<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1open()\0"
    }
  }
  /// Represents a built-in Qt slot `QFileDialog::exec`.
  ///
  /// An object of this type can be created from `FileDialog` with `object.slots().exec()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDialog` object.
  pub struct Exec<'a>(&'a ::file_dialog::FileDialog);
  impl<'a> ::qt_core::connection::Receiver for Exec<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1exec()\0"
    }
  }
  /// Represents a built-in Qt slot `QFileDialog::reject`.
  ///
  /// An object of this type can be created from `FileDialog` with `object.slots().reject()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDialog` object.
  pub struct Reject<'a>(&'a ::file_dialog::FileDialog);
  impl<'a> ::qt_core::connection::Receiver for Reject<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reject()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QFileDialog::showExtension`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_extension(&self) -> ShowExtension {
      ShowExtension(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFileDialog::open`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn open(&self) -> Open {
      Open(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFileDialog::exec`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn exec(&self) -> Exec {
      Exec(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFileDialog::reject`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reject(&self) -> Reject {
      Reject(self.0)
    }
  }
  impl ::file_dialog::FileDialog {
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

/// C++ type: <span style='color: green;'>```QFileDialog::FileMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FileMode {
  /// C++ enum variant: <span style='color: green;'>```AnyFile = 0```</span>
  AnyFile = 0,
  /// C++ enum variant: <span style='color: green;'>```ExistingFile = 1```</span>
  ExistingFile = 1,
  /// C++ enum variant: <span style='color: green;'>```Directory = 2```</span>
  Directory = 2,
  /// C++ enum variant: <span style='color: green;'>```ExistingFiles = 3```</span>
  ExistingFiles = 3,
  /// C++ enum variant: <span style='color: green;'>```DirectoryOnly = 4```</span>
  DirectoryOnly = 4,
}

/// C++ type: <span style='color: green;'>```QFileDialog::Option```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Option {
  /// C++ enum variant: <span style='color: green;'>```ShowDirsOnly = 1```</span>
  ShowDirsOnly = 1,
  /// C++ enum variant: <span style='color: green;'>```DontResolveSymlinks = 2```</span>
  DontResolveSymlinks = 2,
  /// C++ enum variant: <span style='color: green;'>```DontConfirmOverwrite = 4```</span>
  DontConfirmOverwrite = 4,
  /// C++ enum variant: <span style='color: green;'>```DontUseSheet = 8```</span>
  DontUseSheet = 8,
  /// C++ enum variant: <span style='color: green;'>```DontUseNativeDialog = 16```</span>
  DontUseNativeDialog = 16,
  /// C++ enum variant: <span style='color: green;'>```ReadOnly = 32```</span>
  ReadOnly = 32,
  /// C++ enum variant: <span style='color: green;'>```HideNameFilterDetails = 64```</span>
  HideNameFilterDetails = 64,
  /// C++ enum variant: <span style='color: green;'>```DontUseCustomDirectoryIcons = 128```</span>
  DontUseCustomDirectoryIcons = 128,
}

impl ::qt_core::flags::FlaggableEnum for Option {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "Option"
  }
}

/// C++ type: <span style='color: green;'>```QFileDialog::ViewMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ViewMode {
  /// C++ enum variant: <span style='color: green;'>```Detail = 0```</span>
  Detail = 0,
  /// C++ enum variant: <span style='color: green;'>```List = 1```</span>
  List = 1,
}

impl ::cpp_utils::DynamicCast<::file_dialog::FileDialog> for ::dialog::Dialog {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::file_dialog::FileDialog> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFileDialog_G_dynamic_cast_QFileDialog_ptr_QDialog(self as *mut ::dialog::Dialog) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::file_dialog::FileDialog> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFileDialog_G_dynamic_cast_QFileDialog_ptr_QDialog(self as *const ::dialog::Dialog as *mut ::dialog::Dialog) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::file_dialog::FileDialog> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::file_dialog::FileDialog> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFileDialog_G_dynamic_cast_QFileDialog_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::file_dialog::FileDialog> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFileDialog_G_dynamic_cast_QFileDialog_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::file_dialog::FileDialog {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFileDialog_G_static_cast_QObject_ptr(self as *mut ::file_dialog::FileDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFileDialog_G_static_cast_QObject_ptr(self as *const ::file_dialog::FileDialog as *mut ::file_dialog::FileDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::file_dialog::FileDialog {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFileDialog_G_static_cast_QPaintDevice_ptr(self as *mut ::file_dialog::FileDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFileDialog_G_static_cast_QPaintDevice_ptr(self as *const ::file_dialog::FileDialog as *mut ::file_dialog::FileDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::dialog::Dialog> for ::file_dialog::FileDialog {
  fn static_cast_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFileDialog_G_static_cast_QDialog_ptr(self as *mut ::file_dialog::FileDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::dialog::Dialog {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFileDialog_G_static_cast_QDialog_ptr(self as *const ::file_dialog::FileDialog as *mut ::file_dialog::FileDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::file_dialog::FileDialog {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFileDialog_G_static_cast_QWidget_ptr(self as *mut ::file_dialog::FileDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFileDialog_G_static_cast_QWidget_ptr(self as *const ::file_dialog::FileDialog as *mut ::file_dialog::FileDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::file_dialog::FileDialog> for ::dialog::Dialog {
  unsafe fn static_cast_mut(&mut self) -> &mut ::file_dialog::FileDialog {
    let ffi_result =
      ::ffi::qt_widgets_c_QFileDialog_G_static_cast_QFileDialog_ptr_QDialog(self as *mut ::dialog::Dialog);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::file_dialog::FileDialog {
    let ffi_result = ::ffi::qt_widgets_c_QFileDialog_G_static_cast_QFileDialog_ptr_QDialog(self as *const ::dialog::Dialog as *mut ::dialog::Dialog);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::file_dialog::FileDialog> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::file_dialog::FileDialog {
    let ffi_result =
      ::ffi::qt_widgets_c_QFileDialog_G_static_cast_QFileDialog_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::file_dialog::FileDialog {
    let ffi_result = ::ffi::qt_widgets_c_QFileDialog_G_static_cast_QFileDialog_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::file_dialog::FileDialog> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::file_dialog::FileDialog {
    let ffi_result = ::ffi::qt_widgets_c_QFileDialog_G_static_cast_QFileDialog_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::file_dialog::FileDialog {
    let ffi_result = ::ffi::qt_widgets_c_QFileDialog_G_static_cast_QFileDialog_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::file_dialog::FileDialog> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::file_dialog::FileDialog {
    let ffi_result =
      ::ffi::qt_widgets_c_QFileDialog_G_static_cast_QFileDialog_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::file_dialog::FileDialog {
    let ffi_result = ::ffi::qt_widgets_c_QFileDialog_G_static_cast_QFileDialog_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::file_dialog::FileDialog {
  type Target = ::dialog::Dialog;
  fn deref(&self) -> &::dialog::Dialog {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFileDialog_G_static_cast_QDialog_ptr(self as *const ::file_dialog::FileDialog as *mut ::file_dialog::FileDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::file_dialog::FileDialog {
  fn deref_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFileDialog_G_static_cast_QDialog_ptr(self as *mut ::file_dialog::FileDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [FileDialog::get_existing_directory_unsafe](../struct.FileDialog.html#method.get_existing_directory_unsafe) method.
  pub trait FileDialogGetExistingDirectoryUnsafeArgs {
    unsafe fn exec(self) -> ::qt_core::string::String;
  }
  impl FileDialogGetExistingDirectoryUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::qt_core::string::String {
      let parent = self;
      {
        let mut object: ::qt_core::string::String =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getExistingDirectory_to_output_parent(parent, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetExistingDirectoryUnsafeArgs for (*mut ::widget::Widget, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::string::String {
      let parent = self.0;
      let caption = self.1;
      {
        let mut object: ::qt_core::string::String =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getExistingDirectory_to_output_parent_caption(parent, caption as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetExistingDirectoryUnsafeArgs
    for (*mut ::widget::Widget, &'a ::qt_core::string::String, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::string::String {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      {
        let mut object: ::qt_core::string::String =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getExistingDirectory_to_output_parent_caption_dir(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetExistingDirectoryUnsafeArgs
    for (*mut ::widget::Widget,
                                                             &'a ::qt_core::string::String,
                                                             &'a ::qt_core::string::String,
                                                             ::qt_core::flags::Flags<::file_dialog::Option>) {
    unsafe fn exec(self) -> ::qt_core::string::String {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let options = self.3;
      {
        let mut object: ::qt_core::string::String =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getExistingDirectory_to_output_parent_caption_dir_options(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::string::String, options.to_int() as ::libc::c_uint, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FileDialog::get_existing_directory_url_unsafe](../struct.FileDialog.html#method.get_existing_directory_url_unsafe) method.
  pub trait FileDialogGetExistingDirectoryUrlUnsafeArgs {
    unsafe fn exec(self) -> ::qt_core::url::Url;
  }
  impl FileDialogGetExistingDirectoryUrlUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getExistingDirectoryUrl_to_output_parent(parent, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetExistingDirectoryUrlUnsafeArgs for (*mut ::widget::Widget, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self.0;
      let caption = self.1;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getExistingDirectoryUrl_to_output_parent_caption(parent, caption as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetExistingDirectoryUrlUnsafeArgs
    for (*mut ::widget::Widget, &'a ::qt_core::string::String, &'a ::qt_core::url::Url) {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getExistingDirectoryUrl_to_output_parent_caption_dir(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::url::Url, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetExistingDirectoryUrlUnsafeArgs
    for (*mut ::widget::Widget,
                                                                &'a ::qt_core::string::String,
                                                                &'a ::qt_core::url::Url,
                                                                ::qt_core::flags::Flags<::file_dialog::Option>) {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let options = self.3;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getExistingDirectoryUrl_to_output_parent_caption_dir_options(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::url::Url, options.to_int() as ::libc::c_uint, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetExistingDirectoryUrlUnsafeArgs
    for (*mut ::widget::Widget,
                                                                &'a ::qt_core::string::String,
                                                                &'a ::qt_core::url::Url,
                                                                ::qt_core::flags::Flags<::file_dialog::Option>,
                                                                &'a ::qt_core::string_list::StringList) {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let options = self.3;
      let supported_schemes = self.4;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getExistingDirectoryUrl_to_output_parent_caption_dir_options_supportedSchemes(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::url::Url, options.to_int() as ::libc::c_uint, supported_schemes as *const ::qt_core::string_list::StringList, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FileDialog::get_open_file_name_unsafe](../struct.FileDialog.html#method.get_open_file_name_unsafe) method.
  pub trait FileDialogGetOpenFileNameUnsafeArgs {
    unsafe fn exec(self) -> ::qt_core::string::String;
  }
  impl FileDialogGetOpenFileNameUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::qt_core::string::String {
      let parent = self;
      {
        let mut object: ::qt_core::string::String =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileName_to_output_parent(parent, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileNameUnsafeArgs for (*mut ::widget::Widget, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::string::String {
      let parent = self.0;
      let caption = self.1;
      {
        let mut object: ::qt_core::string::String =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileName_to_output_parent_caption(parent, caption as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileNameUnsafeArgs
    for (*mut ::widget::Widget, &'a ::qt_core::string::String, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::string::String {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      {
        let mut object: ::qt_core::string::String =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileName_to_output_parent_caption_dir(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileNameUnsafeArgs
    for (*mut ::widget::Widget,
                                                        &'a ::qt_core::string::String,
                                                        &'a ::qt_core::string::String,
                                                        &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::string::String {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      {
        let mut object: ::qt_core::string::String =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileName_to_output_parent_caption_dir_filter(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::string::String, filter as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileNameUnsafeArgs
    for (*mut ::widget::Widget,
                                                        &'a ::qt_core::string::String,
                                                        &'a ::qt_core::string::String,
                                                        &'a ::qt_core::string::String,
                                                        *mut ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::string::String {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      let selected_filter = self.4;
      {
        let mut object: ::qt_core::string::String =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileName_to_output_parent_caption_dir_filter_selectedFilter(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::string::String, filter as *const ::qt_core::string::String, selected_filter, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileNameUnsafeArgs
    for (*mut ::widget::Widget,
                                                        &'a ::qt_core::string::String,
                                                        &'a ::qt_core::string::String,
                                                        &'a ::qt_core::string::String,
                                                        *mut ::qt_core::string::String,
                                                        ::qt_core::flags::Flags<::file_dialog::Option>) {
    unsafe fn exec(self) -> ::qt_core::string::String {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      let selected_filter = self.4;
      let options = self.5;
      {
        let mut object: ::qt_core::string::String =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileName_to_output_parent_caption_dir_filter_selectedFilter_options(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::string::String, filter as *const ::qt_core::string::String, selected_filter, options.to_int() as ::libc::c_uint, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FileDialog::get_open_file_names_unsafe](../struct.FileDialog.html#method.get_open_file_names_unsafe) method.
  pub trait FileDialogGetOpenFileNamesUnsafeArgs {
    unsafe fn exec(self) -> ::qt_core::string_list::StringList;
  }
  impl FileDialogGetOpenFileNamesUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::qt_core::string_list::StringList {
      let parent = self;
      {
        let mut object: ::qt_core::string_list::StringList =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileNames_to_output_parent(parent, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileNamesUnsafeArgs for (*mut ::widget::Widget, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::string_list::StringList {
      let parent = self.0;
      let caption = self.1;
      {
        let mut object: ::qt_core::string_list::StringList =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileNames_to_output_parent_caption(parent, caption as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileNamesUnsafeArgs
    for (*mut ::widget::Widget, &'a ::qt_core::string::String, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::string_list::StringList {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      {
        let mut object: ::qt_core::string_list::StringList =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileNames_to_output_parent_caption_dir(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileNamesUnsafeArgs
    for (*mut ::widget::Widget,
                                                         &'a ::qt_core::string::String,
                                                         &'a ::qt_core::string::String,
                                                         &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::string_list::StringList {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      {
        let mut object: ::qt_core::string_list::StringList =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileNames_to_output_parent_caption_dir_filter(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::string::String, filter as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileNamesUnsafeArgs
    for (*mut ::widget::Widget,
                                                         &'a ::qt_core::string::String,
                                                         &'a ::qt_core::string::String,
                                                         &'a ::qt_core::string::String,
                                                         *mut ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::string_list::StringList {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      let selected_filter = self.4;
      {
        let mut object: ::qt_core::string_list::StringList =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileNames_to_output_parent_caption_dir_filter_selectedFilter(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::string::String, filter as *const ::qt_core::string::String, selected_filter, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileNamesUnsafeArgs
    for (*mut ::widget::Widget,
                                                         &'a ::qt_core::string::String,
                                                         &'a ::qt_core::string::String,
                                                         &'a ::qt_core::string::String,
                                                         *mut ::qt_core::string::String,
                                                         ::qt_core::flags::Flags<::file_dialog::Option>) {
    unsafe fn exec(self) -> ::qt_core::string_list::StringList {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      let selected_filter = self.4;
      let options = self.5;
      {
        let mut object: ::qt_core::string_list::StringList =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileNames_to_output_parent_caption_dir_filter_selectedFilter_options(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::string::String, filter as *const ::qt_core::string::String, selected_filter, options.to_int() as ::libc::c_uint, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FileDialog::get_open_file_url_unsafe](../struct.FileDialog.html#method.get_open_file_url_unsafe) method.
  pub trait FileDialogGetOpenFileUrlUnsafeArgs {
    unsafe fn exec(self) -> ::qt_core::url::Url;
  }
  impl FileDialogGetOpenFileUrlUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileUrl_to_output_parent(parent, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileUrlUnsafeArgs for (*mut ::widget::Widget, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self.0;
      let caption = self.1;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileUrl_to_output_parent_caption(parent, caption as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileUrlUnsafeArgs
    for (*mut ::widget::Widget, &'a ::qt_core::string::String, &'a ::qt_core::url::Url) {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileUrl_to_output_parent_caption_dir(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::url::Url, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileUrlUnsafeArgs
    for (*mut ::widget::Widget, &'a ::qt_core::string::String, &'a ::qt_core::url::Url, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileUrl_to_output_parent_caption_dir_filter(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::url::Url, filter as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileUrlUnsafeArgs
    for (*mut ::widget::Widget,
                                                       &'a ::qt_core::string::String,
                                                       &'a ::qt_core::url::Url,
                                                       &'a ::qt_core::string::String,
                                                       *mut ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      let selected_filter = self.4;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileUrl_to_output_parent_caption_dir_filter_selectedFilter(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::url::Url, filter as *const ::qt_core::string::String, selected_filter, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileUrlUnsafeArgs
    for (*mut ::widget::Widget,
                                                       &'a ::qt_core::string::String,
                                                       &'a ::qt_core::url::Url,
                                                       &'a ::qt_core::string::String,
                                                       *mut ::qt_core::string::String,
                                                       ::qt_core::flags::Flags<::file_dialog::Option>) {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      let selected_filter = self.4;
      let options = self.5;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileUrl_to_output_parent_caption_dir_filter_selectedFilter_options(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::url::Url, filter as *const ::qt_core::string::String, selected_filter, options.to_int() as ::libc::c_uint, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileUrlUnsafeArgs
    for (*mut ::widget::Widget,
                                                       &'a ::qt_core::string::String,
                                                       &'a ::qt_core::url::Url,
                                                       &'a ::qt_core::string::String,
                                                       *mut ::qt_core::string::String,
                                                       ::qt_core::flags::Flags<::file_dialog::Option>,
                                                       &'a ::qt_core::string_list::StringList) {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      let selected_filter = self.4;
      let options = self.5;
      let supported_schemes = self.6;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileUrl_to_output_parent_caption_dir_filter_selectedFilter_options_supportedSchemes(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::url::Url, filter as *const ::qt_core::string::String, selected_filter, options.to_int() as ::libc::c_uint, supported_schemes as *const ::qt_core::string_list::StringList, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FileDialog::get_open_file_urls_unsafe](../struct.FileDialog.html#method.get_open_file_urls_unsafe) method.
  pub trait FileDialogGetOpenFileUrlsUnsafeArgs {
    unsafe fn exec(self) -> ::qt_core::list::ListUrl;
  }
  impl FileDialogGetOpenFileUrlsUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::qt_core::list::ListUrl {
      let parent = self;
      {
        let mut object: ::qt_core::list::ListUrl =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileUrls_to_output_parent(parent, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileUrlsUnsafeArgs for (*mut ::widget::Widget, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::list::ListUrl {
      let parent = self.0;
      let caption = self.1;
      {
        let mut object: ::qt_core::list::ListUrl =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileUrls_to_output_parent_caption(parent, caption as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileUrlsUnsafeArgs
    for (*mut ::widget::Widget, &'a ::qt_core::string::String, &'a ::qt_core::url::Url) {
    unsafe fn exec(self) -> ::qt_core::list::ListUrl {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      {
        let mut object: ::qt_core::list::ListUrl =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileUrls_to_output_parent_caption_dir(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::url::Url, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileUrlsUnsafeArgs
    for (*mut ::widget::Widget, &'a ::qt_core::string::String, &'a ::qt_core::url::Url, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::list::ListUrl {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      {
        let mut object: ::qt_core::list::ListUrl =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileUrls_to_output_parent_caption_dir_filter(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::url::Url, filter as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileUrlsUnsafeArgs
    for (*mut ::widget::Widget,
                                                        &'a ::qt_core::string::String,
                                                        &'a ::qt_core::url::Url,
                                                        &'a ::qt_core::string::String,
                                                        *mut ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::list::ListUrl {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      let selected_filter = self.4;
      {
        let mut object: ::qt_core::list::ListUrl =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileUrls_to_output_parent_caption_dir_filter_selectedFilter(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::url::Url, filter as *const ::qt_core::string::String, selected_filter, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileUrlsUnsafeArgs
    for (*mut ::widget::Widget,
                                                        &'a ::qt_core::string::String,
                                                        &'a ::qt_core::url::Url,
                                                        &'a ::qt_core::string::String,
                                                        *mut ::qt_core::string::String,
                                                        ::qt_core::flags::Flags<::file_dialog::Option>) {
    unsafe fn exec(self) -> ::qt_core::list::ListUrl {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      let selected_filter = self.4;
      let options = self.5;
      {
        let mut object: ::qt_core::list::ListUrl =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileUrls_to_output_parent_caption_dir_filter_selectedFilter_options(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::url::Url, filter as *const ::qt_core::string::String, selected_filter, options.to_int() as ::libc::c_uint, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetOpenFileUrlsUnsafeArgs
    for (*mut ::widget::Widget,
                                                        &'a ::qt_core::string::String,
                                                        &'a ::qt_core::url::Url,
                                                        &'a ::qt_core::string::String,
                                                        *mut ::qt_core::string::String,
                                                        ::qt_core::flags::Flags<::file_dialog::Option>,
                                                        &'a ::qt_core::string_list::StringList) {
    unsafe fn exec(self) -> ::qt_core::list::ListUrl {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      let selected_filter = self.4;
      let options = self.5;
      let supported_schemes = self.6;
      {
        let mut object: ::qt_core::list::ListUrl =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getOpenFileUrls_to_output_parent_caption_dir_filter_selectedFilter_options_supportedSchemes(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::url::Url, filter as *const ::qt_core::string::String, selected_filter, options.to_int() as ::libc::c_uint, supported_schemes as *const ::qt_core::string_list::StringList, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FileDialog::get_save_file_name_unsafe](../struct.FileDialog.html#method.get_save_file_name_unsafe) method.
  pub trait FileDialogGetSaveFileNameUnsafeArgs {
    unsafe fn exec(self) -> ::qt_core::string::String;
  }
  impl FileDialogGetSaveFileNameUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::qt_core::string::String {
      let parent = self;
      {
        let mut object: ::qt_core::string::String =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getSaveFileName_to_output_parent(parent, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetSaveFileNameUnsafeArgs for (*mut ::widget::Widget, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::string::String {
      let parent = self.0;
      let caption = self.1;
      {
        let mut object: ::qt_core::string::String =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getSaveFileName_to_output_parent_caption(parent, caption as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetSaveFileNameUnsafeArgs
    for (*mut ::widget::Widget, &'a ::qt_core::string::String, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::string::String {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      {
        let mut object: ::qt_core::string::String =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getSaveFileName_to_output_parent_caption_dir(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetSaveFileNameUnsafeArgs
    for (*mut ::widget::Widget,
                                                        &'a ::qt_core::string::String,
                                                        &'a ::qt_core::string::String,
                                                        &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::string::String {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      {
        let mut object: ::qt_core::string::String =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getSaveFileName_to_output_parent_caption_dir_filter(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::string::String, filter as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetSaveFileNameUnsafeArgs
    for (*mut ::widget::Widget,
                                                        &'a ::qt_core::string::String,
                                                        &'a ::qt_core::string::String,
                                                        &'a ::qt_core::string::String,
                                                        *mut ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::string::String {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      let selected_filter = self.4;
      {
        let mut object: ::qt_core::string::String =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getSaveFileName_to_output_parent_caption_dir_filter_selectedFilter(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::string::String, filter as *const ::qt_core::string::String, selected_filter, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetSaveFileNameUnsafeArgs
    for (*mut ::widget::Widget,
                                                        &'a ::qt_core::string::String,
                                                        &'a ::qt_core::string::String,
                                                        &'a ::qt_core::string::String,
                                                        *mut ::qt_core::string::String,
                                                        ::qt_core::flags::Flags<::file_dialog::Option>) {
    unsafe fn exec(self) -> ::qt_core::string::String {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      let selected_filter = self.4;
      let options = self.5;
      {
        let mut object: ::qt_core::string::String =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getSaveFileName_to_output_parent_caption_dir_filter_selectedFilter_options(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::string::String, filter as *const ::qt_core::string::String, selected_filter, options.to_int() as ::libc::c_uint, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FileDialog::get_save_file_url_unsafe](../struct.FileDialog.html#method.get_save_file_url_unsafe) method.
  pub trait FileDialogGetSaveFileUrlUnsafeArgs {
    unsafe fn exec(self) -> ::qt_core::url::Url;
  }
  impl FileDialogGetSaveFileUrlUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getSaveFileUrl_to_output_parent(parent, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetSaveFileUrlUnsafeArgs for (*mut ::widget::Widget, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self.0;
      let caption = self.1;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getSaveFileUrl_to_output_parent_caption(parent, caption as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetSaveFileUrlUnsafeArgs
    for (*mut ::widget::Widget, &'a ::qt_core::string::String, &'a ::qt_core::url::Url) {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getSaveFileUrl_to_output_parent_caption_dir(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::url::Url, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetSaveFileUrlUnsafeArgs
    for (*mut ::widget::Widget, &'a ::qt_core::string::String, &'a ::qt_core::url::Url, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getSaveFileUrl_to_output_parent_caption_dir_filter(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::url::Url, filter as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetSaveFileUrlUnsafeArgs
    for (*mut ::widget::Widget,
                                                       &'a ::qt_core::string::String,
                                                       &'a ::qt_core::url::Url,
                                                       &'a ::qt_core::string::String,
                                                       *mut ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      let selected_filter = self.4;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getSaveFileUrl_to_output_parent_caption_dir_filter_selectedFilter(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::url::Url, filter as *const ::qt_core::string::String, selected_filter, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetSaveFileUrlUnsafeArgs
    for (*mut ::widget::Widget,
                                                       &'a ::qt_core::string::String,
                                                       &'a ::qt_core::url::Url,
                                                       &'a ::qt_core::string::String,
                                                       *mut ::qt_core::string::String,
                                                       ::qt_core::flags::Flags<::file_dialog::Option>) {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      let selected_filter = self.4;
      let options = self.5;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getSaveFileUrl_to_output_parent_caption_dir_filter_selectedFilter_options(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::url::Url, filter as *const ::qt_core::string::String, selected_filter, options.to_int() as ::libc::c_uint, &mut object);
        object
      }
    }
  }
  impl<'a> FileDialogGetSaveFileUrlUnsafeArgs
    for (*mut ::widget::Widget,
                                                       &'a ::qt_core::string::String,
                                                       &'a ::qt_core::url::Url,
                                                       &'a ::qt_core::string::String,
                                                       *mut ::qt_core::string::String,
                                                       ::qt_core::flags::Flags<::file_dialog::Option>,
                                                       &'a ::qt_core::string_list::StringList) {
    unsafe fn exec(self) -> ::qt_core::url::Url {
      let parent = self.0;
      let caption = self.1;
      let dir = self.2;
      let filter = self.3;
      let selected_filter = self.4;
      let options = self.5;
      let supported_schemes = self.6;
      {
        let mut object: ::qt_core::url::Url = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFileDialog_getSaveFileUrl_to_output_parent_caption_dir_filter_selectedFilter_options_supportedSchemes(parent, caption as *const ::qt_core::string::String, dir as *const ::qt_core::url::Url, filter as *const ::qt_core::string::String, selected_filter, options.to_int() as ::libc::c_uint, supported_schemes as *const ::qt_core::string_list::StringList, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FileDialog::new_unsafe](../struct.FileDialog.html#method.new_unsafe) method.
  pub trait FileDialogNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::file_dialog::FileDialog>;
  }
  impl FileDialogNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::file_dialog::FileDialog> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QFileDialog_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> FileDialogNewUnsafeArgs for (*mut ::widget::Widget, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::file_dialog::FileDialog> {
      let parent = self.0;
      let caption = self.1;
      let ffi_result =
        ::ffi::qt_widgets_c_QFileDialog_new_parent_caption(parent, caption as *const ::qt_core::string::String);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> FileDialogNewUnsafeArgs
    for (*mut ::widget::Widget, &'a ::qt_core::string::String, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::file_dialog::FileDialog> {
      let parent = self.0;
      let caption = self.1;
      let directory = self.2;
      let ffi_result =
        ::ffi::qt_widgets_c_QFileDialog_new_parent_caption_directory(parent,
                                                                     caption as *const ::qt_core::string::String,
                                                                     directory as *const ::qt_core::string::String);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> FileDialogNewUnsafeArgs
    for (*mut ::widget::Widget,
                                            &'a ::qt_core::string::String,
                                            &'a ::qt_core::string::String,
                                            &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::file_dialog::FileDialog> {
      let parent = self.0;
      let caption = self.1;
      let directory = self.2;
      let filter = self.3;
      let ffi_result = ::ffi::qt_widgets_c_QFileDialog_new_parent_caption_directory_filter(parent, caption as *const ::qt_core::string::String, directory as *const ::qt_core::string::String, filter as *const ::qt_core::string::String);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [FileDialog::set_directory](../struct.FileDialog.html#method.set_directory) method.
  pub trait FileDialogSetDirectoryArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::file_dialog::FileDialog) -> ();
  }
  impl<'largs> FileDialogSetDirectoryArgs<'largs> for &'largs ::qt_core::dir::Dir {
    fn exec(self, original_self: &'largs mut ::file_dialog::FileDialog) -> () {
      let directory = self;
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_setDirectory_QDir(original_self as *mut ::file_dialog::FileDialog,
                                                          directory as *const ::qt_core::dir::Dir)
      }
    }
  }
  impl<'largs> FileDialogSetDirectoryArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::file_dialog::FileDialog) -> () {
      let directory = self;
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_setDirectory_QString(original_self as *mut ::file_dialog::FileDialog,
                                                             directory as *const ::qt_core::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FileDialog::set_option](../struct.FileDialog.html#method.set_option) method.
  pub trait FileDialogSetOptionArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::file_dialog::FileDialog) -> ();
  }
  impl<'largs> FileDialogSetOptionArgs<'largs> for ::file_dialog::Option {
    fn exec(self, original_self: &'largs mut ::file_dialog::FileDialog) -> () {
      let option = self;
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_setOption_option(original_self as *mut ::file_dialog::FileDialog, option)
      }
    }
  }
  impl<'largs> FileDialogSetOptionArgs<'largs> for (::file_dialog::Option, bool) {
    fn exec(self, original_self: &'largs mut ::file_dialog::FileDialog) -> () {
      let option = self.0;
      let on = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QFileDialog_setOption_option_on(original_self as *mut ::file_dialog::FileDialog, option, on)
      }
    }
  }
}
