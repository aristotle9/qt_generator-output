#include "qt_gui_c_QTextLine.h"

double qt_gui_c_QTextLine_ascent(const QTextLine* this_ptr) {
  return this_ptr->ascent();
}

void qt_gui_c_QTextLine_constructor(QTextLine* output) {
  new(output) QTextLine();
}

double qt_gui_c_QTextLine_cursorToX_int(const QTextLine* this_ptr, int cursorPos) {
  return this_ptr->cursorToX(cursorPos);
}

double qt_gui_c_QTextLine_cursorToX_int_QTextLine_Edge(const QTextLine* this_ptr, int cursorPos, QTextLine::Edge edge) {
  return this_ptr->cursorToX(cursorPos, edge);
}

double qt_gui_c_QTextLine_cursorToX_int_ptr(const QTextLine* this_ptr, int* cursorPos) {
  return this_ptr->cursorToX(cursorPos);
}

double qt_gui_c_QTextLine_cursorToX_int_ptr_QTextLine_Edge(const QTextLine* this_ptr, int* cursorPos, QTextLine::Edge edge) {
  return this_ptr->cursorToX(cursorPos, edge);
}

double qt_gui_c_QTextLine_descent(const QTextLine* this_ptr) {
  return this_ptr->descent();
}

void qt_gui_c_QTextLine_destructor(QTextLine* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QTextLine_draw_p_point(const QTextLine* this_ptr, QPainter* p, const QPointF* point) {
  this_ptr->draw(p, *point);
}

void qt_gui_c_QTextLine_draw_p_point_selection(const QTextLine* this_ptr, QPainter* p, const QPointF* point, const QTextLayout::FormatRange* selection) {
  this_ptr->draw(p, *point, selection);
}

void qt_gui_c_QTextLine_glyphRuns_to_output_from(const QTextLine* this_ptr, int from, QList< QGlyphRun >* output) {
  new(output) QList< QGlyphRun >(this_ptr->glyphRuns(from));
}

void qt_gui_c_QTextLine_glyphRuns_to_output_from_length(const QTextLine* this_ptr, int from, int length, QList< QGlyphRun >* output) {
  new(output) QList< QGlyphRun >(this_ptr->glyphRuns(from, length));
}

void qt_gui_c_QTextLine_glyphRuns_to_output_no_args(const QTextLine* this_ptr, QList< QGlyphRun >* output) {
  new(output) QList< QGlyphRun >(this_ptr->glyphRuns());
}

double qt_gui_c_QTextLine_height(const QTextLine* this_ptr) {
  return this_ptr->height();
}

double qt_gui_c_QTextLine_horizontalAdvance(const QTextLine* this_ptr) {
  return this_ptr->horizontalAdvance();
}

bool qt_gui_c_QTextLine_isValid(const QTextLine* this_ptr) {
  return this_ptr->isValid();
}

double qt_gui_c_QTextLine_leading(const QTextLine* this_ptr) {
  return this_ptr->leading();
}

bool qt_gui_c_QTextLine_leadingIncluded(const QTextLine* this_ptr) {
  return this_ptr->leadingIncluded();
}

int qt_gui_c_QTextLine_lineNumber(const QTextLine* this_ptr) {
  return this_ptr->lineNumber();
}

void qt_gui_c_QTextLine_naturalTextRect_to_output(const QTextLine* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->naturalTextRect());
}

double qt_gui_c_QTextLine_naturalTextWidth(const QTextLine* this_ptr) {
  return this_ptr->naturalTextWidth();
}

void qt_gui_c_QTextLine_position_to_output(const QTextLine* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->position());
}

void qt_gui_c_QTextLine_rect_to_output(const QTextLine* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->rect());
}

void qt_gui_c_QTextLine_setLeadingIncluded(QTextLine* this_ptr, bool included) {
  this_ptr->setLeadingIncluded(included);
}

void qt_gui_c_QTextLine_setLineWidth(QTextLine* this_ptr, double width) {
  this_ptr->setLineWidth(width);
}

void qt_gui_c_QTextLine_setNumColumns_columns(QTextLine* this_ptr, int columns) {
  this_ptr->setNumColumns(columns);
}

void qt_gui_c_QTextLine_setNumColumns_columns_alignmentWidth(QTextLine* this_ptr, int columns, double alignmentWidth) {
  this_ptr->setNumColumns(columns, alignmentWidth);
}

void qt_gui_c_QTextLine_setPosition(QTextLine* this_ptr, const QPointF* pos) {
  this_ptr->setPosition(*pos);
}

int qt_gui_c_QTextLine_textLength(const QTextLine* this_ptr) {
  return this_ptr->textLength();
}

int qt_gui_c_QTextLine_textStart(const QTextLine* this_ptr) {
  return this_ptr->textStart();
}

double qt_gui_c_QTextLine_width(const QTextLine* this_ptr) {
  return this_ptr->width();
}

double qt_gui_c_QTextLine_x(const QTextLine* this_ptr) {
  return this_ptr->x();
}

int qt_gui_c_QTextLine_xToCursor_x(const QTextLine* this_ptr, double x) {
  return this_ptr->xToCursor(x);
}

int qt_gui_c_QTextLine_xToCursor_x_arg2(const QTextLine* this_ptr, double x, QTextLine::CursorPosition arg2) {
  return this_ptr->xToCursor(x, arg2);
}

double qt_gui_c_QTextLine_y(const QTextLine* this_ptr) {
  return this_ptr->y();
}

