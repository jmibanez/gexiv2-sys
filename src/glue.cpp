#include <gexiv2/gexiv2.h>
#include <glib-object.h>

extern "C" {

#ifndef gexiv2_metadata_free
  void gexiv2_metadata_free (GExiv2Metadata *self) {
    g_return_if_fail(GEXIV2_IS_METADATA(self));

    g_object_unref(self);
  }
#endif

#ifndef gexiv2_preview_image_free
  void gexiv2_preview_image_free(GExiv2PreviewImage *self) {
    g_return_if_fail(GEXIV2_IS_PREVIEW_IMAGE(self));

    g_object_unref(self);
  }
#endif

}
