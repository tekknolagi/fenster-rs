#include "fenster.h"
#include <stdio.h>

FENSTER_API struct fenster *fenster_create(const char *title, const int width, const int height, uint32_t *buf)
{
    struct fenster *f = malloc(sizeof(struct fenster));
    if (!f)
    {
        return NULL;
    }
    f->title = title;
    *(int *)&f->width = width;   // :C
    *(int *)&f->height = height; // :C
    f->buf = buf;
    return f;
}

FENSTER_API void fenster_destroy(struct fenster **f)
{
    free(*f);
    *f = NULL;
}
