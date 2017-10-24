#include "qt_core_c_QChar.h"

bool qt_core_c_QChar_G_operator_ge(const QChar* c1, const QChar* c2) {
  return operator>=(*c1, *c2);
}

bool qt_core_c_QChar_G_operator_gt(const QChar* c1, const QChar* c2) {
  return operator>(*c1, *c2);
}

bool qt_core_c_QChar_G_operator_le(const QChar* c1, const QChar* c2) {
  return operator<=(*c1, *c2);
}

bool qt_core_c_QChar_G_operator_neq(const QChar* c1, const QChar* c2) {
  return operator!=(*c1, *c2);
}

QDataStream* qt_core_c_QChar_G_operator_shl(QDataStream* arg1, const QChar* arg2) {
  return &operator<<(*arg1, *arg2);
}

QDataStream* qt_core_c_QChar_G_operator_shr(QDataStream* arg1, QChar* arg2) {
  return &operator>>(*arg1, *arg2);
}

QChar::Category qt_core_c_QChar_category_no_args(const QChar* this_ptr) {
  return this_ptr->category();
}

QChar::Category qt_core_c_QChar_category_ucs4(unsigned int ucs4) {
  return QChar::category(ucs4);
}

unsigned char qt_core_c_QChar_cell(const QChar* this_ptr) {
  return this_ptr->cell();
}

unsigned char qt_core_c_QChar_combiningClass_no_args(const QChar* this_ptr) {
  return this_ptr->combiningClass();
}

unsigned char qt_core_c_QChar_combiningClass_ucs4(unsigned int ucs4) {
  return QChar::combiningClass(ucs4);
}

void qt_core_c_QChar_constructor_QChar_SpecialCharacter(QChar::SpecialCharacter s, QChar* output) {
  new(output) QChar(s);
}

void qt_core_c_QChar_constructor_QLatin1Char(const QLatin1Char* ch, QChar* output) {
  new(output) QChar(*ch);
}

void qt_core_c_QChar_constructor_char(char c, QChar* output) {
  new(output) QChar(c);
}

void qt_core_c_QChar_constructor_int(int rc, QChar* output) {
  new(output) QChar(rc);
}

void qt_core_c_QChar_constructor_no_args(QChar* output) {
  new(output) QChar();
}

void qt_core_c_QChar_constructor_short(short rc, QChar* output) {
  new(output) QChar(rc);
}

void qt_core_c_QChar_constructor_unsigned_char(unsigned char c, QChar* output) {
  new(output) QChar(c);
}

void qt_core_c_QChar_constructor_unsigned_char_unsigned_char(unsigned char c, unsigned char r, QChar* output) {
  new(output) QChar(c, r);
}

void qt_core_c_QChar_constructor_unsigned_int(unsigned int rc, QChar* output) {
  new(output) QChar(rc);
}

void qt_core_c_QChar_constructor_unsigned_short(unsigned short rc, QChar* output) {
  new(output) QChar(rc);
}

QChar::UnicodeVersion qt_core_c_QChar_currentUnicodeVersion() {
  return QChar::currentUnicodeVersion();
}

QChar::Decomposition qt_core_c_QChar_decompositionTag_no_args(const QChar* this_ptr) {
  return this_ptr->decompositionTag();
}

QChar::Decomposition qt_core_c_QChar_decompositionTag_ucs4(unsigned int ucs4) {
  return QChar::decompositionTag(ucs4);
}

void qt_core_c_QChar_decomposition_to_output_no_args(const QChar* this_ptr, QString* output) {
  new(output) QString(this_ptr->decomposition());
}

void qt_core_c_QChar_decomposition_to_output_ucs4(unsigned int ucs4, QString* output) {
  new(output) QString(QChar::decomposition(ucs4));
}

void qt_core_c_QChar_destructor(QChar* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

int qt_core_c_QChar_digitValue_no_args(const QChar* this_ptr) {
  return this_ptr->digitValue();
}

int qt_core_c_QChar_digitValue_ucs4(unsigned int ucs4) {
  return QChar::digitValue(ucs4);
}

QChar::Direction qt_core_c_QChar_direction_no_args(const QChar* this_ptr) {
  return this_ptr->direction();
}

QChar::Direction qt_core_c_QChar_direction_ucs4(unsigned int ucs4) {
  return QChar::direction(ucs4);
}

void qt_core_c_QChar_fromLatin1_to_output(char c, QChar* output) {
  new(output) QChar(QChar::fromLatin1(c));
}

bool qt_core_c_QChar_hasMirrored_no_args(const QChar* this_ptr) {
  return this_ptr->hasMirrored();
}

bool qt_core_c_QChar_hasMirrored_ucs4(unsigned int ucs4) {
  return QChar::hasMirrored(ucs4);
}

unsigned short qt_core_c_QChar_highSurrogate(unsigned int ucs4) {
  return QChar::highSurrogate(ucs4);
}

bool qt_core_c_QChar_isDigit_no_args(const QChar* this_ptr) {
  return this_ptr->isDigit();
}

bool qt_core_c_QChar_isDigit_ucs4(unsigned int ucs4) {
  return QChar::isDigit(ucs4);
}

bool qt_core_c_QChar_isHighSurrogate_no_args(const QChar* this_ptr) {
  return this_ptr->isHighSurrogate();
}

bool qt_core_c_QChar_isHighSurrogate_ucs4(unsigned int ucs4) {
  return QChar::isHighSurrogate(ucs4);
}

bool qt_core_c_QChar_isLetterOrNumber_no_args(const QChar* this_ptr) {
  return this_ptr->isLetterOrNumber();
}

bool qt_core_c_QChar_isLetterOrNumber_ucs4(unsigned int ucs4) {
  return QChar::isLetterOrNumber(ucs4);
}

bool qt_core_c_QChar_isLetter_no_args(const QChar* this_ptr) {
  return this_ptr->isLetter();
}

bool qt_core_c_QChar_isLetter_ucs4(unsigned int ucs4) {
  return QChar::isLetter(ucs4);
}

bool qt_core_c_QChar_isLowSurrogate_no_args(const QChar* this_ptr) {
  return this_ptr->isLowSurrogate();
}

bool qt_core_c_QChar_isLowSurrogate_ucs4(unsigned int ucs4) {
  return QChar::isLowSurrogate(ucs4);
}

bool qt_core_c_QChar_isLower_no_args(const QChar* this_ptr) {
  return this_ptr->isLower();
}

bool qt_core_c_QChar_isLower_ucs4(unsigned int ucs4) {
  return QChar::isLower(ucs4);
}

bool qt_core_c_QChar_isMark_no_args(const QChar* this_ptr) {
  return this_ptr->isMark();
}

bool qt_core_c_QChar_isMark_ucs4(unsigned int ucs4) {
  return QChar::isMark(ucs4);
}

bool qt_core_c_QChar_isNonCharacter_no_args(const QChar* this_ptr) {
  return this_ptr->isNonCharacter();
}

bool qt_core_c_QChar_isNonCharacter_ucs4(unsigned int ucs4) {
  return QChar::isNonCharacter(ucs4);
}

bool qt_core_c_QChar_isNull(const QChar* this_ptr) {
  return this_ptr->isNull();
}

bool qt_core_c_QChar_isNumber_no_args(const QChar* this_ptr) {
  return this_ptr->isNumber();
}

bool qt_core_c_QChar_isNumber_ucs4(unsigned int ucs4) {
  return QChar::isNumber(ucs4);
}

bool qt_core_c_QChar_isPrint_no_args(const QChar* this_ptr) {
  return this_ptr->isPrint();
}

bool qt_core_c_QChar_isPrint_ucs4(unsigned int ucs4) {
  return QChar::isPrint(ucs4);
}

bool qt_core_c_QChar_isPunct_no_args(const QChar* this_ptr) {
  return this_ptr->isPunct();
}

bool qt_core_c_QChar_isPunct_ucs4(unsigned int ucs4) {
  return QChar::isPunct(ucs4);
}

bool qt_core_c_QChar_isSpace_no_args(const QChar* this_ptr) {
  return this_ptr->isSpace();
}

bool qt_core_c_QChar_isSpace_ucs4(unsigned int ucs4) {
  return QChar::isSpace(ucs4);
}

bool qt_core_c_QChar_isSurrogate_no_args(const QChar* this_ptr) {
  return this_ptr->isSurrogate();
}

bool qt_core_c_QChar_isSurrogate_ucs4(unsigned int ucs4) {
  return QChar::isSurrogate(ucs4);
}

bool qt_core_c_QChar_isSymbol_no_args(const QChar* this_ptr) {
  return this_ptr->isSymbol();
}

bool qt_core_c_QChar_isSymbol_ucs4(unsigned int ucs4) {
  return QChar::isSymbol(ucs4);
}

bool qt_core_c_QChar_isTitleCase_no_args(const QChar* this_ptr) {
  return this_ptr->isTitleCase();
}

bool qt_core_c_QChar_isTitleCase_ucs4(unsigned int ucs4) {
  return QChar::isTitleCase(ucs4);
}

bool qt_core_c_QChar_isUpper_no_args(const QChar* this_ptr) {
  return this_ptr->isUpper();
}

bool qt_core_c_QChar_isUpper_ucs4(unsigned int ucs4) {
  return QChar::isUpper(ucs4);
}

QChar::JoiningType qt_core_c_QChar_joiningType_no_args(const QChar* this_ptr) {
  return this_ptr->joiningType();
}

QChar::JoiningType qt_core_c_QChar_joiningType_ucs4(unsigned int ucs4) {
  return QChar::joiningType(ucs4);
}

QChar::Joining qt_core_c_QChar_joining_no_args(const QChar* this_ptr) {
  return this_ptr->joining();
}

QChar::Joining qt_core_c_QChar_joining_ucs4(unsigned int ucs4) {
  return QChar::joining(ucs4);
}

unsigned short qt_core_c_QChar_lowSurrogate(unsigned int ucs4) {
  return QChar::lowSurrogate(ucs4);
}

unsigned int qt_core_c_QChar_mirroredChar(unsigned int ucs4) {
  return QChar::mirroredChar(ucs4);
}

void qt_core_c_QChar_mirroredChar_to_output(const QChar* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->mirroredChar());
}

bool qt_core_c_QChar_requiresSurrogates(unsigned int ucs4) {
  return QChar::requiresSurrogates(ucs4);
}

unsigned char qt_core_c_QChar_row(const QChar* this_ptr) {
  return this_ptr->row();
}

QChar::Script qt_core_c_QChar_script_no_args(const QChar* this_ptr) {
  return this_ptr->script();
}

QChar::Script qt_core_c_QChar_script_ucs4(unsigned int ucs4) {
  return QChar::script(ucs4);
}

void qt_core_c_QChar_setCell(QChar* this_ptr, unsigned char acell) {
  this_ptr->setCell(acell);
}

void qt_core_c_QChar_setRow(QChar* this_ptr, unsigned char arow) {
  this_ptr->setRow(arow);
}

unsigned int qt_core_c_QChar_surrogateToUcs4_QChar_QChar(const QChar* high, const QChar* low) {
  return QChar::surrogateToUcs4(*high, *low);
}

unsigned int qt_core_c_QChar_surrogateToUcs4_unsigned_short_unsigned_short(unsigned short high, unsigned short low) {
  return QChar::surrogateToUcs4(high, low);
}

unsigned int qt_core_c_QChar_toCaseFolded(unsigned int ucs4) {
  return QChar::toCaseFolded(ucs4);
}

void qt_core_c_QChar_toCaseFolded_to_output(const QChar* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->toCaseFolded());
}

char qt_core_c_QChar_toLatin1(const QChar* this_ptr) {
  return this_ptr->toLatin1();
}

unsigned int qt_core_c_QChar_toLower(unsigned int ucs4) {
  return QChar::toLower(ucs4);
}

void qt_core_c_QChar_toLower_to_output(const QChar* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->toLower());
}

unsigned int qt_core_c_QChar_toTitleCase(unsigned int ucs4) {
  return QChar::toTitleCase(ucs4);
}

void qt_core_c_QChar_toTitleCase_to_output(const QChar* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->toTitleCase());
}

unsigned int qt_core_c_QChar_toUpper(unsigned int ucs4) {
  return QChar::toUpper(ucs4);
}

void qt_core_c_QChar_toUpper_to_output(const QChar* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->toUpper());
}

unsigned short* qt_core_c_QChar_unicode(QChar* this_ptr) {
  return &this_ptr->unicode();
}

QChar::UnicodeVersion qt_core_c_QChar_unicodeVersion_no_args(const QChar* this_ptr) {
  return this_ptr->unicodeVersion();
}

QChar::UnicodeVersion qt_core_c_QChar_unicodeVersion_ucs4(unsigned int ucs4) {
  return QChar::unicodeVersion(ucs4);
}

unsigned short qt_core_c_QChar_unicode_const(const QChar* this_ptr) {
  return this_ptr->unicode();
}

