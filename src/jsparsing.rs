use headless_chrome::Tab;

pub fn inspect_dom_elements(tab: &Tab) -> Result<(), Box<dyn std::error::Error>> {
    let inspect_js = r#"
        (() => {
            const elements = Array.from(document.querySelectorAll('button, input, a, form, select, textarea, [role="button"], h1, h2, h3'));
            return elements.map((el, index) => {
                let tag = el.tagName.toLowerCase();
                let id = el.id ? `#${el.id}` : '';
                let classes = el.className && typeof el.className === 'string' 
                    ? '.' + el.className.split(' ').filter(c => c.trim()).join('.') 
                    : '';
                let text = (el.innerText || el.textContent || '').trim().substring(0, 40);
                let placeholder = el.getAttribute('placeholder') || '';
                let name = el.getAttribute('name') || '';
                let type = el.getAttribute('type') || '';
                let ariaLabel = el.getAttribute('aria-label') || '';

                let suggestedSelector = id || (placeholder ? `[placeholder="${placeholder}"]` : '') || (ariaLabel ? `[aria-label="${ariaLabel}"]` : '') || (name ? `[name="${name}"]` : '') || (classes ? `${tag}${classes}` : tag);

                return {
                    index: index + 1,
                    tag,
                    id: el.id,
                    classes: el.className,
                    text,
                    placeholder,
                    ariaLabel,
                    type,
                    selector: suggestedSelector
                };
            });
        })()
    "#;

    println!("🔍 [DOM Inspector] Scanning interactive elements on current page...");
    let result = tab.evaluate(inspect_js, false)?;

    if let Some(json_val) = result.value {
        if let Some(array) = json_val.as_array() {
            println!("📑 Discovered {} interactive DOM elements:", array.len());
            for item in array.iter().take(20) {
                let tag = item["tag"].as_str().unwrap_or("");
                let text = item["text"].as_str().unwrap_or("");
                let selector = item["selector"].as_str().unwrap_or("");
                let placeholder = item["placeholder"].as_str().unwrap_or("");

                let detail = if !placeholder.is_empty() {
                    format!("(placeholder: '{}')", placeholder)
                } else if !text.is_empty() {
                    format!("(text: '{}')", text)
                } else {
                    "".to_string()
                };

                println!("   • <{}> -> Selector: `{}` {}", tag, selector, detail);
            }
            if array.len() > 20 {
                println!("   ... and {} more elements.", array.len() - 20);
            }
        }
    }

    Ok(())
}
