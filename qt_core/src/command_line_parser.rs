/// C++ type: <span style='color: green;'>```QCommandLineParser```</span>
#[repr(C)]
pub struct CommandLineParser([u8; ::type_sizes::QT_CORE_COMMAND_LINE_PARSER_COMMAND_LINE_PARSER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for CommandLineParser {
  unsafe fn new_uninitialized() -> CommandLineParser {
    CommandLineParser(::std::mem::uninitialized())
  }
}

impl CommandLineParser {
  /// C++ method: <span style='color: green;'>```QCommandLineOption QCommandLineParser::addHelpOption()```</span>
  ///
  ///
  pub fn add_help_option(&mut self) -> ::command_line_option::CommandLineOption {
    {
      let mut object: ::command_line_option::CommandLineOption =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCommandLineParser_addHelpOption_to_output(self as *mut ::command_line_parser::CommandLineParser, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QCommandLineParser::addOption(const QCommandLineOption& commandLineOption)```</span>
  ///
  ///
  pub fn add_option(&mut self, command_line_option: &::command_line_option::CommandLineOption) -> bool {
    unsafe { ::ffi::qt_core_c_QCommandLineParser_addOption(self as *mut ::command_line_parser::CommandLineParser, command_line_option as *const ::command_line_option::CommandLineOption) }
  }

  /// C++ method: <span style='color: green;'>```bool QCommandLineParser::addOptions(const QList<QCommandLineOption>& options)```</span>
  ///
  ///
  pub fn add_options(&mut self, options: &::list::ListCommandLineOption) -> bool {
    unsafe {
      ::ffi::qt_core_c_QCommandLineParser_addOptions(self as *mut ::command_line_parser::CommandLineParser,
                                                     options as *const ::list::ListCommandLineOption)
    }
  }

  /// C++ method: <span style='color: green;'>```QCommandLineParser::addPositionalArgument```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_positional_argument(&mut self, (&::string::String, &::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QCommandLineParser::addPositionalArgument(const QString& name, const QString& description)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_positional_argument(&mut self, (&::string::String, &::string::String, &::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QCommandLineParser::addPositionalArgument(const QString& name, const QString& description, const QString& syntax = ?)```</span>
  ///
  ///
  pub fn add_positional_argument<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::CommandLineParserAddPositionalArgumentArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QCommandLineOption QCommandLineParser::addVersionOption()```</span>
  ///
  ///
  pub fn add_version_option(&mut self) -> ::command_line_option::CommandLineOption {
    {
      let mut object: ::command_line_option::CommandLineOption =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCommandLineParser_addVersionOption_to_output(self as *mut ::command_line_parser::CommandLineParser, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QCommandLineParser::applicationDescription() const```</span>
  ///
  ///
  pub fn application_description(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCommandLineParser_applicationDescription_to_output(self as *const ::command_line_parser::CommandLineParser, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QCommandLineParser::clearPositionalArguments()```</span>
  ///
  ///
  pub fn clear_positional_arguments(&mut self) {
    unsafe { ::ffi::qt_core_c_QCommandLineParser_clearPositionalArguments(self as *mut ::command_line_parser::CommandLineParser) }
  }

  /// C++ method: <span style='color: green;'>```QString QCommandLineParser::errorText() const```</span>
  ///
  ///
  pub fn error_text(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCommandLineParser_errorText_to_output(self as *const ::command_line_parser::CommandLineParser, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QCommandLineParser::helpText() const```</span>
  ///
  ///
  pub fn help_text(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCommandLineParser_helpText_to_output(self as *const ::command_line_parser::CommandLineParser, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCommandLineParser::isSet```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn is_set(&self, &::command_line_option::CommandLineOption) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QCommandLineParser::isSet(const QCommandLineOption& option) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn is_set(&self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QCommandLineParser::isSet(const QString& name) const```</span>
  ///
  ///
  pub fn is_set<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::CommandLineParserIsSetArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QCommandLineParser::QCommandLineParser()```</span>
  ///
  ///
  pub fn new() -> ::command_line_parser::CommandLineParser {
    {
      let mut object: ::command_line_parser::CommandLineParser =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCommandLineParser_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QCommandLineParser::optionNames() const```</span>
  ///
  ///
  pub fn option_names(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCommandLineParser_optionNames_to_output(self as *const ::command_line_parser::CommandLineParser, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QCommandLineParser::parse(const QStringList& arguments)```</span>
  ///
  ///
  pub fn parse(&mut self, arguments: &::string_list::StringList) -> bool {
    unsafe {
      ::ffi::qt_core_c_QCommandLineParser_parse(self as *mut ::command_line_parser::CommandLineParser,
                                                arguments as *const ::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QCommandLineParser::positionalArguments() const```</span>
  ///
  ///
  pub fn positional_arguments(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCommandLineParser_positionalArguments_to_output(self as *const ::command_line_parser::CommandLineParser, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCommandLineParser::process```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn process(&mut self, &::core_application::CoreApplication) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QCommandLineParser::process(const QCoreApplication& app)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn process(&mut self, &::string_list::StringList) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QCommandLineParser::process(const QStringList& arguments)```</span>
  ///
  ///
  pub fn process<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::CommandLineParserProcessArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QCommandLineParser::setApplicationDescription(const QString& description)```</span>
  ///
  ///
  pub fn set_application_description(&mut self, description: &::string::String) {
    unsafe { ::ffi::qt_core_c_QCommandLineParser_setApplicationDescription(self as *mut ::command_line_parser::CommandLineParser, description as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```void QCommandLineParser::setOptionsAfterPositionalArgumentsMode(QCommandLineParser::OptionsAfterPositionalArgumentsMode mode)```</span>
  ///
  ///
pub fn set_options_after_positional_arguments_mode(&mut self, mode: ::command_line_parser::OptionsAfterPositionalArgumentsMode) {
    unsafe { ::ffi::qt_core_c_QCommandLineParser_setOptionsAfterPositionalArgumentsMode(self as *mut ::command_line_parser::CommandLineParser, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QCommandLineParser::setSingleDashWordOptionMode(QCommandLineParser::SingleDashWordOptionMode parsingMode)```</span>
  ///
  ///
  pub fn set_single_dash_word_option_mode(&mut self, parsing_mode: ::command_line_parser::SingleDashWordOptionMode) {
    unsafe { ::ffi::qt_core_c_QCommandLineParser_setSingleDashWordOptionMode(self as *mut ::command_line_parser::CommandLineParser, parsing_mode) }
  }

  /// C++ method: <span style='color: green;'>```QCommandLineParser::showHelp```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn show_help(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QCommandLineParser::showHelp()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn show_help(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QCommandLineParser::showHelp(int exitCode = ?)```</span>
  ///
  ///
  pub fn show_help<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::CommandLineParserShowHelpArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QCommandLineParser::showVersion()```</span>
  ///
  ///
  pub fn show_version(&mut self) {
    unsafe { ::ffi::qt_core_c_QCommandLineParser_showVersion(self as *mut ::command_line_parser::CommandLineParser) }
  }

  /// C++ method: <span style='color: green;'>```static QString QCommandLineParser::tr(const char* sourceText, const char* disambiguation, int n)```</span>
  ///
  ///
  pub unsafe fn tr(source_text: *const ::libc::c_char,
                   disambiguation: *const ::libc::c_char,
                   n: ::libc::c_int)
                   -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QCommandLineParser_tr_to_output(source_text, disambiguation, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QCommandLineParser::trUtf8(const char* sourceText, const char* disambiguation, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(source_text: *const ::libc::c_char,
                        disambiguation: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QCommandLineParser_trUtf8_to_output(source_text, disambiguation, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QCommandLineParser::unknownOptionNames() const```</span>
  ///
  ///
  pub fn unknown_option_names(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCommandLineParser_unknownOptionNames_to_output(self as *const ::command_line_parser::CommandLineParser, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCommandLineParser::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, &::command_line_option::CommandLineOption) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QCommandLineParser::value(const QCommandLineOption& option) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, &::string::String) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QCommandLineParser::value(const QString& name) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::CommandLineParserValueArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QCommandLineParser::values```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn values(&self, &::command_line_option::CommandLineOption) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QCommandLineParser::values(const QCommandLineOption& option) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn values(&self, &::string::String) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QCommandLineParser::values(const QString& name) const```</span>
  ///
  ///
  pub fn values<'largs, Args>(&'largs self, args: Args) -> ::string_list::StringList
    where Args: overloading::CommandLineParserValuesArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::command_line_parser::CommandLineParser {
  /// C++ method: <span style='color: green;'>```[destructor] void QCommandLineParser::~QCommandLineParser()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QCommandLineParser_destructor(self as *mut ::command_line_parser::CommandLineParser) }
  }
}

/// C++ type: <span style='color: green;'>```QCommandLineParser::OptionsAfterPositionalArgumentsMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum OptionsAfterPositionalArgumentsMode {
  /// C++ enum variant: <span style='color: green;'>```ParseAsOptions = 0```</span>
  Options = 0,
  /// C++ enum variant: <span style='color: green;'>```ParseAsPositionalArguments = 1```</span>
  PositionalArguments = 1,
}

/// C++ type: <span style='color: green;'>```QCommandLineParser::SingleDashWordOptionMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SingleDashWordOptionMode {
  /// C++ enum variant: <span style='color: green;'>```ParseAsCompactedShortOptions = 0```</span>
  CompactedShort = 0,
  /// C++ enum variant: <span style='color: green;'>```ParseAsLongOptions = 1```</span>
  Long = 1,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [CommandLineParser::add_positional_argument](../struct.CommandLineParser.html#method.add_positional_argument) method.
  pub trait CommandLineParserAddPositionalArgumentArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::command_line_parser::CommandLineParser) -> ();
  }
  impl<'largs> CommandLineParserAddPositionalArgumentArgs<'largs>
    for (&'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::command_line_parser::CommandLineParser) -> () {
      let name = self.0;
      let description = self.1;
      unsafe { ::ffi::qt_core_c_QCommandLineParser_addPositionalArgument_name_description(original_self as *mut ::command_line_parser::CommandLineParser, name as *const ::string::String, description as *const ::string::String) }
    }
  }
  impl<'largs> CommandLineParserAddPositionalArgumentArgs<'largs>
    for (&'largs ::string::String, &'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::command_line_parser::CommandLineParser) -> () {
      let name = self.0;
      let description = self.1;
      let syntax = self.2;
      unsafe { ::ffi::qt_core_c_QCommandLineParser_addPositionalArgument_name_description_syntax(original_self as *mut ::command_line_parser::CommandLineParser, name as *const ::string::String, description as *const ::string::String, syntax as *const ::string::String) }
    }
  }
  /// This trait represents a set of arguments accepted by [CommandLineParser::is_set](../struct.CommandLineParser.html#method.is_set) method.
  pub trait CommandLineParserIsSetArgs<'largs> {
    fn exec(self, original_self: &'largs ::command_line_parser::CommandLineParser) -> bool;
  }
  impl<'largs> CommandLineParserIsSetArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::command_line_parser::CommandLineParser) -> bool {
      let name = self;
      unsafe {
        ::ffi::qt_core_c_QCommandLineParser_isSet_name(original_self as *const ::command_line_parser::CommandLineParser, name as *const ::string::String)
      }
    }
  }
  impl<'largs> CommandLineParserIsSetArgs<'largs> for &'largs ::command_line_option::CommandLineOption {
    fn exec(self, original_self: &'largs ::command_line_parser::CommandLineParser) -> bool {
      let option = self;
      unsafe { ::ffi::qt_core_c_QCommandLineParser_isSet_option(original_self as *const ::command_line_parser::CommandLineParser, option as *const ::command_line_option::CommandLineOption) }
    }
  }
  /// This trait represents a set of arguments accepted by [CommandLineParser::process](../struct.CommandLineParser.html#method.process) method.
  pub trait CommandLineParserProcessArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::command_line_parser::CommandLineParser) -> ();
  }
  impl<'largs> CommandLineParserProcessArgs<'largs> for &'largs ::core_application::CoreApplication {
    fn exec(self, original_self: &'largs mut ::command_line_parser::CommandLineParser) -> () {
      let app = self;
      unsafe {
        ::ffi::qt_core_c_QCommandLineParser_process_app(original_self as *mut ::command_line_parser::CommandLineParser, app as *const ::core_application::CoreApplication)
      }
    }
  }
  impl<'largs> CommandLineParserProcessArgs<'largs> for &'largs ::string_list::StringList {
    fn exec(self, original_self: &'largs mut ::command_line_parser::CommandLineParser) -> () {
      let arguments = self;
      unsafe { ::ffi::qt_core_c_QCommandLineParser_process_arguments(original_self as *mut ::command_line_parser::CommandLineParser, arguments as *const ::string_list::StringList) }
    }
  }
  /// This trait represents a set of arguments accepted by [CommandLineParser::show_help](../struct.CommandLineParser.html#method.show_help) method.
  pub trait CommandLineParserShowHelpArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::command_line_parser::CommandLineParser) -> ();
  }
  impl<'largs> CommandLineParserShowHelpArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::command_line_parser::CommandLineParser) -> () {
      let exit_code = self;
      unsafe { ::ffi::qt_core_c_QCommandLineParser_showHelp_exitCode(original_self as *mut ::command_line_parser::CommandLineParser, exit_code) }
    }
  }
  impl<'largs> CommandLineParserShowHelpArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::command_line_parser::CommandLineParser) -> () {

      unsafe { ::ffi::qt_core_c_QCommandLineParser_showHelp_no_args(original_self as *mut ::command_line_parser::CommandLineParser) }
    }
  }
  /// This trait represents a set of arguments accepted by [CommandLineParser::value](../struct.CommandLineParser.html#method.value) method.
  pub trait CommandLineParserValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::command_line_parser::CommandLineParser) -> ::string::String;
  }
  impl<'largs> CommandLineParserValueArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::command_line_parser::CommandLineParser) -> ::string::String {
      let name = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QCommandLineParser_value_to_output_name(original_self as *const ::command_line_parser::CommandLineParser, name as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> CommandLineParserValueArgs<'largs> for &'largs ::command_line_option::CommandLineOption {
    fn exec(self, original_self: &'largs ::command_line_parser::CommandLineParser) -> ::string::String {
      let option = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QCommandLineParser_value_to_output_option(original_self as *const ::command_line_parser::CommandLineParser, option as *const ::command_line_option::CommandLineOption, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [CommandLineParser::values](../struct.CommandLineParser.html#method.values) method.
  pub trait CommandLineParserValuesArgs<'largs> {
    fn exec(self, original_self: &'largs ::command_line_parser::CommandLineParser) -> ::string_list::StringList;
  }
  impl<'largs> CommandLineParserValuesArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::command_line_parser::CommandLineParser) -> ::string_list::StringList {
      let name = self;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QCommandLineParser_values_to_output_name(original_self as *const ::command_line_parser::CommandLineParser, name as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> CommandLineParserValuesArgs<'largs> for &'largs ::command_line_option::CommandLineOption {
    fn exec(self, original_self: &'largs ::command_line_parser::CommandLineParser) -> ::string_list::StringList {
      let option = self;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QCommandLineParser_values_to_output_option(original_self as *const ::command_line_parser::CommandLineParser, option as *const ::command_line_option::CommandLineOption, &mut object);
        }
        object
      }
    }
  }
}
