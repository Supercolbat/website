use grass::OutputStyle;

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
