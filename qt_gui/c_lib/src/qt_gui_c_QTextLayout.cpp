#include "qt_gui_c_QTextLayout.h"

void qt_gui_c_QTextLayout_FormatRange_delete(QTextLayout::FormatRange* this_ptr) {
  delete this_ptr;
}

const QTextCharFormat* qt_gui_c_QTextLayout_FormatRange_format(const QTextLayout::FormatRange* this_ptr) {
  return &this_ptr->format;
}

QTextCharFormat* qt_gui_c_QTextLayout_FormatRange_format_mut(QTextLayout::FormatRange* this_ptr) {
  return &this_ptr->format;
}

int qt_gui_c_QTextLayout_FormatRange_length(const QTextLayout::FormatRange* this_ptr) {
  return this_ptr->length;
}

void qt_gui_c_QTextLayout_FormatRange_set_format(QTextLayout::FormatRange* this_ptr, const QTextCharFormat* value) {
  this_ptr->format = *value;
}

void qt_gui_c_QTextLayout_FormatRange_set_length(QTextLayout::FormatRange* this_ptr, int value) {
  this_ptr->length = value;
}

void qt_gui_c_QTextLayout_FormatRange_set_start(QTextLayout::FormatRange* this_ptr, int value) {
  this_ptr->start = value;
}

int qt_gui_c_QTextLayout_FormatRange_start(const QTextLayout::FormatRange* this_ptr) {
  return this_ptr->start;
}

void qt_gui_c_QTextLayout_additionalFormats_to_output(const QTextLayout* this_ptr, QList< QTextLayout::FormatRange >* output) {
  new(output) QList< QTextLayout::FormatRange >(this_ptr->additionalFormats());
}

void qt_gui_c_QTextLayout_beginLayout(QTextLayout* this_ptr) {
  this_ptr->beginLayout();
}

void qt_gui_c_QTextLayout_boundingRect_to_output(const QTextLayout* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect());
}

bool qt_gui_c_QTextLayout_cacheEnabled(const QTextLayout* this_ptr) {
  return this_ptr->cacheEnabled();
}

void qt_gui_c_QTextLayout_clearAdditionalFormats(QTextLayout* this_ptr) {
  this_ptr->clearAdditionalFormats();
}

void qt_gui_c_QTextLayout_clearFormats(QTextLayout* this_ptr) {
  this_ptr->clearFormats();
}

void qt_gui_c_QTextLayout_clearLayout(QTextLayout* this_ptr) {
  this_ptr->clearLayout();
}

void qt_gui_c_QTextLayout_createLine_to_output(QTextLayout* this_ptr, QTextLine* output) {
  new(output) QTextLine(this_ptr->createLine());
}

void qt_gui_c_QTextLayout_delete(QTextLayout* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QTextLayout_drawCursor_p_pos_cursorPosition(const QTextLayout* this_ptr, QPainter* p, const QPointF* pos, int cursorPosition) {
  this_ptr->drawCursor(p, *pos, cursorPosition);
}

void qt_gui_c_QTextLayout_drawCursor_p_pos_cursorPosition_width(const QTextLayout* this_ptr, QPainter* p, const QPointF* pos, int cursorPosition, int width) {
  this_ptr->drawCursor(p, *pos, cursorPosition, width);
}

void qt_gui_c_QTextLayout_draw_p_pos(const QTextLayout* this_ptr, QPainter* p, const QPointF* pos) {
  this_ptr->draw(p, *pos);
}

void qt_gui_c_QTextLayout_draw_p_pos_selections(const QTextLayout* this_ptr, QPainter* p, const QPointF* pos, const QVector< QTextLayout::FormatRange >* selections) {
  this_ptr->draw(p, *pos, *selections);
}

void qt_gui_c_QTextLayout_draw_p_pos_selections_clip(const QTextLayout* this_ptr, QPainter* p, const QPointF* pos, const QVector< QTextLayout::FormatRange >* selections, const QRectF* clip) {
  this_ptr->draw(p, *pos, *selections, *clip);
}

void qt_gui_c_QTextLayout_endLayout(QTextLayout* this_ptr) {
  this_ptr->endLayout();
}

void qt_gui_c_QTextLayout_font_to_output(const QTextLayout* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->font());
}

void qt_gui_c_QTextLayout_formats_to_output(const QTextLayout* this_ptr, QVector< QTextLayout::FormatRange >* output) {
  new(output) QVector< QTextLayout::FormatRange >(this_ptr->formats());
}

void qt_gui_c_QTextLayout_glyphRuns_to_output_from(const QTextLayout* this_ptr, int from, QList< QGlyphRun >* output) {
  new(output) QList< QGlyphRun >(this_ptr->glyphRuns(from));
}

void qt_gui_c_QTextLayout_glyphRuns_to_output_from_length(const QTextLayout* this_ptr, int from, int length, QList< QGlyphRun >* output) {
  new(output) QList< QGlyphRun >(this_ptr->glyphRuns(from, length));
}

void qt_gui_c_QTextLayout_glyphRuns_to_output_no_args(const QTextLayout* this_ptr, QList< QGlyphRun >* output) {
  new(output) QList< QGlyphRun >(this_ptr->glyphRuns());
}

bool qt_gui_c_QTextLayout_isValidCursorPosition(const QTextLayout* this_ptr, int pos) {
  return this_ptr->isValidCursorPosition(pos);
}

int qt_gui_c_QTextLayout_leftCursorPosition(const QTextLayout* this_ptr, int oldPos) {
  return this_ptr->leftCursorPosition(oldPos);
}

void qt_gui_c_QTextLayout_lineAt_to_output(const QTextLayout* this_ptr, int i, QTextLine* output) {
  new(output) QTextLine(this_ptr->lineAt(i));
}

int qt_gui_c_QTextLayout_lineCount(const QTextLayout* this_ptr) {
  return this_ptr->lineCount();
}

void qt_gui_c_QTextLayout_lineForTextPosition_to_output(const QTextLayout* this_ptr, int pos, QTextLine* output) {
  new(output) QTextLine(this_ptr->lineForTextPosition(pos));
}

double qt_gui_c_QTextLayout_maximumWidth(const QTextLayout* this_ptr) {
  return this_ptr->maximumWidth();
}

double qt_gui_c_QTextLayout_minimumWidth(const QTextLayout* this_ptr) {
  return this_ptr->minimumWidth();
}

QTextLayout* qt_gui_c_QTextLayout_new_b(const QTextBlock* b) {
  return new QTextLayout(*b);
}

QTextLayout* qt_gui_c_QTextLayout_new_no_args() {
  return new QTextLayout();
}

QTextLayout* qt_gui_c_QTextLayout_new_text(const QString* text) {
  return new QTextLayout(*text);
}

QTextLayout* qt_gui_c_QTextLayout_new_text_font(const QString* text, const QFont* font) {
  return new QTextLayout(*text, *font);
}

QTextLayout* qt_gui_c_QTextLayout_new_text_font_paintdevice(const QString* text, const QFont* font, QPaintDevice* paintdevice) {
  return new QTextLayout(*text, *font, paintdevice);
}

int qt_gui_c_QTextLayout_nextCursorPosition_oldPos(const QTextLayout* this_ptr, int oldPos) {
  return this_ptr->nextCursorPosition(oldPos);
}

int qt_gui_c_QTextLayout_nextCursorPosition_oldPos_mode(const QTextLayout* this_ptr, int oldPos, QTextLayout::CursorMode mode) {
  return this_ptr->nextCursorPosition(oldPos, mode);
}

void qt_gui_c_QTextLayout_position_to_output(const QTextLayout* this_ptr, QPointF* output) {
  new(output) QPointF(this_ptr->position());
}

int qt_gui_c_QTextLayout_preeditAreaPosition(const QTextLayout* this_ptr) {
  return this_ptr->preeditAreaPosition();
}

void qt_gui_c_QTextLayout_preeditAreaText_to_output(const QTextLayout* this_ptr, QString* output) {
  new(output) QString(this_ptr->preeditAreaText());
}

int qt_gui_c_QTextLayout_previousCursorPosition_oldPos(const QTextLayout* this_ptr, int oldPos) {
  return this_ptr->previousCursorPosition(oldPos);
}

int qt_gui_c_QTextLayout_previousCursorPosition_oldPos_mode(const QTextLayout* this_ptr, int oldPos, QTextLayout::CursorMode mode) {
  return this_ptr->previousCursorPosition(oldPos, mode);
}

int qt_gui_c_QTextLayout_rightCursorPosition(const QTextLayout* this_ptr, int oldPos) {
  return this_ptr->rightCursorPosition(oldPos);
}

void qt_gui_c_QTextLayout_setAdditionalFormats(QTextLayout* this_ptr, const QList< QTextLayout::FormatRange >* overrides) {
  this_ptr->setAdditionalFormats(*overrides);
}

void qt_gui_c_QTextLayout_setCacheEnabled(QTextLayout* this_ptr, bool enable) {
  this_ptr->setCacheEnabled(enable);
}

void qt_gui_c_QTextLayout_setCursorMoveStyle(QTextLayout* this_ptr, const Qt::CursorMoveStyle* style) {
  this_ptr->setCursorMoveStyle(*style);
}

void qt_gui_c_QTextLayout_setFlags(QTextLayout* this_ptr, int flags) {
  this_ptr->setFlags(flags);
}

void qt_gui_c_QTextLayout_setFont(QTextLayout* this_ptr, const QFont* f) {
  this_ptr->setFont(*f);
}

void qt_gui_c_QTextLayout_setFormats(QTextLayout* this_ptr, const QVector< QTextLayout::FormatRange >* overrides) {
  this_ptr->setFormats(*overrides);
}

void qt_gui_c_QTextLayout_setPosition(QTextLayout* this_ptr, const QPointF* p) {
  this_ptr->setPosition(*p);
}

void qt_gui_c_QTextLayout_setPreeditArea(QTextLayout* this_ptr, int position, const QString* text) {
  this_ptr->setPreeditArea(position, *text);
}

void qt_gui_c_QTextLayout_setRawFont(QTextLayout* this_ptr, const QRawFont* rawFont) {
  this_ptr->setRawFont(*rawFont);
}

void qt_gui_c_QTextLayout_setText(QTextLayout* this_ptr, const QString* string) {
  this_ptr->setText(*string);
}

void qt_gui_c_QTextLayout_setTextOption(QTextLayout* this_ptr, const QTextOption* option) {
  this_ptr->setTextOption(*option);
}

const QTextOption* qt_gui_c_QTextLayout_textOption(const QTextLayout* this_ptr) {
  return &this_ptr->textOption();
}

void qt_gui_c_QTextLayout_text_to_output(const QTextLayout* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

