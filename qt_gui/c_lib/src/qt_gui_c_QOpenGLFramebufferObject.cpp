#include "qt_gui_c_QOpenGLFramebufferObject.h"

void qt_gui_c_QOpenGLFramebufferObject_addColorAttachment_size(QOpenGLFramebufferObject* this_ptr, const QSize* size) {
  this_ptr->addColorAttachment(*size);
}

void qt_gui_c_QOpenGLFramebufferObject_addColorAttachment_size_internalFormat(QOpenGLFramebufferObject* this_ptr, const QSize* size, unsigned int internalFormat) {
  this_ptr->addColorAttachment(*size, internalFormat);
}

void qt_gui_c_QOpenGLFramebufferObject_addColorAttachment_width_height(QOpenGLFramebufferObject* this_ptr, int width, int height) {
  this_ptr->addColorAttachment(width, height);
}

void qt_gui_c_QOpenGLFramebufferObject_addColorAttachment_width_height_internalFormat(QOpenGLFramebufferObject* this_ptr, int width, int height, unsigned int internalFormat) {
  this_ptr->addColorAttachment(width, height, internalFormat);
}

QOpenGLFramebufferObject::Attachment qt_gui_c_QOpenGLFramebufferObject_attachment(const QOpenGLFramebufferObject* this_ptr) {
  return this_ptr->attachment();
}

bool qt_gui_c_QOpenGLFramebufferObject_bind(QOpenGLFramebufferObject* this_ptr) {
  return this_ptr->bind();
}

bool qt_gui_c_QOpenGLFramebufferObject_bindDefault() {
  return QOpenGLFramebufferObject::bindDefault();
}

void qt_gui_c_QOpenGLFramebufferObject_blitFramebuffer_target_source(QOpenGLFramebufferObject* target, QOpenGLFramebufferObject* source) {
  QOpenGLFramebufferObject::blitFramebuffer(target, source);
}

void qt_gui_c_QOpenGLFramebufferObject_blitFramebuffer_target_source_buffers(QOpenGLFramebufferObject* target, QOpenGLFramebufferObject* source, unsigned int buffers) {
  QOpenGLFramebufferObject::blitFramebuffer(target, source, buffers);
}

void qt_gui_c_QOpenGLFramebufferObject_blitFramebuffer_target_source_buffers_filter(QOpenGLFramebufferObject* target, QOpenGLFramebufferObject* source, unsigned int buffers, unsigned int filter) {
  QOpenGLFramebufferObject::blitFramebuffer(target, source, buffers, filter);
}

void qt_gui_c_QOpenGLFramebufferObject_blitFramebuffer_target_targetRect_source_sourceRect(QOpenGLFramebufferObject* target, const QRect* targetRect, QOpenGLFramebufferObject* source, const QRect* sourceRect) {
  QOpenGLFramebufferObject::blitFramebuffer(target, *targetRect, source, *sourceRect);
}

void qt_gui_c_QOpenGLFramebufferObject_blitFramebuffer_target_targetRect_source_sourceRect_buffers(QOpenGLFramebufferObject* target, const QRect* targetRect, QOpenGLFramebufferObject* source, const QRect* sourceRect, unsigned int buffers) {
  QOpenGLFramebufferObject::blitFramebuffer(target, *targetRect, source, *sourceRect, buffers);
}

void qt_gui_c_QOpenGLFramebufferObject_blitFramebuffer_target_targetRect_source_sourceRect_buffers_filter(QOpenGLFramebufferObject* target, const QRect* targetRect, QOpenGLFramebufferObject* source, const QRect* sourceRect, unsigned int buffers, unsigned int filter) {
  QOpenGLFramebufferObject::blitFramebuffer(target, *targetRect, source, *sourceRect, buffers, filter);
}

void qt_gui_c_QOpenGLFramebufferObject_blitFramebuffer_target_targetRect_source_sourceRect_buffers_filter_readColorAttachmentIndex_drawColorAttachmentIndex(QOpenGLFramebufferObject* target, const QRect* targetRect, QOpenGLFramebufferObject* source, const QRect* sourceRect, unsigned int buffers, unsigned int filter, int readColorAttachmentIndex, int drawColorAttachmentIndex) {
  QOpenGLFramebufferObject::blitFramebuffer(target, *targetRect, source, *sourceRect, buffers, filter, readColorAttachmentIndex, drawColorAttachmentIndex);
}

void qt_gui_c_QOpenGLFramebufferObject_blitFramebuffer_target_targetRect_source_sourceRect_buffers_filter_readColorAttachmentIndex_drawColorAttachmentIndex_restorePolicy(QOpenGLFramebufferObject* target, const QRect* targetRect, QOpenGLFramebufferObject* source, const QRect* sourceRect, unsigned int buffers, unsigned int filter, int readColorAttachmentIndex, int drawColorAttachmentIndex, QOpenGLFramebufferObject::FramebufferRestorePolicy restorePolicy) {
  QOpenGLFramebufferObject::blitFramebuffer(target, *targetRect, source, *sourceRect, buffers, filter, readColorAttachmentIndex, drawColorAttachmentIndex, restorePolicy);
}

void qt_gui_c_QOpenGLFramebufferObject_delete(QOpenGLFramebufferObject* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QOpenGLFramebufferObject_format_to_output(const QOpenGLFramebufferObject* this_ptr, QOpenGLFramebufferObjectFormat* output) {
  new(output) QOpenGLFramebufferObjectFormat(this_ptr->format());
}

GLuint qt_gui_c_QOpenGLFramebufferObject_handle(const QOpenGLFramebufferObject* this_ptr) {
  return this_ptr->handle();
}

bool qt_gui_c_QOpenGLFramebufferObject_hasOpenGLFramebufferBlit() {
  return QOpenGLFramebufferObject::hasOpenGLFramebufferBlit();
}

bool qt_gui_c_QOpenGLFramebufferObject_hasOpenGLFramebufferObjects() {
  return QOpenGLFramebufferObject::hasOpenGLFramebufferObjects();
}

int qt_gui_c_QOpenGLFramebufferObject_height(const QOpenGLFramebufferObject* this_ptr) {
  return this_ptr->height();
}

bool qt_gui_c_QOpenGLFramebufferObject_isBound(const QOpenGLFramebufferObject* this_ptr) {
  return this_ptr->isBound();
}

bool qt_gui_c_QOpenGLFramebufferObject_isValid(const QOpenGLFramebufferObject* this_ptr) {
  return this_ptr->isValid();
}

QOpenGLFramebufferObject* qt_gui_c_QOpenGLFramebufferObject_new_size(const QSize* size) {
  return new QOpenGLFramebufferObject(*size);
}

QOpenGLFramebufferObject* qt_gui_c_QOpenGLFramebufferObject_new_size_attachment(const QSize* size, QOpenGLFramebufferObject::Attachment attachment) {
  return new QOpenGLFramebufferObject(*size, attachment);
}

QOpenGLFramebufferObject* qt_gui_c_QOpenGLFramebufferObject_new_size_attachment_target(const QSize* size, QOpenGLFramebufferObject::Attachment attachment, unsigned int target) {
  return new QOpenGLFramebufferObject(*size, attachment, target);
}

QOpenGLFramebufferObject* qt_gui_c_QOpenGLFramebufferObject_new_size_attachment_target_internalFormat(const QSize* size, QOpenGLFramebufferObject::Attachment attachment, unsigned int target, unsigned int internalFormat) {
  return new QOpenGLFramebufferObject(*size, attachment, target, internalFormat);
}

QOpenGLFramebufferObject* qt_gui_c_QOpenGLFramebufferObject_new_size_format(const QSize* size, const QOpenGLFramebufferObjectFormat* format) {
  return new QOpenGLFramebufferObject(*size, *format);
}

QOpenGLFramebufferObject* qt_gui_c_QOpenGLFramebufferObject_new_size_target(const QSize* size, unsigned int target) {
  return new QOpenGLFramebufferObject(*size, target);
}

QOpenGLFramebufferObject* qt_gui_c_QOpenGLFramebufferObject_new_width_height(int width, int height) {
  return new QOpenGLFramebufferObject(width, height);
}

QOpenGLFramebufferObject* qt_gui_c_QOpenGLFramebufferObject_new_width_height_attachment(int width, int height, QOpenGLFramebufferObject::Attachment attachment) {
  return new QOpenGLFramebufferObject(width, height, attachment);
}

QOpenGLFramebufferObject* qt_gui_c_QOpenGLFramebufferObject_new_width_height_attachment_target(int width, int height, QOpenGLFramebufferObject::Attachment attachment, unsigned int target) {
  return new QOpenGLFramebufferObject(width, height, attachment, target);
}

QOpenGLFramebufferObject* qt_gui_c_QOpenGLFramebufferObject_new_width_height_attachment_target_internalFormat(int width, int height, QOpenGLFramebufferObject::Attachment attachment, unsigned int target, unsigned int internalFormat) {
  return new QOpenGLFramebufferObject(width, height, attachment, target, internalFormat);
}

QOpenGLFramebufferObject* qt_gui_c_QOpenGLFramebufferObject_new_width_height_format(int width, int height, const QOpenGLFramebufferObjectFormat* format) {
  return new QOpenGLFramebufferObject(width, height, *format);
}

QOpenGLFramebufferObject* qt_gui_c_QOpenGLFramebufferObject_new_width_height_target(int width, int height, unsigned int target) {
  return new QOpenGLFramebufferObject(width, height, target);
}

bool qt_gui_c_QOpenGLFramebufferObject_release(QOpenGLFramebufferObject* this_ptr) {
  return this_ptr->release();
}

void qt_gui_c_QOpenGLFramebufferObject_setAttachment(QOpenGLFramebufferObject* this_ptr, QOpenGLFramebufferObject::Attachment attachment) {
  this_ptr->setAttachment(attachment);
}

void qt_gui_c_QOpenGLFramebufferObject_size_to_output(const QOpenGLFramebufferObject* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->size());
}

void qt_gui_c_QOpenGLFramebufferObject_sizes_to_output(const QOpenGLFramebufferObject* this_ptr, QVector< QSize >* output) {
  new(output) QVector< QSize >(this_ptr->sizes());
}

GLuint qt_gui_c_QOpenGLFramebufferObject_takeTexture_colorAttachmentIndex(QOpenGLFramebufferObject* this_ptr, int colorAttachmentIndex) {
  return this_ptr->takeTexture(colorAttachmentIndex);
}

GLuint qt_gui_c_QOpenGLFramebufferObject_takeTexture_no_args(QOpenGLFramebufferObject* this_ptr) {
  return this_ptr->takeTexture();
}

GLuint qt_gui_c_QOpenGLFramebufferObject_texture(const QOpenGLFramebufferObject* this_ptr) {
  return this_ptr->texture();
}

void qt_gui_c_QOpenGLFramebufferObject_textures_to_output(const QOpenGLFramebufferObject* this_ptr, QVector< GLuint >* output) {
  new(output) QVector< GLuint >(this_ptr->textures());
}

QImage* qt_gui_c_QOpenGLFramebufferObject_toImage_as_ptr_flipped(const QOpenGLFramebufferObject* this_ptr, bool flipped) {
  return new QImage(this_ptr->toImage(flipped));
}

QImage* qt_gui_c_QOpenGLFramebufferObject_toImage_as_ptr_flipped_colorAttachmentIndex(const QOpenGLFramebufferObject* this_ptr, bool flipped, int colorAttachmentIndex) {
  return new QImage(this_ptr->toImage(flipped, colorAttachmentIndex));
}

QImage* qt_gui_c_QOpenGLFramebufferObject_toImage_as_ptr_no_args(const QOpenGLFramebufferObject* this_ptr) {
  return new QImage(this_ptr->toImage());
}

int qt_gui_c_QOpenGLFramebufferObject_width(const QOpenGLFramebufferObject* this_ptr) {
  return this_ptr->width();
}

