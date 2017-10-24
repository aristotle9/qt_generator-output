#include "qt_gui_c_QGlyphRun.h"

void qt_gui_c_QGlyphRun_G_swap(QGlyphRun* value1, QGlyphRun* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QGlyphRun_boundingRect_to_output(const QGlyphRun* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect());
}

void qt_gui_c_QGlyphRun_clear(QGlyphRun* this_ptr) {
  this_ptr->clear();
}

void qt_gui_c_QGlyphRun_constructor_no_args(QGlyphRun* output) {
  new(output) QGlyphRun();
}

void qt_gui_c_QGlyphRun_constructor_other(const QGlyphRun* other, QGlyphRun* output) {
  new(output) QGlyphRun(*other);
}

void qt_gui_c_QGlyphRun_destructor(QGlyphRun* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

unsigned int qt_gui_c_QGlyphRun_flags(const QGlyphRun* this_ptr) {
  return uint(this_ptr->flags());
}

void qt_gui_c_QGlyphRun_glyphIndexes_to_output(const QGlyphRun* this_ptr, QVector< quint32 >* output) {
  new(output) QVector< quint32 >(this_ptr->glyphIndexes());
}

bool qt_gui_c_QGlyphRun_isEmpty(const QGlyphRun* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_gui_c_QGlyphRun_isRightToLeft(const QGlyphRun* this_ptr) {
  return this_ptr->isRightToLeft();
}

QGlyphRun* qt_gui_c_QGlyphRun_operator_assign(QGlyphRun* this_ptr, const QGlyphRun* other) {
  return &this_ptr->operator=(*other);
}

bool qt_gui_c_QGlyphRun_operator_eq(const QGlyphRun* this_ptr, const QGlyphRun* other) {
  return this_ptr->operator==(*other);
}

bool qt_gui_c_QGlyphRun_operator_neq(const QGlyphRun* this_ptr, const QGlyphRun* other) {
  return this_ptr->operator!=(*other);
}

bool qt_gui_c_QGlyphRun_overline(const QGlyphRun* this_ptr) {
  return this_ptr->overline();
}

void qt_gui_c_QGlyphRun_positions_to_output(const QGlyphRun* this_ptr, QVector< QPointF >* output) {
  new(output) QVector< QPointF >(this_ptr->positions());
}

void qt_gui_c_QGlyphRun_rawFont_to_output(const QGlyphRun* this_ptr, QRawFont* output) {
  new(output) QRawFont(this_ptr->rawFont());
}

void qt_gui_c_QGlyphRun_setBoundingRect(QGlyphRun* this_ptr, const QRectF* boundingRect) {
  this_ptr->setBoundingRect(*boundingRect);
}

void qt_gui_c_QGlyphRun_setFlag_flag(QGlyphRun* this_ptr, QGlyphRun::GlyphRunFlag flag) {
  this_ptr->setFlag(flag);
}

void qt_gui_c_QGlyphRun_setFlag_flag_enabled(QGlyphRun* this_ptr, QGlyphRun::GlyphRunFlag flag, bool enabled) {
  this_ptr->setFlag(flag, enabled);
}

void qt_gui_c_QGlyphRun_setFlags(QGlyphRun* this_ptr, unsigned int flags) {
  this_ptr->setFlags(QFlags< QGlyphRun::GlyphRunFlag >(flags));
}

void qt_gui_c_QGlyphRun_setGlyphIndexes(QGlyphRun* this_ptr, const QVector< quint32 >* glyphIndexes) {
  this_ptr->setGlyphIndexes(*glyphIndexes);
}

void qt_gui_c_QGlyphRun_setOverline(QGlyphRun* this_ptr, bool overline) {
  this_ptr->setOverline(overline);
}

void qt_gui_c_QGlyphRun_setPositions(QGlyphRun* this_ptr, const QVector< QPointF >* positions) {
  this_ptr->setPositions(*positions);
}

void qt_gui_c_QGlyphRun_setRawData(QGlyphRun* this_ptr, const quint32* glyphIndexArray, const QPointF* glyphPositionArray, int size) {
  this_ptr->setRawData(glyphIndexArray, glyphPositionArray, size);
}

void qt_gui_c_QGlyphRun_setRawFont(QGlyphRun* this_ptr, const QRawFont* rawFont) {
  this_ptr->setRawFont(*rawFont);
}

void qt_gui_c_QGlyphRun_setRightToLeft(QGlyphRun* this_ptr, bool on) {
  this_ptr->setRightToLeft(on);
}

void qt_gui_c_QGlyphRun_setStrikeOut(QGlyphRun* this_ptr, bool strikeOut) {
  this_ptr->setStrikeOut(strikeOut);
}

void qt_gui_c_QGlyphRun_setUnderline(QGlyphRun* this_ptr, bool underline) {
  this_ptr->setUnderline(underline);
}

bool qt_gui_c_QGlyphRun_strikeOut(const QGlyphRun* this_ptr) {
  return this_ptr->strikeOut();
}

void qt_gui_c_QGlyphRun_swap(QGlyphRun* this_ptr, QGlyphRun* other) {
  this_ptr->swap(*other);
}

bool qt_gui_c_QGlyphRun_underline(const QGlyphRun* this_ptr) {
  return this_ptr->underline();
}

