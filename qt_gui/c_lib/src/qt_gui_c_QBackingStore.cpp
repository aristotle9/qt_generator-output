#include "qt_gui_c_QBackingStore.h"

void qt_gui_c_QBackingStore_beginPaint(QBackingStore* this_ptr, const QRegion* arg1) {
  this_ptr->beginPaint(*arg1);
}

void qt_gui_c_QBackingStore_delete(QBackingStore* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QBackingStore_endPaint(QBackingStore* this_ptr) {
  this_ptr->endPaint();
}

void qt_gui_c_QBackingStore_flush_region(QBackingStore* this_ptr, const QRegion* region) {
  this_ptr->flush(*region);
}

void qt_gui_c_QBackingStore_flush_region_window(QBackingStore* this_ptr, const QRegion* region, QWindow* window) {
  this_ptr->flush(*region, window);
}

void qt_gui_c_QBackingStore_flush_region_window_offset(QBackingStore* this_ptr, const QRegion* region, QWindow* window, const QPoint* offset) {
  this_ptr->flush(*region, window, *offset);
}

bool qt_gui_c_QBackingStore_hasStaticContents(const QBackingStore* this_ptr) {
  return this_ptr->hasStaticContents();
}

QBackingStore* qt_gui_c_QBackingStore_new(QWindow* window) {
  return new QBackingStore(window);
}

QPaintDevice* qt_gui_c_QBackingStore_paintDevice(QBackingStore* this_ptr) {
  return this_ptr->paintDevice();
}

void qt_gui_c_QBackingStore_resize(QBackingStore* this_ptr, const QSize* size) {
  this_ptr->resize(*size);
}

bool qt_gui_c_QBackingStore_scroll(QBackingStore* this_ptr, const QRegion* area, int dx, int dy) {
  return this_ptr->scroll(*area, dx, dy);
}

void qt_gui_c_QBackingStore_setStaticContents(QBackingStore* this_ptr, const QRegion* region) {
  this_ptr->setStaticContents(*region);
}

void qt_gui_c_QBackingStore_size_to_output(const QBackingStore* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->size());
}

QRegion* qt_gui_c_QBackingStore_staticContents_as_ptr(const QBackingStore* this_ptr) {
  return new QRegion(this_ptr->staticContents());
}

QWindow* qt_gui_c_QBackingStore_window(const QBackingStore* this_ptr) {
  return this_ptr->window();
}

