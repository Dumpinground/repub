use epub_builder::{EpubBuilder, Result, EpubContent, ReferenceType, TocElement, ZipLibrary};

fn main() {
    let _output = run().expect("Unable to create a epub document");
}

fn run() -> Result<Vec<u8>> {
    let dummy_content = "Dummy content. This should be valid XHTML if you want a valid EPUB!";
    let dummy_image = "Not really a PNG image";
    let dummy_css = "body { background-color: pink }";

    let mut output = Vec::<u8>::new();

    EpubBuilder::new(ZipLibrary::new()?)?
        .metadata("author", "Joan Doe")?
        .metadata("title", "Dummy Book")?
        .stylesheet(dummy_css.as_bytes())?
        .add_cover_image("cover.png", dummy_image.as_bytes(), "image/png")?
        .add_resource("some_image.png", dummy_image.as_bytes(), "image/png")?
        .add_content(
            EpubContent::new("cover.xhtml", dummy_content.as_bytes())
                .title("Cover")
                .reftype(ReferenceType::Cover),
        )?
        .add_content(
            EpubContent::new("title.xhtml", dummy_content.as_bytes())
                .title("Title")
                .reftype(ReferenceType::TitlePage),
        )?
        .add_content(
            EpubContent::new("chapter_1.xhtml", dummy_content.as_bytes())
                .title("Chapter 1")
                .reftype(ReferenceType::Text),
        )?
        .add_content(
            EpubContent::new("chapter_2.xhtml", dummy_content.as_bytes())
                .title("Chapter 2")
                .child(TocElement::new("chapter_2.xhtml#1", "Chapter 2, section 1")),
        )?
        .add_content(
            EpubContent::new("section.xhtml", dummy_content.as_bytes())
                .title("Chapter 2, section 2")
                .level(2),
        )?
        .add_content(EpubContent::new("notes.xhtml", dummy_content.as_bytes()))?
        .inline_toc()
        .generate(&mut output)?;

    Ok(output)
}
