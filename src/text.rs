extern crate image;
use image::RgbaImage;

extern crate freetype;
use freetype::Bitmap;

/// An object that can be used to generate bitmaps of text
pub struct TextGenerator {
    face: freetype::Face,
}

impl TextGenerator {
    /// Returns a new text generator with the given font face
    /// # Arguments
    /// * `path` - A string that holds the file path of the font face to use
    /// * `index` - Face index to load
    pub fn new_from_font(path: &str, index: isize) -> TextGenerator {
        // Create a freetype library
        let ft_library = freetype::Library::init().unwrap();

        // Create a font face from file
        let face = ft_library.new_face(path, index).unwrap();

        // Configure the character size
        face.set_char_size(5000, 0, 50, 0);

        TextGenerator { face }
    }

    /// Generates an image of the text using this objects font
    /// # Arguments
    /// * `text` - The text to render into the image
    pub fn generate(text: &str) -> RgbaImage {
        RgbaImage {}
    }

    /// Creates a bitmap of a character from the font face
    /// # Arguments
    /// * `character` - The character to get the bitmap for
    fn get_glyph(&self, character: char) -> Bitmap {
        // Load the character
        self.face
            .load_char(character as usize, freetype::face::LoadFlag::RENDER);

        // Return the glyph bitmap
        self.face.glyph().bitmap()
    }
}
