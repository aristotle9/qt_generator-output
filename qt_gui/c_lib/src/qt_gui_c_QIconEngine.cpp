#include "qt_gui_c_QIconEngine.h"

void qt_gui_c_QIconEngine_AvailableSizesArgument_delete(QIconEngine::AvailableSizesArgument* this_ptr) {
  delete this_ptr;
}

const QIcon::Mode* qt_gui_c_QIconEngine_AvailableSizesArgument_mode(const QIconEngine::AvailableSizesArgument* this_ptr) {
  return &this_ptr->mode;
}

QIcon::Mode* qt_gui_c_QIconEngine_AvailableSizesArgument_mode_mut(QIconEngine::AvailableSizesArgument* this_ptr) {
  return &this_ptr->mode;
}

void qt_gui_c_QIconEngine_AvailableSizesArgument_set_mode(QIconEngine::AvailableSizesArgument* this_ptr, const QIcon::Mode* value) {
  this_ptr->mode = *value;
}

void qt_gui_c_QIconEngine_AvailableSizesArgument_set_sizes(QIconEngine::AvailableSizesArgument* this_ptr, const QList< QSize >* value) {
  this_ptr->sizes = *value;
}

void qt_gui_c_QIconEngine_AvailableSizesArgument_set_state(QIconEngine::AvailableSizesArgument* this_ptr, const QIcon::State* value) {
  this_ptr->state = *value;
}

const QList< QSize >* qt_gui_c_QIconEngine_AvailableSizesArgument_sizes(const QIconEngine::AvailableSizesArgument* this_ptr) {
  return &this_ptr->sizes;
}

QList< QSize >* qt_gui_c_QIconEngine_AvailableSizesArgument_sizes_mut(QIconEngine::AvailableSizesArgument* this_ptr) {
  return &this_ptr->sizes;
}

const QIcon::State* qt_gui_c_QIconEngine_AvailableSizesArgument_state(const QIconEngine::AvailableSizesArgument* this_ptr) {
  return &this_ptr->state;
}

QIcon::State* qt_gui_c_QIconEngine_AvailableSizesArgument_state_mut(QIconEngine::AvailableSizesArgument* this_ptr) {
  return &this_ptr->state;
}

void qt_gui_c_QIconEngine_ScaledPixmapArgument_delete(QIconEngine::ScaledPixmapArgument* this_ptr) {
  delete this_ptr;
}

const QIcon::Mode* qt_gui_c_QIconEngine_ScaledPixmapArgument_mode(const QIconEngine::ScaledPixmapArgument* this_ptr) {
  return &this_ptr->mode;
}

QIcon::Mode* qt_gui_c_QIconEngine_ScaledPixmapArgument_mode_mut(QIconEngine::ScaledPixmapArgument* this_ptr) {
  return &this_ptr->mode;
}

const QPixmap* qt_gui_c_QIconEngine_ScaledPixmapArgument_pixmap(const QIconEngine::ScaledPixmapArgument* this_ptr) {
  return &this_ptr->pixmap;
}

QPixmap* qt_gui_c_QIconEngine_ScaledPixmapArgument_pixmap_mut(QIconEngine::ScaledPixmapArgument* this_ptr) {
  return &this_ptr->pixmap;
}

double qt_gui_c_QIconEngine_ScaledPixmapArgument_scale(const QIconEngine::ScaledPixmapArgument* this_ptr) {
  return this_ptr->scale;
}

void qt_gui_c_QIconEngine_ScaledPixmapArgument_set_mode(QIconEngine::ScaledPixmapArgument* this_ptr, const QIcon::Mode* value) {
  this_ptr->mode = *value;
}

void qt_gui_c_QIconEngine_ScaledPixmapArgument_set_pixmap(QIconEngine::ScaledPixmapArgument* this_ptr, const QPixmap* value) {
  this_ptr->pixmap = *value;
}

void qt_gui_c_QIconEngine_ScaledPixmapArgument_set_scale(QIconEngine::ScaledPixmapArgument* this_ptr, double value) {
  this_ptr->scale = value;
}

void qt_gui_c_QIconEngine_ScaledPixmapArgument_set_size(QIconEngine::ScaledPixmapArgument* this_ptr, const QSize* value) {
  this_ptr->size = *value;
}

void qt_gui_c_QIconEngine_ScaledPixmapArgument_set_state(QIconEngine::ScaledPixmapArgument* this_ptr, const QIcon::State* value) {
  this_ptr->state = *value;
}

const QSize* qt_gui_c_QIconEngine_ScaledPixmapArgument_size(const QIconEngine::ScaledPixmapArgument* this_ptr) {
  return &this_ptr->size;
}

QSize* qt_gui_c_QIconEngine_ScaledPixmapArgument_size_mut(QIconEngine::ScaledPixmapArgument* this_ptr) {
  return &this_ptr->size;
}

const QIcon::State* qt_gui_c_QIconEngine_ScaledPixmapArgument_state(const QIconEngine::ScaledPixmapArgument* this_ptr) {
  return &this_ptr->state;
}

QIcon::State* qt_gui_c_QIconEngine_ScaledPixmapArgument_state_mut(QIconEngine::ScaledPixmapArgument* this_ptr) {
  return &this_ptr->state;
}

void qt_gui_c_QIconEngine_actualSize_to_output(QIconEngine* this_ptr, const QSize* size, const QIcon::Mode* mode, const QIcon::State* state, QSize* output) {
  new(output) QSize(this_ptr->actualSize(*size, *mode, *state));
}

void qt_gui_c_QIconEngine_addFile(QIconEngine* this_ptr, const QString* fileName, const QSize* size, const QIcon::Mode* mode, const QIcon::State* state) {
  this_ptr->addFile(*fileName, *size, *mode, *state);
}

void qt_gui_c_QIconEngine_addPixmap(QIconEngine* this_ptr, const QPixmap* pixmap, const QIcon::Mode* mode, const QIcon::State* state) {
  this_ptr->addPixmap(*pixmap, *mode, *state);
}

void qt_gui_c_QIconEngine_availableSizes_to_output_mode(const QIconEngine* this_ptr, const QIcon::Mode* mode, QList< QSize >* output) {
  new(output) QList< QSize >(this_ptr->availableSizes(*mode));
}

void qt_gui_c_QIconEngine_availableSizes_to_output_mode_state(const QIconEngine* this_ptr, const QIcon::Mode* mode, const QIcon::State* state, QList< QSize >* output) {
  new(output) QList< QSize >(this_ptr->availableSizes(*mode, *state));
}

void qt_gui_c_QIconEngine_availableSizes_to_output_no_args(const QIconEngine* this_ptr, QList< QSize >* output) {
  new(output) QList< QSize >(this_ptr->availableSizes());
}

QIconEngine* qt_gui_c_QIconEngine_clone(const QIconEngine* this_ptr) {
  return this_ptr->clone();
}

void qt_gui_c_QIconEngine_delete(QIconEngine* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QIconEngine_iconName_to_output(const QIconEngine* this_ptr, QString* output) {
  new(output) QString(this_ptr->iconName());
}

bool qt_gui_c_QIconEngine_isNull(const QIconEngine* this_ptr) {
  return this_ptr->isNull();
}

void qt_gui_c_QIconEngine_key_to_output(const QIconEngine* this_ptr, QString* output) {
  new(output) QString(this_ptr->key());
}

void qt_gui_c_QIconEngine_paint(QIconEngine* this_ptr, QPainter* painter, const QRect* rect, const QIcon::Mode* mode, const QIcon::State* state) {
  this_ptr->paint(painter, *rect, *mode, *state);
}

QPixmap* qt_gui_c_QIconEngine_pixmap_as_ptr(QIconEngine* this_ptr, const QSize* size, const QIcon::Mode* mode, const QIcon::State* state) {
  return new QPixmap(this_ptr->pixmap(*size, *mode, *state));
}

bool qt_gui_c_QIconEngine_read(QIconEngine* this_ptr, QDataStream* in) {
  return this_ptr->read(*in);
}

QPixmap* qt_gui_c_QIconEngine_scaledPixmap_as_ptr(QIconEngine* this_ptr, const QSize* size, const QIcon::Mode* mode, const QIcon::State* state, double scale) {
  return new QPixmap(this_ptr->scaledPixmap(*size, *mode, *state, scale));
}

void qt_gui_c_QIconEngine_virtual_hook(QIconEngine* this_ptr, int id, void* data) {
  this_ptr->virtual_hook(id, data);
}

bool qt_gui_c_QIconEngine_write(const QIconEngine* this_ptr, QDataStream* out) {
  return this_ptr->write(*out);
}

