#ifndef QT_GUI_C_QPICTURE_H
#define QT_GUI_C_QPICTURE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QPicture* qt_gui_c_QPicture_G_dynamic_cast_QPicture_ptr(QPaintDevice* ptr);
QT_GUI_C_EXPORT QPaintDevice* qt_gui_c_QPicture_G_static_cast_QPaintDevice_ptr(QPicture* ptr);
QT_GUI_C_EXPORT QPicture* qt_gui_c_QPicture_G_static_cast_QPicture_ptr(QPaintDevice* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPicture_G_swap(QPicture* value1, QPicture* value2);
QT_GUI_C_EXPORT void qt_gui_c_QPicture_boundingRect_to_output(const QPicture* this_ptr, QRect* output);
QT_GUI_C_EXPORT const char* qt_gui_c_QPicture_data(const QPicture* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPicture_delete(QPicture* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPicture_detach(QPicture* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QPicture_devType(const QPicture* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPicture_inputFormatList_to_output(QStringList* output);
QT_GUI_C_EXPORT void qt_gui_c_QPicture_inputFormats_to_output(QList< QByteArray >* output);
QT_GUI_C_EXPORT bool qt_gui_c_QPicture_isDetached(const QPicture* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPicture_isNull(const QPicture* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPicture_load_dev(QPicture* this_ptr, QIODevice* dev);
QT_GUI_C_EXPORT bool qt_gui_c_QPicture_load_dev_format(QPicture* this_ptr, QIODevice* dev, const char* format);
QT_GUI_C_EXPORT bool qt_gui_c_QPicture_load_fileName(QPicture* this_ptr, const QString* fileName);
QT_GUI_C_EXPORT bool qt_gui_c_QPicture_load_fileName_format(QPicture* this_ptr, const QString* fileName, const char* format);
QT_GUI_C_EXPORT QPicture* qt_gui_c_QPicture_new_arg1(const QPicture* arg1);
QT_GUI_C_EXPORT QPicture* qt_gui_c_QPicture_new_formatVersion(int formatVersion);
QT_GUI_C_EXPORT QPicture* qt_gui_c_QPicture_new_no_args();
QT_GUI_C_EXPORT QPicture* qt_gui_c_QPicture_operator_assign(QPicture* this_ptr, const QPicture* p);
QT_GUI_C_EXPORT void qt_gui_c_QPicture_outputFormatList_to_output(QStringList* output);
QT_GUI_C_EXPORT void qt_gui_c_QPicture_outputFormats_to_output(QList< QByteArray >* output);
QT_GUI_C_EXPORT QPaintEngine* qt_gui_c_QPicture_paintEngine(const QPicture* this_ptr);
QT_GUI_C_EXPORT const char* qt_gui_c_QPicture_pictureFormat(const QString* fileName);
QT_GUI_C_EXPORT bool qt_gui_c_QPicture_play(QPicture* this_ptr, QPainter* p);
QT_GUI_C_EXPORT bool qt_gui_c_QPicture_save_dev(QPicture* this_ptr, QIODevice* dev);
QT_GUI_C_EXPORT bool qt_gui_c_QPicture_save_dev_format(QPicture* this_ptr, QIODevice* dev, const char* format);
QT_GUI_C_EXPORT bool qt_gui_c_QPicture_save_fileName(QPicture* this_ptr, const QString* fileName);
QT_GUI_C_EXPORT bool qt_gui_c_QPicture_save_fileName_format(QPicture* this_ptr, const QString* fileName, const char* format);
QT_GUI_C_EXPORT void qt_gui_c_QPicture_setBoundingRect(QPicture* this_ptr, const QRect* r);
QT_GUI_C_EXPORT void qt_gui_c_QPicture_setData(QPicture* this_ptr, const char* data, unsigned int size);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QPicture_size(const QPicture* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPicture_swap(QPicture* this_ptr, QPicture* other);

} // extern "C"

#endif // QT_GUI_C_QPICTURE_H
