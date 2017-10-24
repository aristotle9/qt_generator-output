#ifndef QT_3D_RENDER_C_QSORTPOLICY_H
#define QT_3D_RENDER_C_QSORTPOLICY_H

#include "qt_3d_render_c_global.h"

extern "C" {

QT_3D_RENDER_C_EXPORT Qt3DRender::QSortPolicy* qt_3d_render_c_QSortPolicy_G_dynamic_cast_Qt3DRender_QSortPolicy_ptr(Qt3DRender::QFrameGraphNode* ptr);
QT_3D_RENDER_C_EXPORT QObject* qt_3d_render_c_QSortPolicy_G_static_cast_QObject_ptr(Qt3DRender::QSortPolicy* ptr);
QT_3D_RENDER_C_EXPORT Qt3DCore::QNode* qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QSortPolicy* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QFrameGraphNode* qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DRender::QSortPolicy* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QSortPolicy* qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DRender_QSortPolicy_ptr_QObject(QObject* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QSortPolicy* qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DRender_QSortPolicy_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QSortPolicy* qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DRender_QSortPolicy_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSortPolicy_delete(Qt3DRender::QSortPolicy* this_ptr);
QT_3D_RENDER_C_EXPORT const QMetaObject* qt_3d_render_c_Qt3DRender_QSortPolicy_metaObject(const Qt3DRender::QSortPolicy* this_ptr);
QT_3D_RENDER_C_EXPORT Qt3DRender::QSortPolicy* qt_3d_render_c_Qt3DRender_QSortPolicy_new_no_args();
QT_3D_RENDER_C_EXPORT Qt3DRender::QSortPolicy* qt_3d_render_c_Qt3DRender_QSortPolicy_new_parent(Qt3DCore::QNode* parent);
QT_3D_RENDER_C_EXPORT int qt_3d_render_c_Qt3DRender_QSortPolicy_qt_metacall(Qt3DRender::QSortPolicy* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_3D_RENDER_C_EXPORT void* qt_3d_render_c_Qt3DRender_QSortPolicy_qt_metacast(Qt3DRender::QSortPolicy* this_ptr, const char* arg1);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSortPolicy_setSortTypes_sortTypes(Qt3DRender::QSortPolicy* this_ptr, const QVector< Qt3DRender::QSortPolicy::SortType >* sortTypes);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSortPolicy_setSortTypes_sortTypesInt(Qt3DRender::QSortPolicy* this_ptr, const QVector< int >* sortTypesInt);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSortPolicy_sortTypesInt_to_output(const Qt3DRender::QSortPolicy* this_ptr, QVector< int >* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSortPolicy_sortTypes_to_output(const Qt3DRender::QSortPolicy* this_ptr, QVector< Qt3DRender::QSortPolicy::SortType >* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSortPolicy_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_3D_RENDER_C_EXPORT void qt_3d_render_c_Qt3DRender_QSortPolicy_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_3D_RENDER_C_QSORTPOLICY_H
