#include "qt_gui_c_QPdfWriter.h"

QPdfWriter* qt_gui_c_QPdfWriter_G_dynamic_cast_QPdfWriter_ptr_QPagedPaintDevice(QPagedPaintDevice* ptr) {
  return dynamic_cast<QPdfWriter*>(ptr);
}

QPdfWriter* qt_gui_c_QPdfWriter_G_dynamic_cast_QPdfWriter_ptr_QPaintDevice(QPaintDevice* ptr) {
  return dynamic_cast<QPdfWriter*>(ptr);
}

QObject* qt_gui_c_QPdfWriter_G_static_cast_QObject_ptr(QPdfWriter* ptr) {
  return static_cast<QObject*>(ptr);
}

QPagedPaintDevice* qt_gui_c_QPdfWriter_G_static_cast_QPagedPaintDevice_ptr(QPdfWriter* ptr) {
  return static_cast<QPagedPaintDevice*>(ptr);
}

QPaintDevice* qt_gui_c_QPdfWriter_G_static_cast_QPaintDevice_ptr(QPdfWriter* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QPdfWriter* qt_gui_c_QPdfWriter_G_static_cast_QPdfWriter_ptr_QObject(QObject* ptr) {
  return static_cast<QPdfWriter*>(ptr);
}

QPdfWriter* qt_gui_c_QPdfWriter_G_static_cast_QPdfWriter_ptr_QPagedPaintDevice(QPagedPaintDevice* ptr) {
  return static_cast<QPdfWriter*>(ptr);
}

QPdfWriter* qt_gui_c_QPdfWriter_G_static_cast_QPdfWriter_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QPdfWriter*>(ptr);
}

void qt_gui_c_QPdfWriter_creator_to_output(const QPdfWriter* this_ptr, QString* output) {
  new(output) QString(this_ptr->creator());
}

void qt_gui_c_QPdfWriter_delete(QPdfWriter* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_gui_c_QPdfWriter_metaObject(const QPdfWriter* this_ptr) {
  return this_ptr->metaObject();
}

bool qt_gui_c_QPdfWriter_newPage(QPdfWriter* this_ptr) {
  return this_ptr->newPage();
}

QPdfWriter* qt_gui_c_QPdfWriter_new_device(QIODevice* device) {
  return new QPdfWriter(device);
}

QPdfWriter* qt_gui_c_QPdfWriter_new_filename(const QString* filename) {
  return new QPdfWriter(*filename);
}

int qt_gui_c_QPdfWriter_qt_metacall(QPdfWriter* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QPdfWriter_qt_metacast(QPdfWriter* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

int qt_gui_c_QPdfWriter_resolution(const QPdfWriter* this_ptr) {
  return this_ptr->resolution();
}

void qt_gui_c_QPdfWriter_setCreator(QPdfWriter* this_ptr, const QString* creator) {
  this_ptr->setCreator(*creator);
}

void qt_gui_c_QPdfWriter_setMargins(QPdfWriter* this_ptr, const QPagedPaintDevice::Margins* m) {
  this_ptr->setMargins(*m);
}

void qt_gui_c_QPdfWriter_setPageSize(QPdfWriter* this_ptr, QPagedPaintDevice::PageSize size) {
  this_ptr->setPageSize(size);
}

void qt_gui_c_QPdfWriter_setPageSizeMM(QPdfWriter* this_ptr, const QSizeF* size) {
  this_ptr->setPageSizeMM(*size);
}

void qt_gui_c_QPdfWriter_setResolution(QPdfWriter* this_ptr, int resolution) {
  this_ptr->setResolution(resolution);
}

void qt_gui_c_QPdfWriter_setTitle(QPdfWriter* this_ptr, const QString* title) {
  this_ptr->setTitle(*title);
}

void qt_gui_c_QPdfWriter_title_to_output(const QPdfWriter* this_ptr, QString* output) {
  new(output) QString(this_ptr->title());
}

void qt_gui_c_QPdfWriter_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPdfWriter::trUtf8(s, c, n));
}

void qt_gui_c_QPdfWriter_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QPdfWriter::tr(s, c, n));
}

