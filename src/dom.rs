use crate::config::ActionStep;
use crate::jsparsing::inspect_dom_elements;
use crate::recorder::FrameRecorder;
use headless_chrome::Tab;
use std::thread;
use std::time::Duration;

pub fn execute_action_steps(
    tab: &Tab,
    steps: &[ActionStep],
    mut recorder: Option<&mut FrameRecorder>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Record initial state frame
    if let Some(ref mut rec) = recorder {
        let _ = rec.capture_frame(tab);
    }

    for (step_idx, step) in steps.iter().enumerate() {
        println!("   ↳ Action [{}/{}]: {:?}", step_idx + 1, steps.len(), step);

        match step {
            ActionStep::InspectDom => {
                inspect_dom_elements(tab)?;
            }
            ActionStep::Click { selector } => {
                let click_js = format!(
                    r#"
                    (() => {{
                        const el = document.querySelector("{}");
                        if (el) {{
                            el.click();
                            return true;
                        }}
                        return false;
                    }})()
                    "#,
                    selector
                );
                let res = tab.evaluate(&click_js, false)?;
                println!("     • Click result on `{}`: {:?}", selector, res.value);
            }
            ActionStep::ClickText { text } => {
                let click_text_js = format!(
                    r#"
                    (() => {{
                        const targetText = "{}".toLowerCase();
                        
                        const clickables = Array.from(document.querySelectorAll('button, a'));
                        let match = clickables.find(b => {{
                            const fullText = (b.innerText || b.textContent || '').trim().toLowerCase();
                            const lines = fullText.split('\n').map(s => s.trim());
                            return fullText === targetText || lines.includes(targetText);
                        }});

                        if (!match) {{
                            const allElements = Array.from(document.querySelectorAll('button, a, div, span, li, p')).reverse();
                            match = allElements.find(el => {{
                                const text = (el.innerText || el.textContent || '').trim().toLowerCase();
                                return el.children.length <= 2 && text === targetText;
                            }});
                        }}

                        if (match) {{
                            match.click();
                            return "CLICKED_TARGET: " + (match.innerText || match.textContent).trim();
                        }}
                        return "TEXT_NOT_FOUND: " + targetText;
                    }})()
                    "#,
                    text
                );
                let res = tab.evaluate(&click_text_js, false)?;
                println!("     • ClickText result: {:?}", res.value);
            }
            ActionStep::SetActiveTab { tab_id } => {
                let set_tab_js = format!(
                    r#"
                    (() => {{
                        localStorage.setItem("siabsen-active-tab", "{}");
                        window.location.reload();
                        return "SET_ACTIVE_TAB: {}";
                    }})()
                    "#,
                    tab_id, tab_id
                );
                let res = tab.evaluate(&set_tab_js, false)?;
                println!("     • SetActiveTab result: {:?}", res.value);
                tab.wait_until_navigated()?;
                thread::sleep(Duration::from_secs(1));
            }
            ActionStep::Type { selector, text } => {
                let type_js = format!(
                    r#"
                    (() => {{
                        const input = document.querySelector("{}");
                        if (input) {{
                            const nativeSetter = Object.getOwnPropertyDescriptor(window.HTMLInputElement.prototype, 'value')?.set;
                            if (nativeSetter) {{
                                nativeSetter.call(input, "{}");
                            }} else {{
                                input.value = "{}";
                            }}
                            input.dispatchEvent(new Event('input', {{ bubbles: true }}));
                            input.dispatchEvent(new Event('change', {{ bubbles: true }}));
                            return true;
                        }}
                        return false;
                    }})()
                    "#,
                    selector, text, text
                );
                let res = tab.evaluate(&type_js, false)?;
                println!("     • Type result on `{}`: {:?}", selector, res.value);
            }
            ActionStep::WaitMs { duration } => {
                println!("     • Sleeping for {} ms...", duration);
                thread::sleep(Duration::from_millis(*duration));
            }
            ActionStep::EvalJs { script } => {
                let res = tab.evaluate(script, false)?;
                println!("     • JS Eval result: {:?}", res.value);
            }
            ActionStep::HideElement { selector } => {
                let hide_js = format!(
                    r#"
                    (() => {{
                        const elements = document.querySelectorAll("{}");
                        elements.forEach(el => el.style.display = 'none');
                        return elements.length;
                    }})()
                    "#,
                    selector
                );
                let res = tab.evaluate(&hide_js, false)?;
                println!(
                    "     • Hidden {} elements matching `{}`",
                    res.value.unwrap_or_default(),
                    selector
                );
            }
        }

        // Record action post-state frame buffer
        if let Some(ref mut rec) = recorder {
            let _ = rec.capture_frame(tab);
        }
    }
    Ok(())
}
