#ifndef QT_GUI_C_QPIXMAPCACHE_H
#define QT_GUI_C_QPIXMAPCACHE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QPixmapCache_G_swap(QPixmapCache::Key* value1, QPixmapCache::Key* value2);
QT_GUI_C_EXPORT void qt_gui_c_QPixmapCache_Key_constructor_no_args(QPixmapCache::Key* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixmapCache_Key_constructor_other(const QPixmapCache::Key* other, QPixmapCache::Key* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixmapCache_Key_destructor(QPixmapCache::Key* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmapCache_Key_isValid(const QPixmapCache::Key* this_ptr);
QT_GUI_C_EXPORT QPixmapCache::Key* qt_gui_c_QPixmapCache_Key_operator_assign(QPixmapCache::Key* this_ptr, const QPixmapCache::Key* other);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmapCache_Key_operator_eq(const QPixmapCache::Key* this_ptr, const QPixmapCache::Key* key);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmapCache_Key_operator_neq(const QPixmapCache::Key* this_ptr, const QPixmapCache::Key* key);
QT_GUI_C_EXPORT void qt_gui_c_QPixmapCache_Key_swap(QPixmapCache::Key* this_ptr, QPixmapCache::Key* other);
QT_GUI_C_EXPORT int qt_gui_c_QPixmapCache_cacheLimit();
QT_GUI_C_EXPORT void qt_gui_c_QPixmapCache_clear();
QT_GUI_C_EXPORT void qt_gui_c_QPixmapCache_delete(QPixmapCache* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmapCache_find_const_QPixmapCache_Key_ref_QPixmap_ptr(const QPixmapCache::Key* key, QPixmap* pixmap);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QPixmapCache_find_const_QString_ref(const QString* key);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmapCache_find_const_QString_ref_QPixmap_ptr(const QString* key, QPixmap* pixmap);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmapCache_find_const_QString_ref_QPixmap_ref(const QString* key, QPixmap* pixmap);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmapCache_insert(const QString* key, const QPixmap* pixmap);
QT_GUI_C_EXPORT void qt_gui_c_QPixmapCache_insert_to_output(const QPixmap* pixmap, QPixmapCache::Key* output);
QT_GUI_C_EXPORT void qt_gui_c_QPixmapCache_remove_QPixmapCache_Key(const QPixmapCache::Key* key);
QT_GUI_C_EXPORT void qt_gui_c_QPixmapCache_remove_QString(const QString* key);
QT_GUI_C_EXPORT bool qt_gui_c_QPixmapCache_replace(const QPixmapCache::Key* key, const QPixmap* pixmap);
QT_GUI_C_EXPORT void qt_gui_c_QPixmapCache_setCacheLimit(int arg1);

} // extern "C"

#endif // QT_GUI_C_QPIXMAPCACHE_H
