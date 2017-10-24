#include "qt_gui_c_QCursor.h"

QDataStream* qt_gui_c_QCursor_G_operator_shl(QDataStream* outS, const QCursor* cursor) {
  return &operator<<(*outS, *cursor);
}

void qt_gui_c_QCursor_G_operator_shl_to_output(const QDebug* arg1, const QCursor* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_gui_c_QCursor_G_operator_shr(QDataStream* inS, QCursor* cursor) {
  return &operator>>(*inS, *cursor);
}

void qt_gui_c_QCursor_G_swap(QCursor* value1, QCursor* value2) {
  swap(*value1, *value2);
}

const QBitmap* qt_gui_c_QCursor_bitmap(const QCursor* this_ptr) {
  return this_ptr->bitmap();
}

void qt_gui_c_QCursor_convert_to_QVariant_to_output(const QCursor* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

void qt_gui_c_QCursor_delete(QCursor* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QCursor_hotSpot_to_output(const QCursor* this_ptr, QPoint* output) {
  new(output) QPoint(this_ptr->hotSpot());
}

const QBitmap* qt_gui_c_QCursor_mask(const QCursor* this_ptr) {
  return this_ptr->mask();
}

QCursor* qt_gui_c_QCursor_new_bitmap_mask(const QBitmap* bitmap, const QBitmap* mask) {
  return new QCursor(*bitmap, *mask);
}

QCursor* qt_gui_c_QCursor_new_bitmap_mask_hotX(const QBitmap* bitmap, const QBitmap* mask, int hotX) {
  return new QCursor(*bitmap, *mask, hotX);
}

QCursor* qt_gui_c_QCursor_new_bitmap_mask_hotX_hotY(const QBitmap* bitmap, const QBitmap* mask, int hotX, int hotY) {
  return new QCursor(*bitmap, *mask, hotX, hotY);
}

QCursor* qt_gui_c_QCursor_new_cursor(const QCursor* cursor) {
  return new QCursor(*cursor);
}

QCursor* qt_gui_c_QCursor_new_no_args() {
  return new QCursor();
}

QCursor* qt_gui_c_QCursor_new_pixmap(const QPixmap* pixmap) {
  return new QCursor(*pixmap);
}

QCursor* qt_gui_c_QCursor_new_pixmap_hotX(const QPixmap* pixmap, int hotX) {
  return new QCursor(*pixmap, hotX);
}

QCursor* qt_gui_c_QCursor_new_pixmap_hotX_hotY(const QPixmap* pixmap, int hotX, int hotY) {
  return new QCursor(*pixmap, hotX, hotY);
}

QCursor* qt_gui_c_QCursor_new_shape(const Qt::CursorShape* shape) {
  return new QCursor(*shape);
}

QCursor* qt_gui_c_QCursor_operator_assign(QCursor* this_ptr, const QCursor* cursor) {
  return &this_ptr->operator=(*cursor);
}

QPixmap* qt_gui_c_QCursor_pixmap_as_ptr(const QCursor* this_ptr) {
  return new QPixmap(this_ptr->pixmap());
}

void qt_gui_c_QCursor_pos_to_output_no_args(QPoint* output) {
  new(output) QPoint(QCursor::pos());
}

void qt_gui_c_QCursor_pos_to_output_screen(const QScreen* screen, QPoint* output) {
  new(output) QPoint(QCursor::pos(screen));
}

void qt_gui_c_QCursor_setPos_p(const QPoint* p) {
  QCursor::setPos(*p);
}

void qt_gui_c_QCursor_setPos_screen_p(QScreen* screen, const QPoint* p) {
  QCursor::setPos(screen, *p);
}

void qt_gui_c_QCursor_setPos_screen_x_y(QScreen* screen, int x, int y) {
  QCursor::setPos(screen, x, y);
}

void qt_gui_c_QCursor_setPos_x_y(int x, int y) {
  QCursor::setPos(x, y);
}

void qt_gui_c_QCursor_setShape(QCursor* this_ptr, const Qt::CursorShape* newShape) {
  this_ptr->setShape(*newShape);
}

void qt_gui_c_QCursor_swap(QCursor* this_ptr, QCursor* other) {
  this_ptr->swap(*other);
}

