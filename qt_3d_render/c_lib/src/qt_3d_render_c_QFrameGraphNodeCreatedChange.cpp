#include "qt_3d_render_c_QFrameGraphNodeCreatedChange.h"

Qt3DCore::QNodeCreatedChangeBase* qt_3d_render_c_QFrameGraphNodeCreatedChange_G_static_cast_Qt3DCore_QNodeCreatedChangeBase_ptr(Qt3DRender::QFrameGraphNodeCreatedChangeBase* ptr) {
  return static_cast<Qt3DCore::QNodeCreatedChangeBase*>(ptr);
}

Qt3DCore::QSceneChange* qt_3d_render_c_QFrameGraphNodeCreatedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(Qt3DRender::QFrameGraphNodeCreatedChangeBase* ptr) {
  return static_cast<Qt3DCore::QSceneChange*>(ptr);
}

Qt3DRender::QFrameGraphNodeCreatedChangeBase* qt_3d_render_c_QFrameGraphNodeCreatedChange_G_static_cast_Qt3DRender_QFrameGraphNodeCreatedChangeBase_ptr_Qt3DCore_QNodeCreatedChangeBase(Qt3DCore::QNodeCreatedChangeBase* ptr) {
  return static_cast<Qt3DRender::QFrameGraphNodeCreatedChangeBase*>(ptr);
}

Qt3DRender::QFrameGraphNodeCreatedChangeBase* qt_3d_render_c_QFrameGraphNodeCreatedChange_G_static_cast_Qt3DRender_QFrameGraphNodeCreatedChangeBase_ptr_Qt3DCore_QSceneChange(Qt3DCore::QSceneChange* ptr) {
  return static_cast<Qt3DRender::QFrameGraphNodeCreatedChangeBase*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QFrameGraphNodeCreatedChangeBase_delete(Qt3DRender::QFrameGraphNodeCreatedChangeBase* this_ptr) {
  delete this_ptr;
}

Qt3DRender::QFrameGraphNodeCreatedChangeBase* qt_3d_render_c_Qt3DRender_QFrameGraphNodeCreatedChangeBase_new(const Qt3DRender::QFrameGraphNode* node) {
  return new Qt3DRender::QFrameGraphNodeCreatedChangeBase(node);
}

void qt_3d_render_c_Qt3DRender_QFrameGraphNodeCreatedChangeBase_parentFrameGraphNodeId_to_output(const Qt3DRender::QFrameGraphNodeCreatedChangeBase* this_ptr, Qt3DCore::QNodeId* output) {
  new(output) Qt3DCore::QNodeId(this_ptr->parentFrameGraphNodeId());
}

