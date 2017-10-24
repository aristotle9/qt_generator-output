/// C++ type: <span style='color: green;'>```QString::NormalizationForm```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum NormalizationForm {
  /// C++ enum variant: <span style='color: green;'>```NormalizationForm_D = 0```</span>
  D = 0,
  /// C++ enum variant: <span style='color: green;'>```NormalizationForm_C = 1```</span>
  C = 1,
  /// C++ enum variant: <span style='color: green;'>```NormalizationForm_KD = 2```</span>
  KD = 2,
  /// C++ enum variant: <span style='color: green;'>```NormalizationForm_KC = 3```</span>
  KC = 3,
}

/// C++ type: <span style='color: green;'>```QString::SectionFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SectionFlag {
  /// C++ enum variant: <span style='color: green;'>```SectionDefault = 0```</span>
  Default = 0,
  /// C++ enum variant: <span style='color: green;'>```SectionSkipEmpty = 1```</span>
  SkipEmpty = 1,
  /// C++ enum variant: <span style='color: green;'>```SectionIncludeLeadingSep = 2```</span>
  IncludeLeadingSep = 2,
  /// C++ enum variant: <span style='color: green;'>```SectionIncludeTrailingSep = 4```</span>
  IncludeTrailingSep = 4,
  /// C++ enum variant: <span style='color: green;'>```SectionCaseInsensitiveSeps = 8```</span>
  CaseInsensitiveSeps = 8,
}

impl ::flags::FlaggableEnum for SectionFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "SectionFlag"
  }
}

/// C++ type: <span style='color: green;'>```QString::SplitBehavior```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SplitBehavior {
  /// C++ enum variant: <span style='color: green;'>```KeepEmptyParts = 0```</span>
  Keep = 0,
  /// C++ enum variant: <span style='color: green;'>```SkipEmptyParts = 1```</span>
  Skip = 1,
}

/// C++ type: <span style='color: green;'>```QString```</span>
#[repr(C)]
pub struct String([u8; ::type_sizes::QT_CORE_STRING_STRING]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for String {
  unsafe fn new_uninitialized() -> String {
    String(::std::mem::uninitialized())
  }
}

impl String {
  /// C++ method: <span style='color: green;'>```QString::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &'l1 ::char::Char) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::append(QChar c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &'l1 ::latin1_string::Latin1String) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::append(QLatin1String s)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn append(&mut self, &'l1 ::byte_array::ByteArray) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::append(const QByteArray& s)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn append(&mut self, &'l1 ::string::String) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::append(const QString& s)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn append(&mut self, &'l1 ::string_ref::StringRef) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::append(const QStringRef& s)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::string::String
    where Args: overloading::StringAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append_unsafe(&mut self, (*const ::char::Char, ::libc::c_int)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::append(const QChar* uc, int len)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append_unsafe(&mut self, *const ::libc::c_char) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::append(const char* s)```</span>
  ///
  ///
  pub unsafe fn append_unsafe<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::string::String
    where Args: overloading::StringAppendUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::arg```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn arg0(&self, &::char::Char) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(QChar a) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn arg0(&self, (&::char::Char, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(QChar a, int fieldWidth = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn arg0(&self, (&::char::Char, ::libc::c_int, &::char::Char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(QChar a, int fieldWidth = ?, QChar fillChar = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn arg0(&self, ::libc::c_char) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(char a) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn arg0(&self, (::libc::c_char, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(char a, int fieldWidth = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn arg0(&self, (::libc::c_char, ::libc::c_int, &::char::Char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(char a, int fieldWidth = ?, QChar fillChar = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn arg0(&self, &::string::String) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(const QString& a) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn arg0(&self, (&::string::String, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(const QString& a, int fieldWidth = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn arg0(&self, (&::string::String, ::libc::c_int, &::char::Char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(const QString& a, int fieldWidth = ?, QChar fillChar = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn arg0(&self, (&::string::String, &::string::String)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(const QString& a1, const QString& a2) const```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn arg0(&self, (&::string::String, &::string::String, &::string::String)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(const QString& a1, const QString& a2, const QString& a3) const```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn arg0(&self, (&::string::String, &::string::String, &::string::String, &::string::String)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(const QString& a1, const QString& a2, const QString& a3, const QString& a4) const```</span>
  ///
  ///
  ///
  /// ## Variant 13
  ///
  /// Rust arguments: ```fn arg0(&self, (&::string::String, &::string::String, &::string::String, &::string::String, &::string::String)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(const QString& a1, const QString& a2, const QString& a3, const QString& a4, const QString& a5) const```</span>
  ///
  ///
  ///
  /// ## Variant 14
  ///
  /// Rust arguments: ```fn arg0(&self, (&::string::String, &::string::String, &::string::String, &::string::String, &::string::String, &::string::String)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(const QString& a1, const QString& a2, const QString& a3, const QString& a4, const QString& a5, const QString& a6) const```</span>
  ///
  ///
  ///
  /// ## Variant 15
  ///
  /// Rust arguments: ```fn arg0(&self, (&::string::String, &::string::String, &::string::String, &::string::String, &::string::String, &::string::String, &::string::String)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(const QString& a1, const QString& a2, const QString& a3, const QString& a4, const QString& a5, const QString& a6, const QString& a7) const```</span>
  ///
  ///
  ///
  /// ## Variant 16
  ///
  /// Rust arguments: ```fn arg0(&self, (&::string::String, &::string::String, &::string::String, &::string::String, &::string::String, &::string::String, &::string::String, &::string::String)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(const QString& a1, const QString& a2, const QString& a3, const QString& a4, const QString& a5, const QString& a6, const QString& a7, const QString& a8) const```</span>
  ///
  ///
  ///
  /// ## Variant 17
  ///
  /// Rust arguments: ```fn arg0(&self, (&::string::String, &::string::String, &::string::String, &::string::String, &::string::String, &::string::String, &::string::String, &::string::String, &::string::String)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(const QString& a1, const QString& a2, const QString& a3, const QString& a4, const QString& a5, const QString& a6, const QString& a7, const QString& a8, const QString& a9) const```</span>
  ///
  ///
  ///
  /// ## Variant 18
  ///
  /// Rust arguments: ```fn arg0(&self, ::libc::c_double) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(double a) const```</span>
  ///
  ///
  ///
  /// ## Variant 19
  ///
  /// Rust arguments: ```fn arg0(&self, (::libc::c_double, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(double a, int fieldWidth = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 20
  ///
  /// Rust arguments: ```fn arg0(&self, (::libc::c_double, ::libc::c_int, ::libc::c_char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(double a, int fieldWidth = ?, char fmt = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 21
  ///
  /// Rust arguments: ```fn arg0(&self, (::libc::c_double, ::libc::c_int, ::libc::c_char, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(double a, int fieldWidth = ?, char fmt = ?, int prec = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 22
  ///
  /// Rust arguments: ```fn arg0(&self, (::libc::c_double, ::libc::c_int, ::libc::c_char, ::libc::c_int, &::char::Char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(double a, int fieldWidth = ?, char fmt = ?, int prec = ?, QChar fillChar = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 23
  ///
  /// Rust arguments: ```fn arg0(&self, (::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(int a, int fieldWidth = ?, int base = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 24
  ///
  /// Rust arguments: ```fn arg0(&self, (::libc::c_int, ::libc::c_int, ::libc::c_int, &::char::Char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(int a, int fieldWidth = ?, int base = ?, QChar fillChar = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 25
  ///
  /// Rust arguments: ```fn arg0(&self, (u64, ::libc::c_int, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(qulonglong a, int fieldwidth = ?, int base = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 26
  ///
  /// Rust arguments: ```fn arg0(&self, (u64, ::libc::c_int, ::libc::c_int, &::char::Char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(qulonglong a, int fieldwidth = ?, int base = ?, QChar fillChar = ?) const```</span>
  ///
  ///
  pub fn arg0<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::StringArg0Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::arg```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn arg1(&self, ::libc::c_int) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(int a) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn arg1(&self, (::libc::c_int, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(int a, int fieldWidth = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn arg1(&self, (::libc::c_long, ::libc::c_int, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(long a, int fieldwidth = ?, int base = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn arg1(&self, (::libc::c_long, ::libc::c_int, ::libc::c_int, &::char::Char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(long a, int fieldwidth = ?, int base = ?, QChar fillChar = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn arg1(&self, u64) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(qulonglong a) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn arg1(&self, (u64, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(qulonglong a, int fieldwidth = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn arg1(&self, (::libc::c_uint, ::libc::c_int, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(unsigned int a, int fieldWidth = ?, int base = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn arg1(&self, (::libc::c_uint, ::libc::c_int, ::libc::c_int, &::char::Char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(unsigned int a, int fieldWidth = ?, int base = ?, QChar fillChar = ?) const```</span>
  ///
  ///
  pub fn arg1<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::StringArg1Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::arg```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn arg2(&self, ::libc::c_long) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(long a) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn arg2(&self, (::libc::c_long, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(long a, int fieldwidth = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn arg2(&self, (i64, ::libc::c_int, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(qlonglong a, int fieldwidth = ?, int base = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn arg2(&self, (i64, ::libc::c_int, ::libc::c_int, &::char::Char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(qlonglong a, int fieldwidth = ?, int base = ?, QChar fillChar = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn arg2(&self, ::libc::c_uint) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(unsigned int a) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn arg2(&self, (::libc::c_uint, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(unsigned int a, int fieldWidth = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn arg2(&self, (::libc::c_ulong, ::libc::c_int, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(unsigned long a, int fieldwidth = ?, int base = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn arg2(&self, (::libc::c_ulong, ::libc::c_int, ::libc::c_int, &::char::Char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(unsigned long a, int fieldwidth = ?, int base = ?, QChar fillChar = ?) const```</span>
  ///
  ///
  pub fn arg2<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::StringArg2Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::arg```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn arg3(&self, i64) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(qlonglong a) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn arg3(&self, (i64, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(qlonglong a, int fieldwidth = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn arg3(&self, (::libc::c_short, ::libc::c_int, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(short a, int fieldWidth = ?, int base = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn arg3(&self, (::libc::c_short, ::libc::c_int, ::libc::c_int, &::char::Char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(short a, int fieldWidth = ?, int base = ?, QChar fillChar = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn arg3(&self, ::libc::c_ulong) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(unsigned long a) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn arg3(&self, (::libc::c_ulong, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(unsigned long a, int fieldwidth = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn arg3(&self, (::libc::c_ushort, ::libc::c_int, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(unsigned short a, int fieldWidth = ?, int base = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn arg3(&self, (::libc::c_ushort, ::libc::c_int, ::libc::c_int, &::char::Char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(unsigned short a, int fieldWidth = ?, int base = ?, QChar fillChar = ?) const```</span>
  ///
  ///
  pub fn arg3<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::StringArg3Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::arg```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn arg4(&self, ::libc::c_short) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(short a) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn arg4(&self, (::libc::c_short, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(short a, int fieldWidth = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn arg4(&self, ::libc::c_ushort) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(unsigned short a) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn arg4(&self, (::libc::c_ushort, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::arg(unsigned short a, int fieldWidth = ?) const```</span>
  ///
  ///
  pub fn arg4<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::StringArg4Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QChar QString::at(int i) const```</span>
  ///
  ///
  pub fn at(&self, i: ::libc::c_int) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_at_to_output(self as *const ::string::String, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QChar* QString::begin() const```</span>
  ///
  ///
  pub fn begin(&self) -> *const ::char::Char {
    unsafe { ::ffi::qt_core_c_QString_begin_const(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QChar* QString::begin()```</span>
  ///
  ///
  pub fn begin_mut(&mut self) -> *mut ::char::Char {
    unsafe { ::ffi::qt_core_c_QString_begin(self as *mut ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```int QString::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QString_capacity(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```const QChar* QString::cbegin() const```</span>
  ///
  ///
  pub fn cbegin(&self) -> *const ::char::Char {
    unsafe { ::ffi::qt_core_c_QString_cbegin(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```const QChar* QString::cend() const```</span>
  ///
  ///
  pub fn cend(&self) -> *const ::char::Char {
    unsafe { ::ffi::qt_core_c_QString_cend(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```void QString::chop(int n)```</span>
  ///
  ///
  pub fn chop(&mut self, n: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QString_chop(self as *mut ::string::String, n) }
  }

  /// C++ method: <span style='color: green;'>```void QString::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QString_clear(self as *mut ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QString::compare```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn compare(&self, &::latin1_string::Latin1String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::compare(QLatin1String other) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn compare(&self, (&::latin1_string::Latin1String, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::compare(QLatin1String other, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn compare(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::compare(const QString& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn compare(&self, (&::string::String, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::compare(const QString& s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn compare(&self, &::string_ref::StringRef) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::compare(const QStringRef& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn compare(&self, (&::string_ref::StringRef, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::compare(const QStringRef& s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  pub fn compare<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StringCompareArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::compare```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn compare_static((&::latin1_string::Latin1String, &::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QString::compare(QLatin1String s1, const QString& s2)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn compare_static((&::latin1_string::Latin1String, &::string::String, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QString::compare(QLatin1String s1, const QString& s2, Qt::CaseSensitivity cs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn compare_static((&::string::String, &::latin1_string::Latin1String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QString::compare(const QString& s1, QLatin1String s2)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn compare_static((&::string::String, &::latin1_string::Latin1String, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QString::compare(const QString& s1, QLatin1String s2, Qt::CaseSensitivity cs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn compare_static((&::string::String, &::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QString::compare(const QString& s1, const QString& s2)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn compare_static((&::string::String, &::string::String, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QString::compare(const QString& s1, const QString& s2, Qt::CaseSensitivity cs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn compare_static((&::string::String, &::string_ref::StringRef)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QString::compare(const QString& s1, const QStringRef& s2)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn compare_static((&::string::String, &::string_ref::StringRef, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QString::compare(const QString& s1, const QStringRef& s2, Qt::CaseSensitivity arg3 = ?)```</span>
  ///
  ///
  pub fn compare_static<Args>(args: Args) -> ::libc::c_int
    where Args: overloading::StringCompareStaticArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```const QChar* QString::constBegin() const```</span>
  ///
  ///
  pub fn const_begin(&self) -> *const ::char::Char {
    unsafe { ::ffi::qt_core_c_QString_constBegin(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```const QChar* QString::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::char::Char {
    unsafe { ::ffi::qt_core_c_QString_constData(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```const QChar* QString::constEnd() const```</span>
  ///
  ///
  pub fn const_end(&self) -> *const ::char::Char {
    unsafe { ::ffi::qt_core_c_QString_constEnd(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QString::contains```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn contains(&self, &::char::Char) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::contains(QChar c) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn contains(&self, (&::char::Char, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::contains(QChar c, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn contains(&self, &::latin1_string::Latin1String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::contains(QLatin1String s) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn contains(&self, (&::latin1_string::Latin1String, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::contains(QLatin1String s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn contains(&self, &mut ::reg_exp::RegExp) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::contains(QRegExp& rx) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn contains(&self, &::reg_exp::RegExp) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::contains(const QRegExp& rx) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn contains(&self, &::regular_expression::RegularExpression) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::contains(const QRegularExpression& re) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn contains(&self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::contains(const QString& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn contains(&self, (&::string::String, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::contains(const QString& s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn contains(&self, &::string_ref::StringRef) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::contains(const QStringRef& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn contains(&self, (&::string_ref::StringRef, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::contains(const QStringRef& s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  pub fn contains<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::StringContainsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QString::contains(const QRegularExpression& re, QRegularExpressionMatch* match) const```</span>
  ///
  ///
  pub unsafe fn contains_unsafe(&self,
                                re: &::regular_expression::RegularExpression,
                                match_: *mut ::regular_expression_match::RegularExpressionMatch)
                                -> bool {
    ::ffi::qt_core_c_QString_contains_const_QRegularExpression_ref_QRegularExpressionMatch_ptr(self as *const ::string::String, re as *const ::regular_expression::RegularExpression, match_)
  }

  /// C++ method: <span style='color: green;'>```QString::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::char::Char) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::count(QChar c) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn count(&self, (&::char::Char, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::count(QChar c, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn count(&self, &::reg_exp::RegExp) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::count(const QRegExp& arg1) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn count(&self, &::regular_expression::RegularExpression) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::count(const QRegularExpression& re) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn count(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::count(const QString& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn count(&self, (&::string::String, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::count(const QString& s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn count(&self, &::string_ref::StringRef) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::count(const QStringRef& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn count(&self, (&::string_ref::StringRef, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::count(const QStringRef& s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StringCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QChar* QString::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::char::Char {
    unsafe { ::ffi::qt_core_c_QString_data_const(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QChar* QString::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::char::Char {
    unsafe { ::ffi::qt_core_c_QString_data(self as *mut ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```const QChar* QString::end() const```</span>
  ///
  ///
  pub fn end(&self) -> *const ::char::Char {
    unsafe { ::ffi::qt_core_c_QString_end_const(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QChar* QString::end()```</span>
  ///
  ///
  pub fn end_mut(&mut self) -> *mut ::char::Char {
    unsafe { ::ffi::qt_core_c_QString_end(self as *mut ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QString::endsWith```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn ends_with(&self, &::char::Char) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::endsWith(QChar c) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn ends_with(&self, (&::char::Char, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::endsWith(QChar c, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn ends_with(&self, &::latin1_string::Latin1String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::endsWith(QLatin1String s) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn ends_with(&self, (&::latin1_string::Latin1String, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::endsWith(QLatin1String s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn ends_with(&self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::endsWith(const QString& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn ends_with(&self, (&::string::String, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::endsWith(const QString& s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn ends_with(&self, &::string_ref::StringRef) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::endsWith(const QStringRef& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn ends_with(&self, (&::string_ref::StringRef, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::endsWith(const QStringRef& s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  pub fn ends_with<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::StringEndsWithArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::char::Char) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::fill(QChar c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::char::Char, ::libc::c_int)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::fill(QChar c, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::string::String
    where Args: overloading::StringFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QString QString::fromLatin1(const QByteArray& str)```</span>
  ///
  ///
  pub fn from_latin1(str: &::byte_array::ByteArray) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_fromLatin1_to_output_QByteArray(str as *const ::byte_array::ByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString::fromLatin1```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_latin1_unsafe(*const ::libc::c_char) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::fromLatin1(const char* str)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_latin1_unsafe((*const ::libc::c_char, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::fromLatin1(const char* str, int size = ?)```</span>
  ///
  ///
  pub unsafe fn from_latin1_unsafe<Args>(args: Args) -> ::string::String
    where Args: overloading::StringFromLatin1UnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QString QString::fromLocal8Bit(const QByteArray& str)```</span>
  ///
  ///
  pub fn from_local8_bit(str: &::byte_array::ByteArray) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_fromLocal8Bit_to_output_QByteArray(str as *const ::byte_array::ByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString::fromLocal8Bit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_local8_bit_unsafe(*const ::libc::c_char) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::fromLocal8Bit(const char* str)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_local8_bit_unsafe((*const ::libc::c_char, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::fromLocal8Bit(const char* str, int size = ?)```</span>
  ///
  ///
  pub unsafe fn from_local8_bit_unsafe<Args>(args: Args) -> ::string::String
    where Args: overloading::StringFromLocal8BitUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QString QString::fromRawData(const QChar* arg1, int size)```</span>
  ///
  ///
  pub unsafe fn from_raw_data(arg1: *const ::char::Char, size: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QString_fromRawData_to_output(arg1, size, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString::fromUcs4```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_ucs4(*const ::libc::c_uint) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::fromUcs4(const unsigned int* arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_ucs4((*const ::libc::c_uint, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::fromUcs4(const unsigned int* arg1, int size = ?)```</span>
  ///
  ///
  pub unsafe fn from_ucs4<Args>(args: Args) -> ::string::String
    where Args: overloading::StringFromUcs4Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QString::fromUtf16```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_utf16(*const ::libc::c_ushort) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::fromUtf16(const unsigned short* arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_utf16((*const ::libc::c_ushort, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::fromUtf16(const unsigned short* arg1, int size = ?)```</span>
  ///
  ///
  pub unsafe fn from_utf16<Args>(args: Args) -> ::string::String
    where Args: overloading::StringFromUtf16Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QString QString::fromUtf8(const QByteArray& str)```</span>
  ///
  ///
  pub fn from_utf8(str: &::byte_array::ByteArray) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_fromUtf8_to_output_QByteArray(str as *const ::byte_array::ByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString::fromUtf8```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_utf8_unsafe(*const ::libc::c_char) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::fromUtf8(const char* str)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_utf8_unsafe((*const ::libc::c_char, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::fromUtf8(const char* str, int size = ?)```</span>
  ///
  ///
  pub unsafe fn from_utf8_unsafe<Args>(args: Args) -> ::string::String
    where Args: overloading::StringFromUtf8UnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QString::fromWCharArray```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_w_char_array(*const ::libc::wchar_t) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::fromWCharArray(const wchar_t* string)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_w_char_array((*const ::libc::wchar_t, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::fromWCharArray(const wchar_t* string, int size = ?)```</span>
  ///
  ///
  pub unsafe fn from_w_char_array<Args>(args: Args) -> ::string::String
    where Args: overloading::StringFromWCharArrayArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QString::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::char::Char) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::indexOf(QChar c) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::char::Char, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::indexOf(QChar c, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn index_of(&self, (&::char::Char, ::libc::c_int, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::indexOf(QChar c, int from = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn index_of(&self, &::latin1_string::Latin1String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::indexOf(QLatin1String s) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn index_of(&self, (&::latin1_string::Latin1String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::indexOf(QLatin1String s, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn index_of(&self, (&::latin1_string::Latin1String, ::libc::c_int, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::indexOf(QLatin1String s, int from = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn index_of(&self, &mut ::reg_exp::RegExp) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::indexOf(QRegExp& arg1) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn index_of(&self, (&mut ::reg_exp::RegExp, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::indexOf(QRegExp& arg1, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn index_of(&self, &::reg_exp::RegExp) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::indexOf(const QRegExp& arg1) const```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn index_of(&self, (&::reg_exp::RegExp, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::indexOf(const QRegExp& arg1, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn index_of(&self, &::regular_expression::RegularExpression) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::indexOf(const QRegularExpression& re) const```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn index_of(&self, (&::regular_expression::RegularExpression, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::indexOf(const QRegularExpression& re, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 13
  ///
  /// Rust arguments: ```fn index_of(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::indexOf(const QString& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 14
  ///
  /// Rust arguments: ```fn index_of(&self, (&::string::String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::indexOf(const QString& s, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 15
  ///
  /// Rust arguments: ```fn index_of(&self, (&::string::String, ::libc::c_int, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::indexOf(const QString& s, int from = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 16
  ///
  /// Rust arguments: ```fn index_of(&self, &::string_ref::StringRef) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::indexOf(const QStringRef& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 17
  ///
  /// Rust arguments: ```fn index_of(&self, (&::string_ref::StringRef, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::indexOf(const QStringRef& s, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 18
  ///
  /// Rust arguments: ```fn index_of(&self, (&::string_ref::StringRef, ::libc::c_int, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::indexOf(const QStringRef& s, int from = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StringIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QString::indexOf(const QRegularExpression& re, int from, QRegularExpressionMatch* rmatch) const```</span>
  ///
  ///
  pub unsafe fn index_of_unsafe(&self,
                                re: &::regular_expression::RegularExpression,
                                from: ::libc::c_int,
                                rmatch: *mut ::regular_expression_match::RegularExpressionMatch)
                                -> ::libc::c_int {
    ::ffi::qt_core_c_QString_indexOf_const_QRegularExpression_ref_int_QRegularExpressionMatch_ptr(self as *const ::string::String, re as *const ::regular_expression::RegularExpression, from, rmatch)
  }

  /// C++ method: <span style='color: green;'>```QString::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &'l1 ::char::Char)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::insert(int i, QChar c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &'l1 ::latin1_string::Latin1String)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::insert(int i, QLatin1String s)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &'l1 ::byte_array::ByteArray)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::insert(int i, const QByteArray& s)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &'l1 ::string::String)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::insert(int i, const QString& s)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &'l1 ::string_ref::StringRef)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::insert(int i, const QStringRef& s)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::string::String
    where Args: overloading::StringInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_unsafe(&mut self, (::libc::c_int, *const ::char::Char, ::libc::c_int)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::insert(int i, const QChar* uc, int len)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_unsafe(&mut self, (::libc::c_int, *const ::libc::c_char)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::insert(int i, const char* s)```</span>
  ///
  ///
  pub unsafe fn insert_unsafe<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::string::String
    where Args: overloading::StringInsertUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QString::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QString_isEmpty(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```bool QString::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QString_isNull(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```bool QString::isRightToLeft() const```</span>
  ///
  ///
  pub fn is_right_to_left(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QString_isRightToLeft(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QString::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::char::Char) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(QChar c) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::char::Char, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(QChar c, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::char::Char, ::libc::c_int, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(QChar c, int from = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::latin1_string::Latin1String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(QLatin1String s) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::latin1_string::Latin1String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(QLatin1String s, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::latin1_string::Latin1String, ::libc::c_int, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(QLatin1String s, int from = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn last_index_of(&self, &mut ::reg_exp::RegExp) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(QRegExp& arg1) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&mut ::reg_exp::RegExp, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(QRegExp& arg1, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::reg_exp::RegExp) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(const QRegExp& arg1) const```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::reg_exp::RegExp, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(const QRegExp& arg1, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::regular_expression::RegularExpression) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(const QRegularExpression& re) const```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::regular_expression::RegularExpression, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(const QRegularExpression& re, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 13
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(const QString& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 14
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::string::String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(const QString& s, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 15
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::string::String, ::libc::c_int, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(const QString& s, int from = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 16
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::string_ref::StringRef) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(const QStringRef& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 17
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::string_ref::StringRef, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(const QStringRef& s, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 18
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::string_ref::StringRef, ::libc::c_int, &::qt::CaseSensitivity)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(const QStringRef& s, int from = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StringLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QString::lastIndexOf(const QRegularExpression& re, int from, QRegularExpressionMatch* rmatch) const```</span>
  ///
  ///
  pub unsafe fn last_index_of_unsafe(&self,
                                     re: &::regular_expression::RegularExpression,
                                     from: ::libc::c_int,
                                     rmatch: *mut ::regular_expression_match::RegularExpressionMatch)
                                     -> ::libc::c_int {
    ::ffi::qt_core_c_QString_lastIndexOf_const_QRegularExpression_ref_int_QRegularExpressionMatch_ptr(self as *const ::string::String, re as *const ::regular_expression::RegularExpression, from, rmatch)
  }

  /// C++ method: <span style='color: green;'>```QString QString::left(int n) const```</span>
  ///
  ///
  pub fn left(&self, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_left_to_output(self as *const ::string::String, n, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString::leftJustified```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn left_justified(&self, ::libc::c_int) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::leftJustified(int width) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn left_justified(&self, (::libc::c_int, &::char::Char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::leftJustified(int width, QChar fill = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn left_justified(&self, (::libc::c_int, &::char::Char, bool)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::leftJustified(int width, QChar fill = ?, bool trunc = ?) const```</span>
  ///
  ///
  pub fn left_justified<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::StringLeftJustifiedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringRef QString::leftRef(int n) const```</span>
  ///
  ///
  pub fn left_ref(&self, n: ::libc::c_int) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_leftRef_to_output(self as *const ::string::String, n, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QString::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QString_length(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QString::localeAwareCompare```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn locale_aware_compare(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::localeAwareCompare(const QString& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn locale_aware_compare(&self, &::string_ref::StringRef) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::localeAwareCompare(const QStringRef& s) const```</span>
  ///
  ///
  pub fn locale_aware_compare<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StringLocaleAwareCompareArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::localeAwareCompare```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn locale_aware_compare_static((&::string::String, &::string::String)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QString::localeAwareCompare(const QString& s1, const QString& s2)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn locale_aware_compare_static((&::string::String, &::string_ref::StringRef)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```static int QString::localeAwareCompare(const QString& s1, const QStringRef& s2)```</span>
  ///
  ///
  pub fn locale_aware_compare_static<Args>(args: Args) -> ::libc::c_int
    where Args: overloading::StringLocaleAwareCompareStaticArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QString::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::mid(int position) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::mid(int position, int n = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::StringMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::midRef```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid_ref(&self, ::libc::c_int) -> ::string_ref::StringRef```<br>
  /// C++ method: <span style='color: green;'>```QStringRef QString::midRef(int position) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid_ref(&self, (::libc::c_int, ::libc::c_int)) -> ::string_ref::StringRef```<br>
  /// C++ method: <span style='color: green;'>```QStringRef QString::midRef(int position, int n = ?) const```</span>
  ///
  ///
  pub fn mid_ref<'largs, Args>(&'largs self, args: Args) -> ::string_ref::StringRef
    where Args: overloading::StringMidRefArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::QString```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QString::QString()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::char::Char) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QString::QString(QChar c)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::latin1_string::Latin1String) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QString::QString(QLatin1String latin1)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::byte_array::ByteArray) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QString::QString(const QByteArray& a)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QString::QString(const QString& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::char::Char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QString::QString(int size, QChar c)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::string::String
    where Args: overloading::StringNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QString::QString```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*const ::char::Char) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QString::QString(const QChar* unicode)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::char::Char, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QString::QString(const QChar* unicode, int size = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe(*const ::libc::c_char) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QString::QString(const char* ch)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::string::String
    where Args: overloading::StringNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QString::normalized```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn normalized(&self, ::string::NormalizationForm) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::normalized(QString::NormalizationForm mode) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn normalized(&self, (::string::NormalizationForm, &::char::UnicodeVersion)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::normalized(QString::NormalizationForm mode, QChar::UnicodeVersion version = ?) const```</span>
  ///
  ///
  pub fn normalized<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::StringNormalizedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::number```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn number0(::libc::c_double) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::number(double arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn number0((::libc::c_double, ::libc::c_char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::number(double arg1, char f = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn number0((::libc::c_double, ::libc::c_char, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::number(double arg1, char f = ?, int prec = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn number0(::libc::c_int) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::number(int arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn number0((::libc::c_int, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::number(int arg1, int base = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn number0(u64) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::number(qulonglong arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn number0((u64, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::number(qulonglong arg1, int base = ?)```</span>
  ///
  ///
  pub fn number0<Args>(args: Args) -> ::string::String
    where Args: overloading::StringNumber0Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QString::number```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn number1(::libc::c_long) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::number(long arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn number1((::libc::c_long, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::number(long arg1, int base = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn number1(::libc::c_uint) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::number(unsigned int arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn number1((::libc::c_uint, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::number(unsigned int arg1, int base = ?)```</span>
  ///
  ///
  pub fn number1<Args>(args: Args) -> ::string::String
    where Args: overloading::StringNumber1Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QString::number```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn number2(i64) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::number(qlonglong arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn number2((i64, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::number(qlonglong arg1, int base = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn number2(::libc::c_ulong) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::number(unsigned long arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn number2((::libc::c_ulong, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QString::number(unsigned long arg1, int base = ?)```</span>
  ///
  ///
  pub fn number2<Args>(args: Args) -> ::string::String
    where Args: overloading::StringNumber2Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QString::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::char::Char) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::operator+=(QChar c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::char::SpecialCharacter) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::operator+=(QChar::SpecialCharacter c)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::latin1_string::Latin1String) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::operator+=(QLatin1String s)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, ::libc::c_char) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::operator+=(char c)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::byte_array::ByteArray) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::operator+=(const QByteArray& s)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::string::String) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::operator+=(const QString& s)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::string_ref::StringRef) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::operator+=(const QStringRef& s)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::string::String
    where Args: overloading::StringOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString& QString::operator+=(const char* s)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0>(&'l0 mut self, s: *const ::libc::c_char) -> &'l0 mut ::string::String {
    let ffi_result = ::ffi::qt_core_c_QString_operator_add_assign_char_s(self as *mut ::string::String, s);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString::operator=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::char::Char) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::operator=(QChar c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::latin1_string::Latin1String) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::operator=(QLatin1String latin1)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn op_assign(&mut self, ::libc::c_char) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::operator=(char c)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::byte_array::ByteArray) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::operator=(const QByteArray& a)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::string::String) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::operator=(const QString& arg1)```</span>
  ///
  ///
  pub fn op_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::string::String
    where Args: overloading::StringOpAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString& QString::operator=(const char* ch)```</span>
  ///
  ///
  pub unsafe fn op_assign_unsafe<'l0>(&'l0 mut self, ch: *const ::libc::c_char) -> &'l0 mut ::string::String {
    let ffi_result = ::ffi::qt_core_c_QString_operator_assign_char_ch(self as *mut ::string::String, ch);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString::operator==```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_eq(&self, &::latin1_string::Latin1String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::operator==(QLatin1String s) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_eq(&self, &::byte_array::ByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::operator==(const QByteArray& s) const```</span>
  ///
  ///
  pub fn op_eq<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::StringOpEqArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QString::operator==(const char* s) const```</span>
  ///
  ///
  pub unsafe fn op_eq_unsafe(&self, s: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QString_operator_eq_char(self as *const ::string::String, s)
  }

  /// C++ method: <span style='color: green;'>```QString::operator>=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_ge(&self, &::latin1_string::Latin1String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::operator>=(QLatin1String s) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_ge(&self, &::byte_array::ByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::operator>=(const QByteArray& s) const```</span>
  ///
  ///
  pub fn op_ge<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::StringOpGeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QString::operator>=(const char* s) const```</span>
  ///
  ///
  pub unsafe fn op_ge_unsafe(&self, s: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QString_operator_ge_char(self as *const ::string::String, s)
  }

  /// C++ method: <span style='color: green;'>```QString::operator>```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_gt(&self, &::latin1_string::Latin1String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::operator>(QLatin1String s) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_gt(&self, &::byte_array::ByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::operator>(const QByteArray& s) const```</span>
  ///
  ///
  pub fn op_gt<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::StringOpGtArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QString::operator>(const char* s) const```</span>
  ///
  ///
  pub unsafe fn op_gt_unsafe(&self, s: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QString_operator_gt_char(self as *const ::string::String, s)
  }

  /// C++ method: <span style='color: green;'>```QString::operator[]```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_index(&self, ::libc::c_int) -> ::char::Char```<br>
  /// C++ method: <span style='color: green;'>```const QChar QString::operator[](int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_index(&self, ::libc::c_uint) -> ::char::Char```<br>
  /// C++ method: <span style='color: green;'>```const QChar QString::operator[](unsigned int i) const```</span>
  ///
  ///
  pub fn op_index<'largs, Args>(&'largs self, args: Args) -> ::char::Char
    where Args: overloading::StringOpIndexArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::operator[]```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_index_mut(&mut self, ::libc::c_int) -> ::char_ref::CharRef```<br>
  /// C++ method: <span style='color: green;'>```QCharRef QString::operator[](int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_index_mut(&mut self, ::libc::c_uint) -> ::char_ref::CharRef```<br>
  /// C++ method: <span style='color: green;'>```QCharRef QString::operator[](unsigned int i)```</span>
  ///
  ///
  pub fn op_index_mut<'largs, Args>(&'largs mut self, args: Args) -> ::char_ref::CharRef
    where Args: overloading::StringOpIndexMutArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::operator<=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_le(&self, &::latin1_string::Latin1String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::operator<=(QLatin1String s) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_le(&self, &::byte_array::ByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::operator<=(const QByteArray& s) const```</span>
  ///
  ///
  pub fn op_le<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::StringOpLeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QString::operator<=(const char* s) const```</span>
  ///
  ///
  pub unsafe fn op_le_unsafe(&self, s: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QString_operator_le_char(self as *const ::string::String, s)
  }

  /// C++ method: <span style='color: green;'>```QString::operator<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_lt(&self, &::latin1_string::Latin1String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::operator<(QLatin1String s) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_lt(&self, &::byte_array::ByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::operator<(const QByteArray& s) const```</span>
  ///
  ///
  pub fn op_lt<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::StringOpLtArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QString::operator<(const char* s) const```</span>
  ///
  ///
  pub unsafe fn op_lt_unsafe(&self, s: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QString_operator_lt_char(self as *const ::string::String, s)
  }

  /// C++ method: <span style='color: green;'>```QString::operator!=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_neq(&self, &::latin1_string::Latin1String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::operator!=(QLatin1String s) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_neq(&self, &::byte_array::ByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::operator!=(const QByteArray& s) const```</span>
  ///
  ///
  pub fn op_neq<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::StringOpNeqArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QString::operator!=(const char* s) const```</span>
  ///
  ///
  pub unsafe fn op_neq_unsafe(&self, s: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QString_operator_neq_char(self as *const ::string::String, s)
  }

  /// C++ method: <span style='color: green;'>```QString::prepend```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn prepend(&mut self, &'l1 ::char::Char) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::prepend(QChar c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn prepend(&mut self, &'l1 ::latin1_string::Latin1String) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::prepend(QLatin1String s)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn prepend(&mut self, &'l1 ::byte_array::ByteArray) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::prepend(const QByteArray& s)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn prepend(&mut self, &'l1 ::string::String) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::prepend(const QString& s)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn prepend(&mut self, &'l1 ::string_ref::StringRef) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::prepend(const QStringRef& s)```</span>
  ///
  ///
  pub fn prepend<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::string::String
    where Args: overloading::StringPrependArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::prepend```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn prepend_unsafe(&mut self, (*const ::char::Char, ::libc::c_int)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::prepend(const QChar* uc, int len)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn prepend_unsafe(&mut self, *const ::libc::c_char) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::prepend(const char* s)```</span>
  ///
  ///
  pub unsafe fn prepend_unsafe<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::string::String
    where Args: overloading::StringPrependUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::push_back```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn push_back(&mut self, &::char::Char) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QString::push_back(QChar c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn push_back(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QString::push_back(const QString& s)```</span>
  ///
  ///
  pub fn push_back<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::StringPushBackArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::push_front```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn push_front(&mut self, &::char::Char) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QString::push_front(QChar c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn push_front(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QString::push_front(const QString& s)```</span>
  ///
  ///
  pub fn push_front<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::StringPushFrontArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, &'l1 ::char::Char) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::remove(QChar c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (&'l1 ::char::Char, &'l2 ::qt::CaseSensitivity)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::remove(QChar c, Qt::CaseSensitivity cs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn remove(&mut self, &'l1 ::reg_exp::RegExp) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::remove(const QRegExp& rx)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn remove(&mut self, &'l1 ::regular_expression::RegularExpression) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::remove(const QRegularExpression& re)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn remove(&mut self, &'l1 ::string::String) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::remove(const QString& s)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn remove(&mut self, (&'l1 ::string::String, &'l2 ::qt::CaseSensitivity)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::remove(const QString& s, Qt::CaseSensitivity cs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::remove(int i, int len)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::string::String
    where Args: overloading::StringRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString QString::repeated(int times) const```</span>
  ///
  ///
  pub fn repeated(&self, times: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_repeated_to_output(self as *const ::string::String, times, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString::replace```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn replace(&mut self, (&'l1 ::char::Char, &'l2 ::char::Char)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(QChar before, QChar after)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn replace(&mut self, (&'l1 ::char::Char, &'l2 ::char::Char, &'l3 ::qt::CaseSensitivity)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(QChar before, QChar after, Qt::CaseSensitivity cs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn replace(&mut self, (&'l1 ::char::Char, &'l2 ::latin1_string::Latin1String)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(QChar c, QLatin1String after)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn replace(&mut self, (&'l1 ::char::Char, &'l2 ::latin1_string::Latin1String, &'l3 ::qt::CaseSensitivity)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(QChar c, QLatin1String after, Qt::CaseSensitivity cs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn replace(&mut self, (&'l1 ::char::Char, &'l2 ::string::String)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(QChar c, const QString& after)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn replace(&mut self, (&'l1 ::char::Char, &'l2 ::string::String, &'l3 ::qt::CaseSensitivity)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(QChar c, const QString& after, Qt::CaseSensitivity cs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn replace(&mut self, (&'l1 ::latin1_string::Latin1String, &'l2 ::latin1_string::Latin1String)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(QLatin1String before, QLatin1String after)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn replace(&mut self, (&'l1 ::latin1_string::Latin1String, &'l2 ::latin1_string::Latin1String, &'l3 ::qt::CaseSensitivity)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(QLatin1String before, QLatin1String after, Qt::CaseSensitivity cs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn replace(&mut self, (&'l1 ::latin1_string::Latin1String, &'l2 ::string::String)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(QLatin1String before, const QString& after)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn replace(&mut self, (&'l1 ::latin1_string::Latin1String, &'l2 ::string::String, &'l3 ::qt::CaseSensitivity)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(QLatin1String before, const QString& after, Qt::CaseSensitivity cs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn replace(&mut self, (&'l1 ::reg_exp::RegExp, &'l2 ::string::String)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(const QRegExp& rx, const QString& after)```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn replace(&mut self, (&'l1 ::regular_expression::RegularExpression, &'l2 ::string::String)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(const QRegularExpression& re, const QString& after)```</span>
  ///
  ///
  ///
  /// ## Variant 13
  ///
  /// Rust arguments: ```fn replace(&mut self, (&'l1 ::string::String, &'l2 ::latin1_string::Latin1String)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(const QString& before, QLatin1String after)```</span>
  ///
  ///
  ///
  /// ## Variant 14
  ///
  /// Rust arguments: ```fn replace(&mut self, (&'l1 ::string::String, &'l2 ::latin1_string::Latin1String, &'l3 ::qt::CaseSensitivity)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(const QString& before, QLatin1String after, Qt::CaseSensitivity cs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 15
  ///
  /// Rust arguments: ```fn replace(&mut self, (&'l1 ::string::String, &'l2 ::string::String)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(const QString& before, const QString& after)```</span>
  ///
  ///
  ///
  /// ## Variant 16
  ///
  /// Rust arguments: ```fn replace(&mut self, (&'l1 ::string::String, &'l2 ::string::String, &'l3 ::qt::CaseSensitivity)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(const QString& before, const QString& after, Qt::CaseSensitivity cs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 17
  ///
  /// Rust arguments: ```fn replace(&mut self, (::libc::c_int, ::libc::c_int, &'l1 ::char::Char)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(int i, int len, QChar after)```</span>
  ///
  ///
  ///
  /// ## Variant 18
  ///
  /// Rust arguments: ```fn replace(&mut self, (::libc::c_int, ::libc::c_int, &'l1 ::string::String)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(int i, int len, const QString& after)```</span>
  ///
  ///
  pub fn replace<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::string::String
    where Args: overloading::StringReplaceArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::replace```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn replace_unsafe(&mut self, (*const ::char::Char, ::libc::c_int, *const ::char::Char, ::libc::c_int)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(const QChar* before, int blen, const QChar* after, int alen)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn replace_unsafe(&mut self, (*const ::char::Char, ::libc::c_int, *const ::char::Char, ::libc::c_int, &'l1 ::qt::CaseSensitivity)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(const QChar* before, int blen, const QChar* after, int alen, Qt::CaseSensitivity cs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn replace_unsafe(&mut self, (::libc::c_int, ::libc::c_int, *const ::char::Char, ::libc::c_int)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::replace(int i, int len, const QChar* s, int slen)```</span>
  ///
  ///
  pub unsafe fn replace_unsafe<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::string::String
    where Args: overloading::StringReplaceUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QString::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QString_reserve(self as *mut ::string::String, size) }
  }

  /// C++ method: <span style='color: green;'>```QString::resize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn resize(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QString::resize(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn resize(&mut self, (::libc::c_int, &::char::Char)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QString::resize(int size, QChar fillChar)```</span>
  ///
  ///
  pub fn resize<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::StringResizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString QString::right(int n) const```</span>
  ///
  ///
  pub fn right(&self, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_right_to_output(self as *const ::string::String, n, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString::rightJustified```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn right_justified(&self, ::libc::c_int) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::rightJustified(int width) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn right_justified(&self, (::libc::c_int, &::char::Char)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::rightJustified(int width, QChar fill = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn right_justified(&self, (::libc::c_int, &::char::Char, bool)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::rightJustified(int width, QChar fill = ?, bool trunc = ?) const```</span>
  ///
  ///
  pub fn right_justified<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::StringRightJustifiedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringRef QString::rightRef(int n) const```</span>
  ///
  ///
  pub fn right_ref(&self, n: ::libc::c_int) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_rightRef_to_output(self as *const ::string::String, n, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString::section```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn section(&self, (&::char::Char, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::section(QChar sep, int start) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn section(&self, (&::char::Char, ::libc::c_int, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::section(QChar sep, int start, int end = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn section(&self, (&::char::Char, ::libc::c_int, ::libc::c_int, ::flags::Flags<::string::SectionFlag>)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::section(QChar sep, int start, int end = ?, QFlags<QString::SectionFlag> flags = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn section(&self, (&::reg_exp::RegExp, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::section(const QRegExp& reg, int start) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn section(&self, (&::reg_exp::RegExp, ::libc::c_int, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::section(const QRegExp& reg, int start, int end = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn section(&self, (&::reg_exp::RegExp, ::libc::c_int, ::libc::c_int, ::flags::Flags<::string::SectionFlag>)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::section(const QRegExp& reg, int start, int end = ?, QFlags<QString::SectionFlag> flags = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn section(&self, (&::regular_expression::RegularExpression, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::section(const QRegularExpression& re, int start) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn section(&self, (&::regular_expression::RegularExpression, ::libc::c_int, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::section(const QRegularExpression& re, int start, int end = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn section(&self, (&::regular_expression::RegularExpression, ::libc::c_int, ::libc::c_int, ::flags::Flags<::string::SectionFlag>)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::section(const QRegularExpression& re, int start, int end = ?, QFlags<QString::SectionFlag> flags = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn section(&self, (&::string::String, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::section(const QString& in_sep, int start) const```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn section(&self, (&::string::String, ::libc::c_int, ::libc::c_int)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::section(const QString& in_sep, int start, int end = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn section(&self, (&::string::String, ::libc::c_int, ::libc::c_int, ::flags::Flags<::string::SectionFlag>)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QString::section(const QString& in_sep, int start, int end = ?, QFlags<QString::SectionFlag> flags = ?) const```</span>
  ///
  ///
  pub fn section<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::StringSectionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::setNum```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_num0(&mut self, ::libc::c_double) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(double arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_num0(&mut self, (::libc::c_double, ::libc::c_char)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(double arg1, char f = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_num0(&mut self, (::libc::c_double, ::libc::c_char, ::libc::c_int)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(double arg1, char f = ?, int prec = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn set_num0(&mut self, ::libc::c_int) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(int arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn set_num0(&mut self, (::libc::c_int, ::libc::c_int)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(int arg1, int base = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn set_num0(&mut self, u64) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(qulonglong arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn set_num0(&mut self, (u64, ::libc::c_int)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(qulonglong arg1, int base = ?)```</span>
  ///
  ///
  pub fn set_num0<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::string::String
    where Args: overloading::StringSetNum0Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::setNum```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_num1(&mut self, ::libc::c_float) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(float arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_num1(&mut self, (::libc::c_float, ::libc::c_char)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(float arg1, char f = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_num1(&mut self, (::libc::c_float, ::libc::c_char, ::libc::c_int)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(float arg1, char f = ?, int prec = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn set_num1(&mut self, ::libc::c_long) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(long arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn set_num1(&mut self, (::libc::c_long, ::libc::c_int)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(long arg1, int base = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn set_num1(&mut self, ::libc::c_uint) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(unsigned int arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn set_num1(&mut self, (::libc::c_uint, ::libc::c_int)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(unsigned int arg1, int base = ?)```</span>
  ///
  ///
  pub fn set_num1<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::string::String
    where Args: overloading::StringSetNum1Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::setNum```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_num2(&mut self, i64) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(qlonglong arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_num2(&mut self, (i64, ::libc::c_int)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(qlonglong arg1, int base = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_num2(&mut self, ::libc::c_ulong) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(unsigned long arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn set_num2(&mut self, (::libc::c_ulong, ::libc::c_int)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(unsigned long arg1, int base = ?)```</span>
  ///
  ///
  pub fn set_num2<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::string::String
    where Args: overloading::StringSetNum2Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::setNum```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_num3(&mut self, ::libc::c_short) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(short arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_num3(&mut self, (::libc::c_short, ::libc::c_int)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(short arg1, int base = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_num3(&mut self, ::libc::c_ushort) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(unsigned short arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn set_num3(&mut self, (::libc::c_ushort, ::libc::c_int)) -> &'l0 mut ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString& QString::setNum(unsigned short arg1, int base = ?)```</span>
  ///
  ///
  pub fn set_num3<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::string::String
    where Args: overloading::StringSetNum3Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString& QString::setRawData(const QChar* unicode, int size)```</span>
  ///
  ///
  pub unsafe fn set_raw_data<'l0>(&'l0 mut self,
                                  unicode: *const ::char::Char,
                                  size: ::libc::c_int)
                                  -> &'l0 mut ::string::String {
    let ffi_result = ::ffi::qt_core_c_QString_setRawData(self as *mut ::string::String, unicode, size);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QString::setUnicode(const QChar* unicode, int size)```</span>
  ///
  ///
  pub unsafe fn set_unicode<'l0>(&'l0 mut self,
                                 unicode: *const ::char::Char,
                                 size: ::libc::c_int)
                                 -> &'l0 mut ::string::String {
    let ffi_result = ::ffi::qt_core_c_QString_setUnicode(self as *mut ::string::String, unicode, size);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QString::setUtf16(const unsigned short* utf16, int size)```</span>
  ///
  ///
  pub unsafe fn set_utf16<'l0>(&'l0 mut self,
                               utf16: *const ::libc::c_ushort,
                               size: ::libc::c_int)
                               -> &'l0 mut ::string::String {
    let ffi_result = ::ffi::qt_core_c_QString_setUtf16(self as *mut ::string::String, utf16, size);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString QString::simplified() const```</span>
  ///
  ///
  pub fn simplified(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_simplified_to_output_const(self as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QString::simplified()```</span>
  ///
  ///
  pub fn simplified_mut(&mut self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_simplified_to_output(self as *mut ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QString::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QString_size(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QString::split```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn split(&self, &::char::Char) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QString::split(QChar sep) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn split(&self, (&::char::Char, ::string::SplitBehavior)) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QString::split(QChar sep, QString::SplitBehavior behavior = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn split(&self, (&::char::Char, ::string::SplitBehavior, &::qt::CaseSensitivity)) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QString::split(QChar sep, QString::SplitBehavior behavior = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn split(&self, &::reg_exp::RegExp) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QString::split(const QRegExp& sep) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn split(&self, (&::reg_exp::RegExp, ::string::SplitBehavior)) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QString::split(const QRegExp& sep, QString::SplitBehavior behavior = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn split(&self, &::regular_expression::RegularExpression) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QString::split(const QRegularExpression& sep) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn split(&self, (&::regular_expression::RegularExpression, ::string::SplitBehavior)) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QString::split(const QRegularExpression& sep, QString::SplitBehavior behavior = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn split(&self, &::string::String) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QString::split(const QString& sep) const```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn split(&self, (&::string::String, ::string::SplitBehavior)) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QString::split(const QString& sep, QString::SplitBehavior behavior = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn split(&self, (&::string::String, ::string::SplitBehavior, &::qt::CaseSensitivity)) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QString::split(const QString& sep, QString::SplitBehavior behavior = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  pub fn split<'largs, Args>(&'largs self, args: Args) -> ::string_list::StringList
    where Args: overloading::StringSplitArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::splitRef```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn split_ref(&self, &::char::Char) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QString::splitRef(QChar sep) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn split_ref(&self, (&::char::Char, ::string::SplitBehavior)) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QString::splitRef(QChar sep, QString::SplitBehavior behavior = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn split_ref(&self, (&::char::Char, ::string::SplitBehavior, &::qt::CaseSensitivity)) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QString::splitRef(QChar sep, QString::SplitBehavior behavior = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn split_ref(&self, &::reg_exp::RegExp) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QString::splitRef(const QRegExp& sep) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn split_ref(&self, (&::reg_exp::RegExp, ::string::SplitBehavior)) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QString::splitRef(const QRegExp& sep, QString::SplitBehavior behavior = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn split_ref(&self, &::regular_expression::RegularExpression) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QString::splitRef(const QRegularExpression& sep) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn split_ref(&self, (&::regular_expression::RegularExpression, ::string::SplitBehavior)) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QString::splitRef(const QRegularExpression& sep, QString::SplitBehavior behavior = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn split_ref(&self, &::string::String) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QString::splitRef(const QString& sep) const```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn split_ref(&self, (&::string::String, ::string::SplitBehavior)) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QString::splitRef(const QString& sep, QString::SplitBehavior behavior = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn split_ref(&self, (&::string::String, ::string::SplitBehavior, &::qt::CaseSensitivity)) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QString::splitRef(const QString& sep, QString::SplitBehavior behavior = ?, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  pub fn split_ref<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorStringRef
    where Args: overloading::StringSplitRefArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QString::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_core_c_QString_squeeze(self as *mut ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QString::startsWith```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn starts_with(&self, &::char::Char) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::startsWith(QChar c) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn starts_with(&self, (&::char::Char, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::startsWith(QChar c, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn starts_with(&self, &::latin1_string::Latin1String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::startsWith(QLatin1String s) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn starts_with(&self, (&::latin1_string::Latin1String, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::startsWith(QLatin1String s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn starts_with(&self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::startsWith(const QString& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn starts_with(&self, (&::string::String, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::startsWith(const QString& s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn starts_with(&self, &::string_ref::StringRef) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::startsWith(const QStringRef& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn starts_with(&self, (&::string_ref::StringRef, &::qt::CaseSensitivity)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QString::startsWith(const QStringRef& s, Qt::CaseSensitivity cs = ?) const```</span>
  ///
  ///
  pub fn starts_with<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::StringStartsWithArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QString::swap(QString& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::string::String) {
    unsafe {
      ::ffi::qt_core_c_QString_swap(self as *mut ::string::String,
                                    other as *mut ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QString QString::toCaseFolded() const```</span>
  ///
  ///
  pub fn to_case_folded(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_toCaseFolded_to_output_const(self as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QString::toCaseFolded()```</span>
  ///
  ///
  pub fn to_case_folded_mut(&mut self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_toCaseFolded_to_output(self as *mut ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QString::toDouble() const```</span>
  ///
  ///
  pub fn to_double(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QString_toDouble_no_args(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```double QString::toDouble(bool* ok = ?) const```</span>
  ///
  ///
  pub unsafe fn to_double_unsafe(&self, ok: *mut bool) -> ::libc::c_double {
    ::ffi::qt_core_c_QString_toDouble_ok(self as *const ::string::String, ok)
  }

  /// C++ method: <span style='color: green;'>```float QString::toFloat() const```</span>
  ///
  ///
  pub fn to_float(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_core_c_QString_toFloat_no_args(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```float QString::toFloat(bool* ok = ?) const```</span>
  ///
  ///
  pub unsafe fn to_float_unsafe(&self, ok: *mut bool) -> ::libc::c_float {
    ::ffi::qt_core_c_QString_toFloat_ok(self as *const ::string::String, ok)
  }

  /// C++ method: <span style='color: green;'>```QString QString::toHtmlEscaped() const```</span>
  ///
  ///
  pub fn to_html_escaped(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_toHtmlEscaped_to_output(self as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QString::toInt() const```</span>
  ///
  ///
  pub fn to_int(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QString_toInt_no_args(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QString::toInt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_int_unsafe(&self, *mut bool) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::toInt(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_int_unsafe(&self, (*mut bool, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QString::toInt(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_int_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StringToIntUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray QString::toLatin1() const```</span>
  ///
  ///
  pub fn to_latin1(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_toLatin1_to_output(self as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QString::toLocal8Bit() const```</span>
  ///
  ///
  pub fn to_local8_bit(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_toLocal8Bit_to_output(self as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```long QString::toLong() const```</span>
  ///
  ///
  pub fn to_long(&self) -> ::libc::c_long {
    unsafe { ::ffi::qt_core_c_QString_toLong_no_args(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```qlonglong QString::toLongLong() const```</span>
  ///
  ///
  pub fn to_long_long(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QString_toLongLong_no_args(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QString::toLongLong```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_long_long_unsafe(&self, *mut bool) -> i64```<br>
  /// C++ method: <span style='color: green;'>```qlonglong QString::toLongLong(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_long_long_unsafe(&self, (*mut bool, ::libc::c_int)) -> i64```<br>
  /// C++ method: <span style='color: green;'>```qlonglong QString::toLongLong(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_long_long_unsafe<'largs, Args>(&'largs self, args: Args) -> i64
    where Args: overloading::StringToLongLongUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::toLong```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_long_unsafe(&self, *mut bool) -> ::libc::c_long```<br>
  /// C++ method: <span style='color: green;'>```long QString::toLong(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_long_unsafe(&self, (*mut bool, ::libc::c_int)) -> ::libc::c_long```<br>
  /// C++ method: <span style='color: green;'>```long QString::toLong(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_long_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_long
    where Args: overloading::StringToLongUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString QString::toLower() const```</span>
  ///
  ///
  pub fn to_lower(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_toLower_to_output_const(self as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QString::toLower()```</span>
  ///
  ///
  pub fn to_lower_mut(&mut self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_toLower_to_output(self as *mut ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```short QString::toShort() const```</span>
  ///
  ///
  pub fn to_short(&self) -> ::libc::c_short {
    unsafe { ::ffi::qt_core_c_QString_toShort_no_args(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QString::toShort```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_short_unsafe(&self, *mut bool) -> ::libc::c_short```<br>
  /// C++ method: <span style='color: green;'>```short QString::toShort(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_short_unsafe(&self, (*mut bool, ::libc::c_int)) -> ::libc::c_short```<br>
  /// C++ method: <span style='color: green;'>```short QString::toShort(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_short_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_short
    where Args: overloading::StringToShortUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```unsigned int QString::toUInt() const```</span>
  ///
  ///
  pub fn to_u_int(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QString_toUInt_no_args(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QString::toUInt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_u_int_unsafe(&self, *mut bool) -> ::libc::c_uint```<br>
  /// C++ method: <span style='color: green;'>```unsigned int QString::toUInt(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_u_int_unsafe(&self, (*mut bool, ::libc::c_int)) -> ::libc::c_uint```<br>
  /// C++ method: <span style='color: green;'>```unsigned int QString::toUInt(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_u_int_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_uint
    where Args: overloading::StringToUIntUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```unsigned long QString::toULong() const```</span>
  ///
  ///
  pub fn to_u_long(&self) -> ::libc::c_ulong {
    unsafe { ::ffi::qt_core_c_QString_toULong_no_args(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```qulonglong QString::toULongLong() const```</span>
  ///
  ///
  pub fn to_u_long_long(&self) -> u64 {
    unsafe { ::ffi::qt_core_c_QString_toULongLong_no_args(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QString::toULongLong```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_u_long_long_unsafe(&self, *mut bool) -> u64```<br>
  /// C++ method: <span style='color: green;'>```qulonglong QString::toULongLong(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_u_long_long_unsafe(&self, (*mut bool, ::libc::c_int)) -> u64```<br>
  /// C++ method: <span style='color: green;'>```qulonglong QString::toULongLong(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_u_long_long_unsafe<'largs, Args>(&'largs self, args: Args) -> u64
    where Args: overloading::StringToULongLongUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString::toULong```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_u_long_unsafe(&self, *mut bool) -> ::libc::c_ulong```<br>
  /// C++ method: <span style='color: green;'>```unsigned long QString::toULong(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_u_long_unsafe(&self, (*mut bool, ::libc::c_int)) -> ::libc::c_ulong```<br>
  /// C++ method: <span style='color: green;'>```unsigned long QString::toULong(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_u_long_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_ulong
    where Args: overloading::StringToULongUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```unsigned short QString::toUShort() const```</span>
  ///
  ///
  pub fn to_u_short(&self) -> ::libc::c_ushort {
    unsafe { ::ffi::qt_core_c_QString_toUShort_no_args(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QString::toUShort```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_u_short_unsafe(&self, *mut bool) -> ::libc::c_ushort```<br>
  /// C++ method: <span style='color: green;'>```unsigned short QString::toUShort(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_u_short_unsafe(&self, (*mut bool, ::libc::c_int)) -> ::libc::c_ushort```<br>
  /// C++ method: <span style='color: green;'>```unsigned short QString::toUShort(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_u_short_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_ushort
    where Args: overloading::StringToUShortUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<unsigned int> QString::toUcs4() const```</span>
  ///
  ///
  pub fn to_ucs4(&self) -> ::vector::VectorCUint {
    {
      let mut object: ::vector::VectorCUint =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_toUcs4_to_output(self as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QString::toUpper() const```</span>
  ///
  ///
  pub fn to_upper(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_toUpper_to_output_const(self as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QString::toUpper()```</span>
  ///
  ///
  pub fn to_upper_mut(&mut self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_toUpper_to_output(self as *mut ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QString::toUtf8() const```</span>
  ///
  ///
  pub fn to_utf8(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_toUtf8_to_output(self as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QString::toWCharArray(wchar_t* array) const```</span>
  ///
  ///
  pub unsafe fn to_w_char_array(&self, array: *mut ::libc::wchar_t) -> ::libc::c_int {
    ::ffi::qt_core_c_QString_toWCharArray(self as *const ::string::String, array)
  }

  /// C++ method: <span style='color: green;'>```QString QString::trimmed() const```</span>
  ///
  ///
  pub fn trimmed(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_trimmed_to_output_const(self as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QString::trimmed()```</span>
  ///
  ///
  pub fn trimmed_mut(&mut self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QString_trimmed_to_output(self as *mut ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QString::truncate(int pos)```</span>
  ///
  ///
  pub fn truncate(&mut self, pos: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QString_truncate(self as *mut ::string::String, pos) }
  }

  /// C++ method: <span style='color: green;'>```const QChar* QString::unicode() const```</span>
  ///
  ///
  pub fn unicode(&self) -> *const ::char::Char {
    unsafe { ::ffi::qt_core_c_QString_unicode(self as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```const unsigned short* QString::utf16() const```</span>
  ///
  ///
  pub fn utf16(&self) -> *const ::libc::c_ushort {
    unsafe { ::ffi::qt_core_c_QString_utf16(self as *const ::string::String) }
  }
}

impl Drop for ::string::String {
  /// C++ method: <span style='color: green;'>```[destructor] void QString::~QString()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QString_destructor(self as *mut ::string::String) }
  }
}

/// C++ method: <span style='color: green;'>```operator+```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_add((&::char::Char, &::string_ref::StringRef)) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```QString operator+(QChar s1, const QStringRef& s2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_add((&::latin1_string::Latin1String, &::string_ref::StringRef)) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```QString operator+(QLatin1String s1, const QStringRef& s2)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_add((&::string::String, &::string_ref::StringRef)) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```QString operator+(const QString& s1, const QStringRef& s2)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_add((&::string_ref::StringRef, &::char::Char)) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```QString operator+(const QStringRef& s1, QChar s2)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn op_add((&::string_ref::StringRef, &::latin1_string::Latin1String)) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```QString operator+(const QStringRef& s1, QLatin1String s2)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn op_add((&::string_ref::StringRef, &::string::String)) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```QString operator+(const QStringRef& s1, const QString& s2)```</span>
///
///
///
/// ## Variant 7
///
/// Rust arguments: ```fn op_add((&::string_ref::StringRef, &::string_ref::StringRef)) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```QString operator+(const QStringRef& s1, const QStringRef& s2)```</span>
///
///
///
/// ## Variant 8
///
/// Rust arguments: ```fn op_add((&::char::Char, &::string::String)) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```const QString operator+(QChar s1, const QString& s2)```</span>
///
///
///
/// ## Variant 9
///
/// Rust arguments: ```fn op_add((::libc::c_char, &::string::String)) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```const QString operator+(char c, const QString& s)```</span>
///
///
///
/// ## Variant 10
///
/// Rust arguments: ```fn op_add((&::byte_array::ByteArray, &::string::String)) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```const QString operator+(const QByteArray& ba, const QString& s)```</span>
///
///
///
/// ## Variant 11
///
/// Rust arguments: ```fn op_add((&::string::String, ::libc::c_char)) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```const QString operator+(const QString& s, char c)```</span>
///
///
///
/// ## Variant 12
///
/// Rust arguments: ```fn op_add((&::string::String, &::byte_array::ByteArray)) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```const QString operator+(const QString& s, const QByteArray& ba)```</span>
///
///
///
/// ## Variant 13
///
/// Rust arguments: ```fn op_add((&::string::String, &::char::Char)) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```const QString operator+(const QString& s1, QChar s2)```</span>
///
///
///
/// ## Variant 14
///
/// Rust arguments: ```fn op_add((&::string::String, &::string::String)) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```const QString operator+(const QString& s1, const QString& s2)```</span>
///
///
pub fn op_add<Args>(args: Args) -> ::string::String
  where Args: overloading::OpAddArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator+```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_add_unsafe((&::string::String, *const ::libc::c_char)) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```const QString operator+(const QString& s1, const char* s2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_add_unsafe((*const ::libc::c_char, &::string::String)) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```const QString operator+(const char* s1, const QString& s2)```</span>
///
///
pub unsafe fn op_add_unsafe<Args>(args: Args) -> ::string::String
  where Args: overloading::OpAddUnsafeArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator==```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_eq((&::latin1_string::Latin1String, &::char::Char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator==(QLatin1String lhs, QChar rhs)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_eq((&::latin1_string::Latin1String, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator==(QLatin1String lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_eq((&::latin1_string::Latin1String, &::latin1_string::Latin1String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator==(QLatin1String s1, QLatin1String s2)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_eq((&::byte_array::ByteArray, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator==(const QByteArray& lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn op_eq((&::string::String, &::char::Char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator==(const QString& lhs, QChar rhs)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn op_eq((&::string::String, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator==(const QString& lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 7
///
/// Rust arguments: ```fn op_eq((&::string_ref::StringRef, &::char::Char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator==(const QStringRef& lhs, QChar rhs)```</span>
///
///
///
/// ## Variant 8
///
/// Rust arguments: ```fn op_eq((&::string_ref::StringRef, &::latin1_string::Latin1String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator==(const QStringRef& lhs, QLatin1String rhs)```</span>
///
///
///
/// ## Variant 9
///
/// Rust arguments: ```fn op_eq((&::string_ref::StringRef, &::byte_array::ByteArray)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator==(const QStringRef& lhs, const QByteArray& rhs)```</span>
///
///
///
/// ## Variant 10
///
/// Rust arguments: ```fn op_eq((&::string_ref::StringRef, &::string::String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator==(const QStringRef& lhs, const QString& rhs)```</span>
///
///
///
/// ## Variant 11
///
/// Rust arguments: ```fn op_eq((&::string_ref::StringRef, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator==(const QStringRef& s1, const QStringRef& s2)```</span>
///
///
pub fn op_eq<Args>(args: Args) -> bool
  where Args: overloading::OpEqArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator==(const char* s1, QLatin1String s2)```</span>
///
///
pub unsafe fn op_eq_unsafe(s1: *const ::libc::c_char, s2: &::latin1_string::Latin1String) -> bool {
  ::ffi::qt_core_c_QString_G_operator_eq_char_QLatin1String(s1, s2 as *const ::latin1_string::Latin1String)
}

/// C++ method: <span style='color: green;'>```operator>=```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_ge((&::char::Char, &::latin1_string::Latin1String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>=(QChar lhs, QLatin1String rhs)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_ge((&::char::Char, &::string::String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>=(QChar lhs, const QString& rhs)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_ge((&::char::Char, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>=(QChar lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_ge((&::latin1_string::Latin1String, &::char::Char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>=(QLatin1String lhs, QChar rhs)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn op_ge((&::latin1_string::Latin1String, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>=(QLatin1String lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn op_ge((&::latin1_string::Latin1String, &::latin1_string::Latin1String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>=(QLatin1String s1, QLatin1String s2)```</span>
///
///
///
/// ## Variant 7
///
/// Rust arguments: ```fn op_ge((&::byte_array::ByteArray, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>=(const QByteArray& lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 8
///
/// Rust arguments: ```fn op_ge((&::string::String, &::char::Char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>=(const QString& lhs, QChar rhs)```</span>
///
///
///
/// ## Variant 9
///
/// Rust arguments: ```fn op_ge((&::string::String, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>=(const QString& lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 10
///
/// Rust arguments: ```fn op_ge((&::string_ref::StringRef, &::char::Char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>=(const QStringRef& lhs, QChar rhs)```</span>
///
///
///
/// ## Variant 11
///
/// Rust arguments: ```fn op_ge((&::string_ref::StringRef, &::latin1_string::Latin1String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>=(const QStringRef& lhs, QLatin1String rhs)```</span>
///
///
///
/// ## Variant 12
///
/// Rust arguments: ```fn op_ge((&::string_ref::StringRef, &::byte_array::ByteArray)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>=(const QStringRef& lhs, const QByteArray& rhs)```</span>
///
///
///
/// ## Variant 13
///
/// Rust arguments: ```fn op_ge((&::string_ref::StringRef, &::string::String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>=(const QStringRef& lhs, const QString& rhs)```</span>
///
///
///
/// ## Variant 14
///
/// Rust arguments: ```fn op_ge((&::string_ref::StringRef, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>=(const QStringRef& s1, const QStringRef& s2)```</span>
///
///
pub fn op_ge<Args>(args: Args) -> bool
  where Args: overloading::OpGeArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator>=(const char* s1, QLatin1String s2)```</span>
///
///
pub unsafe fn op_ge_unsafe(s1: *const ::libc::c_char, s2: &::latin1_string::Latin1String) -> bool {
  ::ffi::qt_core_c_QString_G_operator_ge_char_QLatin1String(s1, s2 as *const ::latin1_string::Latin1String)
}

/// C++ method: <span style='color: green;'>```operator>```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_gt((&::latin1_string::Latin1String, &::char::Char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>(QLatin1String lhs, QChar rhs)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_gt((&::latin1_string::Latin1String, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>(QLatin1String lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_gt((&::latin1_string::Latin1String, &::latin1_string::Latin1String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>(QLatin1String s1, QLatin1String s2)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_gt((&::byte_array::ByteArray, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>(const QByteArray& lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn op_gt((&::string::String, &::char::Char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>(const QString& lhs, QChar rhs)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn op_gt((&::string::String, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>(const QString& lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 7
///
/// Rust arguments: ```fn op_gt((&::string_ref::StringRef, &::char::Char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>(const QStringRef& lhs, QChar rhs)```</span>
///
///
///
/// ## Variant 8
///
/// Rust arguments: ```fn op_gt((&::string_ref::StringRef, &::latin1_string::Latin1String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>(const QStringRef& lhs, QLatin1String rhs)```</span>
///
///
///
/// ## Variant 9
///
/// Rust arguments: ```fn op_gt((&::string_ref::StringRef, &::byte_array::ByteArray)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>(const QStringRef& lhs, const QByteArray& rhs)```</span>
///
///
///
/// ## Variant 10
///
/// Rust arguments: ```fn op_gt((&::string_ref::StringRef, &::string::String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>(const QStringRef& lhs, const QString& rhs)```</span>
///
///
///
/// ## Variant 11
///
/// Rust arguments: ```fn op_gt((&::string_ref::StringRef, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>(const QStringRef& s1, const QStringRef& s2)```</span>
///
///
pub fn op_gt<Args>(args: Args) -> bool
  where Args: overloading::OpGtArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator>(const char* s1, QLatin1String s2)```</span>
///
///
pub unsafe fn op_gt_unsafe(s1: *const ::libc::c_char, s2: &::latin1_string::Latin1String) -> bool {
  ::ffi::qt_core_c_QString_G_operator_gt_char_QLatin1String(s1, s2 as *const ::latin1_string::Latin1String)
}

/// C++ method: <span style='color: green;'>```operator<=```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_le((&::char::Char, &::latin1_string::Latin1String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<=(QChar lhs, QLatin1String rhs)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_le((&::char::Char, &::string::String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<=(QChar lhs, const QString& rhs)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_le((&::char::Char, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<=(QChar lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_le((&::latin1_string::Latin1String, &::char::Char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<=(QLatin1String lhs, QChar rhs)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn op_le((&::latin1_string::Latin1String, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<=(QLatin1String lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn op_le((&::latin1_string::Latin1String, &::latin1_string::Latin1String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<=(QLatin1String s1, QLatin1String s2)```</span>
///
///
///
/// ## Variant 7
///
/// Rust arguments: ```fn op_le((&::byte_array::ByteArray, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<=(const QByteArray& lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 8
///
/// Rust arguments: ```fn op_le((&::string::String, &::char::Char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<=(const QString& lhs, QChar rhs)```</span>
///
///
///
/// ## Variant 9
///
/// Rust arguments: ```fn op_le((&::string::String, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<=(const QString& lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 10
///
/// Rust arguments: ```fn op_le((&::string_ref::StringRef, &::char::Char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<=(const QStringRef& lhs, QChar rhs)```</span>
///
///
///
/// ## Variant 11
///
/// Rust arguments: ```fn op_le((&::string_ref::StringRef, &::latin1_string::Latin1String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<=(const QStringRef& lhs, QLatin1String rhs)```</span>
///
///
///
/// ## Variant 12
///
/// Rust arguments: ```fn op_le((&::string_ref::StringRef, &::byte_array::ByteArray)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<=(const QStringRef& lhs, const QByteArray& rhs)```</span>
///
///
///
/// ## Variant 13
///
/// Rust arguments: ```fn op_le((&::string_ref::StringRef, &::string::String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<=(const QStringRef& lhs, const QString& rhs)```</span>
///
///
///
/// ## Variant 14
///
/// Rust arguments: ```fn op_le((&::string_ref::StringRef, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<=(const QStringRef& s1, const QStringRef& s2)```</span>
///
///
pub fn op_le<Args>(args: Args) -> bool
  where Args: overloading::OpLeArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator<=(const char* s1, QLatin1String s2)```</span>
///
///
pub unsafe fn op_le_unsafe(s1: *const ::libc::c_char, s2: &::latin1_string::Latin1String) -> bool {
  ::ffi::qt_core_c_QString_G_operator_le_char_QLatin1String(s1, s2 as *const ::latin1_string::Latin1String)
}

/// C++ method: <span style='color: green;'>```operator<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_lt((&::latin1_string::Latin1String, &::char::Char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<(QLatin1String lhs, QChar rhs)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_lt((&::latin1_string::Latin1String, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<(QLatin1String lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_lt((&::latin1_string::Latin1String, &::latin1_string::Latin1String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<(QLatin1String s1, QLatin1String s2)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_lt((&::byte_array::ByteArray, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<(const QByteArray& lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn op_lt((&::string::String, &::char::Char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<(const QString& lhs, QChar rhs)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn op_lt((&::string::String, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<(const QString& lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 7
///
/// Rust arguments: ```fn op_lt((&::string_ref::StringRef, &::char::Char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<(const QStringRef& lhs, QChar rhs)```</span>
///
///
///
/// ## Variant 8
///
/// Rust arguments: ```fn op_lt((&::string_ref::StringRef, &::latin1_string::Latin1String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<(const QStringRef& lhs, QLatin1String rhs)```</span>
///
///
///
/// ## Variant 9
///
/// Rust arguments: ```fn op_lt((&::string_ref::StringRef, &::byte_array::ByteArray)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<(const QStringRef& lhs, const QByteArray& rhs)```</span>
///
///
///
/// ## Variant 10
///
/// Rust arguments: ```fn op_lt((&::string_ref::StringRef, &::string::String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<(const QStringRef& lhs, const QString& rhs)```</span>
///
///
///
/// ## Variant 11
///
/// Rust arguments: ```fn op_lt((&::string_ref::StringRef, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<(const QStringRef& s1, const QStringRef& s2)```</span>
///
///
pub fn op_lt<Args>(args: Args) -> bool
  where Args: overloading::OpLtArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator<(const char* s1, QLatin1String s2)```</span>
///
///
pub unsafe fn op_lt_unsafe(s1: *const ::libc::c_char, s2: &::latin1_string::Latin1String) -> bool {
  ::ffi::qt_core_c_QString_G_operator_lt_char_QLatin1String(s1, s2 as *const ::latin1_string::Latin1String)
}

/// C++ method: <span style='color: green;'>```operator!=```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_neq((&::char::Char, &::latin1_string::Latin1String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator!=(QChar lhs, QLatin1String rhs)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_neq((&::char::Char, &::string::String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator!=(QChar lhs, const QString& rhs)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_neq((&::char::Char, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator!=(QChar lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_neq((&::latin1_string::Latin1String, &::char::Char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator!=(QLatin1String lhs, QChar rhs)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn op_neq((&::latin1_string::Latin1String, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator!=(QLatin1String lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn op_neq((&::latin1_string::Latin1String, &::latin1_string::Latin1String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator!=(QLatin1String s1, QLatin1String s2)```</span>
///
///
///
/// ## Variant 7
///
/// Rust arguments: ```fn op_neq((&::byte_array::ByteArray, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator!=(const QByteArray& lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 8
///
/// Rust arguments: ```fn op_neq((&::string::String, &::char::Char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator!=(const QString& lhs, QChar rhs)```</span>
///
///
///
/// ## Variant 9
///
/// Rust arguments: ```fn op_neq((&::string::String, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator!=(const QString& lhs, const QStringRef& rhs)```</span>
///
///
///
/// ## Variant 10
///
/// Rust arguments: ```fn op_neq((&::string_ref::StringRef, &::char::Char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator!=(const QStringRef& lhs, QChar rhs)```</span>
///
///
///
/// ## Variant 11
///
/// Rust arguments: ```fn op_neq((&::string_ref::StringRef, &::latin1_string::Latin1String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator!=(const QStringRef& lhs, QLatin1String rhs)```</span>
///
///
///
/// ## Variant 12
///
/// Rust arguments: ```fn op_neq((&::string_ref::StringRef, &::byte_array::ByteArray)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator!=(const QStringRef& lhs, const QByteArray& rhs)```</span>
///
///
///
/// ## Variant 13
///
/// Rust arguments: ```fn op_neq((&::string_ref::StringRef, &::string::String)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator!=(const QStringRef& lhs, const QString& rhs)```</span>
///
///
///
/// ## Variant 14
///
/// Rust arguments: ```fn op_neq((&::string_ref::StringRef, &::string_ref::StringRef)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator!=(const QStringRef& s1, const QStringRef& s2)```</span>
///
///
pub fn op_neq<Args>(args: Args) -> bool
  where Args: overloading::OpNeqArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator!=(const char* s1, QLatin1String s2)```</span>
///
///
pub unsafe fn op_neq_unsafe(s1: *const ::libc::c_char, s2: &::latin1_string::Latin1String) -> bool {
  ::ffi::qt_core_c_QString_G_operator_neq_char_QLatin1String(s1, s2 as *const ::latin1_string::Latin1String)
}

/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QString& arg2)```</span>
///
///
pub fn op_shl<'l0, 'l1>(arg1: &'l0 mut ::data_stream::DataStream,
                        arg2: &'l1 ::string::String)
                        -> &'l0 mut ::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_core_c_QString_G_operator_shl(arg1 as *mut ::data_stream::DataStream,
                                            arg2 as *const ::string::String)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QString& arg2)```</span>
///
///
pub fn op_shr<'l0, 'l1>(arg1: &'l0 mut ::data_stream::DataStream,
                        arg2: &'l1 mut ::string::String)
                        -> &'l0 mut ::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_core_c_QString_G_operator_shr(arg1 as *mut ::data_stream::DataStream,
                                            arg2 as *mut ::string::String)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```void swap(QString& value1, QString& value2)```</span>
///
///
pub fn swap(value1: &mut ::string::String, value2: &mut ::string::String) {
  unsafe {
    ::ffi::qt_core_c_QString_G_swap(value1 as *mut ::string::String,
                                    value2 as *mut ::string::String)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [String::append](../struct.String.html#method.append) method.
  pub trait StringAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String;
  }
  impl<'largs> StringAppendArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let s = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_append_QByteArray(original_self as *mut ::string::String,
                                                   s as *const ::byte_array::ByteArray)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringAppendArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let c = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_append_QChar(original_self as *mut ::string::String,
                                              c as *const ::char::Char)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringAppendArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let s = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_append_QLatin1String(original_self as *mut ::string::String,
                                                      s as *const ::latin1_string::Latin1String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringAppendArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let s = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_append_QString(original_self as *mut ::string::String,
                                                s as *const ::string::String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringAppendArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let s = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_append_QStringRef(original_self as *mut ::string::String,
                                                   s as *const ::string_ref::StringRef)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [String::append_unsafe](../struct.String.html#method.append_unsafe) method.
  pub trait StringAppendUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String;
  }
  impl<'largs> StringAppendUnsafeArgs<'largs> for (*const ::char::Char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let uc = self.0;
      let len = self.1;
      let ffi_result = ::ffi::qt_core_c_QString_append_QChar_int(original_self as *mut ::string::String, uc, len);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringAppendUnsafeArgs<'largs> for *const ::libc::c_char {
    unsafe fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let s = self;
      let ffi_result = ::ffi::qt_core_c_QString_append_char(original_self as *mut ::string::String, s);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [String::arg0](../struct.String.html#method.arg0) method.
  pub trait StringArg0Args<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String;
  }
  impl<'largs> StringArg0Args<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_QChar(original_self as *const ::string::String,
                                                       a as *const ::char::Char,
                                                       &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs> for (&'largs ::char::Char, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_QChar_int(original_self as *const ::string::String,
                                                           a as *const ::char::Char,
                                                           field_width,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs> for (&'largs ::char::Char, ::libc::c_int, &'largs ::char::Char) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      let fill_char = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_QChar_int_QChar(original_self as *const ::string::String,
                                                                 a as *const ::char::Char,
                                                                 field_width,
                                                                 fill_char as *const ::char::Char,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_QString(original_self as *const ::string::String,
                                                         a as *const ::string::String,
                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs> for (&'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a1 = self.0;
      let a2 = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_QString_QString(original_self as *const ::string::String,
                                                                 a1 as *const ::string::String,
                                                                 a2 as *const ::string::String,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs> for (&'largs ::string::String, &'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a1 = self.0;
      let a2 = self.1;
      let a3 = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_QString_QString_QString(original_self as *const ::string::String,
                                                                         a1 as *const ::string::String,
                                                                         a2 as *const ::string::String,
                                                                         a3 as *const ::string::String,
                                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs>
    for (&'largs ::string::String, &'largs ::string::String, &'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a1 = self.0;
      let a2 = self.1;
      let a3 = self.2;
      let a4 = self.3;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_QString_QString_QString_QString(original_self as *const ::string::String, a1 as *const ::string::String, a2 as *const ::string::String, a3 as *const ::string::String, a4 as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs>
    for (&'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a1 = self.0;
      let a2 = self.1;
      let a3 = self.2;
      let a4 = self.3;
      let a5 = self.4;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_QString_QString_QString_QString_QString(original_self as *const ::string::String, a1 as *const ::string::String, a2 as *const ::string::String, a3 as *const ::string::String, a4 as *const ::string::String, a5 as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs>
    for (&'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a1 = self.0;
      let a2 = self.1;
      let a3 = self.2;
      let a4 = self.3;
      let a5 = self.4;
      let a6 = self.5;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_QString_QString_QString_QString_QString_QString(original_self as *const ::string::String, a1 as *const ::string::String, a2 as *const ::string::String, a3 as *const ::string::String, a4 as *const ::string::String, a5 as *const ::string::String, a6 as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs>
    for (&'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a1 = self.0;
      let a2 = self.1;
      let a3 = self.2;
      let a4 = self.3;
      let a5 = self.4;
      let a6 = self.5;
      let a7 = self.6;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_QString_QString_QString_QString_QString_QString_QString(original_self as *const ::string::String, a1 as *const ::string::String, a2 as *const ::string::String, a3 as *const ::string::String, a4 as *const ::string::String, a5 as *const ::string::String, a6 as *const ::string::String, a7 as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs>
    for (&'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a1 = self.0;
      let a2 = self.1;
      let a3 = self.2;
      let a4 = self.3;
      let a5 = self.4;
      let a6 = self.5;
      let a7 = self.6;
      let a8 = self.7;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_QString_QString_QString_QString_QString_QString_QString_QString(original_self as *const ::string::String, a1 as *const ::string::String, a2 as *const ::string::String, a3 as *const ::string::String, a4 as *const ::string::String, a5 as *const ::string::String, a6 as *const ::string::String, a7 as *const ::string::String, a8 as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs>
    for (&'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String,
                                               &'largs ::string::String) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a1 = self.0;
      let a2 = self.1;
      let a3 = self.2;
      let a4 = self.3;
      let a5 = self.4;
      let a6 = self.5;
      let a7 = self.6;
      let a8 = self.7;
      let a9 = self.8;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_QString_QString_QString_QString_QString_QString_QString_QString_QString(original_self as *const ::string::String, a1 as *const ::string::String, a2 as *const ::string::String, a3 as *const ::string::String, a4 as *const ::string::String, a5 as *const ::string::String, a6 as *const ::string::String, a7 as *const ::string::String, a8 as *const ::string::String, a9 as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs> for (&'largs ::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_QString_int(original_self as *const ::string::String,
                                                             a as *const ::string::String,
                                                             field_width,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs> for (&'largs ::string::String, ::libc::c_int, &'largs ::char::Char) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      let fill_char = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_QString_int_QChar(original_self as *const ::string::String,
                                                                   a as *const ::string::String,
                                                                   field_width,
                                                                   fill_char as *const ::char::Char,
                                                                   &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_char(original_self as *const ::string::String, a, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs> for (::libc::c_char, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_char_int(original_self as *const ::string::String,
                                                          a,
                                                          field_width,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs> for (::libc::c_char, ::libc::c_int, &'largs ::char::Char) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      let fill_char = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_char_int_QChar(original_self as *const ::string::String,
                                                                a,
                                                                field_width,
                                                                fill_char as *const ::char::Char,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_double(original_self as *const ::string::String, a, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs> for (::libc::c_double, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_double_int(original_self as *const ::string::String,
                                                            a,
                                                            field_width,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs> for (::libc::c_double, ::libc::c_int, ::libc::c_char) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      let fmt = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_double_int_char(original_self as *const ::string::String,
                                                                 a,
                                                                 field_width,
                                                                 fmt,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs> for (::libc::c_double, ::libc::c_int, ::libc::c_char, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      let fmt = self.2;
      let prec = self.3;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_double_int_char_int(original_self as *const ::string::String,
                                                                     a,
                                                                     field_width,
                                                                     fmt,
                                                                     prec,
                                                                     &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs>
    for (::libc::c_double, ::libc::c_int, ::libc::c_char, ::libc::c_int, &'largs ::char::Char) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      let fmt = self.2;
      let prec = self.3;
      let fill_char = self.4;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_double_int_char_int_QChar(original_self as *const ::string::String,
                                                                           a,
                                                                           field_width,
                                                                           fmt,
                                                                           prec,
                                                                           fill_char as *const ::char::Char,
                                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      let base = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_int_int_int(original_self as *const ::string::String,
                                                             a,
                                                             field_width,
                                                             base,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, &'largs ::char::Char) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      let base = self.2;
      let fill_char = self.3;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_int_int_int_QChar(original_self as *const ::string::String,
                                                                   a,
                                                                   field_width,
                                                                   base,
                                                                   fill_char as *const ::char::Char,
                                                                   &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs> for (u64, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let fieldwidth = self.1;
      let base = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_qulonglong_int_int(original_self as *const ::string::String,
                                                                    a,
                                                                    fieldwidth,
                                                                    base,
                                                                    &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg0Args<'largs> for (u64, ::libc::c_int, ::libc::c_int, &'largs ::char::Char) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let fieldwidth = self.1;
      let base = self.2;
      let fill_char = self.3;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_qulonglong_int_int_QChar(original_self as *const ::string::String,
                                                                          a,
                                                                          fieldwidth,
                                                                          base,
                                                                          fill_char as *const ::char::Char,
                                                                          &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::arg1](../struct.String.html#method.arg1) method.
  pub trait StringArg1Args<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String;
  }
  impl<'largs> StringArg1Args<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_int(original_self as *const ::string::String, a, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg1Args<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_int_int(original_self as *const ::string::String,
                                                         a,
                                                         field_width,
                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg1Args<'largs> for (::libc::c_long, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let fieldwidth = self.1;
      let base = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_long_int_int(original_self as *const ::string::String,
                                                              a,
                                                              fieldwidth,
                                                              base,
                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg1Args<'largs> for (::libc::c_long, ::libc::c_int, ::libc::c_int, &'largs ::char::Char) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let fieldwidth = self.1;
      let base = self.2;
      let fill_char = self.3;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_long_int_int_QChar(original_self as *const ::string::String,
                                                                    a,
                                                                    fieldwidth,
                                                                    base,
                                                                    fill_char as *const ::char::Char,
                                                                    &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg1Args<'largs> for u64 {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_qulonglong(original_self as *const ::string::String, a, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg1Args<'largs> for (u64, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let fieldwidth = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_qulonglong_int(original_self as *const ::string::String,
                                                                a,
                                                                fieldwidth,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg1Args<'largs> for (::libc::c_uint, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      let base = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_unsigned_int_int_int(original_self as *const ::string::String,
                                                                      a,
                                                                      field_width,
                                                                      base,
                                                                      &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg1Args<'largs> for (::libc::c_uint, ::libc::c_int, ::libc::c_int, &'largs ::char::Char) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      let base = self.2;
      let fill_char = self.3;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_unsigned_int_int_int_QChar(original_self as *const ::string::String,
                                                                            a,
                                                                            field_width,
                                                                            base,
                                                                            fill_char as *const ::char::Char,
                                                                            &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::arg2](../struct.String.html#method.arg2) method.
  pub trait StringArg2Args<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String;
  }
  impl<'largs> StringArg2Args<'largs> for ::libc::c_long {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_long(original_self as *const ::string::String, a, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg2Args<'largs> for (::libc::c_long, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let fieldwidth = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_long_int(original_self as *const ::string::String,
                                                          a,
                                                          fieldwidth,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg2Args<'largs> for (i64, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let fieldwidth = self.1;
      let base = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_qlonglong_int_int(original_self as *const ::string::String,
                                                                   a,
                                                                   fieldwidth,
                                                                   base,
                                                                   &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg2Args<'largs> for (i64, ::libc::c_int, ::libc::c_int, &'largs ::char::Char) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let fieldwidth = self.1;
      let base = self.2;
      let fill_char = self.3;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_qlonglong_int_int_QChar(original_self as *const ::string::String,
                                                                         a,
                                                                         fieldwidth,
                                                                         base,
                                                                         fill_char as *const ::char::Char,
                                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg2Args<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_unsigned_int(original_self as *const ::string::String, a, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg2Args<'largs> for (::libc::c_uint, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_unsigned_int_int(original_self as *const ::string::String,
                                                                  a,
                                                                  field_width,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg2Args<'largs> for (::libc::c_ulong, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let fieldwidth = self.1;
      let base = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_unsigned_long_int_int(original_self as *const ::string::String,
                                                                       a,
                                                                       fieldwidth,
                                                                       base,
                                                                       &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg2Args<'largs> for (::libc::c_ulong, ::libc::c_int, ::libc::c_int, &'largs ::char::Char) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let fieldwidth = self.1;
      let base = self.2;
      let fill_char = self.3;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_unsigned_long_int_int_QChar(original_self as *const ::string::String, a, fieldwidth, base, fill_char as *const ::char::Char, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::arg3](../struct.String.html#method.arg3) method.
  pub trait StringArg3Args<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String;
  }
  impl<'largs> StringArg3Args<'largs> for i64 {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_qlonglong(original_self as *const ::string::String, a, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg3Args<'largs> for (i64, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let fieldwidth = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_qlonglong_int(original_self as *const ::string::String,
                                                               a,
                                                               fieldwidth,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg3Args<'largs> for (::libc::c_short, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      let base = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_short_int_int(original_self as *const ::string::String,
                                                               a,
                                                               field_width,
                                                               base,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg3Args<'largs> for (::libc::c_short, ::libc::c_int, ::libc::c_int, &'largs ::char::Char) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      let base = self.2;
      let fill_char = self.3;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_short_int_int_QChar(original_self as *const ::string::String,
                                                                     a,
                                                                     field_width,
                                                                     base,
                                                                     fill_char as *const ::char::Char,
                                                                     &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg3Args<'largs> for ::libc::c_ulong {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_unsigned_long(original_self as *const ::string::String,
                                                               a,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg3Args<'largs> for (::libc::c_ulong, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let fieldwidth = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_unsigned_long_int(original_self as *const ::string::String,
                                                                   a,
                                                                   fieldwidth,
                                                                   &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg3Args<'largs> for (::libc::c_ushort, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      let base = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_unsigned_short_int_int(original_self as *const ::string::String,
                                                                        a,
                                                                        field_width,
                                                                        base,
                                                                        &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg3Args<'largs> for (::libc::c_ushort, ::libc::c_int, ::libc::c_int, &'largs ::char::Char) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      let base = self.2;
      let fill_char = self.3;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_unsigned_short_int_int_QChar(original_self as *const ::string::String, a, field_width, base, fill_char as *const ::char::Char, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::arg4](../struct.String.html#method.arg4) method.
  pub trait StringArg4Args<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String;
  }
  impl<'largs> StringArg4Args<'largs> for ::libc::c_short {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_short(original_self as *const ::string::String, a, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg4Args<'largs> for (::libc::c_short, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_short_int(original_self as *const ::string::String,
                                                           a,
                                                           field_width,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg4Args<'largs> for ::libc::c_ushort {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_unsigned_short(original_self as *const ::string::String,
                                                                a,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringArg4Args<'largs> for (::libc::c_ushort, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let a = self.0;
      let field_width = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_arg_to_output_unsigned_short_int(original_self as *const ::string::String,
                                                                    a,
                                                                    field_width,
                                                                    &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::compare](../struct.String.html#method.compare) method.
  pub trait StringCompareArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int;
  }
  impl<'largs> StringCompareArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QString_compare_QLatin1String(original_self as *const ::string::String,
                                                       other as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'largs> StringCompareArgs<'largs> for (&'largs ::latin1_string::Latin1String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let other = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_compare_QLatin1String_Qt_CaseSensitivity(original_self as *const ::string::String, other as *const ::latin1_string::Latin1String, cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringCompareArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_compare_QString(original_self as *const ::string::String,
                                                 s as *const ::string::String)
      }
    }
  }
  impl<'largs> StringCompareArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_compare_QStringRef(original_self as *const ::string::String,
                                                    s as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'largs> StringCompareArgs<'largs> for (&'largs ::string_ref::StringRef, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_compare_QStringRef_Qt_CaseSensitivity(original_self as *const ::string::String,
                                                                       s as *const ::string_ref::StringRef,
                                                                       cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringCompareArgs<'largs> for (&'largs ::string::String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_compare_QString_Qt_CaseSensitivity(original_self as *const ::string::String,
                                                                    s as *const ::string::String,
                                                                    cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::compare_static](../struct.String.html#method.compare_static) method.
  pub trait StringCompareStaticArgs {
    fn exec(self) -> ::libc::c_int;
  }
  impl<'a> StringCompareStaticArgs for (&'a ::latin1_string::Latin1String, &'a ::string::String) {
    fn exec(self) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_compare_QLatin1String_QString(s1 as *const ::latin1_string::Latin1String,
                                                               s2 as *const ::string::String)
      }
    }
  }
  impl<'a> StringCompareStaticArgs
    for (&'a ::latin1_string::Latin1String, &'a ::string::String, &'a ::qt::CaseSensitivity) {
    fn exec(self) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      let cs = self.2;
      unsafe { ::ffi::qt_core_c_QString_compare_QLatin1String_QString_Qt_CaseSensitivity(s1 as *const ::latin1_string::Latin1String, s2 as *const ::string::String, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'a> StringCompareStaticArgs for (&'a ::string::String, &'a ::latin1_string::Latin1String) {
    fn exec(self) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_compare_QString_QLatin1String(s1 as *const ::string::String,
                                                               s2 as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'a> StringCompareStaticArgs
    for (&'a ::string::String, &'a ::latin1_string::Latin1String, &'a ::qt::CaseSensitivity) {
    fn exec(self) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      let cs = self.2;
      unsafe { ::ffi::qt_core_c_QString_compare_QString_QLatin1String_Qt_CaseSensitivity(s1 as *const ::string::String, s2 as *const ::latin1_string::Latin1String, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'a> StringCompareStaticArgs for (&'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_compare_QString_QString(s1 as *const ::string::String, s2 as *const ::string::String)
      }
    }
  }
  impl<'a> StringCompareStaticArgs for (&'a ::string::String, &'a ::string_ref::StringRef) {
    fn exec(self) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_compare_QString_QStringRef(s1 as *const ::string::String,
                                                            s2 as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> StringCompareStaticArgs for (&'a ::string::String, &'a ::string_ref::StringRef, &'a ::qt::CaseSensitivity) {
    fn exec(self) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      let arg3 = self.2;
      unsafe {
        ::ffi::qt_core_c_QString_compare_QString_QStringRef_Qt_CaseSensitivity(s1 as *const ::string::String,
                                                                               s2 as *const ::string_ref::StringRef,
                                                                               arg3 as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'a> StringCompareStaticArgs for (&'a ::string::String, &'a ::string::String, &'a ::qt::CaseSensitivity) {
    fn exec(self) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      let cs = self.2;
      unsafe {
        ::ffi::qt_core_c_QString_compare_QString_QString_Qt_CaseSensitivity(s1 as *const ::string::String,
                                                                            s2 as *const ::string::String,
                                                                            cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::contains](../struct.String.html#method.contains) method.
  pub trait StringContainsArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> bool;
  }
  impl<'largs> StringContainsArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let c = self;
      unsafe {
        ::ffi::qt_core_c_QString_contains_QChar(original_self as *const ::string::String,
                                                c as *const ::char::Char)
      }
    }
  }
  impl<'largs> StringContainsArgs<'largs> for (&'largs ::char::Char, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let c = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_contains_QChar_Qt_CaseSensitivity(original_self as *const ::string::String,
                                                                   c as *const ::char::Char,
                                                                   cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringContainsArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_contains_QLatin1String(original_self as *const ::string::String,
                                                        s as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'largs> StringContainsArgs<'largs> for (&'largs ::latin1_string::Latin1String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_contains_QLatin1String_Qt_CaseSensitivity(original_self as *const ::string::String,
                                                                           s as *const ::latin1_string::Latin1String,
                                                                           cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringContainsArgs<'largs> for &'largs mut ::reg_exp::RegExp {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let rx = self;
      unsafe {
        ::ffi::qt_core_c_QString_contains_QRegExp_ref(original_self as *const ::string::String,
                                                      rx as *mut ::reg_exp::RegExp)
      }
    }
  }
  impl<'largs> StringContainsArgs<'largs> for &'largs ::reg_exp::RegExp {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let rx = self;
      unsafe {
        ::ffi::qt_core_c_QString_contains_const_QRegExp_ref(original_self as *const ::string::String,
                                                            rx as *const ::reg_exp::RegExp)
      }
    }
  }
  impl<'largs> StringContainsArgs<'largs> for &'largs ::regular_expression::RegularExpression {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let re = self;
      unsafe { ::ffi::qt_core_c_QString_contains_const_QRegularExpression_ref(original_self as *const ::string::String, re as *const ::regular_expression::RegularExpression) }
    }
  }
  impl<'largs> StringContainsArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_contains_const_QStringRef_ref(original_self as *const ::string::String,
                                                               s as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'largs> StringContainsArgs<'largs> for (&'largs ::string_ref::StringRef, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self.0;
      let cs = self.1;
      unsafe { ::ffi::qt_core_c_QString_contains_const_QStringRef_ref_Qt_CaseSensitivity(original_self as *const ::string::String, s as *const ::string_ref::StringRef, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringContainsArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_contains_const_QString_ref(original_self as *const ::string::String,
                                                            s as *const ::string::String)
      }
    }
  }
  impl<'largs> StringContainsArgs<'largs> for (&'largs ::string::String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_contains_const_QString_ref_Qt_CaseSensitivity(original_self as *const ::string::String, s as *const ::string::String, cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::count](../struct.String.html#method.count) method.
  pub trait StringCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int;
  }
  impl<'largs> StringCountArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let c = self;
      unsafe {
        ::ffi::qt_core_c_QString_count_QChar(original_self as *const ::string::String,
                                             c as *const ::char::Char)
      }
    }
  }
  impl<'largs> StringCountArgs<'largs> for (&'largs ::char::Char, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let c = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_count_QChar_Qt_CaseSensitivity(original_self as *const ::string::String,
                                                                c as *const ::char::Char,
                                                                cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringCountArgs<'largs> for &'largs ::reg_exp::RegExp {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let arg1 = self;
      unsafe {
        ::ffi::qt_core_c_QString_count_QRegExp(original_self as *const ::string::String,
                                               arg1 as *const ::reg_exp::RegExp)
      }
    }
  }
  impl<'largs> StringCountArgs<'largs> for &'largs ::regular_expression::RegularExpression {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let re = self;
      unsafe {
        ::ffi::qt_core_c_QString_count_QRegularExpression(original_self as *const ::string::String,
                                                          re as *const ::regular_expression::RegularExpression)
      }
    }
  }
  impl<'largs> StringCountArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_count_QString(original_self as *const ::string::String,
                                               s as *const ::string::String)
      }
    }
  }
  impl<'largs> StringCountArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_count_QStringRef(original_self as *const ::string::String,
                                                  s as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'largs> StringCountArgs<'largs> for (&'largs ::string_ref::StringRef, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_count_QStringRef_Qt_CaseSensitivity(original_self as *const ::string::String,
                                                                     s as *const ::string_ref::StringRef,
                                                                     cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringCountArgs<'largs> for (&'largs ::string::String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_count_QString_Qt_CaseSensitivity(original_self as *const ::string::String,
                                                                  s as *const ::string::String,
                                                                  cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QString_count_no_args(original_self as *const ::string::String) }
    }
  }
  /// This trait represents a set of arguments accepted by [String::ends_with](../struct.String.html#method.ends_with) method.
  pub trait StringEndsWithArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> bool;
  }
  impl<'largs> StringEndsWithArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let c = self;
      unsafe {
        ::ffi::qt_core_c_QString_endsWith_QChar(original_self as *const ::string::String,
                                                c as *const ::char::Char)
      }
    }
  }
  impl<'largs> StringEndsWithArgs<'largs> for (&'largs ::char::Char, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let c = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_endsWith_QChar_Qt_CaseSensitivity(original_self as *const ::string::String,
                                                                   c as *const ::char::Char,
                                                                   cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringEndsWithArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_endsWith_QLatin1String(original_self as *const ::string::String,
                                                        s as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'largs> StringEndsWithArgs<'largs> for (&'largs ::latin1_string::Latin1String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_endsWith_QLatin1String_Qt_CaseSensitivity(original_self as *const ::string::String,
                                                                           s as *const ::latin1_string::Latin1String,
                                                                           cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringEndsWithArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_endsWith_QString(original_self as *const ::string::String,
                                                  s as *const ::string::String)
      }
    }
  }
  impl<'largs> StringEndsWithArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_endsWith_QStringRef(original_self as *const ::string::String,
                                                     s as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'largs> StringEndsWithArgs<'largs> for (&'largs ::string_ref::StringRef, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_endsWith_QStringRef_Qt_CaseSensitivity(original_self as *const ::string::String,
                                                                        s as *const ::string_ref::StringRef,
                                                                        cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringEndsWithArgs<'largs> for (&'largs ::string::String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_endsWith_QString_Qt_CaseSensitivity(original_self as *const ::string::String,
                                                                     s as *const ::string::String,
                                                                     cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::fill](../struct.String.html#method.fill) method.
  pub trait StringFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String;
  }
  impl<'largs> StringFillArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let c = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_fill_c(original_self as *mut ::string::String,
                                        c as *const ::char::Char)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringFillArgs<'largs> for (&'largs ::char::Char, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let c = self.0;
      let size = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_fill_c_size(original_self as *mut ::string::String,
                                             c as *const ::char::Char,
                                             size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [String::from_latin1_unsafe](../struct.String.html#method.from_latin1_unsafe) method.
  pub trait StringFromLatin1UnsafeArgs {
    unsafe fn exec(self) -> ::string::String;
  }
  impl StringFromLatin1UnsafeArgs for *const ::libc::c_char {
    unsafe fn exec(self) -> ::string::String {
      let str = self;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QString_fromLatin1_to_output_char(str, &mut object);
        object
      }
    }
  }
  impl StringFromLatin1UnsafeArgs for (*const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self) -> ::string::String {
      let str = self.0;
      let size = self.1;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QString_fromLatin1_to_output_char_int(str, size, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::from_local8_bit_unsafe](../struct.String.html#method.from_local8_bit_unsafe) method.
  pub trait StringFromLocal8BitUnsafeArgs {
    unsafe fn exec(self) -> ::string::String;
  }
  impl StringFromLocal8BitUnsafeArgs for *const ::libc::c_char {
    unsafe fn exec(self) -> ::string::String {
      let str = self;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QString_fromLocal8Bit_to_output_char(str, &mut object);
        object
      }
    }
  }
  impl StringFromLocal8BitUnsafeArgs for (*const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self) -> ::string::String {
      let str = self.0;
      let size = self.1;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QString_fromLocal8Bit_to_output_char_int(str, size, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::from_ucs4](../struct.String.html#method.from_ucs4) method.
  pub trait StringFromUcs4Args {
    unsafe fn exec(self) -> ::string::String;
  }
  impl StringFromUcs4Args for *const ::libc::c_uint {
    unsafe fn exec(self) -> ::string::String {
      let arg1 = self;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QString_fromUcs4_to_output_arg1(arg1, &mut object);
        object
      }
    }
  }
  impl StringFromUcs4Args for (*const ::libc::c_uint, ::libc::c_int) {
    unsafe fn exec(self) -> ::string::String {
      let arg1 = self.0;
      let size = self.1;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QString_fromUcs4_to_output_arg1_size(arg1, size, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::from_utf16](../struct.String.html#method.from_utf16) method.
  pub trait StringFromUtf16Args {
    unsafe fn exec(self) -> ::string::String;
  }
  impl StringFromUtf16Args for *const ::libc::c_ushort {
    unsafe fn exec(self) -> ::string::String {
      let arg1 = self;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QString_fromUtf16_to_output_arg1(arg1, &mut object);
        object
      }
    }
  }
  impl StringFromUtf16Args for (*const ::libc::c_ushort, ::libc::c_int) {
    unsafe fn exec(self) -> ::string::String {
      let arg1 = self.0;
      let size = self.1;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QString_fromUtf16_to_output_arg1_size(arg1, size, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::from_utf8_unsafe](../struct.String.html#method.from_utf8_unsafe) method.
  pub trait StringFromUtf8UnsafeArgs {
    unsafe fn exec(self) -> ::string::String;
  }
  impl StringFromUtf8UnsafeArgs for *const ::libc::c_char {
    unsafe fn exec(self) -> ::string::String {
      let str = self;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QString_fromUtf8_to_output_char(str, &mut object);
        object
      }
    }
  }
  impl StringFromUtf8UnsafeArgs for (*const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self) -> ::string::String {
      let str = self.0;
      let size = self.1;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QString_fromUtf8_to_output_char_int(str, size, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::from_w_char_array](../struct.String.html#method.from_w_char_array) method.
  pub trait StringFromWCharArrayArgs {
    unsafe fn exec(self) -> ::string::String;
  }
  impl StringFromWCharArrayArgs for *const ::libc::wchar_t {
    unsafe fn exec(self) -> ::string::String {
      let string = self;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QString_fromWCharArray_to_output_string(string, &mut object);
        object
      }
    }
  }
  impl StringFromWCharArrayArgs for (*const ::libc::wchar_t, ::libc::c_int) {
    unsafe fn exec(self) -> ::string::String {
      let string = self.0;
      let size = self.1;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QString_fromWCharArray_to_output_string_size(string, size, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::index_of](../struct.String.html#method.index_of) method.
  pub trait StringIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int;
  }
  impl<'largs> StringIndexOfArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let c = self;
      unsafe {
        ::ffi::qt_core_c_QString_indexOf_QChar(original_self as *const ::string::String,
                                               c as *const ::char::Char)
      }
    }
  }
  impl<'largs> StringIndexOfArgs<'largs> for (&'largs ::char::Char, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let c = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_indexOf_QChar_int(original_self as *const ::string::String,
                                                   c as *const ::char::Char,
                                                   from)
      }
    }
  }
  impl<'largs> StringIndexOfArgs<'largs> for (&'largs ::char::Char, ::libc::c_int, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let c = self.0;
      let from = self.1;
      let cs = self.2;
      unsafe {
        ::ffi::qt_core_c_QString_indexOf_QChar_int_Qt_CaseSensitivity(original_self as *const ::string::String,
                                                                      c as *const ::char::Char,
                                                                      from,
                                                                      cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringIndexOfArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_indexOf_QLatin1String(original_self as *const ::string::String,
                                                       s as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'largs> StringIndexOfArgs<'largs> for (&'largs ::latin1_string::Latin1String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_indexOf_QLatin1String_int(original_self as *const ::string::String,
                                                           s as *const ::latin1_string::Latin1String,
                                                           from)
      }
    }
  }
  impl<'largs> StringIndexOfArgs<'largs>
    for (&'largs ::latin1_string::Latin1String, ::libc::c_int, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self.0;
      let from = self.1;
      let cs = self.2;
      unsafe {
        ::ffi::qt_core_c_QString_indexOf_QLatin1String_int_Qt_CaseSensitivity(original_self as *const ::string::String, s as *const ::latin1_string::Latin1String, from, cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringIndexOfArgs<'largs> for &'largs mut ::reg_exp::RegExp {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let arg1 = self;
      unsafe {
        ::ffi::qt_core_c_QString_indexOf_QRegExp_ref(original_self as *const ::string::String,
                                                     arg1 as *mut ::reg_exp::RegExp)
      }
    }
  }
  impl<'largs> StringIndexOfArgs<'largs> for (&'largs mut ::reg_exp::RegExp, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let arg1 = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_indexOf_QRegExp_ref_int(original_self as *const ::string::String,
                                                         arg1 as *mut ::reg_exp::RegExp,
                                                         from)
      }
    }
  }
  impl<'largs> StringIndexOfArgs<'largs> for &'largs ::reg_exp::RegExp {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let arg1 = self;
      unsafe {
        ::ffi::qt_core_c_QString_indexOf_const_QRegExp_ref(original_self as *const ::string::String,
                                                           arg1 as *const ::reg_exp::RegExp)
      }
    }
  }
  impl<'largs> StringIndexOfArgs<'largs> for (&'largs ::reg_exp::RegExp, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let arg1 = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_indexOf_const_QRegExp_ref_int(original_self as *const ::string::String,
                                                               arg1 as *const ::reg_exp::RegExp,
                                                               from)
      }
    }
  }
  impl<'largs> StringIndexOfArgs<'largs> for &'largs ::regular_expression::RegularExpression {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let re = self;
      unsafe { ::ffi::qt_core_c_QString_indexOf_const_QRegularExpression_ref(original_self as *const ::string::String, re as *const ::regular_expression::RegularExpression) }
    }
  }
  impl<'largs> StringIndexOfArgs<'largs> for (&'largs ::regular_expression::RegularExpression, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let re = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_core_c_QString_indexOf_const_QRegularExpression_ref_int(original_self as *const ::string::String, re as *const ::regular_expression::RegularExpression, from) }
    }
  }
  impl<'largs> StringIndexOfArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_indexOf_const_QStringRef_ref(original_self as *const ::string::String,
                                                              s as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'largs> StringIndexOfArgs<'largs> for (&'largs ::string_ref::StringRef, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_indexOf_const_QStringRef_ref_int(original_self as *const ::string::String,
                                                                  s as *const ::string_ref::StringRef,
                                                                  from)
      }
    }
  }
  impl<'largs> StringIndexOfArgs<'largs>
    for (&'largs ::string_ref::StringRef, ::libc::c_int, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self.0;
      let from = self.1;
      let cs = self.2;
      unsafe { ::ffi::qt_core_c_QString_indexOf_const_QStringRef_ref_int_Qt_CaseSensitivity(original_self as *const ::string::String, s as *const ::string_ref::StringRef, from, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringIndexOfArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_indexOf_const_QString_ref(original_self as *const ::string::String,
                                                           s as *const ::string::String)
      }
    }
  }
  impl<'largs> StringIndexOfArgs<'largs> for (&'largs ::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_indexOf_const_QString_ref_int(original_self as *const ::string::String,
                                                               s as *const ::string::String,
                                                               from)
      }
    }
  }
  impl<'largs> StringIndexOfArgs<'largs> for (&'largs ::string::String, ::libc::c_int, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self.0;
      let from = self.1;
      let cs = self.2;
      unsafe { ::ffi::qt_core_c_QString_indexOf_const_QString_ref_int_Qt_CaseSensitivity(original_self as *const ::string::String, s as *const ::string::String, from, cs as *const ::qt::CaseSensitivity) }
    }
  }
  /// This trait represents a set of arguments accepted by [String::insert](../struct.String.html#method.insert) method.
  pub trait StringInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String;
  }
  impl<'largs> StringInsertArgs<'largs> for (::libc::c_int, &'largs ::byte_array::ByteArray) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let i = self.0;
      let s = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_insert_int_QByteArray(original_self as *mut ::string::String,
                                                       i,
                                                       s as *const ::byte_array::ByteArray)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringInsertArgs<'largs> for (::libc::c_int, &'largs ::char::Char) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let i = self.0;
      let c = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_insert_int_QChar(original_self as *mut ::string::String,
                                                  i,
                                                  c as *const ::char::Char)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringInsertArgs<'largs> for (::libc::c_int, &'largs ::latin1_string::Latin1String) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let i = self.0;
      let s = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_insert_int_QLatin1String(original_self as *mut ::string::String,
                                                          i,
                                                          s as *const ::latin1_string::Latin1String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringInsertArgs<'largs> for (::libc::c_int, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let i = self.0;
      let s = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_insert_int_QString(original_self as *mut ::string::String,
                                                    i,
                                                    s as *const ::string::String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringInsertArgs<'largs> for (::libc::c_int, &'largs ::string_ref::StringRef) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let i = self.0;
      let s = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_insert_int_QStringRef(original_self as *mut ::string::String,
                                                       i,
                                                       s as *const ::string_ref::StringRef)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [String::insert_unsafe](../struct.String.html#method.insert_unsafe) method.
  pub trait StringInsertUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String;
  }
  impl<'largs> StringInsertUnsafeArgs<'largs> for (::libc::c_int, *const ::char::Char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let i = self.0;
      let uc = self.1;
      let len = self.2;
      let ffi_result =
        ::ffi::qt_core_c_QString_insert_int_QChar_int(original_self as *mut ::string::String, i, uc, len);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringInsertUnsafeArgs<'largs> for (::libc::c_int, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let i = self.0;
      let s = self.1;
      let ffi_result = ::ffi::qt_core_c_QString_insert_int_char(original_self as *mut ::string::String, i, s);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [String::last_index_of](../struct.String.html#method.last_index_of) method.
  pub trait StringLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int;
  }
  impl<'largs> StringLastIndexOfArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let c = self;
      unsafe {
        ::ffi::qt_core_c_QString_lastIndexOf_QChar(original_self as *const ::string::String,
                                                   c as *const ::char::Char)
      }
    }
  }
  impl<'largs> StringLastIndexOfArgs<'largs> for (&'largs ::char::Char, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let c = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_lastIndexOf_QChar_int(original_self as *const ::string::String,
                                                       c as *const ::char::Char,
                                                       from)
      }
    }
  }
  impl<'largs> StringLastIndexOfArgs<'largs> for (&'largs ::char::Char, ::libc::c_int, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let c = self.0;
      let from = self.1;
      let cs = self.2;
      unsafe {
        ::ffi::qt_core_c_QString_lastIndexOf_QChar_int_Qt_CaseSensitivity(original_self as *const ::string::String,
                                                                          c as *const ::char::Char,
                                                                          from,
                                                                          cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringLastIndexOfArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_lastIndexOf_QLatin1String(original_self as *const ::string::String,
                                                           s as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'largs> StringLastIndexOfArgs<'largs> for (&'largs ::latin1_string::Latin1String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_lastIndexOf_QLatin1String_int(original_self as *const ::string::String,
                                                               s as *const ::latin1_string::Latin1String,
                                                               from)
      }
    }
  }
  impl<'largs> StringLastIndexOfArgs<'largs>
    for (&'largs ::latin1_string::Latin1String, ::libc::c_int, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self.0;
      let from = self.1;
      let cs = self.2;
      unsafe { ::ffi::qt_core_c_QString_lastIndexOf_QLatin1String_int_Qt_CaseSensitivity(original_self as *const ::string::String, s as *const ::latin1_string::Latin1String, from, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringLastIndexOfArgs<'largs> for &'largs mut ::reg_exp::RegExp {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let arg1 = self;
      unsafe {
        ::ffi::qt_core_c_QString_lastIndexOf_QRegExp_ref(original_self as *const ::string::String,
                                                         arg1 as *mut ::reg_exp::RegExp)
      }
    }
  }
  impl<'largs> StringLastIndexOfArgs<'largs> for (&'largs mut ::reg_exp::RegExp, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let arg1 = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_lastIndexOf_QRegExp_ref_int(original_self as *const ::string::String,
                                                             arg1 as *mut ::reg_exp::RegExp,
                                                             from)
      }
    }
  }
  impl<'largs> StringLastIndexOfArgs<'largs> for &'largs ::reg_exp::RegExp {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let arg1 = self;
      unsafe {
        ::ffi::qt_core_c_QString_lastIndexOf_const_QRegExp_ref(original_self as *const ::string::String,
                                                               arg1 as *const ::reg_exp::RegExp)
      }
    }
  }
  impl<'largs> StringLastIndexOfArgs<'largs> for (&'largs ::reg_exp::RegExp, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let arg1 = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_lastIndexOf_const_QRegExp_ref_int(original_self as *const ::string::String,
                                                                   arg1 as *const ::reg_exp::RegExp,
                                                                   from)
      }
    }
  }
  impl<'largs> StringLastIndexOfArgs<'largs> for &'largs ::regular_expression::RegularExpression {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let re = self;
      unsafe { ::ffi::qt_core_c_QString_lastIndexOf_const_QRegularExpression_ref(original_self as *const ::string::String, re as *const ::regular_expression::RegularExpression) }
    }
  }
  impl<'largs> StringLastIndexOfArgs<'largs> for (&'largs ::regular_expression::RegularExpression, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let re = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_core_c_QString_lastIndexOf_const_QRegularExpression_ref_int(original_self as *const ::string::String, re as *const ::regular_expression::RegularExpression, from) }
    }
  }
  impl<'largs> StringLastIndexOfArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_lastIndexOf_const_QStringRef_ref(original_self as *const ::string::String,
                                                                  s as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'largs> StringLastIndexOfArgs<'largs> for (&'largs ::string_ref::StringRef, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_lastIndexOf_const_QStringRef_ref_int(original_self as *const ::string::String,
                                                                      s as *const ::string_ref::StringRef,
                                                                      from)
      }
    }
  }
  impl<'largs> StringLastIndexOfArgs<'largs>
    for (&'largs ::string_ref::StringRef, ::libc::c_int, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self.0;
      let from = self.1;
      let cs = self.2;
      unsafe { ::ffi::qt_core_c_QString_lastIndexOf_const_QStringRef_ref_int_Qt_CaseSensitivity(original_self as *const ::string::String, s as *const ::string_ref::StringRef, from, cs as *const ::qt::CaseSensitivity) }
    }
  }
  impl<'largs> StringLastIndexOfArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_lastIndexOf_const_QString_ref(original_self as *const ::string::String,
                                                               s as *const ::string::String)
      }
    }
  }
  impl<'largs> StringLastIndexOfArgs<'largs> for (&'largs ::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_lastIndexOf_const_QString_ref_int(original_self as *const ::string::String,
                                                                   s as *const ::string::String,
                                                                   from)
      }
    }
  }
  impl<'largs> StringLastIndexOfArgs<'largs>
    for (&'largs ::string::String, ::libc::c_int, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self.0;
      let from = self.1;
      let cs = self.2;
      unsafe { ::ffi::qt_core_c_QString_lastIndexOf_const_QString_ref_int_Qt_CaseSensitivity(original_self as *const ::string::String, s as *const ::string::String, from, cs as *const ::qt::CaseSensitivity) }
    }
  }
  /// This trait represents a set of arguments accepted by [String::left_justified](../struct.String.html#method.left_justified) method.
  pub trait StringLeftJustifiedArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String;
  }
  impl<'largs> StringLeftJustifiedArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let width = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_leftJustified_to_output_width(original_self as *const ::string::String,
                                                                 width,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringLeftJustifiedArgs<'largs> for (::libc::c_int, &'largs ::char::Char) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let width = self.0;
      let fill = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_leftJustified_to_output_width_fill(original_self as *const ::string::String,
                                                                      width,
                                                                      fill as *const ::char::Char,
                                                                      &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringLeftJustifiedArgs<'largs> for (::libc::c_int, &'largs ::char::Char, bool) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let width = self.0;
      let fill = self.1;
      let trunc = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_leftJustified_to_output_width_fill_trunc(original_self as *const ::string::String,
                                                                            width,
                                                                            fill as *const ::char::Char,
                                                                            trunc,
                                                                            &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::locale_aware_compare](../struct.String.html#method.locale_aware_compare) method.
  pub trait StringLocaleAwareCompareArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int;
  }
  impl<'largs> StringLocaleAwareCompareArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_localeAwareCompare_QString(original_self as *const ::string::String,
                                                            s as *const ::string::String)
      }
    }
  }
  impl<'largs> StringLocaleAwareCompareArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_localeAwareCompare_QStringRef(original_self as *const ::string::String,
                                                               s as *const ::string_ref::StringRef)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::locale_aware_compare_static](../struct.String.html#method.locale_aware_compare_static) method.
  pub trait StringLocaleAwareCompareStaticArgs {
    fn exec(self) -> ::libc::c_int;
  }
  impl<'a> StringLocaleAwareCompareStaticArgs for (&'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_localeAwareCompare_QString_QString(s1 as *const ::string::String,
                                                                    s2 as *const ::string::String)
      }
    }
  }
  impl<'a> StringLocaleAwareCompareStaticArgs for (&'a ::string::String, &'a ::string_ref::StringRef) {
    fn exec(self) -> ::libc::c_int {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_localeAwareCompare_QString_QStringRef(s1 as *const ::string::String,
                                                                       s2 as *const ::string_ref::StringRef)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::mid](../struct.String.html#method.mid) method.
  pub trait StringMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String;
  }
  impl<'largs> StringMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let position = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_mid_to_output_position(original_self as *const ::string::String,
                                                          position,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let position = self.0;
      let n = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_mid_to_output_position_n(original_self as *const ::string::String,
                                                            position,
                                                            n,
                                                            &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::mid_ref](../struct.String.html#method.mid_ref) method.
  pub trait StringMidRefArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::string_ref::StringRef;
  }
  impl<'largs> StringMidRefArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::string::String) -> ::string_ref::StringRef {
      let position = self;
      {
        let mut object: ::string_ref::StringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_midRef_to_output_position(original_self as *const ::string::String,
                                                             position,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringMidRefArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string_ref::StringRef {
      let position = self.0;
      let n = self.1;
      {
        let mut object: ::string_ref::StringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_midRef_to_output_position_n(original_self as *const ::string::String,
                                                               position,
                                                               n,
                                                               &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::new](../struct.String.html#method.new) method.
  pub trait StringNewArgs {
    fn exec(self) -> ::string::String;
  }
  impl<'a> StringNewArgs for &'a ::byte_array::ByteArray {
    fn exec(self) -> ::string::String {
      let a = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_constructor_a(a as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl<'a> StringNewArgs for &'a ::string::String {
    fn exec(self) -> ::string::String {
      let arg1 = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_constructor_arg1(arg1 as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> StringNewArgs for &'a ::char::Char {
    fn exec(self) -> ::string::String {
      let c = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_constructor_c(c as *const ::char::Char, &mut object);
        }
        object
      }
    }
  }
  impl<'a> StringNewArgs for &'a ::latin1_string::Latin1String {
    fn exec(self) -> ::string::String {
      let latin1 = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_constructor_latin1(latin1 as *const ::latin1_string::Latin1String, &mut object);
        }
        object
      }
    }
  }
  impl StringNewArgs for () {
    fn exec(self) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> StringNewArgs for (::libc::c_int, &'a ::char::Char) {
    fn exec(self) -> ::string::String {
      let size = self.0;
      let c = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_constructor_size_c(size, c as *const ::char::Char, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::new_unsafe](../struct.String.html#method.new_unsafe) method.
  pub trait StringNewUnsafeArgs {
    unsafe fn exec(self) -> ::string::String;
  }
  impl StringNewUnsafeArgs for *const ::libc::c_char {
    unsafe fn exec(self) -> ::string::String {
      let ch = self;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QString_constructor_ch(ch, &mut object);
        object
      }
    }
  }
  impl StringNewUnsafeArgs for *const ::char::Char {
    unsafe fn exec(self) -> ::string::String {
      let unicode = self;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QString_constructor_unicode(unicode, &mut object);
        object
      }
    }
  }
  impl StringNewUnsafeArgs for (*const ::char::Char, ::libc::c_int) {
    unsafe fn exec(self) -> ::string::String {
      let unicode = self.0;
      let size = self.1;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QString_constructor_unicode_size(unicode, size, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::normalized](../struct.String.html#method.normalized) method.
  pub trait StringNormalizedArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String;
  }
  impl<'largs> StringNormalizedArgs<'largs> for ::string::NormalizationForm {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let mode = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_normalized_to_output_mode(original_self as *const ::string::String,
                                                             mode,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringNormalizedArgs<'largs> for (::string::NormalizationForm, &'largs ::char::UnicodeVersion) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let mode = self.0;
      let version = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_normalized_to_output_mode_version(original_self as *const ::string::String,
                                                                     mode,
                                                                     version as *const ::char::UnicodeVersion,
                                                                     &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::number0](../struct.String.html#method.number0) method.
  pub trait StringNumber0Args {
    fn exec(self) -> ::string::String;
  }
  impl StringNumber0Args for ::libc::c_double {
    fn exec(self) -> ::string::String {
      let arg1 = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_number_to_output_double(arg1, &mut object);
        }
        object
      }
    }
  }
  impl StringNumber0Args for (::libc::c_double, ::libc::c_char) {
    fn exec(self) -> ::string::String {
      let arg1 = self.0;
      let f = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_number_to_output_double_char(arg1, f, &mut object);
        }
        object
      }
    }
  }
  impl StringNumber0Args for (::libc::c_double, ::libc::c_char, ::libc::c_int) {
    fn exec(self) -> ::string::String {
      let arg1 = self.0;
      let f = self.1;
      let prec = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_number_to_output_double_char_int(arg1, f, prec, &mut object);
        }
        object
      }
    }
  }
  impl StringNumber0Args for ::libc::c_int {
    fn exec(self) -> ::string::String {
      let arg1 = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_number_to_output_int(arg1, &mut object);
        }
        object
      }
    }
  }
  impl StringNumber0Args for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::string::String {
      let arg1 = self.0;
      let base = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_number_to_output_int_int(arg1, base, &mut object);
        }
        object
      }
    }
  }
  impl StringNumber0Args for u64 {
    fn exec(self) -> ::string::String {
      let arg1 = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_number_to_output_qulonglong(arg1, &mut object);
        }
        object
      }
    }
  }
  impl StringNumber0Args for (u64, ::libc::c_int) {
    fn exec(self) -> ::string::String {
      let arg1 = self.0;
      let base = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_number_to_output_qulonglong_int(arg1, base, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::number1](../struct.String.html#method.number1) method.
  pub trait StringNumber1Args {
    fn exec(self) -> ::string::String;
  }
  impl StringNumber1Args for ::libc::c_long {
    fn exec(self) -> ::string::String {
      let arg1 = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_number_to_output_long(arg1, &mut object);
        }
        object
      }
    }
  }
  impl StringNumber1Args for (::libc::c_long, ::libc::c_int) {
    fn exec(self) -> ::string::String {
      let arg1 = self.0;
      let base = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_number_to_output_long_int(arg1, base, &mut object);
        }
        object
      }
    }
  }
  impl StringNumber1Args for ::libc::c_uint {
    fn exec(self) -> ::string::String {
      let arg1 = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_number_to_output_unsigned_int(arg1, &mut object);
        }
        object
      }
    }
  }
  impl StringNumber1Args for (::libc::c_uint, ::libc::c_int) {
    fn exec(self) -> ::string::String {
      let arg1 = self.0;
      let base = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_number_to_output_unsigned_int_int(arg1, base, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::number2](../struct.String.html#method.number2) method.
  pub trait StringNumber2Args {
    fn exec(self) -> ::string::String;
  }
  impl StringNumber2Args for i64 {
    fn exec(self) -> ::string::String {
      let arg1 = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_number_to_output_qlonglong(arg1, &mut object);
        }
        object
      }
    }
  }
  impl StringNumber2Args for (i64, ::libc::c_int) {
    fn exec(self) -> ::string::String {
      let arg1 = self.0;
      let base = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_number_to_output_qlonglong_int(arg1, base, &mut object);
        }
        object
      }
    }
  }
  impl StringNumber2Args for ::libc::c_ulong {
    fn exec(self) -> ::string::String {
      let arg1 = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_number_to_output_unsigned_long(arg1, &mut object);
        }
        object
      }
    }
  }
  impl StringNumber2Args for (::libc::c_ulong, ::libc::c_int) {
    fn exec(self) -> ::string::String {
      let arg1 = self.0;
      let base = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_number_to_output_unsigned_long_int(arg1, base, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::op_add_assign](../struct.String.html#method.op_add_assign) method.
  pub trait StringOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String;
  }
  impl<'largs> StringOpAddAssignArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let s = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QString_operator_add_assign_QByteArray_s(original_self as *mut ::string::String,
                                                                    s as *const ::byte_array::ByteArray)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringOpAddAssignArgs<'largs> for &'largs ::char::SpecialCharacter {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let c = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QString_operator_add_assign_QChar_SpecialCharacter_c(original_self as *mut ::string::String, c as *const ::char::SpecialCharacter)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringOpAddAssignArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let c = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_operator_add_assign_QChar_c(original_self as *mut ::string::String,
                                                             c as *const ::char::Char)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringOpAddAssignArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let s = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QString_operator_add_assign_QLatin1String_s(original_self as *mut ::string::String,
                                                                       s as *const ::latin1_string::Latin1String)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringOpAddAssignArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let s = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QString_operator_add_assign_QStringRef_s(original_self as *mut ::string::String,
                                                                    s as *const ::string_ref::StringRef)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringOpAddAssignArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let s = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_operator_add_assign_QString_s(original_self as *mut ::string::String,
                                                               s as *const ::string::String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringOpAddAssignArgs<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let c = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QString_operator_add_assign_char_c(original_self as *mut ::string::String, c) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [String::op_assign](../struct.String.html#method.op_assign) method.
  pub trait StringOpAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String;
  }
  impl<'largs> StringOpAssignArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let a = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_operator_assign_QByteArray_a(original_self as *mut ::string::String,
                                                              a as *const ::byte_array::ByteArray)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringOpAssignArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let c = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_operator_assign_QChar_c(original_self as *mut ::string::String,
                                                         c as *const ::char::Char)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringOpAssignArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let latin1 = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QString_operator_assign_QLatin1String_latin1(original_self as *mut ::string::String, latin1 as *const ::latin1_string::Latin1String)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringOpAssignArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_operator_assign_QString_arg1(original_self as *mut ::string::String,
                                                              arg1 as *const ::string::String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringOpAssignArgs<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let c = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QString_operator_assign_char_c(original_self as *mut ::string::String, c) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [String::op_eq](../struct.String.html#method.op_eq) method.
  pub trait StringOpEqArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> bool;
  }
  impl<'largs> StringOpEqArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_operator_eq_QByteArray(original_self as *const ::string::String,
                                                        s as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> StringOpEqArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_operator_eq_QLatin1String(original_self as *const ::string::String,
                                                           s as *const ::latin1_string::Latin1String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::op_ge](../struct.String.html#method.op_ge) method.
  pub trait StringOpGeArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> bool;
  }
  impl<'largs> StringOpGeArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_operator_ge_QByteArray(original_self as *const ::string::String,
                                                        s as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> StringOpGeArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_operator_ge_QLatin1String(original_self as *const ::string::String,
                                                           s as *const ::latin1_string::Latin1String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::op_gt](../struct.String.html#method.op_gt) method.
  pub trait StringOpGtArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> bool;
  }
  impl<'largs> StringOpGtArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_operator_gt_QByteArray(original_self as *const ::string::String,
                                                        s as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> StringOpGtArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_operator_gt_QLatin1String(original_self as *const ::string::String,
                                                           s as *const ::latin1_string::Latin1String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::op_index](../struct.String.html#method.op_index) method.
  pub trait StringOpIndexArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::char::Char;
  }
  impl<'largs> StringOpIndexArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::string::String) -> ::char::Char {
      let i = self;
      {
        let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_operator_index_to_output_const_int(original_self as *const ::string::String,
                                                                      i,
                                                                      &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringOpIndexArgs<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs ::string::String) -> ::char::Char {
      let i = self;
      {
        let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_operator_index_to_output_const_unsigned_int(original_self as *const ::string::String, i, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::op_index_mut](../struct.String.html#method.op_index_mut) method.
  pub trait StringOpIndexMutArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::string::String) -> ::char_ref::CharRef;
  }
  impl<'largs> StringOpIndexMutArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::string::String) -> ::char_ref::CharRef {
      let i = self;
      {
        let mut object: ::char_ref::CharRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_operator_index_to_output_int(original_self as *mut ::string::String, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringOpIndexMutArgs<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs mut ::string::String) -> ::char_ref::CharRef {
      let i = self;
      {
        let mut object: ::char_ref::CharRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_operator_index_to_output_unsigned_int(original_self as *mut ::string::String,
                                                                         i,
                                                                         &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::op_le](../struct.String.html#method.op_le) method.
  pub trait StringOpLeArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> bool;
  }
  impl<'largs> StringOpLeArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_operator_le_QByteArray(original_self as *const ::string::String,
                                                        s as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> StringOpLeArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_operator_le_QLatin1String(original_self as *const ::string::String,
                                                           s as *const ::latin1_string::Latin1String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::op_lt](../struct.String.html#method.op_lt) method.
  pub trait StringOpLtArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> bool;
  }
  impl<'largs> StringOpLtArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_operator_lt_QByteArray(original_self as *const ::string::String,
                                                        s as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> StringOpLtArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_operator_lt_QLatin1String(original_self as *const ::string::String,
                                                           s as *const ::latin1_string::Latin1String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::op_neq](../struct.String.html#method.op_neq) method.
  pub trait StringOpNeqArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> bool;
  }
  impl<'largs> StringOpNeqArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_operator_neq_QByteArray(original_self as *const ::string::String,
                                                         s as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> StringOpNeqArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_operator_neq_QLatin1String(original_self as *const ::string::String,
                                                            s as *const ::latin1_string::Latin1String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::prepend](../struct.String.html#method.prepend) method.
  pub trait StringPrependArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String;
  }
  impl<'largs> StringPrependArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let s = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_prepend_QByteArray(original_self as *mut ::string::String,
                                                    s as *const ::byte_array::ByteArray)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringPrependArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let c = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_prepend_QChar(original_self as *mut ::string::String,
                                               c as *const ::char::Char)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringPrependArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let s = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_prepend_QLatin1String(original_self as *mut ::string::String,
                                                       s as *const ::latin1_string::Latin1String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringPrependArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let s = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_prepend_QString(original_self as *mut ::string::String,
                                                 s as *const ::string::String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringPrependArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let s = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_prepend_QStringRef(original_self as *mut ::string::String,
                                                    s as *const ::string_ref::StringRef)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [String::prepend_unsafe](../struct.String.html#method.prepend_unsafe) method.
  pub trait StringPrependUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String;
  }
  impl<'largs> StringPrependUnsafeArgs<'largs> for (*const ::char::Char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let uc = self.0;
      let len = self.1;
      let ffi_result = ::ffi::qt_core_c_QString_prepend_QChar_int(original_self as *mut ::string::String, uc, len);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringPrependUnsafeArgs<'largs> for *const ::libc::c_char {
    unsafe fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let s = self;
      let ffi_result = ::ffi::qt_core_c_QString_prepend_char(original_self as *mut ::string::String, s);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [String::push_back](../struct.String.html#method.push_back) method.
  pub trait StringPushBackArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::string::String) -> ();
  }
  impl<'largs> StringPushBackArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs mut ::string::String) -> () {
      let c = self;
      unsafe {
        ::ffi::qt_core_c_QString_push_back_c(original_self as *mut ::string::String,
                                             c as *const ::char::Char)
      }
    }
  }
  impl<'largs> StringPushBackArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::string::String) -> () {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_push_back_s(original_self as *mut ::string::String,
                                             s as *const ::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::push_front](../struct.String.html#method.push_front) method.
  pub trait StringPushFrontArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::string::String) -> ();
  }
  impl<'largs> StringPushFrontArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs mut ::string::String) -> () {
      let c = self;
      unsafe {
        ::ffi::qt_core_c_QString_push_front_c(original_self as *mut ::string::String,
                                              c as *const ::char::Char)
      }
    }
  }
  impl<'largs> StringPushFrontArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::string::String) -> () {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_push_front_s(original_self as *mut ::string::String,
                                              s as *const ::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::remove](../struct.String.html#method.remove) method.
  pub trait StringRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String;
  }
  impl<'largs> StringRemoveArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let c = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_remove_c(original_self as *mut ::string::String,
                                          c as *const ::char::Char)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringRemoveArgs<'largs> for (&'largs ::char::Char, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let c = self.0;
      let cs = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_remove_c_cs(original_self as *mut ::string::String,
                                             c as *const ::char::Char,
                                             cs as *const ::qt::CaseSensitivity)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let i = self.0;
      let len = self.1;
      let ffi_result = unsafe { ::ffi::qt_core_c_QString_remove_i_len(original_self as *mut ::string::String, i, len) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringRemoveArgs<'largs> for &'largs ::regular_expression::RegularExpression {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let re = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_remove_re(original_self as *mut ::string::String,
                                           re as *const ::regular_expression::RegularExpression)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringRemoveArgs<'largs> for &'largs ::reg_exp::RegExp {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let rx = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_remove_rx(original_self as *mut ::string::String,
                                           rx as *const ::reg_exp::RegExp)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringRemoveArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let s = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_remove_s(original_self as *mut ::string::String,
                                          s as *const ::string::String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringRemoveArgs<'largs> for (&'largs ::string::String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let s = self.0;
      let cs = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_remove_s_cs(original_self as *mut ::string::String,
                                             s as *const ::string::String,
                                             cs as *const ::qt::CaseSensitivity)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [String::replace](../struct.String.html#method.replace) method.
  pub trait StringReplaceArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String;
  }
  impl<'largs> StringReplaceArgs<'largs> for (&'largs ::char::Char, &'largs ::char::Char) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let before = self.0;
      let after = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_replace_QChar_QChar(original_self as *mut ::string::String,
                                                     before as *const ::char::Char,
                                                     after as *const ::char::Char)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceArgs<'largs> for (&'largs ::char::Char, &'largs ::char::Char, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let before = self.0;
      let after = self.1;
      let cs = self.2;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QString_replace_QChar_QChar_Qt_CaseSensitivity(original_self as *mut ::string::String,
                                                                          before as *const ::char::Char,
                                                                          after as *const ::char::Char,
                                                                          cs as *const ::qt::CaseSensitivity)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceArgs<'largs> for (&'largs ::char::Char, &'largs ::latin1_string::Latin1String) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let c = self.0;
      let after = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QString_replace_QChar_QLatin1String(original_self as *mut ::string::String,
                                                               c as *const ::char::Char,
                                                               after as *const ::latin1_string::Latin1String)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceArgs<'largs>
    for (&'largs ::char::Char, &'largs ::latin1_string::Latin1String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let c = self.0;
      let after = self.1;
      let cs = self.2;
      let ffi_result = unsafe { ::ffi::qt_core_c_QString_replace_QChar_QLatin1String_Qt_CaseSensitivity(original_self as *mut ::string::String, c as *const ::char::Char, after as *const ::latin1_string::Latin1String, cs as *const ::qt::CaseSensitivity) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceArgs<'largs> for (&'largs ::char::Char, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let c = self.0;
      let after = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_replace_QChar_QString(original_self as *mut ::string::String,
                                                       c as *const ::char::Char,
                                                       after as *const ::string::String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceArgs<'largs>
    for (&'largs ::char::Char, &'largs ::string::String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let c = self.0;
      let after = self.1;
      let cs = self.2;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QString_replace_QChar_QString_Qt_CaseSensitivity(original_self as *mut ::string::String,
                                                                            c as *const ::char::Char,
                                                                            after as *const ::string::String,
                                                                            cs as *const ::qt::CaseSensitivity)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceArgs<'largs>
    for (&'largs ::latin1_string::Latin1String, &'largs ::latin1_string::Latin1String) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let before = self.0;
      let after = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QString_replace_QLatin1String_QLatin1String(original_self as *mut ::string::String,
                                                                       before as *const ::latin1_string::Latin1String,
                                                                       after as *const ::latin1_string::Latin1String)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceArgs<'largs>
    for (&'largs ::latin1_string::Latin1String, &'largs ::latin1_string::Latin1String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let before = self.0;
      let after = self.1;
      let cs = self.2;
      let ffi_result = unsafe { ::ffi::qt_core_c_QString_replace_QLatin1String_QLatin1String_Qt_CaseSensitivity(original_self as *mut ::string::String, before as *const ::latin1_string::Latin1String, after as *const ::latin1_string::Latin1String, cs as *const ::qt::CaseSensitivity) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceArgs<'largs> for (&'largs ::latin1_string::Latin1String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let before = self.0;
      let after = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QString_replace_QLatin1String_QString(original_self as *mut ::string::String,
                                                                 before as *const ::latin1_string::Latin1String,
                                                                 after as *const ::string::String)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceArgs<'largs>
    for (&'largs ::latin1_string::Latin1String, &'largs ::string::String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let before = self.0;
      let after = self.1;
      let cs = self.2;
      let ffi_result = unsafe { ::ffi::qt_core_c_QString_replace_QLatin1String_QString_Qt_CaseSensitivity(original_self as *mut ::string::String, before as *const ::latin1_string::Latin1String, after as *const ::string::String, cs as *const ::qt::CaseSensitivity) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceArgs<'largs> for (&'largs ::reg_exp::RegExp, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let rx = self.0;
      let after = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_replace_QRegExp_QString(original_self as *mut ::string::String,
                                                         rx as *const ::reg_exp::RegExp,
                                                         after as *const ::string::String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceArgs<'largs> for (&'largs ::regular_expression::RegularExpression, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let re = self.0;
      let after = self.1;
      let ffi_result = unsafe { ::ffi::qt_core_c_QString_replace_QRegularExpression_QString(original_self as *mut ::string::String, re as *const ::regular_expression::RegularExpression, after as *const ::string::String) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceArgs<'largs> for (&'largs ::string::String, &'largs ::latin1_string::Latin1String) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let before = self.0;
      let after = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QString_replace_QString_QLatin1String(original_self as *mut ::string::String,
                                                                 before as *const ::string::String,
                                                                 after as *const ::latin1_string::Latin1String)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceArgs<'largs>
    for (&'largs ::string::String, &'largs ::latin1_string::Latin1String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let before = self.0;
      let after = self.1;
      let cs = self.2;
      let ffi_result = unsafe { ::ffi::qt_core_c_QString_replace_QString_QLatin1String_Qt_CaseSensitivity(original_self as *mut ::string::String, before as *const ::string::String, after as *const ::latin1_string::Latin1String, cs as *const ::qt::CaseSensitivity) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceArgs<'largs> for (&'largs ::string::String, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let before = self.0;
      let after = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_replace_QString_QString(original_self as *mut ::string::String,
                                                         before as *const ::string::String,
                                                         after as *const ::string::String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceArgs<'largs>
    for (&'largs ::string::String, &'largs ::string::String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let before = self.0;
      let after = self.1;
      let cs = self.2;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QString_replace_QString_QString_Qt_CaseSensitivity(original_self as *mut ::string::String,
                                                                              before as *const ::string::String,
                                                                              after as *const ::string::String,
                                                                              cs as *const ::qt::CaseSensitivity)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::char::Char) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let i = self.0;
      let len = self.1;
      let after = self.2;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_replace_int_int_QChar(original_self as *mut ::string::String,
                                                       i,
                                                       len,
                                                       after as *const ::char::Char)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let i = self.0;
      let len = self.1;
      let after = self.2;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_replace_int_int_QString(original_self as *mut ::string::String,
                                                         i,
                                                         len,
                                                         after as *const ::string::String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [String::replace_unsafe](../struct.String.html#method.replace_unsafe) method.
  pub trait StringReplaceUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String;
  }
  impl<'largs> StringReplaceUnsafeArgs<'largs>
    for (*const ::char::Char, ::libc::c_int, *const ::char::Char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let before = self.0;
      let blen = self.1;
      let after = self.2;
      let alen = self.3;
      let ffi_result = ::ffi::qt_core_c_QString_replace_QChar_int_QChar_int(original_self as *mut ::string::String,
                                                                            before,
                                                                            blen,
                                                                            after,
                                                                            alen);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceUnsafeArgs<'largs>
    for (*const ::char::Char, ::libc::c_int, *const ::char::Char, ::libc::c_int, &'largs ::qt::CaseSensitivity) {
    unsafe fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let before = self.0;
      let blen = self.1;
      let after = self.2;
      let alen = self.3;
      let cs = self.4;
      let ffi_result = ::ffi::qt_core_c_QString_replace_QChar_int_QChar_int_Qt_CaseSensitivity(original_self as *mut ::string::String, before, blen, after, alen, cs as *const ::qt::CaseSensitivity);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringReplaceUnsafeArgs<'largs> for (::libc::c_int, ::libc::c_int, *const ::char::Char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let i = self.0;
      let len = self.1;
      let s = self.2;
      let slen = self.3;
      let ffi_result =
        ::ffi::qt_core_c_QString_replace_int_int_QChar_int(original_self as *mut ::string::String, i, len, s, slen);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [String::resize](../struct.String.html#method.resize) method.
  pub trait StringResizeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::string::String) -> ();
  }
  impl<'largs> StringResizeArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::string::String) -> () {
      let size = self;
      unsafe { ::ffi::qt_core_c_QString_resize_size(original_self as *mut ::string::String, size) }
    }
  }
  impl<'largs> StringResizeArgs<'largs> for (::libc::c_int, &'largs ::char::Char) {
    fn exec(self, original_self: &'largs mut ::string::String) -> () {
      let size = self.0;
      let fill_char = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_resize_size_fillChar(original_self as *mut ::string::String,
                                                      size,
                                                      fill_char as *const ::char::Char)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::right_justified](../struct.String.html#method.right_justified) method.
  pub trait StringRightJustifiedArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String;
  }
  impl<'largs> StringRightJustifiedArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let width = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_rightJustified_to_output_width(original_self as *const ::string::String,
                                                                  width,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringRightJustifiedArgs<'largs> for (::libc::c_int, &'largs ::char::Char) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let width = self.0;
      let fill = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_rightJustified_to_output_width_fill(original_self as *const ::string::String,
                                                                       width,
                                                                       fill as *const ::char::Char,
                                                                       &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringRightJustifiedArgs<'largs> for (::libc::c_int, &'largs ::char::Char, bool) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let width = self.0;
      let fill = self.1;
      let trunc = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_rightJustified_to_output_width_fill_trunc(original_self as *const ::string::String, width, fill as *const ::char::Char, trunc, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::section](../struct.String.html#method.section) method.
  pub trait StringSectionArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String;
  }
  impl<'largs> StringSectionArgs<'largs> for (&'largs ::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let in_sep = self.0;
      let start = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_section_to_output_in_sep_start(original_self as *const ::string::String,
                                                                  in_sep as *const ::string::String,
                                                                  start,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSectionArgs<'largs> for (&'largs ::string::String, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let in_sep = self.0;
      let start = self.1;
      let end = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_section_to_output_in_sep_start_end(original_self as *const ::string::String,
                                                                      in_sep as *const ::string::String,
                                                                      start,
                                                                      end,
                                                                      &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSectionArgs<'largs>
    for (&'largs ::string::String, ::libc::c_int, ::libc::c_int, ::flags::Flags<::string::SectionFlag>) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let in_sep = self.0;
      let start = self.1;
      let end = self.2;
      let flags = self.3;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_section_to_output_in_sep_start_end_flags(original_self as *const ::string::String,
                                                                            in_sep as *const ::string::String,
                                                                            start,
                                                                            end,
                                                                            flags.to_int() as ::libc::c_uint,
                                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSectionArgs<'largs> for (&'largs ::regular_expression::RegularExpression, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let re = self.0;
      let start = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_section_to_output_re_start(original_self as *const ::string::String,
                                                              re as *const ::regular_expression::RegularExpression,
                                                              start,
                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSectionArgs<'largs>
    for (&'largs ::regular_expression::RegularExpression, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let re = self.0;
      let start = self.1;
      let end = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_section_to_output_re_start_end(original_self as *const ::string::String, re as *const ::regular_expression::RegularExpression, start, end, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSectionArgs<'largs>
    for (&'largs ::regular_expression::RegularExpression,
                                                  ::libc::c_int,
                                                  ::libc::c_int,
                                                  ::flags::Flags<::string::SectionFlag>) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let re = self.0;
      let start = self.1;
      let end = self.2;
      let flags = self.3;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_section_to_output_re_start_end_flags(original_self as *const ::string::String, re as *const ::regular_expression::RegularExpression, start, end, flags.to_int() as ::libc::c_uint, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSectionArgs<'largs> for (&'largs ::reg_exp::RegExp, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let reg = self.0;
      let start = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_section_to_output_reg_start(original_self as *const ::string::String,
                                                               reg as *const ::reg_exp::RegExp,
                                                               start,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSectionArgs<'largs> for (&'largs ::reg_exp::RegExp, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let reg = self.0;
      let start = self.1;
      let end = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_section_to_output_reg_start_end(original_self as *const ::string::String,
                                                                   reg as *const ::reg_exp::RegExp,
                                                                   start,
                                                                   end,
                                                                   &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSectionArgs<'largs>
    for (&'largs ::reg_exp::RegExp, ::libc::c_int, ::libc::c_int, ::flags::Flags<::string::SectionFlag>) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let reg = self.0;
      let start = self.1;
      let end = self.2;
      let flags = self.3;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_section_to_output_reg_start_end_flags(original_self as *const ::string::String,
                                                                         reg as *const ::reg_exp::RegExp,
                                                                         start,
                                                                         end,
                                                                         flags.to_int() as ::libc::c_uint,
                                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSectionArgs<'largs> for (&'largs ::char::Char, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let sep = self.0;
      let start = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_section_to_output_sep_start(original_self as *const ::string::String,
                                                               sep as *const ::char::Char,
                                                               start,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSectionArgs<'largs> for (&'largs ::char::Char, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let sep = self.0;
      let start = self.1;
      let end = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_section_to_output_sep_start_end(original_self as *const ::string::String,
                                                                   sep as *const ::char::Char,
                                                                   start,
                                                                   end,
                                                                   &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSectionArgs<'largs>
    for (&'largs ::char::Char, ::libc::c_int, ::libc::c_int, ::flags::Flags<::string::SectionFlag>) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string::String {
      let sep = self.0;
      let start = self.1;
      let end = self.2;
      let flags = self.3;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_section_to_output_sep_start_end_flags(original_self as *const ::string::String,
                                                                         sep as *const ::char::Char,
                                                                         start,
                                                                         end,
                                                                         flags.to_int() as ::libc::c_uint,
                                                                         &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::set_num0](../struct.String.html#method.set_num0) method.
  pub trait StringSetNum0Args<'largs> {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String;
  }
  impl<'largs> StringSetNum0Args<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QString_setNum_double(original_self as *mut ::string::String, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringSetNum0Args<'largs> for (::libc::c_double, ::libc::c_char) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self.0;
      let f = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QString_setNum_double_char(original_self as *mut ::string::String, arg1, f) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringSetNum0Args<'largs> for (::libc::c_double, ::libc::c_char, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self.0;
      let f = self.1;
      let prec = self.2;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_setNum_double_char_int(original_self as *mut ::string::String, arg1, f, prec)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringSetNum0Args<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QString_setNum_int(original_self as *mut ::string::String, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringSetNum0Args<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self.0;
      let base = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QString_setNum_int_int(original_self as *mut ::string::String, arg1, base) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringSetNum0Args<'largs> for u64 {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QString_setNum_qulonglong(original_self as *mut ::string::String, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringSetNum0Args<'largs> for (u64, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self.0;
      let base = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QString_setNum_qulonglong_int(original_self as *mut ::string::String, arg1, base) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [String::set_num1](../struct.String.html#method.set_num1) method.
  pub trait StringSetNum1Args<'largs> {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String;
  }
  impl<'largs> StringSetNum1Args<'largs> for ::libc::c_float {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QString_setNum_float(original_self as *mut ::string::String, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringSetNum1Args<'largs> for (::libc::c_float, ::libc::c_char) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self.0;
      let f = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QString_setNum_float_char(original_self as *mut ::string::String, arg1, f) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringSetNum1Args<'largs> for (::libc::c_float, ::libc::c_char, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self.0;
      let f = self.1;
      let prec = self.2;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_setNum_float_char_int(original_self as *mut ::string::String, arg1, f, prec)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringSetNum1Args<'largs> for ::libc::c_long {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QString_setNum_long(original_self as *mut ::string::String, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringSetNum1Args<'largs> for (::libc::c_long, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self.0;
      let base = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QString_setNum_long_int(original_self as *mut ::string::String, arg1, base) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringSetNum1Args<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QString_setNum_unsigned_int(original_self as *mut ::string::String, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringSetNum1Args<'largs> for (::libc::c_uint, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self.0;
      let base = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QString_setNum_unsigned_int_int(original_self as *mut ::string::String, arg1, base) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [String::set_num2](../struct.String.html#method.set_num2) method.
  pub trait StringSetNum2Args<'largs> {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String;
  }
  impl<'largs> StringSetNum2Args<'largs> for i64 {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QString_setNum_qlonglong(original_self as *mut ::string::String, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringSetNum2Args<'largs> for (i64, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self.0;
      let base = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QString_setNum_qlonglong_int(original_self as *mut ::string::String, arg1, base) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringSetNum2Args<'largs> for ::libc::c_ulong {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QString_setNum_unsigned_long(original_self as *mut ::string::String, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringSetNum2Args<'largs> for (::libc::c_ulong, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self.0;
      let base = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_setNum_unsigned_long_int(original_self as *mut ::string::String, arg1, base)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [String::set_num3](../struct.String.html#method.set_num3) method.
  pub trait StringSetNum3Args<'largs> {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String;
  }
  impl<'largs> StringSetNum3Args<'largs> for ::libc::c_short {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QString_setNum_short(original_self as *mut ::string::String, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringSetNum3Args<'largs> for (::libc::c_short, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self.0;
      let base = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QString_setNum_short_int(original_self as *mut ::string::String, arg1, base) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringSetNum3Args<'largs> for ::libc::c_ushort {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QString_setNum_unsigned_short(original_self as *mut ::string::String, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> StringSetNum3Args<'largs> for (::libc::c_ushort, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::string::String) -> &'largs mut ::string::String {
      let arg1 = self.0;
      let base = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QString_setNum_unsigned_short_int(original_self as *mut ::string::String, arg1, base)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [String::split](../struct.String.html#method.split) method.
  pub trait StringSplitArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::string_list::StringList;
  }
  impl<'largs> StringSplitArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs ::string::String) -> ::string_list::StringList {
      let sep = self;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_split_to_output_QChar(original_self as *const ::string::String,
                                                         sep as *const ::char::Char,
                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSplitArgs<'largs> for (&'largs ::char::Char, ::string::SplitBehavior) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string_list::StringList {
      let sep = self.0;
      let behavior = self.1;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_split_to_output_QChar_QString_SplitBehavior(original_self as *const ::string::String, sep as *const ::char::Char, behavior, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSplitArgs<'largs>
    for (&'largs ::char::Char, ::string::SplitBehavior, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string_list::StringList {
      let sep = self.0;
      let behavior = self.1;
      let cs = self.2;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_split_to_output_QChar_QString_SplitBehavior_Qt_CaseSensitivity(original_self as *const ::string::String, sep as *const ::char::Char, behavior, cs as *const ::qt::CaseSensitivity, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSplitArgs<'largs> for &'largs ::reg_exp::RegExp {
    fn exec(self, original_self: &'largs ::string::String) -> ::string_list::StringList {
      let sep = self;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_split_to_output_QRegExp(original_self as *const ::string::String,
                                                           sep as *const ::reg_exp::RegExp,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSplitArgs<'largs> for (&'largs ::reg_exp::RegExp, ::string::SplitBehavior) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string_list::StringList {
      let sep = self.0;
      let behavior = self.1;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_split_to_output_QRegExp_QString_SplitBehavior(original_self as *const ::string::String, sep as *const ::reg_exp::RegExp, behavior, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSplitArgs<'largs> for &'largs ::regular_expression::RegularExpression {
    fn exec(self, original_self: &'largs ::string::String) -> ::string_list::StringList {
      let sep = self;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_split_to_output_QRegularExpression(original_self as *const ::string::String, sep as *const ::regular_expression::RegularExpression, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSplitArgs<'largs> for (&'largs ::regular_expression::RegularExpression, ::string::SplitBehavior) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string_list::StringList {
      let sep = self.0;
      let behavior = self.1;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_split_to_output_QRegularExpression_QString_SplitBehavior(original_self as *const ::string::String, sep as *const ::regular_expression::RegularExpression, behavior, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSplitArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string::String) -> ::string_list::StringList {
      let sep = self;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_split_to_output_QString(original_self as *const ::string::String,
                                                           sep as *const ::string::String,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSplitArgs<'largs> for (&'largs ::string::String, ::string::SplitBehavior) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string_list::StringList {
      let sep = self.0;
      let behavior = self.1;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_split_to_output_QString_QString_SplitBehavior(original_self as *const ::string::String, sep as *const ::string::String, behavior, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSplitArgs<'largs>
    for (&'largs ::string::String, ::string::SplitBehavior, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> ::string_list::StringList {
      let sep = self.0;
      let behavior = self.1;
      let cs = self.2;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_split_to_output_QString_QString_SplitBehavior_Qt_CaseSensitivity(original_self as *const ::string::String, sep as *const ::string::String, behavior, cs as *const ::qt::CaseSensitivity, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::split_ref](../struct.String.html#method.split_ref) method.
  pub trait StringSplitRefArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> ::vector::VectorStringRef;
  }
  impl<'largs> StringSplitRefArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs ::string::String) -> ::vector::VectorStringRef {
      let sep = self;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_splitRef_to_output_QChar(original_self as *const ::string::String,
                                                            sep as *const ::char::Char,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSplitRefArgs<'largs> for (&'largs ::char::Char, ::string::SplitBehavior) {
    fn exec(self, original_self: &'largs ::string::String) -> ::vector::VectorStringRef {
      let sep = self.0;
      let behavior = self.1;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_splitRef_to_output_QChar_QString_SplitBehavior(original_self as *const ::string::String, sep as *const ::char::Char, behavior, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSplitRefArgs<'largs>
    for (&'largs ::char::Char, ::string::SplitBehavior, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> ::vector::VectorStringRef {
      let sep = self.0;
      let behavior = self.1;
      let cs = self.2;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_splitRef_to_output_QChar_QString_SplitBehavior_Qt_CaseSensitivity(original_self as *const ::string::String, sep as *const ::char::Char, behavior, cs as *const ::qt::CaseSensitivity, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSplitRefArgs<'largs> for &'largs ::reg_exp::RegExp {
    fn exec(self, original_self: &'largs ::string::String) -> ::vector::VectorStringRef {
      let sep = self;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_splitRef_to_output_QRegExp(original_self as *const ::string::String,
                                                              sep as *const ::reg_exp::RegExp,
                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSplitRefArgs<'largs> for (&'largs ::reg_exp::RegExp, ::string::SplitBehavior) {
    fn exec(self, original_self: &'largs ::string::String) -> ::vector::VectorStringRef {
      let sep = self.0;
      let behavior = self.1;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_splitRef_to_output_QRegExp_QString_SplitBehavior(original_self as *const ::string::String, sep as *const ::reg_exp::RegExp, behavior, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSplitRefArgs<'largs> for &'largs ::regular_expression::RegularExpression {
    fn exec(self, original_self: &'largs ::string::String) -> ::vector::VectorStringRef {
      let sep = self;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_splitRef_to_output_QRegularExpression(original_self as *const ::string::String, sep as *const ::regular_expression::RegularExpression, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSplitRefArgs<'largs> for (&'largs ::regular_expression::RegularExpression, ::string::SplitBehavior) {
    fn exec(self, original_self: &'largs ::string::String) -> ::vector::VectorStringRef {
      let sep = self.0;
      let behavior = self.1;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_splitRef_to_output_QRegularExpression_QString_SplitBehavior(original_self as *const ::string::String, sep as *const ::regular_expression::RegularExpression, behavior, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSplitRefArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string::String) -> ::vector::VectorStringRef {
      let sep = self;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_splitRef_to_output_QString(original_self as *const ::string::String,
                                                              sep as *const ::string::String,
                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSplitRefArgs<'largs> for (&'largs ::string::String, ::string::SplitBehavior) {
    fn exec(self, original_self: &'largs ::string::String) -> ::vector::VectorStringRef {
      let sep = self.0;
      let behavior = self.1;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_splitRef_to_output_QString_QString_SplitBehavior(original_self as *const ::string::String, sep as *const ::string::String, behavior, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StringSplitRefArgs<'largs>
    for (&'largs ::string::String, ::string::SplitBehavior, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> ::vector::VectorStringRef {
      let sep = self.0;
      let behavior = self.1;
      let cs = self.2;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_splitRef_to_output_QString_QString_SplitBehavior_Qt_CaseSensitivity(original_self as *const ::string::String, sep as *const ::string::String, behavior, cs as *const ::qt::CaseSensitivity, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::starts_with](../struct.String.html#method.starts_with) method.
  pub trait StringStartsWithArgs<'largs> {
    fn exec(self, original_self: &'largs ::string::String) -> bool;
  }
  impl<'largs> StringStartsWithArgs<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let c = self;
      unsafe {
        ::ffi::qt_core_c_QString_startsWith_QChar(original_self as *const ::string::String,
                                                  c as *const ::char::Char)
      }
    }
  }
  impl<'largs> StringStartsWithArgs<'largs> for (&'largs ::char::Char, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let c = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_startsWith_QChar_Qt_CaseSensitivity(original_self as *const ::string::String,
                                                                     c as *const ::char::Char,
                                                                     cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringStartsWithArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_startsWith_QLatin1String(original_self as *const ::string::String,
                                                          s as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'largs> StringStartsWithArgs<'largs> for (&'largs ::latin1_string::Latin1String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_startsWith_QLatin1String_Qt_CaseSensitivity(original_self as *const ::string::String, s as *const ::latin1_string::Latin1String, cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringStartsWithArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_startsWith_QString(original_self as *const ::string::String,
                                                    s as *const ::string::String)
      }
    }
  }
  impl<'largs> StringStartsWithArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QString_startsWith_QStringRef(original_self as *const ::string::String,
                                                       s as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'largs> StringStartsWithArgs<'largs> for (&'largs ::string_ref::StringRef, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_startsWith_QStringRef_Qt_CaseSensitivity(original_self as *const ::string::String,
                                                                          s as *const ::string_ref::StringRef,
                                                                          cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  impl<'largs> StringStartsWithArgs<'largs> for (&'largs ::string::String, &'largs ::qt::CaseSensitivity) {
    fn exec(self, original_self: &'largs ::string::String) -> bool {
      let s = self.0;
      let cs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_startsWith_QString_Qt_CaseSensitivity(original_self as *const ::string::String,
                                                                       s as *const ::string::String,
                                                                       cs as *const ::qt::CaseSensitivity)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [String::to_int_unsafe](../struct.String.html#method.to_int_unsafe) method.
  pub trait StringToIntUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int;
  }
  impl<'largs> StringToIntUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let ok = self;
      ::ffi::qt_core_c_QString_toInt_ok(original_self as *const ::string::String, ok)
    }
  }
  impl<'largs> StringToIntUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_int {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QString_toInt_ok_base(original_self as *const ::string::String, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [String::to_long_long_unsafe](../struct.String.html#method.to_long_long_unsafe) method.
  pub trait StringToLongLongUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> i64;
  }
  impl<'largs> StringToLongLongUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> i64 {
      let ok = self;
      ::ffi::qt_core_c_QString_toLongLong_ok(original_self as *const ::string::String, ok)
    }
  }
  impl<'largs> StringToLongLongUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> i64 {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QString_toLongLong_ok_base(original_self as *const ::string::String, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [String::to_long_unsafe](../struct.String.html#method.to_long_unsafe) method.
  pub trait StringToLongUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_long;
  }
  impl<'largs> StringToLongUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_long {
      let ok = self;
      ::ffi::qt_core_c_QString_toLong_ok(original_self as *const ::string::String, ok)
    }
  }
  impl<'largs> StringToLongUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_long {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QString_toLong_ok_base(original_self as *const ::string::String, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [String::to_short_unsafe](../struct.String.html#method.to_short_unsafe) method.
  pub trait StringToShortUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_short;
  }
  impl<'largs> StringToShortUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_short {
      let ok = self;
      ::ffi::qt_core_c_QString_toShort_ok(original_self as *const ::string::String, ok)
    }
  }
  impl<'largs> StringToShortUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_short {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QString_toShort_ok_base(original_self as *const ::string::String, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [String::to_u_int_unsafe](../struct.String.html#method.to_u_int_unsafe) method.
  pub trait StringToUIntUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_uint;
  }
  impl<'largs> StringToUIntUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_uint {
      let ok = self;
      ::ffi::qt_core_c_QString_toUInt_ok(original_self as *const ::string::String, ok)
    }
  }
  impl<'largs> StringToUIntUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_uint {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QString_toUInt_ok_base(original_self as *const ::string::String, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [String::to_u_long_long_unsafe](../struct.String.html#method.to_u_long_long_unsafe) method.
  pub trait StringToULongLongUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> u64;
  }
  impl<'largs> StringToULongLongUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> u64 {
      let ok = self;
      ::ffi::qt_core_c_QString_toULongLong_ok(original_self as *const ::string::String, ok)
    }
  }
  impl<'largs> StringToULongLongUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> u64 {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QString_toULongLong_ok_base(original_self as *const ::string::String, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [String::to_u_long_unsafe](../struct.String.html#method.to_u_long_unsafe) method.
  pub trait StringToULongUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_ulong;
  }
  impl<'largs> StringToULongUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_ulong {
      let ok = self;
      ::ffi::qt_core_c_QString_toULong_ok(original_self as *const ::string::String, ok)
    }
  }
  impl<'largs> StringToULongUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_ulong {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QString_toULong_ok_base(original_self as *const ::string::String, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [String::to_u_short_unsafe](../struct.String.html#method.to_u_short_unsafe) method.
  pub trait StringToUShortUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_ushort;
  }
  impl<'largs> StringToUShortUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_ushort {
      let ok = self;
      ::ffi::qt_core_c_QString_toUShort_ok(original_self as *const ::string::String, ok)
    }
  }
  impl<'largs> StringToUShortUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::string::String) -> ::libc::c_ushort {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QString_toUShort_ok_base(original_self as *const ::string::String, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [op_add](../fn.op_add.html) method.
  pub trait OpAddArgs {
    fn exec(self) -> ::string::String;
  }
  impl<'a> OpAddArgs for (&'a ::byte_array::ByteArray, &'a ::string::String) {
    fn exec(self) -> ::string::String {
      let ba = self.0;
      let s = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_G_operator_add_to_output_QByteArray_ba_QString_s(ba as *const ::byte_array::ByteArray, s as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::char::Char, &'a ::string_ref::StringRef) {
    fn exec(self) -> ::string::String {
      let s1 = self.0;
      let s2 = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_G_operator_add_to_output_QChar_s1_QStringRef_s2(s1 as *const ::char::Char, s2 as *const ::string_ref::StringRef, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::char::Char, &'a ::string::String) {
    fn exec(self) -> ::string::String {
      let s1 = self.0;
      let s2 = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_G_operator_add_to_output_QChar_s1_QString_s2(s1 as *const ::char::Char,
                                                                                s2 as *const ::string::String,
                                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::latin1_string::Latin1String, &'a ::string_ref::StringRef) {
    fn exec(self) -> ::string::String {
      let s1 = self.0;
      let s2 = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_G_operator_add_to_output_QLatin1String_s1_QStringRef_s2(s1 as *const ::latin1_string::Latin1String, s2 as *const ::string_ref::StringRef, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::string_ref::StringRef, &'a ::char::Char) {
    fn exec(self) -> ::string::String {
      let s1 = self.0;
      let s2 = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_G_operator_add_to_output_QStringRef_s1_QChar_s2(s1 as *const ::string_ref::StringRef, s2 as *const ::char::Char, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::string_ref::StringRef, &'a ::latin1_string::Latin1String) {
    fn exec(self) -> ::string::String {
      let s1 = self.0;
      let s2 = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_G_operator_add_to_output_QStringRef_s1_QLatin1String_s2(s1 as *const ::string_ref::StringRef, s2 as *const ::latin1_string::Latin1String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::string_ref::StringRef, &'a ::string_ref::StringRef) {
    fn exec(self) -> ::string::String {
      let s1 = self.0;
      let s2 = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_G_operator_add_to_output_QStringRef_s1_QStringRef_s2(s1 as *const ::string_ref::StringRef, s2 as *const ::string_ref::StringRef, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::string_ref::StringRef, &'a ::string::String) {
    fn exec(self) -> ::string::String {
      let s1 = self.0;
      let s2 = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_G_operator_add_to_output_QStringRef_s1_QString_s2(s1 as *const ::string_ref::StringRef, s2 as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::string::String, &'a ::char::Char) {
    fn exec(self) -> ::string::String {
      let s1 = self.0;
      let s2 = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_G_operator_add_to_output_QString_s1_QChar_s2(s1 as *const ::string::String,
                                                                                s2 as *const ::char::Char,
                                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::string::String, &'a ::string_ref::StringRef) {
    fn exec(self) -> ::string::String {
      let s1 = self.0;
      let s2 = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_G_operator_add_to_output_QString_s1_QStringRef_s2(s1 as *const ::string::String, s2 as *const ::string_ref::StringRef, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::string::String {
      let s1 = self.0;
      let s2 = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_G_operator_add_to_output_QString_s1_QString_s2(s1 as *const ::string::String,
                                                                                  s2 as *const ::string::String,
                                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::string::String, &'a ::byte_array::ByteArray) {
    fn exec(self) -> ::string::String {
      let s = self.0;
      let ba = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_G_operator_add_to_output_QString_s_QByteArray_ba(s as *const ::string::String, ba as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::string::String, ::libc::c_char) {
    fn exec(self) -> ::string::String {
      let s = self.0;
      let c = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_G_operator_add_to_output_QString_s_char_c(s as *const ::string::String,
                                                                             c,
                                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (::libc::c_char, &'a ::string::String) {
    fn exec(self) -> ::string::String {
      let c = self.0;
      let s = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QString_G_operator_add_to_output_char_c_QString_s(c,
                                                                             s as *const ::string::String,
                                                                             &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_add_unsafe](../fn.op_add_unsafe.html) method.
  pub trait OpAddUnsafeArgs {
    unsafe fn exec(self) -> ::string::String;
  }
  impl<'a> OpAddUnsafeArgs for (&'a ::string::String, *const ::libc::c_char) {
    unsafe fn exec(self) -> ::string::String {
      let s1 = self.0;
      let s2 = self.1;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QString_G_operator_add_to_output_QString_s1_char_s2(s1 as *const ::string::String,
                                                                             s2,
                                                                             &mut object);
        object
      }
    }
  }
  impl<'a> OpAddUnsafeArgs for (*const ::libc::c_char, &'a ::string::String) {
    unsafe fn exec(self) -> ::string::String {
      let s1 = self.0;
      let s2 = self.1;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QString_G_operator_add_to_output_char_s1_QString_s2(s1,
                                                                             s2 as *const ::string::String,
                                                                             &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_eq](../fn.op_eq.html) method.
  pub trait OpEqArgs {
    fn exec(self) -> bool;
  }
  impl<'a> OpEqArgs for (&'a ::byte_array::ByteArray, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_eq_QByteArray_QStringRef(lhs as *const ::byte_array::ByteArray,
                                                                     rhs as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpEqArgs for (&'a ::latin1_string::Latin1String, &'a ::char::Char) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_eq_QLatin1String_QChar(lhs as *const ::latin1_string::Latin1String,
                                                                   rhs as *const ::char::Char)
      }
    }
  }
  impl<'a> OpEqArgs for (&'a ::latin1_string::Latin1String, &'a ::latin1_string::Latin1String) {
    fn exec(self) -> bool {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_eq_QLatin1String_QLatin1String(s1 as *const ::latin1_string::Latin1String,
                                                                           s2 as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'a> OpEqArgs for (&'a ::latin1_string::Latin1String, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_eq_QLatin1String_QStringRef(lhs as *const ::latin1_string::Latin1String,
                                                                        rhs as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpEqArgs for (&'a ::string_ref::StringRef, &'a ::byte_array::ByteArray) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_eq_QStringRef_QByteArray(lhs as *const ::string_ref::StringRef,
                                                                     rhs as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'a> OpEqArgs for (&'a ::string_ref::StringRef, &'a ::char::Char) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_eq_QStringRef_QChar(lhs as *const ::string_ref::StringRef,
                                                                rhs as *const ::char::Char)
      }
    }
  }
  impl<'a> OpEqArgs for (&'a ::string_ref::StringRef, &'a ::latin1_string::Latin1String) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_eq_QStringRef_QLatin1String(lhs as *const ::string_ref::StringRef,
                                                                        rhs as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'a> OpEqArgs for (&'a ::string_ref::StringRef, &'a ::string::String) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_eq_QStringRef_QString(lhs as *const ::string_ref::StringRef,
                                                                  rhs as *const ::string::String)
      }
    }
  }
  impl<'a> OpEqArgs for (&'a ::string_ref::StringRef, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_eq_QStringRef_QStringRef(s1 as *const ::string_ref::StringRef,
                                                                     s2 as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpEqArgs for (&'a ::string::String, &'a ::char::Char) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_eq_QString_QChar(lhs as *const ::string::String, rhs as *const ::char::Char)
      }
    }
  }
  impl<'a> OpEqArgs for (&'a ::string::String, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_eq_QString_QStringRef(lhs as *const ::string::String,
                                                                  rhs as *const ::string_ref::StringRef)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_ge](../fn.op_ge.html) method.
  pub trait OpGeArgs {
    fn exec(self) -> bool;
  }
  impl<'a> OpGeArgs for (&'a ::byte_array::ByteArray, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_ge_QByteArray_QStringRef(lhs as *const ::byte_array::ByteArray,
                                                                     rhs as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpGeArgs for (&'a ::char::Char, &'a ::latin1_string::Latin1String) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_ge_QChar_QLatin1String(lhs as *const ::char::Char,
                                                                   rhs as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'a> OpGeArgs for (&'a ::char::Char, &'a ::string::String) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_ge_QChar_QString(lhs as *const ::char::Char, rhs as *const ::string::String)
      }
    }
  }
  impl<'a> OpGeArgs for (&'a ::char::Char, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_ge_QChar_QStringRef(lhs as *const ::char::Char,
                                                                rhs as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpGeArgs for (&'a ::latin1_string::Latin1String, &'a ::char::Char) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_ge_QLatin1String_QChar(lhs as *const ::latin1_string::Latin1String,
                                                                   rhs as *const ::char::Char)
      }
    }
  }
  impl<'a> OpGeArgs for (&'a ::latin1_string::Latin1String, &'a ::latin1_string::Latin1String) {
    fn exec(self) -> bool {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_ge_QLatin1String_QLatin1String(s1 as *const ::latin1_string::Latin1String,
                                                                           s2 as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'a> OpGeArgs for (&'a ::latin1_string::Latin1String, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_ge_QLatin1String_QStringRef(lhs as *const ::latin1_string::Latin1String,
                                                                        rhs as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpGeArgs for (&'a ::string_ref::StringRef, &'a ::byte_array::ByteArray) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_ge_QStringRef_QByteArray(lhs as *const ::string_ref::StringRef,
                                                                     rhs as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'a> OpGeArgs for (&'a ::string_ref::StringRef, &'a ::char::Char) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_ge_QStringRef_QChar(lhs as *const ::string_ref::StringRef,
                                                                rhs as *const ::char::Char)
      }
    }
  }
  impl<'a> OpGeArgs for (&'a ::string_ref::StringRef, &'a ::latin1_string::Latin1String) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_ge_QStringRef_QLatin1String(lhs as *const ::string_ref::StringRef,
                                                                        rhs as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'a> OpGeArgs for (&'a ::string_ref::StringRef, &'a ::string::String) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_ge_QStringRef_QString(lhs as *const ::string_ref::StringRef,
                                                                  rhs as *const ::string::String)
      }
    }
  }
  impl<'a> OpGeArgs for (&'a ::string_ref::StringRef, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_ge_QStringRef_QStringRef(s1 as *const ::string_ref::StringRef,
                                                                     s2 as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpGeArgs for (&'a ::string::String, &'a ::char::Char) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_ge_QString_QChar(lhs as *const ::string::String, rhs as *const ::char::Char)
      }
    }
  }
  impl<'a> OpGeArgs for (&'a ::string::String, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_ge_QString_QStringRef(lhs as *const ::string::String,
                                                                  rhs as *const ::string_ref::StringRef)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_gt](../fn.op_gt.html) method.
  pub trait OpGtArgs {
    fn exec(self) -> bool;
  }
  impl<'a> OpGtArgs for (&'a ::byte_array::ByteArray, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_gt_QByteArray_QStringRef(lhs as *const ::byte_array::ByteArray,
                                                                     rhs as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpGtArgs for (&'a ::latin1_string::Latin1String, &'a ::char::Char) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_gt_QLatin1String_QChar(lhs as *const ::latin1_string::Latin1String,
                                                                   rhs as *const ::char::Char)
      }
    }
  }
  impl<'a> OpGtArgs for (&'a ::latin1_string::Latin1String, &'a ::latin1_string::Latin1String) {
    fn exec(self) -> bool {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_gt_QLatin1String_QLatin1String(s1 as *const ::latin1_string::Latin1String,
                                                                           s2 as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'a> OpGtArgs for (&'a ::latin1_string::Latin1String, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_gt_QLatin1String_QStringRef(lhs as *const ::latin1_string::Latin1String,
                                                                        rhs as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpGtArgs for (&'a ::string_ref::StringRef, &'a ::byte_array::ByteArray) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_gt_QStringRef_QByteArray(lhs as *const ::string_ref::StringRef,
                                                                     rhs as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'a> OpGtArgs for (&'a ::string_ref::StringRef, &'a ::char::Char) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_gt_QStringRef_QChar(lhs as *const ::string_ref::StringRef,
                                                                rhs as *const ::char::Char)
      }
    }
  }
  impl<'a> OpGtArgs for (&'a ::string_ref::StringRef, &'a ::latin1_string::Latin1String) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_gt_QStringRef_QLatin1String(lhs as *const ::string_ref::StringRef,
                                                                        rhs as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'a> OpGtArgs for (&'a ::string_ref::StringRef, &'a ::string::String) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_gt_QStringRef_QString(lhs as *const ::string_ref::StringRef,
                                                                  rhs as *const ::string::String)
      }
    }
  }
  impl<'a> OpGtArgs for (&'a ::string_ref::StringRef, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_gt_QStringRef_QStringRef(s1 as *const ::string_ref::StringRef,
                                                                     s2 as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpGtArgs for (&'a ::string::String, &'a ::char::Char) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_gt_QString_QChar(lhs as *const ::string::String, rhs as *const ::char::Char)
      }
    }
  }
  impl<'a> OpGtArgs for (&'a ::string::String, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_gt_QString_QStringRef(lhs as *const ::string::String,
                                                                  rhs as *const ::string_ref::StringRef)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_le](../fn.op_le.html) method.
  pub trait OpLeArgs {
    fn exec(self) -> bool;
  }
  impl<'a> OpLeArgs for (&'a ::byte_array::ByteArray, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_le_QByteArray_QStringRef(lhs as *const ::byte_array::ByteArray,
                                                                     rhs as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpLeArgs for (&'a ::char::Char, &'a ::latin1_string::Latin1String) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_le_QChar_QLatin1String(lhs as *const ::char::Char,
                                                                   rhs as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'a> OpLeArgs for (&'a ::char::Char, &'a ::string::String) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_le_QChar_QString(lhs as *const ::char::Char, rhs as *const ::string::String)
      }
    }
  }
  impl<'a> OpLeArgs for (&'a ::char::Char, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_le_QChar_QStringRef(lhs as *const ::char::Char,
                                                                rhs as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpLeArgs for (&'a ::latin1_string::Latin1String, &'a ::char::Char) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_le_QLatin1String_QChar(lhs as *const ::latin1_string::Latin1String,
                                                                   rhs as *const ::char::Char)
      }
    }
  }
  impl<'a> OpLeArgs for (&'a ::latin1_string::Latin1String, &'a ::latin1_string::Latin1String) {
    fn exec(self) -> bool {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_le_QLatin1String_QLatin1String(s1 as *const ::latin1_string::Latin1String,
                                                                           s2 as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'a> OpLeArgs for (&'a ::latin1_string::Latin1String, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_le_QLatin1String_QStringRef(lhs as *const ::latin1_string::Latin1String,
                                                                        rhs as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpLeArgs for (&'a ::string_ref::StringRef, &'a ::byte_array::ByteArray) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_le_QStringRef_QByteArray(lhs as *const ::string_ref::StringRef,
                                                                     rhs as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'a> OpLeArgs for (&'a ::string_ref::StringRef, &'a ::char::Char) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_le_QStringRef_QChar(lhs as *const ::string_ref::StringRef,
                                                                rhs as *const ::char::Char)
      }
    }
  }
  impl<'a> OpLeArgs for (&'a ::string_ref::StringRef, &'a ::latin1_string::Latin1String) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_le_QStringRef_QLatin1String(lhs as *const ::string_ref::StringRef,
                                                                        rhs as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'a> OpLeArgs for (&'a ::string_ref::StringRef, &'a ::string::String) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_le_QStringRef_QString(lhs as *const ::string_ref::StringRef,
                                                                  rhs as *const ::string::String)
      }
    }
  }
  impl<'a> OpLeArgs for (&'a ::string_ref::StringRef, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_le_QStringRef_QStringRef(s1 as *const ::string_ref::StringRef,
                                                                     s2 as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpLeArgs for (&'a ::string::String, &'a ::char::Char) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_le_QString_QChar(lhs as *const ::string::String, rhs as *const ::char::Char)
      }
    }
  }
  impl<'a> OpLeArgs for (&'a ::string::String, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_le_QString_QStringRef(lhs as *const ::string::String,
                                                                  rhs as *const ::string_ref::StringRef)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_lt](../fn.op_lt.html) method.
  pub trait OpLtArgs {
    fn exec(self) -> bool;
  }
  impl<'a> OpLtArgs for (&'a ::byte_array::ByteArray, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_lt_QByteArray_QStringRef(lhs as *const ::byte_array::ByteArray,
                                                                     rhs as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpLtArgs for (&'a ::latin1_string::Latin1String, &'a ::char::Char) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_lt_QLatin1String_QChar(lhs as *const ::latin1_string::Latin1String,
                                                                   rhs as *const ::char::Char)
      }
    }
  }
  impl<'a> OpLtArgs for (&'a ::latin1_string::Latin1String, &'a ::latin1_string::Latin1String) {
    fn exec(self) -> bool {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_lt_QLatin1String_QLatin1String(s1 as *const ::latin1_string::Latin1String,
                                                                           s2 as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'a> OpLtArgs for (&'a ::latin1_string::Latin1String, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_lt_QLatin1String_QStringRef(lhs as *const ::latin1_string::Latin1String,
                                                                        rhs as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpLtArgs for (&'a ::string_ref::StringRef, &'a ::byte_array::ByteArray) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_lt_QStringRef_QByteArray(lhs as *const ::string_ref::StringRef,
                                                                     rhs as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'a> OpLtArgs for (&'a ::string_ref::StringRef, &'a ::char::Char) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_lt_QStringRef_QChar(lhs as *const ::string_ref::StringRef,
                                                                rhs as *const ::char::Char)
      }
    }
  }
  impl<'a> OpLtArgs for (&'a ::string_ref::StringRef, &'a ::latin1_string::Latin1String) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_lt_QStringRef_QLatin1String(lhs as *const ::string_ref::StringRef,
                                                                        rhs as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'a> OpLtArgs for (&'a ::string_ref::StringRef, &'a ::string::String) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_lt_QStringRef_QString(lhs as *const ::string_ref::StringRef,
                                                                  rhs as *const ::string::String)
      }
    }
  }
  impl<'a> OpLtArgs for (&'a ::string_ref::StringRef, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_lt_QStringRef_QStringRef(s1 as *const ::string_ref::StringRef,
                                                                     s2 as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpLtArgs for (&'a ::string::String, &'a ::char::Char) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_lt_QString_QChar(lhs as *const ::string::String, rhs as *const ::char::Char)
      }
    }
  }
  impl<'a> OpLtArgs for (&'a ::string::String, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_lt_QString_QStringRef(lhs as *const ::string::String,
                                                                  rhs as *const ::string_ref::StringRef)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_neq](../fn.op_neq.html) method.
  pub trait OpNeqArgs {
    fn exec(self) -> bool;
  }
  impl<'a> OpNeqArgs for (&'a ::byte_array::ByteArray, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_neq_QByteArray_QStringRef(lhs as *const ::byte_array::ByteArray,
                                                                      rhs as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpNeqArgs for (&'a ::char::Char, &'a ::latin1_string::Latin1String) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_neq_QChar_QLatin1String(lhs as *const ::char::Char,
                                                                    rhs as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'a> OpNeqArgs for (&'a ::char::Char, &'a ::string::String) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_neq_QChar_QString(lhs as *const ::char::Char,
                                                              rhs as *const ::string::String)
      }
    }
  }
  impl<'a> OpNeqArgs for (&'a ::char::Char, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_neq_QChar_QStringRef(lhs as *const ::char::Char,
                                                                 rhs as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpNeqArgs for (&'a ::latin1_string::Latin1String, &'a ::char::Char) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_neq_QLatin1String_QChar(lhs as *const ::latin1_string::Latin1String,
                                                                    rhs as *const ::char::Char)
      }
    }
  }
  impl<'a> OpNeqArgs for (&'a ::latin1_string::Latin1String, &'a ::latin1_string::Latin1String) {
    fn exec(self) -> bool {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_neq_QLatin1String_QLatin1String(s1 as *const ::latin1_string::Latin1String, s2 as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'a> OpNeqArgs for (&'a ::latin1_string::Latin1String, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_neq_QLatin1String_QStringRef(lhs as *const ::latin1_string::Latin1String,
                                                                         rhs as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpNeqArgs for (&'a ::string_ref::StringRef, &'a ::byte_array::ByteArray) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_neq_QStringRef_QByteArray(lhs as *const ::string_ref::StringRef,
                                                                      rhs as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'a> OpNeqArgs for (&'a ::string_ref::StringRef, &'a ::char::Char) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_neq_QStringRef_QChar(lhs as *const ::string_ref::StringRef,
                                                                 rhs as *const ::char::Char)
      }
    }
  }
  impl<'a> OpNeqArgs for (&'a ::string_ref::StringRef, &'a ::latin1_string::Latin1String) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_neq_QStringRef_QLatin1String(lhs as *const ::string_ref::StringRef,
                                                                         rhs as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'a> OpNeqArgs for (&'a ::string_ref::StringRef, &'a ::string::String) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_neq_QStringRef_QString(lhs as *const ::string_ref::StringRef,
                                                                   rhs as *const ::string::String)
      }
    }
  }
  impl<'a> OpNeqArgs for (&'a ::string_ref::StringRef, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let s1 = self.0;
      let s2 = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_neq_QStringRef_QStringRef(s1 as *const ::string_ref::StringRef,
                                                                      s2 as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'a> OpNeqArgs for (&'a ::string::String, &'a ::char::Char) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_neq_QString_QChar(lhs as *const ::string::String,
                                                              rhs as *const ::char::Char)
      }
    }
  }
  impl<'a> OpNeqArgs for (&'a ::string::String, &'a ::string_ref::StringRef) {
    fn exec(self) -> bool {
      let lhs = self.0;
      let rhs = self.1;
      unsafe {
        ::ffi::qt_core_c_QString_G_operator_neq_QString_QStringRef(lhs as *const ::string::String,
                                                                   rhs as *const ::string_ref::StringRef)
      }
    }
  }
}



use libc::{c_char, c_int};
use std;

/// Allows to convert built-in strings `&str` to Qt strings
impl<'a> From<&'a str> for ::string::String {
  fn from(s: &'a str) -> ::string::String {
    ::string::String::from_std_str(s)
  }
}

/// Allows to convert Qt strings to `std` strings
impl<'a> From<&'a ::string::String> for ::std::string::String {
  fn from(s: &'a ::string::String) -> ::std::string::String {
    s.to_std_string()
  }
}

impl ::string::String {
  /// Creates Qt string from an `std` string.
  pub fn from_std_str<S: AsRef<str>>(s: S) -> ::string::String {
    let slice = s.as_ref().as_bytes();
    unsafe { ::string::String::from_utf8_unsafe((slice.as_ptr() as *const c_char, slice.len() as c_int)) }
  }

  /// Creates `std` string from a Qt string.
  pub fn to_std_string(&self) -> std::string::String {
    let buf = self.to_utf8();
    unsafe {
      let bytes = std::slice::from_raw_parts(buf.const_data() as *const u8, buf.count(()) as usize);
      std::str::from_utf8_unchecked(bytes).to_string()
    }
  }
}
