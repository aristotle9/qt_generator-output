#include "qt_gui_c_QTextFragment.h"

int qt_gui_c_QTextFragment_charFormatIndex(const QTextFragment* this_ptr) {
  return this_ptr->charFormatIndex();
}

void qt_gui_c_QTextFragment_charFormat_to_output(const QTextFragment* this_ptr, QTextCharFormat* output) {
  new(output) QTextCharFormat(this_ptr->charFormat());
}

void qt_gui_c_QTextFragment_constructor_no_args(QTextFragment* output) {
  new(output) QTextFragment();
}

void qt_gui_c_QTextFragment_constructor_o(const QTextFragment* o, QTextFragment* output) {
  new(output) QTextFragment(*o);
}

bool qt_gui_c_QTextFragment_contains(const QTextFragment* this_ptr, int position) {
  return this_ptr->contains(position);
}

void qt_gui_c_QTextFragment_destructor(QTextFragment* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QTextFragment_glyphRuns_to_output_from(const QTextFragment* this_ptr, int from, QList< QGlyphRun >* output) {
  new(output) QList< QGlyphRun >(this_ptr->glyphRuns(from));
}

void qt_gui_c_QTextFragment_glyphRuns_to_output_from_length(const QTextFragment* this_ptr, int from, int length, QList< QGlyphRun >* output) {
  new(output) QList< QGlyphRun >(this_ptr->glyphRuns(from, length));
}

void qt_gui_c_QTextFragment_glyphRuns_to_output_no_args(const QTextFragment* this_ptr, QList< QGlyphRun >* output) {
  new(output) QList< QGlyphRun >(this_ptr->glyphRuns());
}

bool qt_gui_c_QTextFragment_isValid(const QTextFragment* this_ptr) {
  return this_ptr->isValid();
}

int qt_gui_c_QTextFragment_length(const QTextFragment* this_ptr) {
  return this_ptr->length();
}

QTextFragment* qt_gui_c_QTextFragment_operator_assign(QTextFragment* this_ptr, const QTextFragment* o) {
  return &this_ptr->operator=(*o);
}

bool qt_gui_c_QTextFragment_operator_eq(const QTextFragment* this_ptr, const QTextFragment* o) {
  return this_ptr->operator==(*o);
}

bool qt_gui_c_QTextFragment_operator_lt(const QTextFragment* this_ptr, const QTextFragment* o) {
  return this_ptr->operator<(*o);
}

bool qt_gui_c_QTextFragment_operator_neq(const QTextFragment* this_ptr, const QTextFragment* o) {
  return this_ptr->operator!=(*o);
}

int qt_gui_c_QTextFragment_position(const QTextFragment* this_ptr) {
  return this_ptr->position();
}

void qt_gui_c_QTextFragment_text_to_output(const QTextFragment* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

