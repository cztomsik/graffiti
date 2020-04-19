// -- image

#define STB_IMAGE_IMPLEMENTATION

// mem only
#define STBI_NO_STDIO

// disable some formats
#define STBI_NO_HDR
#define STBI_NO_PSD
#define STBI_NO_PIC
#define STBI_NO_PNM

#include "stb_image.h"


// -- truetype

#define STB_TRUETYPE_IMPLEMENTATION

#include "stb_truetype.h"
