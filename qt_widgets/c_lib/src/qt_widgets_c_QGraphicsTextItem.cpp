#include "qt_widgets_c_QGraphicsTextItem.h"

QGraphicsTextItem* qt_widgets_c_QGraphicsTextItem_G_dynamic_cast_QGraphicsTextItem_ptr_QGraphicsItem(QGraphicsItem* ptr) {
  return dynamic_cast<QGraphicsTextItem*>(ptr);
}

QGraphicsTextItem* qt_widgets_c_QGraphicsTextItem_G_dynamic_cast_QGraphicsTextItem_ptr_QGraphicsObject(QGraphicsObject* ptr) {
  return dynamic_cast<QGraphicsTextItem*>(ptr);
}

QGraphicsItem* qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsItem_ptr(QGraphicsTextItem* ptr) {
  return static_cast<QGraphicsItem*>(ptr);
}

QGraphicsObject* qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsObject_ptr(QGraphicsTextItem* ptr) {
  return static_cast<QGraphicsObject*>(ptr);
}

QGraphicsTextItem* qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsTextItem_ptr_QGraphicsItem(QGraphicsItem* ptr) {
  return static_cast<QGraphicsTextItem*>(ptr);
}

QGraphicsTextItem* qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsTextItem_ptr_QGraphicsObject(QGraphicsObject* ptr) {
  return static_cast<QGraphicsTextItem*>(ptr);
}

QGraphicsTextItem* qt_widgets_c_QGraphicsTextItem_G_static_cast_QGraphicsTextItem_ptr_QObject(QObject* ptr) {
  return static_cast<QGraphicsTextItem*>(ptr);
}

QObject* qt_widgets_c_QGraphicsTextItem_G_static_cast_QObject_ptr(QGraphicsTextItem* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QGraphicsTextItem_adjustSize(QGraphicsTextItem* this_ptr) {
  this_ptr->adjustSize();
}

void qt_widgets_c_QGraphicsTextItem_boundingRect_to_output(const QGraphicsTextItem* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect());
}

bool qt_widgets_c_QGraphicsTextItem_contains(const QGraphicsTextItem* this_ptr, const QPointF* point) {
  return this_ptr->contains(*point);
}

void qt_widgets_c_QGraphicsTextItem_defaultTextColor_to_output(const QGraphicsTextItem* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->defaultTextColor());
}

void qt_widgets_c_QGraphicsTextItem_delete(QGraphicsTextItem* this_ptr) {
  delete this_ptr;
}

QTextDocument* qt_widgets_c_QGraphicsTextItem_document(const QGraphicsTextItem* this_ptr) {
  return this_ptr->document();
}

void qt_widgets_c_QGraphicsTextItem_font_to_output(const QGraphicsTextItem* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->font());
}

bool qt_widgets_c_QGraphicsTextItem_isObscuredBy(const QGraphicsTextItem* this_ptr, const QGraphicsItem* item) {
  return this_ptr->isObscuredBy(item);
}

const QMetaObject* qt_widgets_c_QGraphicsTextItem_metaObject(const QGraphicsTextItem* this_ptr) {
  return this_ptr->metaObject();
}

QGraphicsTextItem* qt_widgets_c_QGraphicsTextItem_new_no_args() {
  return new QGraphicsTextItem();
}

QGraphicsTextItem* qt_widgets_c_QGraphicsTextItem_new_parent(QGraphicsItem* parent) {
  return new QGraphicsTextItem(parent);
}

QGraphicsTextItem* qt_widgets_c_QGraphicsTextItem_new_text(const QString* text) {
  return new QGraphicsTextItem(*text);
}

QGraphicsTextItem* qt_widgets_c_QGraphicsTextItem_new_text_parent(const QString* text, QGraphicsItem* parent) {
  return new QGraphicsTextItem(*text, parent);
}

void qt_widgets_c_QGraphicsTextItem_opaqueArea_to_output(const QGraphicsTextItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->opaqueArea());
}

bool qt_widgets_c_QGraphicsTextItem_openExternalLinks(const QGraphicsTextItem* this_ptr) {
  return this_ptr->openExternalLinks();
}

void qt_widgets_c_QGraphicsTextItem_paint(QGraphicsTextItem* this_ptr, QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget) {
  this_ptr->paint(painter, option, widget);
}

int qt_widgets_c_QGraphicsTextItem_qt_metacall(QGraphicsTextItem* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGraphicsTextItem_qt_metacast(QGraphicsTextItem* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QGraphicsTextItem_setDefaultTextColor(QGraphicsTextItem* this_ptr, const QColor* c) {
  this_ptr->setDefaultTextColor(*c);
}

void qt_widgets_c_QGraphicsTextItem_setDocument(QGraphicsTextItem* this_ptr, QTextDocument* document) {
  this_ptr->setDocument(document);
}

void qt_widgets_c_QGraphicsTextItem_setFont(QGraphicsTextItem* this_ptr, const QFont* font) {
  this_ptr->setFont(*font);
}

void qt_widgets_c_QGraphicsTextItem_setHtml(QGraphicsTextItem* this_ptr, const QString* html) {
  this_ptr->setHtml(*html);
}

void qt_widgets_c_QGraphicsTextItem_setOpenExternalLinks(QGraphicsTextItem* this_ptr, bool open) {
  this_ptr->setOpenExternalLinks(open);
}

void qt_widgets_c_QGraphicsTextItem_setPlainText(QGraphicsTextItem* this_ptr, const QString* text) {
  this_ptr->setPlainText(*text);
}

void qt_widgets_c_QGraphicsTextItem_setTabChangesFocus(QGraphicsTextItem* this_ptr, bool b) {
  this_ptr->setTabChangesFocus(b);
}

void qt_widgets_c_QGraphicsTextItem_setTextCursor(QGraphicsTextItem* this_ptr, const QTextCursor* cursor) {
  this_ptr->setTextCursor(*cursor);
}

void qt_widgets_c_QGraphicsTextItem_setTextWidth(QGraphicsTextItem* this_ptr, double width) {
  this_ptr->setTextWidth(width);
}

void qt_widgets_c_QGraphicsTextItem_shape_to_output(const QGraphicsTextItem* this_ptr, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->shape());
}

bool qt_widgets_c_QGraphicsTextItem_tabChangesFocus(const QGraphicsTextItem* this_ptr) {
  return this_ptr->tabChangesFocus();
}

QTextCursor* qt_widgets_c_QGraphicsTextItem_textCursor_as_ptr(const QGraphicsTextItem* this_ptr) {
  return new QTextCursor(this_ptr->textCursor());
}

double qt_widgets_c_QGraphicsTextItem_textWidth(const QGraphicsTextItem* this_ptr) {
  return this_ptr->textWidth();
}

void qt_widgets_c_QGraphicsTextItem_toHtml_to_output(const QGraphicsTextItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->toHtml());
}

void qt_widgets_c_QGraphicsTextItem_toPlainText_to_output(const QGraphicsTextItem* this_ptr, QString* output) {
  new(output) QString(this_ptr->toPlainText());
}

void qt_widgets_c_QGraphicsTextItem_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsTextItem::trUtf8(s, c, n));
}

void qt_widgets_c_QGraphicsTextItem_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsTextItem::tr(s, c, n));
}

int qt_widgets_c_QGraphicsTextItem_type(const QGraphicsTextItem* this_ptr) {
  return this_ptr->type();
}

