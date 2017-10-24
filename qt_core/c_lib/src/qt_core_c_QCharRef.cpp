#include "qt_core_c_QCharRef.h"

unsigned char qt_core_c_QCharRef_cell(const QCharRef* this_ptr) {
  return this_ptr->cell();
}

unsigned char qt_core_c_QCharRef_combiningClass(const QCharRef* this_ptr) {
  return this_ptr->combiningClass();
}

void qt_core_c_QCharRef_convert_to_QChar_to_output(const QCharRef* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->operator QChar());
}

void qt_core_c_QCharRef_decomposition_to_output(const QCharRef* this_ptr, QString* output) {
  new(output) QString(this_ptr->decomposition());
}

void qt_core_c_QCharRef_destructor(QCharRef* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

int qt_core_c_QCharRef_digitValue(const QCharRef* this_ptr) {
  return this_ptr->digitValue();
}

bool qt_core_c_QCharRef_hasMirrored(const QCharRef* this_ptr) {
  return this_ptr->hasMirrored();
}

bool qt_core_c_QCharRef_isDigit(const QCharRef* this_ptr) {
  return this_ptr->isDigit();
}

bool qt_core_c_QCharRef_isLetter(const QCharRef* this_ptr) {
  return this_ptr->isLetter();
}

bool qt_core_c_QCharRef_isLetterOrNumber(QCharRef* this_ptr) {
  return this_ptr->isLetterOrNumber();
}

bool qt_core_c_QCharRef_isLower(const QCharRef* this_ptr) {
  return this_ptr->isLower();
}

bool qt_core_c_QCharRef_isMark(const QCharRef* this_ptr) {
  return this_ptr->isMark();
}

bool qt_core_c_QCharRef_isNull(const QCharRef* this_ptr) {
  return this_ptr->isNull();
}

bool qt_core_c_QCharRef_isNumber(const QCharRef* this_ptr) {
  return this_ptr->isNumber();
}

bool qt_core_c_QCharRef_isPrint(const QCharRef* this_ptr) {
  return this_ptr->isPrint();
}

bool qt_core_c_QCharRef_isPunct(const QCharRef* this_ptr) {
  return this_ptr->isPunct();
}

bool qt_core_c_QCharRef_isSpace(const QCharRef* this_ptr) {
  return this_ptr->isSpace();
}

bool qt_core_c_QCharRef_isTitleCase(const QCharRef* this_ptr) {
  return this_ptr->isTitleCase();
}

bool qt_core_c_QCharRef_isUpper(const QCharRef* this_ptr) {
  return this_ptr->isUpper();
}

void qt_core_c_QCharRef_mirroredChar_to_output(const QCharRef* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->mirroredChar());
}

QCharRef* qt_core_c_QCharRef_operator_assign_QChar(QCharRef* this_ptr, const QChar* c) {
  return &this_ptr->operator=(*c);
}

QCharRef* qt_core_c_QCharRef_operator_assign_QCharRef(QCharRef* this_ptr, const QCharRef* c) {
  return &this_ptr->operator=(*c);
}

QCharRef* qt_core_c_QCharRef_operator_assign_char(QCharRef* this_ptr, char c) {
  return &this_ptr->operator=(c);
}

QCharRef* qt_core_c_QCharRef_operator_assign_int(QCharRef* this_ptr, int rc) {
  return &this_ptr->operator=(rc);
}

QCharRef* qt_core_c_QCharRef_operator_assign_short(QCharRef* this_ptr, short rc) {
  return &this_ptr->operator=(rc);
}

QCharRef* qt_core_c_QCharRef_operator_assign_unsigned_char(QCharRef* this_ptr, unsigned char c) {
  return &this_ptr->operator=(c);
}

QCharRef* qt_core_c_QCharRef_operator_assign_unsigned_int(QCharRef* this_ptr, unsigned int rc) {
  return &this_ptr->operator=(rc);
}

QCharRef* qt_core_c_QCharRef_operator_assign_unsigned_short(QCharRef* this_ptr, unsigned short rc) {
  return &this_ptr->operator=(rc);
}

unsigned char qt_core_c_QCharRef_row(const QCharRef* this_ptr) {
  return this_ptr->row();
}

void qt_core_c_QCharRef_setCell(QCharRef* this_ptr, unsigned char cell) {
  this_ptr->setCell(cell);
}

void qt_core_c_QCharRef_setRow(QCharRef* this_ptr, unsigned char row) {
  this_ptr->setRow(row);
}

char qt_core_c_QCharRef_toLatin1(const QCharRef* this_ptr) {
  return this_ptr->toLatin1();
}

void qt_core_c_QCharRef_toLower_to_output(const QCharRef* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->toLower());
}

void qt_core_c_QCharRef_toTitleCase_to_output(const QCharRef* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->toTitleCase());
}

void qt_core_c_QCharRef_toUpper_to_output(const QCharRef* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->toUpper());
}

unsigned short* qt_core_c_QCharRef_unicode(QCharRef* this_ptr) {
  return &this_ptr->unicode();
}

unsigned short qt_core_c_QCharRef_unicode_const(const QCharRef* this_ptr) {
  return this_ptr->unicode();
}

