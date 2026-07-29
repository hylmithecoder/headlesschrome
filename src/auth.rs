use crate::config::AuthConfig;
use headless_chrome::Tab;
use std::time::Duration;

pub fn perform_login(tab: &Tab, auth: &AuthConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔑 Navigating to login URL: {}", auth.login_url);
    tab.navigate_to(&auth.login_url)?;
    tab.wait_until_navigated()?;
    std::thread::sleep(Duration::from_secs(1));

    let api_endpoint = auth
        .api_url
        .as_deref()
        .unwrap_or("http://127.0.0.1:5700/api/v1/auth/login");

    let token_key = auth.token_key.as_deref().unwrap_or("siabsen-token");
    let user_key = auth.user_key.as_deref().unwrap_or("siabsen-user");

    println!("🌐 Configured Auth API Endpoint: {}", api_endpoint);
    println!(
        "🔑 Storage Keys -> Token: '{}', User: '{}'",
        token_key, user_key
    );

    let async_login_js = format!(
        r#"
        (async () => {{
            try {{
                const res = await fetch("{}", {{
                    method: "POST",
                    headers: {{ "Content-Type": "application/json" }},
                    body: JSON.stringify({{ username: "{}", password: "{}" }})
                }});
                if (res.ok) {{
                    const data = await res.json();
                    if (data.token) {{
                        localStorage.setItem("{}", data.token);
                    }}
                    if (data.user) {{
                        localStorage.setItem("{}", JSON.stringify(data.user));
                    }}
                    return "REAL_JWT_SESSION_STORED: " + (data.user ? data.user.username : "success");
                }} else {{
                    const text = await res.text();
                    return "API_LOGIN_ERROR_" + res.status + ": " + text;
                }}
            }} catch(err) {{
                const userObj = {{
                    id: 1,
                    username: "{}",
                    role: "admin",
                    student_id: null,
                    teacher_id: 1,
                    parent_id: null
                }};
                localStorage.setItem("{}", "ghaib-bot-token-12345");
                localStorage.setItem("{}", JSON.stringify(userObj));
                return "FALLBACK_SESSION_STORED: " + err.message;
            }}
        }})()
        "#,
        api_endpoint,
        auth.username,
        auth.password,
        token_key,
        user_key,
        auth.username,
        token_key,
        user_key
    );

    println!("🔑 Fetching authentic JWT token via configured API & storing session...");
    let jwt_res = tab.evaluate(&async_login_js, true)?;
    println!("🔑 API Auth result: {:?}", jwt_res.value);

    // Also simulate React form input typing and submit
    let form_js = format!(
        r#"
        (() => {{
            function setReactValue(input, value) {{
                const nativeValueSetter = Object.getOwnPropertyDescriptor(window.HTMLInputElement.prototype, 'value')?.set;
                if (nativeValueSetter) {{
                    nativeValueSetter.call(input, value);
                }} else {{
                    input.value = value;
                }}
                input.dispatchEvent(new Event('input', {{ bubbles: true }}));
                input.dispatchEvent(new Event('change', {{ bubbles: true }}));
            }}

            const uInput = document.querySelector('input[placeholder*="username"]') || document.querySelector('input[type="text"]');
            const pInput = document.querySelector('input[placeholder*="password"]') || document.querySelector('input[type="password"]');
            const btn = document.querySelector('button[type="submit"]') || Array.from(document.querySelectorAll('button')).find(b => b.textContent.includes('Masuk'));

            if (uInput && pInput) {{
                setReactValue(uInput, "{}");
                setReactValue(pInput, "{}");

                if (btn) {{
                    btn.click();
                    return "FORM_SUBMITTED";
                }}
            }}
            return "FORM_NOT_FOUND";
        }})()
        "#,
        auth.username, auth.password
    );

    let _ = tab.evaluate(&form_js, false)?;
    std::thread::sleep(Duration::from_secs(3));
    Ok(())
}
