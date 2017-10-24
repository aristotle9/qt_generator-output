#include "qt_3d_render_c_QGraphicsApiFilter.h"

QObject* qt_3d_render_c_QGraphicsApiFilter_G_static_cast_QObject_ptr(Qt3DRender::QGraphicsApiFilter* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DRender::QGraphicsApiFilter* qt_3d_render_c_QGraphicsApiFilter_G_static_cast_Qt3DRender_QGraphicsApiFilter_ptr(QObject* ptr) {
  return static_cast<Qt3DRender::QGraphicsApiFilter*>(ptr);
}

Qt3DRender::QGraphicsApiFilter::Api qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_api(const Qt3DRender::QGraphicsApiFilter* this_ptr) {
  return this_ptr->api();
}

void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_delete(Qt3DRender::QGraphicsApiFilter* this_ptr) {
  delete this_ptr;
}

void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_extensions_to_output(const Qt3DRender::QGraphicsApiFilter* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->extensions());
}

int qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_majorVersion(const Qt3DRender::QGraphicsApiFilter* this_ptr) {
  return this_ptr->majorVersion();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_metaObject(const Qt3DRender::QGraphicsApiFilter* this_ptr) {
  return this_ptr->metaObject();
}

int qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_minorVersion(const Qt3DRender::QGraphicsApiFilter* this_ptr) {
  return this_ptr->minorVersion();
}

Qt3DRender::QGraphicsApiFilter* qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_new_no_args() {
  return new Qt3DRender::QGraphicsApiFilter();
}

Qt3DRender::QGraphicsApiFilter* qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_new_parent(QObject* parent) {
  return new Qt3DRender::QGraphicsApiFilter(parent);
}

Qt3DRender::QGraphicsApiFilter::OpenGLProfile qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_profile(const Qt3DRender::QGraphicsApiFilter* this_ptr) {
  return this_ptr->profile();
}

int qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_qt_metacall(Qt3DRender::QGraphicsApiFilter* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_qt_metacast(Qt3DRender::QGraphicsApiFilter* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_setApi(Qt3DRender::QGraphicsApiFilter* this_ptr, Qt3DRender::QGraphicsApiFilter::Api api) {
  this_ptr->setApi(api);
}

void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_setExtensions(Qt3DRender::QGraphicsApiFilter* this_ptr, const QStringList* extensions) {
  this_ptr->setExtensions(*extensions);
}

void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_setMajorVersion(Qt3DRender::QGraphicsApiFilter* this_ptr, int majorVersion) {
  this_ptr->setMajorVersion(majorVersion);
}

void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_setMinorVersion(Qt3DRender::QGraphicsApiFilter* this_ptr, int minorVersion) {
  this_ptr->setMinorVersion(minorVersion);
}

void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_setProfile(Qt3DRender::QGraphicsApiFilter* this_ptr, Qt3DRender::QGraphicsApiFilter::OpenGLProfile profile) {
  this_ptr->setProfile(profile);
}

void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_setVendor(Qt3DRender::QGraphicsApiFilter* this_ptr, const QString* vendor) {
  this_ptr->setVendor(*vendor);
}

void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QGraphicsApiFilter::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QGraphicsApiFilter::tr(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_vendor_to_output(const Qt3DRender::QGraphicsApiFilter* this_ptr, QString* output) {
  new(output) QString(this_ptr->vendor());
}

