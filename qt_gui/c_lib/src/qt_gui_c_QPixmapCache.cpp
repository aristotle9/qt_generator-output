#include "qt_gui_c_QPixmapCache.h"

void qt_gui_c_QPixmapCache_G_swap(QPixmapCache::Key* value1, QPixmapCache::Key* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QPixmapCache_Key_constructor_no_args(QPixmapCache::Key* output) {
  new(output) QPixmapCache::Key();
}

void qt_gui_c_QPixmapCache_Key_constructor_other(const QPixmapCache::Key* other, QPixmapCache::Key* output) {
  new(output) QPixmapCache::Key(*other);
}

void qt_gui_c_QPixmapCache_Key_destructor(QPixmapCache::Key* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QPixmapCache_Key_isValid(const QPixmapCache::Key* this_ptr) {
  return this_ptr->isValid();
}

QPixmapCache::Key* qt_gui_c_QPixmapCache_Key_operator_assign(QPixmapCache::Key* this_ptr, const QPixmapCache::Key* other) {
  return &this_ptr->operator=(*other);
}

bool qt_gui_c_QPixmapCache_Key_operator_eq(const QPixmapCache::Key* this_ptr, const QPixmapCache::Key* key) {
  return this_ptr->operator==(*key);
}

bool qt_gui_c_QPixmapCache_Key_operator_neq(const QPixmapCache::Key* this_ptr, const QPixmapCache::Key* key) {
  return this_ptr->operator!=(*key);
}

void qt_gui_c_QPixmapCache_Key_swap(QPixmapCache::Key* this_ptr, QPixmapCache::Key* other) {
  this_ptr->swap(*other);
}

int qt_gui_c_QPixmapCache_cacheLimit() {
  return QPixmapCache::cacheLimit();
}

void qt_gui_c_QPixmapCache_clear() {
  QPixmapCache::clear();
}

void qt_gui_c_QPixmapCache_delete(QPixmapCache* this_ptr) {
  delete this_ptr;
}

bool qt_gui_c_QPixmapCache_find_const_QPixmapCache_Key_ref_QPixmap_ptr(const QPixmapCache::Key* key, QPixmap* pixmap) {
  return QPixmapCache::find(*key, pixmap);
}

QPixmap* qt_gui_c_QPixmapCache_find_const_QString_ref(const QString* key) {
  return QPixmapCache::find(*key);
}

bool qt_gui_c_QPixmapCache_find_const_QString_ref_QPixmap_ptr(const QString* key, QPixmap* pixmap) {
  return QPixmapCache::find(*key, pixmap);
}

bool qt_gui_c_QPixmapCache_find_const_QString_ref_QPixmap_ref(const QString* key, QPixmap* pixmap) {
  return QPixmapCache::find(*key, *pixmap);
}

bool qt_gui_c_QPixmapCache_insert(const QString* key, const QPixmap* pixmap) {
  return QPixmapCache::insert(*key, *pixmap);
}

void qt_gui_c_QPixmapCache_insert_to_output(const QPixmap* pixmap, QPixmapCache::Key* output) {
  new(output) QPixmapCache::Key(QPixmapCache::insert(*pixmap));
}

void qt_gui_c_QPixmapCache_remove_QPixmapCache_Key(const QPixmapCache::Key* key) {
  QPixmapCache::remove(*key);
}

void qt_gui_c_QPixmapCache_remove_QString(const QString* key) {
  QPixmapCache::remove(*key);
}

bool qt_gui_c_QPixmapCache_replace(const QPixmapCache::Key* key, const QPixmap* pixmap) {
  return QPixmapCache::replace(*key, *pixmap);
}

void qt_gui_c_QPixmapCache_setCacheLimit(int arg1) {
  QPixmapCache::setCacheLimit(arg1);
}

