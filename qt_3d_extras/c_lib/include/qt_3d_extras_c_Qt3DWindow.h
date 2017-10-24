#ifndef QT_3D_EXTRAS_C_QT3DWINDOW_H
#define QT_3D_EXTRAS_C_QT3DWINDOW_H

#include "qt_3d_extras_c_global.h"

extern "C" {

QT_3D_EXTRAS_C_EXPORT Qt3DRender::QFrameGraphNode* qt_3d_extras_c_Qt3DExtras_Qt3DWindow_activeFrameGraph(const Qt3DExtras::Qt3DWindow* this_ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DRender::QCamera* qt_3d_extras_c_Qt3DExtras_Qt3DWindow_camera(const Qt3DExtras::Qt3DWindow* this_ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DExtras::QForwardRenderer* qt_3d_extras_c_Qt3DExtras_Qt3DWindow_defaultFrameGraph(const Qt3DExtras::Qt3DWindow* this_ptr);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_Qt3DWindow_delete(Qt3DExtras::Qt3DWindow* this_ptr);
QT_3D_EXTRAS_C_EXPORT const QMetaObject* qt_3d_extras_c_Qt3DExtras_Qt3DWindow_metaObject(const Qt3DExtras::Qt3DWindow* this_ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DExtras::Qt3DWindow* qt_3d_extras_c_Qt3DExtras_Qt3DWindow_new_no_args();
QT_3D_EXTRAS_C_EXPORT Qt3DExtras::Qt3DWindow* qt_3d_extras_c_Qt3DExtras_Qt3DWindow_new_screen(QScreen* screen);
QT_3D_EXTRAS_C_EXPORT int qt_3d_extras_c_Qt3DExtras_Qt3DWindow_qt_metacall(Qt3DExtras::Qt3DWindow* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_EXTRAS_C_EXPORT void* qt_3d_extras_c_Qt3DExtras_Qt3DWindow_qt_metacast(Qt3DExtras::Qt3DWindow* this_ptr, const char* arg1);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_Qt3DWindow_registerAspect_aspect(Qt3DExtras::Qt3DWindow* this_ptr, Qt3DCore::QAbstractAspect* aspect);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_Qt3DWindow_registerAspect_name(Qt3DExtras::Qt3DWindow* this_ptr, const QString* name);
QT_3D_EXTRAS_C_EXPORT Qt3DRender::QRenderSettings* qt_3d_extras_c_Qt3DExtras_Qt3DWindow_renderSettings(const Qt3DExtras::Qt3DWindow* this_ptr);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_Qt3DWindow_setActiveFrameGraph(Qt3DExtras::Qt3DWindow* this_ptr, Qt3DRender::QFrameGraphNode* activeFrameGraph);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_Qt3DWindow_setRootEntity(Qt3DExtras::Qt3DWindow* this_ptr, Qt3DCore::QEntity* root);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_Qt3DWindow_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_EXTRAS_C_EXPORT void qt_3d_extras_c_Qt3DExtras_Qt3DWindow_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_EXTRAS_C_EXPORT QObject* qt_3d_extras_c_Qt3DWindow_G_static_cast_QObject_ptr(Qt3DExtras::Qt3DWindow* ptr);
QT_3D_EXTRAS_C_EXPORT QSurface* qt_3d_extras_c_Qt3DWindow_G_static_cast_QSurface_ptr(Qt3DExtras::Qt3DWindow* ptr);
QT_3D_EXTRAS_C_EXPORT QWindow* qt_3d_extras_c_Qt3DWindow_G_static_cast_QWindow_ptr(Qt3DExtras::Qt3DWindow* ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DExtras::Qt3DWindow* qt_3d_extras_c_Qt3DWindow_G_static_cast_Qt3DExtras_Qt3DWindow_ptr_QObject(QObject* ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DExtras::Qt3DWindow* qt_3d_extras_c_Qt3DWindow_G_static_cast_Qt3DExtras_Qt3DWindow_ptr_QSurface(QSurface* ptr);
QT_3D_EXTRAS_C_EXPORT Qt3DExtras::Qt3DWindow* qt_3d_extras_c_Qt3DWindow_G_static_cast_Qt3DExtras_Qt3DWindow_ptr_QWindow(QWindow* ptr);

} // extern "C"

#endif // QT_3D_EXTRAS_C_QT3DWINDOW_H
