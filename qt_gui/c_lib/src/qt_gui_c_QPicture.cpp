#include "qt_gui_c_QPicture.h"

QPicture* qt_gui_c_QPicture_G_dynamic_cast_QPicture_ptr(QPaintDevice* ptr) {
  return dynamic_cast<QPicture*>(ptr);
}

QPaintDevice* qt_gui_c_QPicture_G_static_cast_QPaintDevice_ptr(QPicture* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QPicture* qt_gui_c_QPicture_G_static_cast_QPicture_ptr(QPaintDevice* ptr) {
  return static_cast<QPicture*>(ptr);
}

void qt_gui_c_QPicture_G_swap(QPicture* value1, QPicture* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QPicture_boundingRect_to_output(const QPicture* this_ptr, QRect* output) {
  new(output) QRect(this_ptr->boundingRect());
}

const char* qt_gui_c_QPicture_data(const QPicture* this_ptr) {
  return this_ptr->data();
}

void qt_gui_c_QPicture_delete(QPicture* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QPicture_detach(QPicture* this_ptr) {
  this_ptr->detach();
}

int qt_gui_c_QPicture_devType(const QPicture* this_ptr) {
  return this_ptr->devType();
}

void qt_gui_c_QPicture_inputFormatList_to_output(QStringList* output) {
  new(output) QStringList(QPicture::inputFormatList());
}

void qt_gui_c_QPicture_inputFormats_to_output(QList< QByteArray >* output) {
  new(output) QList< QByteArray >(QPicture::inputFormats());
}

bool qt_gui_c_QPicture_isDetached(const QPicture* this_ptr) {
  return this_ptr->isDetached();
}

bool qt_gui_c_QPicture_isNull(const QPicture* this_ptr) {
  return this_ptr->isNull();
}

bool qt_gui_c_QPicture_load_dev(QPicture* this_ptr, QIODevice* dev) {
  return this_ptr->load(dev);
}

bool qt_gui_c_QPicture_load_dev_format(QPicture* this_ptr, QIODevice* dev, const char* format) {
  return this_ptr->load(dev, format);
}

bool qt_gui_c_QPicture_load_fileName(QPicture* this_ptr, const QString* fileName) {
  return this_ptr->load(*fileName);
}

bool qt_gui_c_QPicture_load_fileName_format(QPicture* this_ptr, const QString* fileName, const char* format) {
  return this_ptr->load(*fileName, format);
}

QPicture* qt_gui_c_QPicture_new_arg1(const QPicture* arg1) {
  return new QPicture(*arg1);
}

QPicture* qt_gui_c_QPicture_new_formatVersion(int formatVersion) {
  return new QPicture(formatVersion);
}

QPicture* qt_gui_c_QPicture_new_no_args() {
  return new QPicture();
}

QPicture* qt_gui_c_QPicture_operator_assign(QPicture* this_ptr, const QPicture* p) {
  return &this_ptr->operator=(*p);
}

void qt_gui_c_QPicture_outputFormatList_to_output(QStringList* output) {
  new(output) QStringList(QPicture::outputFormatList());
}

void qt_gui_c_QPicture_outputFormats_to_output(QList< QByteArray >* output) {
  new(output) QList< QByteArray >(QPicture::outputFormats());
}

QPaintEngine* qt_gui_c_QPicture_paintEngine(const QPicture* this_ptr) {
  return this_ptr->paintEngine();
}

const char* qt_gui_c_QPicture_pictureFormat(const QString* fileName) {
  return QPicture::pictureFormat(*fileName);
}

bool qt_gui_c_QPicture_play(QPicture* this_ptr, QPainter* p) {
  return this_ptr->play(p);
}

bool qt_gui_c_QPicture_save_dev(QPicture* this_ptr, QIODevice* dev) {
  return this_ptr->save(dev);
}

bool qt_gui_c_QPicture_save_dev_format(QPicture* this_ptr, QIODevice* dev, const char* format) {
  return this_ptr->save(dev, format);
}

bool qt_gui_c_QPicture_save_fileName(QPicture* this_ptr, const QString* fileName) {
  return this_ptr->save(*fileName);
}

bool qt_gui_c_QPicture_save_fileName_format(QPicture* this_ptr, const QString* fileName, const char* format) {
  return this_ptr->save(*fileName, format);
}

void qt_gui_c_QPicture_setBoundingRect(QPicture* this_ptr, const QRect* r) {
  this_ptr->setBoundingRect(*r);
}

void qt_gui_c_QPicture_setData(QPicture* this_ptr, const char* data, unsigned int size) {
  this_ptr->setData(data, size);
}

unsigned int qt_gui_c_QPicture_size(const QPicture* this_ptr) {
  return this_ptr->size();
}

void qt_gui_c_QPicture_swap(QPicture* this_ptr, QPicture* other) {
  this_ptr->swap(*other);
}

