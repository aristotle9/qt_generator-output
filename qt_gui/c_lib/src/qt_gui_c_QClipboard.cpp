#include "qt_gui_c_QClipboard.h"

QClipboard* qt_gui_c_QClipboard_G_static_cast_QClipboard_ptr(QObject* ptr) {
  return static_cast<QClipboard*>(ptr);
}

QObject* qt_gui_c_QClipboard_G_static_cast_QObject_ptr(QClipboard* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_gui_c_QClipboard_clear_mode(QClipboard* this_ptr, QClipboard::Mode mode) {
  this_ptr->clear(mode);
}

void qt_gui_c_QClipboard_clear_no_args(QClipboard* this_ptr) {
  this_ptr->clear();
}

QImage* qt_gui_c_QClipboard_image_as_ptr_mode(const QClipboard* this_ptr, QClipboard::Mode mode) {
  return new QImage(this_ptr->image(mode));
}

QImage* qt_gui_c_QClipboard_image_as_ptr_no_args(const QClipboard* this_ptr) {
  return new QImage(this_ptr->image());
}

const QMetaObject* qt_gui_c_QClipboard_metaObject(const QClipboard* this_ptr) {
  return this_ptr->metaObject();
}

const QMimeData* qt_gui_c_QClipboard_mimeData_mode(const QClipboard* this_ptr, QClipboard::Mode mode) {
  return this_ptr->mimeData(mode);
}

const QMimeData* qt_gui_c_QClipboard_mimeData_no_args(const QClipboard* this_ptr) {
  return this_ptr->mimeData();
}

bool qt_gui_c_QClipboard_ownsClipboard(const QClipboard* this_ptr) {
  return this_ptr->ownsClipboard();
}

bool qt_gui_c_QClipboard_ownsFindBuffer(const QClipboard* this_ptr) {
  return this_ptr->ownsFindBuffer();
}

bool qt_gui_c_QClipboard_ownsSelection(const QClipboard* this_ptr) {
  return this_ptr->ownsSelection();
}

QPixmap* qt_gui_c_QClipboard_pixmap_as_ptr_mode(const QClipboard* this_ptr, QClipboard::Mode mode) {
  return new QPixmap(this_ptr->pixmap(mode));
}

QPixmap* qt_gui_c_QClipboard_pixmap_as_ptr_no_args(const QClipboard* this_ptr) {
  return new QPixmap(this_ptr->pixmap());
}

int qt_gui_c_QClipboard_qt_metacall(QClipboard* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QClipboard_qt_metacast(QClipboard* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_gui_c_QClipboard_setImage_arg1(QClipboard* this_ptr, const QImage* arg1) {
  this_ptr->setImage(*arg1);
}

void qt_gui_c_QClipboard_setImage_arg1_mode(QClipboard* this_ptr, const QImage* arg1, QClipboard::Mode mode) {
  this_ptr->setImage(*arg1, mode);
}

void qt_gui_c_QClipboard_setMimeData_data(QClipboard* this_ptr, QMimeData* data) {
  this_ptr->setMimeData(data);
}

void qt_gui_c_QClipboard_setMimeData_data_mode(QClipboard* this_ptr, QMimeData* data, QClipboard::Mode mode) {
  this_ptr->setMimeData(data, mode);
}

void qt_gui_c_QClipboard_setPixmap_arg1(QClipboard* this_ptr, const QPixmap* arg1) {
  this_ptr->setPixmap(*arg1);
}

void qt_gui_c_QClipboard_setPixmap_arg1_mode(QClipboard* this_ptr, const QPixmap* arg1, QClipboard::Mode mode) {
  this_ptr->setPixmap(*arg1, mode);
}

void qt_gui_c_QClipboard_setText_arg1(QClipboard* this_ptr, const QString* arg1) {
  this_ptr->setText(*arg1);
}

void qt_gui_c_QClipboard_setText_arg1_mode(QClipboard* this_ptr, const QString* arg1, QClipboard::Mode mode) {
  this_ptr->setText(*arg1, mode);
}

bool qt_gui_c_QClipboard_supportsFindBuffer(const QClipboard* this_ptr) {
  return this_ptr->supportsFindBuffer();
}

bool qt_gui_c_QClipboard_supportsSelection(const QClipboard* this_ptr) {
  return this_ptr->supportsSelection();
}

void qt_gui_c_QClipboard_text_to_output_mode(const QClipboard* this_ptr, QClipboard::Mode mode, QString* output) {
  new(output) QString(this_ptr->text(mode));
}

void qt_gui_c_QClipboard_text_to_output_no_args(const QClipboard* this_ptr, QString* output) {
  new(output) QString(this_ptr->text());
}

void qt_gui_c_QClipboard_text_to_output_subtype(const QClipboard* this_ptr, QString* subtype, QString* output) {
  new(output) QString(this_ptr->text(*subtype));
}

void qt_gui_c_QClipboard_text_to_output_subtype_mode(const QClipboard* this_ptr, QString* subtype, QClipboard::Mode mode, QString* output) {
  new(output) QString(this_ptr->text(*subtype, mode));
}

void qt_gui_c_QClipboard_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QClipboard::trUtf8(s, c, n));
}

void qt_gui_c_QClipboard_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QClipboard::tr(s, c, n));
}

