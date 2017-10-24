#include "qt_gui_c_QOpenGLPixelTransferOptions.h"

void qt_gui_c_QOpenGLPixelTransferOptions_G_swap(QOpenGLPixelTransferOptions* value1, QOpenGLPixelTransferOptions* value2) {
  swap(*value1, *value2);
}

int qt_gui_c_QOpenGLPixelTransferOptions_alignment(const QOpenGLPixelTransferOptions* this_ptr) {
  return this_ptr->alignment();
}

void qt_gui_c_QOpenGLPixelTransferOptions_delete(QOpenGLPixelTransferOptions* this_ptr) {
  delete this_ptr;
}

int qt_gui_c_QOpenGLPixelTransferOptions_imageHeight(const QOpenGLPixelTransferOptions* this_ptr) {
  return this_ptr->imageHeight();
}

bool qt_gui_c_QOpenGLPixelTransferOptions_isLeastSignificantBitFirst(const QOpenGLPixelTransferOptions* this_ptr) {
  return this_ptr->isLeastSignificantBitFirst();
}

bool qt_gui_c_QOpenGLPixelTransferOptions_isSwapBytesEnabled(const QOpenGLPixelTransferOptions* this_ptr) {
  return this_ptr->isSwapBytesEnabled();
}

QOpenGLPixelTransferOptions* qt_gui_c_QOpenGLPixelTransferOptions_new_arg1(const QOpenGLPixelTransferOptions* arg1) {
  return new QOpenGLPixelTransferOptions(*arg1);
}

QOpenGLPixelTransferOptions* qt_gui_c_QOpenGLPixelTransferOptions_new_no_args() {
  return new QOpenGLPixelTransferOptions();
}

QOpenGLPixelTransferOptions* qt_gui_c_QOpenGLPixelTransferOptions_operator_assign(QOpenGLPixelTransferOptions* this_ptr, const QOpenGLPixelTransferOptions* arg1) {
  return &this_ptr->operator=(*arg1);
}

int qt_gui_c_QOpenGLPixelTransferOptions_rowLength(const QOpenGLPixelTransferOptions* this_ptr) {
  return this_ptr->rowLength();
}

void qt_gui_c_QOpenGLPixelTransferOptions_setAlignment(QOpenGLPixelTransferOptions* this_ptr, int alignment) {
  this_ptr->setAlignment(alignment);
}

void qt_gui_c_QOpenGLPixelTransferOptions_setImageHeight(QOpenGLPixelTransferOptions* this_ptr, int imageHeight) {
  this_ptr->setImageHeight(imageHeight);
}

void qt_gui_c_QOpenGLPixelTransferOptions_setLeastSignificantByteFirst(QOpenGLPixelTransferOptions* this_ptr, bool lsbFirst) {
  this_ptr->setLeastSignificantByteFirst(lsbFirst);
}

void qt_gui_c_QOpenGLPixelTransferOptions_setRowLength(QOpenGLPixelTransferOptions* this_ptr, int rowLength) {
  this_ptr->setRowLength(rowLength);
}

void qt_gui_c_QOpenGLPixelTransferOptions_setSkipImages(QOpenGLPixelTransferOptions* this_ptr, int skipImages) {
  this_ptr->setSkipImages(skipImages);
}

void qt_gui_c_QOpenGLPixelTransferOptions_setSkipPixels(QOpenGLPixelTransferOptions* this_ptr, int skipPixels) {
  this_ptr->setSkipPixels(skipPixels);
}

void qt_gui_c_QOpenGLPixelTransferOptions_setSkipRows(QOpenGLPixelTransferOptions* this_ptr, int skipRows) {
  this_ptr->setSkipRows(skipRows);
}

void qt_gui_c_QOpenGLPixelTransferOptions_setSwapBytesEnabled(QOpenGLPixelTransferOptions* this_ptr, bool swapBytes) {
  this_ptr->setSwapBytesEnabled(swapBytes);
}

int qt_gui_c_QOpenGLPixelTransferOptions_skipImages(const QOpenGLPixelTransferOptions* this_ptr) {
  return this_ptr->skipImages();
}

int qt_gui_c_QOpenGLPixelTransferOptions_skipPixels(const QOpenGLPixelTransferOptions* this_ptr) {
  return this_ptr->skipPixels();
}

int qt_gui_c_QOpenGLPixelTransferOptions_skipRows(const QOpenGLPixelTransferOptions* this_ptr) {
  return this_ptr->skipRows();
}

void qt_gui_c_QOpenGLPixelTransferOptions_swap(QOpenGLPixelTransferOptions* this_ptr, QOpenGLPixelTransferOptions* other) {
  this_ptr->swap(*other);
}

