use grass::OutputStyle;

/// Compiles an SCSS file using the `grass` compiler. Defaults to minified.
///
/// If it fails to compile the function checks the build mode. If in debug mode, the error message
/// is returned in CSS form. If in release, an empty string will be returned.
pub fn compile_scss(path: &str) -> String {
    match grass::from_path(path, &grass::Options::default().style(OutputStyle::Compressed)) {
        // Compiled CSS on success
        Ok(css) => css,

        // Empty of debug CSS on error
        Err(err) => {
            // Report error in console and optionally return a debug stylesheet with the error
            let kind = err.kind();
            eprintln!("{:?}", kind);

            // Debug build
            if cfg!(debug_assertions) {
                let error = format!("{:?}", kind).replace('\'', "\\'");
                // CSS magic to display hide page content and display the error message
                // ```css
                // body { display: none; }
                // html::before { content: 'error message here...' }
                // ```
                String::from(format!("body{{display:none}}html::before{{content:'{}'}}", error))
            }

            // Release build
            else {
                String::default()
            }
        }
    }
}

// test
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compile_scss() {
        // Test with a valid path
        let css = vec![
            compile_scss("src/sass/blog.scss"),
            compile_scss("src/sass/error.scss"),
            compile_scss("src/sass/index.scss"),
            compile_scss("src/sass/privacy.scss"),
            compile_scss("src/sass/read.scss"),
        ];

        for doc in css {
            assert!(doc.len() > 0);
        }

        // Test with an invalid path
        let css = compile_scss("sass/doesnotexist.scss");
        assert!(css.is_empty());
    }
}

