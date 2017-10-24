#include "qt_3d_render_c_QSortPolicy.h"

Qt3DRender::QSortPolicy* qt_3d_render_c_QSortPolicy_G_dynamic_cast_Qt3DRender_QSortPolicy_ptr(Qt3DRender::QFrameGraphNode* ptr) {
  return dynamic_cast<Qt3DRender::QSortPolicy*>(ptr);
}

QObject* qt_3d_render_c_QSortPolicy_G_static_cast_QObject_ptr(Qt3DRender::QSortPolicy* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QSortPolicy* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DRender::QSortPolicy* ptr) {
  return static_cast<Qt3DRender::QFrameGraphNode*>(ptr);
}

Qt3DRender::QSortPolicy* qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DRender_QSortPolicy_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QSortPolicy*>(ptr);
}

Qt3DRender::QSortPolicy* qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DRender_QSortPolicy_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QSortPolicy*>(ptr);
}

Qt3DRender::QSortPolicy* qt_3d_render_c_QSortPolicy_G_static_cast_Qt3DRender_QSortPolicy_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr) {
  return static_cast<Qt3DRender::QSortPolicy*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QSortPolicy_delete(Qt3DRender::QSortPolicy* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QSortPolicy_metaObject(const Qt3DRender::QSortPolicy* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QSortPolicy* qt_3d_render_c_Qt3DRender_QSortPolicy_new_no_args() {
  return new Qt3DRender::QSortPolicy();
}

Qt3DRender::QSortPolicy* qt_3d_render_c_Qt3DRender_QSortPolicy_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QSortPolicy(parent);
}

int qt_3d_render_c_Qt3DRender_QSortPolicy_qt_metacall(Qt3DRender::QSortPolicy* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QSortPolicy_qt_metacast(Qt3DRender::QSortPolicy* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QSortPolicy_setSortTypes_sortTypes(Qt3DRender::QSortPolicy* this_ptr, const QVector< Qt3DRender::QSortPolicy::SortType >* sortTypes) {
  this_ptr->setSortTypes(*sortTypes);
}

void qt_3d_render_c_Qt3DRender_QSortPolicy_setSortTypes_sortTypesInt(Qt3DRender::QSortPolicy* this_ptr, const QVector< int >* sortTypesInt) {
  this_ptr->setSortTypes(*sortTypesInt);
}

void qt_3d_render_c_Qt3DRender_QSortPolicy_sortTypesInt_to_output(const Qt3DRender::QSortPolicy* this_ptr, QVector< int >* output) {
  new(output) QVector< int >(this_ptr->sortTypesInt());
}

void qt_3d_render_c_Qt3DRender_QSortPolicy_sortTypes_to_output(const Qt3DRender::QSortPolicy* this_ptr, QVector< Qt3DRender::QSortPolicy::SortType >* output) {
  new(output) QVector< Qt3DRender::QSortPolicy::SortType >(this_ptr->sortTypes());
}

void qt_3d_render_c_Qt3DRender_QSortPolicy_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QSortPolicy::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QSortPolicy_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QSortPolicy::tr(s, c, n));
}

