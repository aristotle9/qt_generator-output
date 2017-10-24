#include "qt_core_c_QTextStream.h"

QTextStream* qt_core_c_QTextStream_G_bin(QTextStream* s) {
  return &bin(*s);
}

QTextStream* qt_core_c_QTextStream_G_bom(QTextStream* s) {
  return &bom(*s);
}

QTextStream* qt_core_c_QTextStream_G_center(QTextStream* s) {
  return &center(*s);
}

QTextStream* qt_core_c_QTextStream_G_dec(QTextStream* s) {
  return &dec(*s);
}

QTextStream* qt_core_c_QTextStream_G_endl(QTextStream* s) {
  return &endl(*s);
}

QTextStream* qt_core_c_QTextStream_G_fixed(QTextStream* s) {
  return &fixed(*s);
}

QTextStream* qt_core_c_QTextStream_G_flush(QTextStream* s) {
  return &flush(*s);
}

QTextStream* qt_core_c_QTextStream_G_forcepoint(QTextStream* s) {
  return &forcepoint(*s);
}

QTextStream* qt_core_c_QTextStream_G_forcesign(QTextStream* s) {
  return &forcesign(*s);
}

QTextStream* qt_core_c_QTextStream_G_hex(QTextStream* s) {
  return &hex(*s);
}

QTextStream* qt_core_c_QTextStream_G_left(QTextStream* s) {
  return &left(*s);
}

QTextStream* qt_core_c_QTextStream_G_lowercasebase(QTextStream* s) {
  return &lowercasebase(*s);
}

QTextStream* qt_core_c_QTextStream_G_lowercasedigits(QTextStream* s) {
  return &lowercasedigits(*s);
}

QTextStream* qt_core_c_QTextStream_G_noforcepoint(QTextStream* s) {
  return &noforcepoint(*s);
}

QTextStream* qt_core_c_QTextStream_G_noforcesign(QTextStream* s) {
  return &noforcesign(*s);
}

QTextStream* qt_core_c_QTextStream_G_noshowbase(QTextStream* s) {
  return &noshowbase(*s);
}

QTextStream* qt_core_c_QTextStream_G_oct(QTextStream* s) {
  return &oct(*s);
}

QTextStream* qt_core_c_QTextStream_G_operator_shl(QTextStream* s, const QTextStreamManipulator* m) {
  return &operator<<(*s, *m);
}

void qt_core_c_QTextStream_G_qSetFieldWidth_to_output(int width, QTextStreamManipulator* output) {
  new(output) QTextStreamManipulator(qSetFieldWidth(width));
}

void qt_core_c_QTextStream_G_qSetPadChar_to_output(const QChar* ch, QTextStreamManipulator* output) {
  new(output) QTextStreamManipulator(qSetPadChar(*ch));
}

void qt_core_c_QTextStream_G_qSetRealNumberPrecision_to_output(int precision, QTextStreamManipulator* output) {
  new(output) QTextStreamManipulator(qSetRealNumberPrecision(precision));
}

QTextStream* qt_core_c_QTextStream_G_reset(QTextStream* s) {
  return &reset(*s);
}

QTextStream* qt_core_c_QTextStream_G_right(QTextStream* s) {
  return &right(*s);
}

QTextStream* qt_core_c_QTextStream_G_scientific(QTextStream* s) {
  return &scientific(*s);
}

QTextStream* qt_core_c_QTextStream_G_showbase(QTextStream* s) {
  return &showbase(*s);
}

QTextStream* qt_core_c_QTextStream_G_uppercasebase(QTextStream* s) {
  return &uppercasebase(*s);
}

QTextStream* qt_core_c_QTextStream_G_uppercasedigits(QTextStream* s) {
  return &uppercasedigits(*s);
}

QTextStream* qt_core_c_QTextStream_G_ws(QTextStream* s) {
  return &ws(*s);
}

bool qt_core_c_QTextStream_atEnd(const QTextStream* this_ptr) {
  return this_ptr->atEnd();
}

bool qt_core_c_QTextStream_autoDetectUnicode(const QTextStream* this_ptr) {
  return this_ptr->autoDetectUnicode();
}

QTextCodec* qt_core_c_QTextStream_codec(const QTextStream* this_ptr) {
  return this_ptr->codec();
}

void qt_core_c_QTextStream_delete(QTextStream* this_ptr) {
  delete this_ptr;
}

QIODevice* qt_core_c_QTextStream_device(const QTextStream* this_ptr) {
  return this_ptr->device();
}

QTextStream::FieldAlignment qt_core_c_QTextStream_fieldAlignment(const QTextStream* this_ptr) {
  return this_ptr->fieldAlignment();
}

int qt_core_c_QTextStream_fieldWidth(const QTextStream* this_ptr) {
  return this_ptr->fieldWidth();
}

void qt_core_c_QTextStream_flush(QTextStream* this_ptr) {
  this_ptr->flush();
}

bool qt_core_c_QTextStream_generateByteOrderMark(const QTextStream* this_ptr) {
  return this_ptr->generateByteOrderMark();
}

int qt_core_c_QTextStream_integerBase(const QTextStream* this_ptr) {
  return this_ptr->integerBase();
}

void qt_core_c_QTextStream_locale_to_output(const QTextStream* this_ptr, QLocale* output) {
  new(output) QLocale(this_ptr->locale());
}

QTextStream* qt_core_c_QTextStream_new_device(QIODevice* device) {
  return new QTextStream(device);
}

QTextStream* qt_core_c_QTextStream_new_no_args() {
  return new QTextStream();
}

unsigned int qt_core_c_QTextStream_numberFlags(const QTextStream* this_ptr) {
  return uint(this_ptr->numberFlags());
}

QTextStream* qt_core_c_QTextStream_operator_shl_QByteArray_array(QTextStream* this_ptr, const QByteArray* array) {
  return &this_ptr->operator<<(*array);
}

QTextStream* qt_core_c_QTextStream_operator_shl_QChar_ch(QTextStream* this_ptr, const QChar* ch) {
  return &this_ptr->operator<<(*ch);
}

QTextStream* qt_core_c_QTextStream_operator_shl_QLatin1String_s(QTextStream* this_ptr, const QLatin1String* s) {
  return &this_ptr->operator<<(*s);
}

QTextStream* qt_core_c_QTextStream_operator_shl_QStringRef_s(QTextStream* this_ptr, const QStringRef* s) {
  return &this_ptr->operator<<(*s);
}

QTextStream* qt_core_c_QTextStream_operator_shl_QString_s(QTextStream* this_ptr, const QString* s) {
  return &this_ptr->operator<<(*s);
}

QTextStream* qt_core_c_QTextStream_operator_shl_char_c(QTextStream* this_ptr, const char* c) {
  return &this_ptr->operator<<(c);
}

QTextStream* qt_core_c_QTextStream_operator_shl_char_ch(QTextStream* this_ptr, char ch) {
  return &this_ptr->operator<<(ch);
}

QTextStream* qt_core_c_QTextStream_operator_shl_double_f(QTextStream* this_ptr, double f) {
  return &this_ptr->operator<<(f);
}

QTextStream* qt_core_c_QTextStream_operator_shl_float_f(QTextStream* this_ptr, float f) {
  return &this_ptr->operator<<(f);
}

QTextStream* qt_core_c_QTextStream_operator_shl_int_i(QTextStream* this_ptr, int i) {
  return &this_ptr->operator<<(i);
}

QTextStream* qt_core_c_QTextStream_operator_shl_long_i(QTextStream* this_ptr, long i) {
  return &this_ptr->operator<<(i);
}

QTextStream* qt_core_c_QTextStream_operator_shl_qlonglong_i(QTextStream* this_ptr, qlonglong i) {
  return &this_ptr->operator<<(i);
}

QTextStream* qt_core_c_QTextStream_operator_shl_qulonglong_i(QTextStream* this_ptr, qulonglong i) {
  return &this_ptr->operator<<(i);
}

QTextStream* qt_core_c_QTextStream_operator_shl_short_i(QTextStream* this_ptr, short i) {
  return &this_ptr->operator<<(i);
}

QTextStream* qt_core_c_QTextStream_operator_shl_unsigned_int_i(QTextStream* this_ptr, unsigned int i) {
  return &this_ptr->operator<<(i);
}

QTextStream* qt_core_c_QTextStream_operator_shl_unsigned_long_i(QTextStream* this_ptr, unsigned long i) {
  return &this_ptr->operator<<(i);
}

QTextStream* qt_core_c_QTextStream_operator_shl_unsigned_short_i(QTextStream* this_ptr, unsigned short i) {
  return &this_ptr->operator<<(i);
}

QTextStream* qt_core_c_QTextStream_operator_shl_void_ptr(QTextStream* this_ptr, const void* ptr) {
  return &this_ptr->operator<<(ptr);
}

QTextStream* qt_core_c_QTextStream_operator_shr_QByteArray_array(QTextStream* this_ptr, QByteArray* array) {
  return &this_ptr->operator>>(*array);
}

QTextStream* qt_core_c_QTextStream_operator_shr_QChar_ch(QTextStream* this_ptr, QChar* ch) {
  return &this_ptr->operator>>(*ch);
}

QTextStream* qt_core_c_QTextStream_operator_shr_QString_s(QTextStream* this_ptr, QString* s) {
  return &this_ptr->operator>>(*s);
}

QTextStream* qt_core_c_QTextStream_operator_shr_char_c(QTextStream* this_ptr, char* c) {
  return &this_ptr->operator>>(c);
}

QTextStream* qt_core_c_QTextStream_operator_shr_char_ch(QTextStream* this_ptr, char* ch) {
  return &this_ptr->operator>>(*ch);
}

QTextStream* qt_core_c_QTextStream_operator_shr_double_f(QTextStream* this_ptr, double* f) {
  return &this_ptr->operator>>(*f);
}

QTextStream* qt_core_c_QTextStream_operator_shr_float_f(QTextStream* this_ptr, float* f) {
  return &this_ptr->operator>>(*f);
}

QTextStream* qt_core_c_QTextStream_operator_shr_int_i(QTextStream* this_ptr, int* i) {
  return &this_ptr->operator>>(*i);
}

QTextStream* qt_core_c_QTextStream_operator_shr_long_i(QTextStream* this_ptr, long* i) {
  return &this_ptr->operator>>(*i);
}

QTextStream* qt_core_c_QTextStream_operator_shr_qlonglong_i(QTextStream* this_ptr, qlonglong* i) {
  return &this_ptr->operator>>(*i);
}

QTextStream* qt_core_c_QTextStream_operator_shr_qulonglong_i(QTextStream* this_ptr, qulonglong* i) {
  return &this_ptr->operator>>(*i);
}

QTextStream* qt_core_c_QTextStream_operator_shr_short_i(QTextStream* this_ptr, short* i) {
  return &this_ptr->operator>>(*i);
}

QTextStream* qt_core_c_QTextStream_operator_shr_unsigned_int_i(QTextStream* this_ptr, unsigned int* i) {
  return &this_ptr->operator>>(*i);
}

QTextStream* qt_core_c_QTextStream_operator_shr_unsigned_long_i(QTextStream* this_ptr, unsigned long* i) {
  return &this_ptr->operator>>(*i);
}

QTextStream* qt_core_c_QTextStream_operator_shr_unsigned_short_i(QTextStream* this_ptr, unsigned short* i) {
  return &this_ptr->operator>>(*i);
}

void qt_core_c_QTextStream_padChar_to_output(const QTextStream* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->padChar());
}

qint64 qt_core_c_QTextStream_pos(const QTextStream* this_ptr) {
  return this_ptr->pos();
}

void qt_core_c_QTextStream_readAll_to_output(QTextStream* this_ptr, QString* output) {
  new(output) QString(this_ptr->readAll());
}

bool qt_core_c_QTextStream_readLineInto_line(QTextStream* this_ptr, QString* line) {
  return this_ptr->readLineInto(line);
}

bool qt_core_c_QTextStream_readLineInto_line_maxlen(QTextStream* this_ptr, QString* line, qint64 maxlen) {
  return this_ptr->readLineInto(line, maxlen);
}

void qt_core_c_QTextStream_readLine_to_output_maxlen(QTextStream* this_ptr, qint64 maxlen, QString* output) {
  new(output) QString(this_ptr->readLine(maxlen));
}

void qt_core_c_QTextStream_readLine_to_output_no_args(QTextStream* this_ptr, QString* output) {
  new(output) QString(this_ptr->readLine());
}

void qt_core_c_QTextStream_read_to_output(QTextStream* this_ptr, qint64 maxlen, QString* output) {
  new(output) QString(this_ptr->read(maxlen));
}

QTextStream::RealNumberNotation qt_core_c_QTextStream_realNumberNotation(const QTextStream* this_ptr) {
  return this_ptr->realNumberNotation();
}

int qt_core_c_QTextStream_realNumberPrecision(const QTextStream* this_ptr) {
  return this_ptr->realNumberPrecision();
}

void qt_core_c_QTextStream_reset(QTextStream* this_ptr) {
  this_ptr->reset();
}

void qt_core_c_QTextStream_resetStatus(QTextStream* this_ptr) {
  this_ptr->resetStatus();
}

bool qt_core_c_QTextStream_seek(QTextStream* this_ptr, qint64 pos) {
  return this_ptr->seek(pos);
}

void qt_core_c_QTextStream_setAutoDetectUnicode(QTextStream* this_ptr, bool enabled) {
  this_ptr->setAutoDetectUnicode(enabled);
}

void qt_core_c_QTextStream_setCodec_codec(QTextStream* this_ptr, QTextCodec* codec) {
  this_ptr->setCodec(codec);
}

void qt_core_c_QTextStream_setCodec_codecName(QTextStream* this_ptr, const char* codecName) {
  this_ptr->setCodec(codecName);
}

void qt_core_c_QTextStream_setDevice(QTextStream* this_ptr, QIODevice* device) {
  this_ptr->setDevice(device);
}

void qt_core_c_QTextStream_setFieldAlignment(QTextStream* this_ptr, QTextStream::FieldAlignment alignment) {
  this_ptr->setFieldAlignment(alignment);
}

void qt_core_c_QTextStream_setFieldWidth(QTextStream* this_ptr, int width) {
  this_ptr->setFieldWidth(width);
}

void qt_core_c_QTextStream_setGenerateByteOrderMark(QTextStream* this_ptr, bool generate) {
  this_ptr->setGenerateByteOrderMark(generate);
}

void qt_core_c_QTextStream_setIntegerBase(QTextStream* this_ptr, int base) {
  this_ptr->setIntegerBase(base);
}

void qt_core_c_QTextStream_setLocale(QTextStream* this_ptr, const QLocale* locale) {
  this_ptr->setLocale(*locale);
}

void qt_core_c_QTextStream_setNumberFlags(QTextStream* this_ptr, unsigned int flags) {
  this_ptr->setNumberFlags(QFlags< QTextStream::NumberFlag >(flags));
}

void qt_core_c_QTextStream_setPadChar(QTextStream* this_ptr, const QChar* ch) {
  this_ptr->setPadChar(*ch);
}

void qt_core_c_QTextStream_setRealNumberNotation(QTextStream* this_ptr, QTextStream::RealNumberNotation notation) {
  this_ptr->setRealNumberNotation(notation);
}

void qt_core_c_QTextStream_setRealNumberPrecision(QTextStream* this_ptr, int precision) {
  this_ptr->setRealNumberPrecision(precision);
}

void qt_core_c_QTextStream_setStatus(QTextStream* this_ptr, QTextStream::Status status) {
  this_ptr->setStatus(status);
}

void qt_core_c_QTextStream_skipWhiteSpace(QTextStream* this_ptr) {
  this_ptr->skipWhiteSpace();
}

QTextStream::Status qt_core_c_QTextStream_status(const QTextStream* this_ptr) {
  return this_ptr->status();
}

QString* qt_core_c_QTextStream_string(const QTextStream* this_ptr) {
  return this_ptr->string();
}

