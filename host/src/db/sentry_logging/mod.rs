use crate::config::SENTRY;

pub struct Sentry<'a> {
    pub(crate) is_on: bool,
    pub(crate) key: &'a str,
}

pub fn enable_sentry() -> Option<sentry::ClientInitGuard> {
    if SENTRY.is_on {
        let guard = sentry::init((SENTRY.key, sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        }));
        return Some(guard);
    }
    None
}

