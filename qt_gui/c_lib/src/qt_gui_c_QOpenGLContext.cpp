#include "qt_gui_c_QOpenGLContext.h"

bool qt_gui_c_QOpenGLContext_G_operator_eq(const QOpenGLVersionProfile* lhs, const QOpenGLVersionProfile* rhs) {
  return operator==(*lhs, *rhs);
}

bool qt_gui_c_QOpenGLContext_G_operator_neq(const QOpenGLVersionProfile* lhs, const QOpenGLVersionProfile* rhs) {
  return operator!=(*lhs, *rhs);
}

unsigned int qt_gui_c_QOpenGLContext_G_qHash_v(const QOpenGLVersionProfile* v) {
  return qHash(*v);
}

unsigned int qt_gui_c_QOpenGLContext_G_qHash_v_seed(const QOpenGLVersionProfile* v, unsigned int seed) {
  return qHash(*v, seed);
}

QObject* qt_gui_c_QOpenGLContext_G_static_cast_QObject_ptr(QOpenGLContext* ptr) {
  return static_cast<QObject*>(ptr);
}

QOpenGLContext* qt_gui_c_QOpenGLContext_G_static_cast_QOpenGLContext_ptr(QObject* ptr) {
  return static_cast<QOpenGLContext*>(ptr);
}

bool qt_gui_c_QOpenGLContext_areSharing(QOpenGLContext* first, QOpenGLContext* second) {
  return QOpenGLContext::areSharing(first, second);
}

bool qt_gui_c_QOpenGLContext_create(QOpenGLContext* this_ptr) {
  return this_ptr->create();
}

QOpenGLContext* qt_gui_c_QOpenGLContext_currentContext() {
  return QOpenGLContext::currentContext();
}

GLuint qt_gui_c_QOpenGLContext_defaultFramebufferObject(const QOpenGLContext* this_ptr) {
  return this_ptr->defaultFramebufferObject();
}

void qt_gui_c_QOpenGLContext_delete(QOpenGLContext* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QOpenGLContext_doneCurrent(QOpenGLContext* this_ptr) {
  this_ptr->doneCurrent();
}

void qt_gui_c_QOpenGLContext_extensions_to_output(const QOpenGLContext* this_ptr, QSet< QByteArray >* output) {
  new(output) QSet< QByteArray >(this_ptr->extensions());
}

QOpenGLExtraFunctions* qt_gui_c_QOpenGLContext_extraFunctions(const QOpenGLContext* this_ptr) {
  return this_ptr->extraFunctions();
}

void qt_gui_c_QOpenGLContext_format_to_output(const QOpenGLContext* this_ptr, QSurfaceFormat* output) {
  new(output) QSurfaceFormat(this_ptr->format());
}

QOpenGLFunctions* qt_gui_c_QOpenGLContext_functions(const QOpenGLContext* this_ptr) {
  return this_ptr->functions();
}

void (*qt_gui_c_QOpenGLContext_getProcAddress_QByteArray(const QOpenGLContext* this_ptr, const QByteArray* procName))() {
  return this_ptr->getProcAddress(*procName);
}

void (*qt_gui_c_QOpenGLContext_getProcAddress_char(const QOpenGLContext* this_ptr, const char* procName))() {
  return this_ptr->getProcAddress(procName);
}

QOpenGLContext* qt_gui_c_QOpenGLContext_globalShareContext() {
  return QOpenGLContext::globalShareContext();
}

bool qt_gui_c_QOpenGLContext_hasExtension(const QOpenGLContext* this_ptr, const QByteArray* extension) {
  return this_ptr->hasExtension(*extension);
}

bool qt_gui_c_QOpenGLContext_isOpenGLES(const QOpenGLContext* this_ptr) {
  return this_ptr->isOpenGLES();
}

bool qt_gui_c_QOpenGLContext_isValid(const QOpenGLContext* this_ptr) {
  return this_ptr->isValid();
}

bool qt_gui_c_QOpenGLContext_makeCurrent(QOpenGLContext* this_ptr, QSurface* surface) {
  return this_ptr->makeCurrent(surface);
}

const QMetaObject* qt_gui_c_QOpenGLContext_metaObject(const QOpenGLContext* this_ptr) {
  return this_ptr->metaObject();
}

void qt_gui_c_QOpenGLContext_nativeHandle_to_output(const QOpenGLContext* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->nativeHandle());
}

QOpenGLContext* qt_gui_c_QOpenGLContext_new_no_args() {
  return new QOpenGLContext();
}

QOpenGLContext* qt_gui_c_QOpenGLContext_new_parent(QObject* parent) {
  return new QOpenGLContext(parent);
}

void* qt_gui_c_QOpenGLContext_openGLModuleHandle() {
  return QOpenGLContext::openGLModuleHandle();
}

QOpenGLContext::OpenGLModuleType qt_gui_c_QOpenGLContext_openGLModuleType() {
  return QOpenGLContext::openGLModuleType();
}

int qt_gui_c_QOpenGLContext_qt_metacall(QOpenGLContext* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_gui_c_QOpenGLContext_qt_metacast(QOpenGLContext* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

QScreen* qt_gui_c_QOpenGLContext_screen(const QOpenGLContext* this_ptr) {
  return this_ptr->screen();
}

void qt_gui_c_QOpenGLContext_setFormat(QOpenGLContext* this_ptr, const QSurfaceFormat* format) {
  this_ptr->setFormat(*format);
}

void qt_gui_c_QOpenGLContext_setNativeHandle(QOpenGLContext* this_ptr, const QVariant* handle) {
  this_ptr->setNativeHandle(*handle);
}

void qt_gui_c_QOpenGLContext_setScreen(QOpenGLContext* this_ptr, QScreen* screen) {
  this_ptr->setScreen(screen);
}

void qt_gui_c_QOpenGLContext_setShareContext(QOpenGLContext* this_ptr, QOpenGLContext* shareContext) {
  this_ptr->setShareContext(shareContext);
}

QOpenGLContext* qt_gui_c_QOpenGLContext_shareContext(const QOpenGLContext* this_ptr) {
  return this_ptr->shareContext();
}

QOpenGLContextGroup* qt_gui_c_QOpenGLContext_shareGroup(const QOpenGLContext* this_ptr) {
  return this_ptr->shareGroup();
}

bool qt_gui_c_QOpenGLContext_supportsThreadedOpenGL() {
  return QOpenGLContext::supportsThreadedOpenGL();
}

QSurface* qt_gui_c_QOpenGLContext_surface(const QOpenGLContext* this_ptr) {
  return this_ptr->surface();
}

void qt_gui_c_QOpenGLContext_swapBuffers(QOpenGLContext* this_ptr, QSurface* surface) {
  this_ptr->swapBuffers(surface);
}

void qt_gui_c_QOpenGLContext_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLContext::trUtf8(s, c, n));
}

void qt_gui_c_QOpenGLContext_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QOpenGLContext::tr(s, c, n));
}

QAbstractOpenGLFunctions* qt_gui_c_QOpenGLContext_versionFunctions_no_args(const QOpenGLContext* this_ptr) {
  return this_ptr->versionFunctions();
}

QAbstractOpenGLFunctions* qt_gui_c_QOpenGLContext_versionFunctions_versionProfile(const QOpenGLContext* this_ptr, const QOpenGLVersionProfile* versionProfile) {
  return this_ptr->versionFunctions(*versionProfile);
}

