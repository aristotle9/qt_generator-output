#ifndef QT_3D_RENDER_C_QGRAPHICSAPIFILTER_H
#define QT_3D_RENDER_C_QGRAPHICSAPIFILTER_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QGraphicsApiFilter_G_static_cast_QObject_ptr(Qt3DRender::QGraphicsApiFilter* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QGraphicsApiFilter* qt_3d_render_c_QGraphicsApiFilter_G_static_cast_Qt3DRender_QGraphicsApiFilter_ptr(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QGraphicsApiFilter::Api qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_api(const Qt3DRender::QGraphicsApiFilter* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_delete(Qt3DRender::QGraphicsApiFilter* this_ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_extensions_to_output(const Qt3DRender::QGraphicsApiFilter* this_ptr, QStringList* output);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_majorVersion(const Qt3DRender::QGraphicsApiFilter* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_metaObject(const Qt3DRender::QGraphicsApiFilter* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_minorVersion(const Qt3DRender::QGraphicsApiFilter* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QGraphicsApiFilter* qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QGraphicsApiFilter* qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_new_parent(QObject* parent);
QT_3D_RENDER_C_EXPORT Qt3DRender::QGraphicsApiFilter::OpenGLProfile qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_profile(const Qt3DRender::QGraphicsApiFilter* this_ptr);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_qt_metacall(Qt3DRender::QGraphicsApiFilter* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_qt_metacast(Qt3DRender::QGraphicsApiFilter* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_setApi(Qt3DRender::QGraphicsApiFilter* this_ptr, Qt3DRender::QGraphicsApiFilter::Api api);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_setExtensions(Qt3DRender::QGraphicsApiFilter* this_ptr, const QStringList* extensions);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_setMajorVersion(Qt3DRender::QGraphicsApiFilter* this_ptr, int majorVersion);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_setMinorVersion(Qt3DRender::QGraphicsApiFilter* this_ptr, int minorVersion);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_setProfile(Qt3DRender::QGraphicsApiFilter* this_ptr, Qt3DRender::QGraphicsApiFilter::OpenGLProfile profile);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_setVendor(Qt3DRender::QGraphicsApiFilter* this_ptr, const QString* vendor);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_vendor_to_output(const Qt3DRender::QGraphicsApiFilter* this_ptr, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QGRAPHICSAPIFILTER_H
