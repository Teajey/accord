#[cfg(target_arch = "wasm32")]
pub mod node;

#[macro_export]
macro_rules! cross_print {
    ($($t:tt)*) => {
        #[cfg(not(target_arch = "wasm32"))]
        print!($($t)*);
        #[cfg(target_arch = "wasm32")]
        $crate::node_stdout!($($t)*);
    }
}

#[macro_export]
macro_rules! cross_eprint {
    ($($t:tt)*) => {
        #[cfg(not(target_arch = "wasm32"))]
        eprint!($($t)*);
        #[cfg(target_arch = "wasm32")]
        $crate::node_stderr!($($t)*);
    }
}

#[macro_export]
macro_rules! cross_println {
    ($($t:tt)*) => {
        #[cfg(not(target_arch = "wasm32"))]
        println!($($t)*);
        #[cfg(target_arch = "wasm32")]
        {
            $crate::node_stdout!($($t)*);
            $crate::node_stdout!("\n");
        }
    }
}

#[macro_export]
macro_rules! cross_eprintln {
    ($($t:tt)*) => {
        #[cfg(not(target_arch = "wasm32"))]
        eprintln!($($t)*);
        #[cfg(target_arch = "wasm32")]
        {
            $crate::node_stderr!($($t)*);
            $crate::node_stderr!("\n");
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn path_to_string<P: AsRef<std::path::Path>>(path: P) -> eyre::Result<String> {
    Ok(path
        .as_ref()
        .to_str()
        .ok_or_else(|| eyre::eyre!("set_current_dir: Couldn't convert provided path to UTF-8"))?
        .to_owned())
}

pub mod env {
    use std::{ffi::OsString, path::Path};

    use eyre::Result;

    pub fn set_current_dir<P: AsRef<Path>>(path: P) -> Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(std::env::set_current_dir(path)?)
        }
        #[cfg(target_arch = "wasm32")]
        {
            super::node::process_chdir(&super::path_to_string(path)?)
                .map_err(|err| eyre::eyre!("set_current_dir: {:?}", err))
        }
    }

    pub fn var(key: &str) -> Result<String, std::env::VarError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::env::var(key)
        }
        #[cfg(target_arch = "wasm32")]
        {
            let env = super::node::process_env();

            env.get(key).cloned().ok_or(std::env::VarError::NotPresent)
        }
    }

    pub fn argv() -> impl Iterator<Item = Result<OsString>> {
        #[cfg(target_arch = "wasm32")]
        {
            super::node::argv()
                .into_iter()
                .map(|arg| {
                    arg.as_string()
                        .ok_or(arg)
                        .map_err(|arg| eyre::eyre!("Failed to stringify arg: {arg:?}"))
                        .map(OsString::from)
                })
                .skip(1)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::env::args_os().map(Ok)
        }
    }
}

pub mod fs {
    use std::path::Path;

    use eyre::Result;

    pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(std::fs::read_to_string(path)?)
        }
        #[cfg(target_arch = "wasm32")]
        {
            super::node::read_file_to_string(&super::path_to_string(path)?)
                .map_err(|err| eyre::eyre!("{err:?}"))
        }
    }

    pub fn write_to_file(path: &str, data: &str) -> Result<()> {
        #[cfg(target_arch = "wasm32")]
        {
            super::node::write_file(path, data).map_err(|err| eyre::eyre!("{:?}", err))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::io::Write;

            Ok(write!(std::fs::File::create(path)?, "{data}")?)
        }
    }

    // pub fn try_path_exists<P: AsRef<Path>>(path: P) -> Result<bool> {
    //     #[cfg(not(target_arch = "wasm32"))]
    //     {
    //         path.try_exists()
    //     }
    //     #[cfg(target_arch = "wasm32")]
    //     {
    //         super::node::path_try_exists(&super::path_to_string(path)?)
    //             .map_err(|err| eyre!("{:?}", err))
    //     }
    // }
}

pub mod net {
    use eyre::Result;
    use serde::Serialize;

    pub async fn fetch_json<B: Serialize>(
        url: &str,
        no_ssl: bool,
        body: B,
    ) -> Result<serde_json::Value> {
        #[cfg(target_arch = "wasm32")]
        {
            use eyre::eyre;

            let body_str = serde_json::to_string(&body)?;

            let options = serde_json::json!({
                "method": "POST",
                "body": body_str,
                "headers": {
                    "Accept": "application/json",
                    "Content-Type": "application/json"
                }
            });

            let res = super::node::fetch_json(
                url,
                no_ssl,
                serde_wasm_bindgen::to_value(&options)
                    .map_err(|err| eyre!("Couldn't deserialize json into JsValue: {err}"))?,
            )
            .await
            .map_err(|err| eyre!("{:?}", err))?;

            serde_wasm_bindgen::from_value(res).map_err(|err| eyre!("{:?}", err))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let client = reqwest::Client::builder()
                .danger_accept_invalid_certs(no_ssl)
                .build()?;

            let res = client.post(url).json(&body).send().await?;

            Ok(res.json().await?)
        }
    }
}

pub mod process {
    pub fn exit(code: i32) -> ! {
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::process::exit(code);
        }
        #[cfg(target_arch = "wasm32")]
        {
            super::node::process_exit(code);
            unreachable!();
        }
    }
}
