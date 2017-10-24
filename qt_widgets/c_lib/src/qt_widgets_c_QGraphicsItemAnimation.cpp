#include "qt_widgets_c_QGraphicsItemAnimation.h"

QGraphicsItemAnimation* qt_widgets_c_QGraphicsItemAnimation_G_static_cast_QGraphicsItemAnimation_ptr(QObject* ptr) {
  return static_cast<QGraphicsItemAnimation*>(ptr);
}

QObject* qt_widgets_c_QGraphicsItemAnimation_G_static_cast_QObject_ptr(QGraphicsItemAnimation* ptr) {
  return static_cast<QObject*>(ptr);
}

void qt_widgets_c_QGraphicsItemAnimation_clear(QGraphicsItemAnimation* this_ptr) {
  this_ptr->clear();
}

void qt_widgets_c_QGraphicsItemAnimation_delete(QGraphicsItemAnimation* this_ptr) {
  delete this_ptr;
}

double qt_widgets_c_QGraphicsItemAnimation_horizontalScaleAt(const QGraphicsItemAnimation* this_ptr, double step) {
  return this_ptr->horizontalScaleAt(step);
}

double qt_widgets_c_QGraphicsItemAnimation_horizontalShearAt(const QGraphicsItemAnimation* this_ptr, double step) {
  return this_ptr->horizontalShearAt(step);
}

QGraphicsItem* qt_widgets_c_QGraphicsItemAnimation_item(const QGraphicsItemAnimation* this_ptr) {
  return this_ptr->item();
}

void qt_widgets_c_QGraphicsItemAnimation_matrixAt_to_output(const QGraphicsItemAnimation* this_ptr, double step, QMatrix* output) {
  new(output) QMatrix(this_ptr->matrixAt(step));
}

const QMetaObject* qt_widgets_c_QGraphicsItemAnimation_metaObject(const QGraphicsItemAnimation* this_ptr) {
  return this_ptr->metaObject();
}

QGraphicsItemAnimation* qt_widgets_c_QGraphicsItemAnimation_new_no_args() {
  return new QGraphicsItemAnimation();
}

QGraphicsItemAnimation* qt_widgets_c_QGraphicsItemAnimation_new_parent(QObject* parent) {
  return new QGraphicsItemAnimation(parent);
}

void qt_widgets_c_QGraphicsItemAnimation_posAt_to_output(const QGraphicsItemAnimation* this_ptr, double step, QPointF* output) {
  new(output) QPointF(this_ptr->posAt(step));
}

void qt_widgets_c_QGraphicsItemAnimation_posList_to_output(const QGraphicsItemAnimation* this_ptr, QList< QPair< double, QPointF > >* output) {
  new(output) QList< QPair< double, QPointF > >(this_ptr->posList());
}

int qt_widgets_c_QGraphicsItemAnimation_qt_metacall(QGraphicsItemAnimation* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QGraphicsItemAnimation_qt_metacast(QGraphicsItemAnimation* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QGraphicsItemAnimation_reset(QGraphicsItemAnimation* this_ptr) {
  this_ptr->reset();
}

double qt_widgets_c_QGraphicsItemAnimation_rotationAt(const QGraphicsItemAnimation* this_ptr, double step) {
  return this_ptr->rotationAt(step);
}

void qt_widgets_c_QGraphicsItemAnimation_rotationList_to_output(const QGraphicsItemAnimation* this_ptr, QList< QPair< double, double > >* output) {
  new(output) QList< QPair< double, double > >(this_ptr->rotationList());
}

void qt_widgets_c_QGraphicsItemAnimation_scaleList_to_output(const QGraphicsItemAnimation* this_ptr, QList< QPair< double, QPointF > >* output) {
  new(output) QList< QPair< double, QPointF > >(this_ptr->scaleList());
}

void qt_widgets_c_QGraphicsItemAnimation_setItem(QGraphicsItemAnimation* this_ptr, QGraphicsItem* item) {
  this_ptr->setItem(item);
}

void qt_widgets_c_QGraphicsItemAnimation_setPosAt(QGraphicsItemAnimation* this_ptr, double step, const QPointF* pos) {
  this_ptr->setPosAt(step, *pos);
}

void qt_widgets_c_QGraphicsItemAnimation_setRotationAt(QGraphicsItemAnimation* this_ptr, double step, double angle) {
  this_ptr->setRotationAt(step, angle);
}

void qt_widgets_c_QGraphicsItemAnimation_setScaleAt(QGraphicsItemAnimation* this_ptr, double step, double sx, double sy) {
  this_ptr->setScaleAt(step, sx, sy);
}

void qt_widgets_c_QGraphicsItemAnimation_setShearAt(QGraphicsItemAnimation* this_ptr, double step, double sh, double sv) {
  this_ptr->setShearAt(step, sh, sv);
}

void qt_widgets_c_QGraphicsItemAnimation_setStep(QGraphicsItemAnimation* this_ptr, double x) {
  this_ptr->setStep(x);
}

void qt_widgets_c_QGraphicsItemAnimation_setTimeLine(QGraphicsItemAnimation* this_ptr, QTimeLine* timeLine) {
  this_ptr->setTimeLine(timeLine);
}

void qt_widgets_c_QGraphicsItemAnimation_setTranslationAt(QGraphicsItemAnimation* this_ptr, double step, double dx, double dy) {
  this_ptr->setTranslationAt(step, dx, dy);
}

void qt_widgets_c_QGraphicsItemAnimation_shearList_to_output(const QGraphicsItemAnimation* this_ptr, QList< QPair< double, QPointF > >* output) {
  new(output) QList< QPair< double, QPointF > >(this_ptr->shearList());
}

QTimeLine* qt_widgets_c_QGraphicsItemAnimation_timeLine(const QGraphicsItemAnimation* this_ptr) {
  return this_ptr->timeLine();
}

void qt_widgets_c_QGraphicsItemAnimation_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsItemAnimation::trUtf8(s, c, n));
}

void qt_widgets_c_QGraphicsItemAnimation_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QGraphicsItemAnimation::tr(s, c, n));
}

void qt_widgets_c_QGraphicsItemAnimation_translationList_to_output(const QGraphicsItemAnimation* this_ptr, QList< QPair< double, QPointF > >* output) {
  new(output) QList< QPair< double, QPointF > >(this_ptr->translationList());
}

double qt_widgets_c_QGraphicsItemAnimation_verticalScaleAt(const QGraphicsItemAnimation* this_ptr, double step) {
  return this_ptr->verticalScaleAt(step);
}

double qt_widgets_c_QGraphicsItemAnimation_verticalShearAt(const QGraphicsItemAnimation* this_ptr, double step) {
  return this_ptr->verticalShearAt(step);
}

double qt_widgets_c_QGraphicsItemAnimation_xTranslationAt(const QGraphicsItemAnimation* this_ptr, double step) {
  return this_ptr->xTranslationAt(step);
}

double qt_widgets_c_QGraphicsItemAnimation_yTranslationAt(const QGraphicsItemAnimation* this_ptr, double step) {
  return this_ptr->yTranslationAt(step);
}

